use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, fs, io, path::{Path, PathBuf}};

const MANIFEST: &str = "opaml.toml";
const LOCKFILE: &str = "opaml.lock";

#[derive(Parser)]
#[command(name = "opaml", version, about = "Safety-first package manager for Liquid ecosystem projects")]
struct Cli { #[command(subcommand)] command: Command }

#[derive(Subcommand)]
enum Command {
    /// Create a project in a new or existing directory.
    Init { name: String },
    /// Add a package requirement to opaml.toml.
    Add { name: String, version: String },
    /// Remove a package requirement from opaml.toml.
    Remove { name: String },
    /// Print declared dependencies.
    List,
    /// Install packages from the local registry and write opaml.lock.
    Install,
    /// Verify installed files against opaml.lock.
    Verify,
}

#[derive(Debug, Serialize, Deserialize)]
struct Project {
    package: PackageMeta,
    #[serde(default)]
    dependencies: BTreeMap<String, String>,
}
#[derive(Debug, Serialize, Deserialize)]
struct PackageMeta { name: String, version: String, edition: String }

#[derive(Debug, Serialize, Deserialize)]
struct Lockfile { lock_version: u32, packages: Vec<LockedPackage> }
#[derive(Debug, Serialize, Deserialize)]
struct LockedPackage {
    name: String,
    version: String,
    sha256: String,
    source: String,
}

fn main() {
    if let Err(e) = run() {
        eprintln!("opaml: error: {e}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    match Cli::parse().command {
        Command::Init { name } => init(&name),
        Command::Add { name, version } => mutate_dependency(&name, Some(&version)),
        Command::Remove { name } => mutate_dependency(&name, None),
        Command::List => list_dependencies(),
        Command::Install => install(),
        Command::Verify => verify(),
    }
}

fn valid_name(name: &str) -> bool {
    !name.is_empty() && name.len() <= 100 &&
        name.chars().all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
}

fn init(name: &str) -> Result<(), Box<dyn std::error::Error>> {
    if !valid_name(name) { return Err("project name may contain only letters, numbers, '-' and '_'".into()); }
    let dir = Path::new(name);
    fs::create_dir_all(dir)?;
    let manifest = dir.join(MANIFEST);
    if manifest.exists() { return Err(format!("{} already exists", manifest.display()).into()); }
    let project = Project {
        package: PackageMeta { name: name.to_owned(), version: "0.1.0".into(), edition: "2021".into() },
        dependencies: BTreeMap::new(),
    };
    fs::write(&manifest, toml::to_string_pretty(&project)?)?;
    println!("Created {}/{}", name, MANIFEST);
    Ok(())
}

fn load_project() -> Result<Project, Box<dyn std::error::Error>> {
    let raw = fs::read_to_string(MANIFEST)
        .map_err(|e| io::Error::new(e.kind(), format!("cannot read {MANIFEST}: {e}; run opaml init first")))?;
    Ok(toml::from_str(&raw)?)
}
fn save_project(project: &Project) -> Result<(), Box<dyn std::error::Error>> {
    fs::write(MANIFEST, toml::to_string_pretty(project)?)?;
    Ok(())
}

fn mutate_dependency(name: &str, version: Option<&str>) -> Result<(), Box<dyn std::error::Error>> {
    if !valid_name(name) { return Err("invalid package name".into()); }
    let mut project = load_project()?;
    match version {
        Some(v) if v.trim().is_empty() || v.contains('/') || v.contains('\\') => return Err("invalid version constraint".into()),
        Some(v) => { project.dependencies.insert(name.to_owned(), v.to_owned()); println!("Added {name} {v}"); }
        None => { if project.dependencies.remove(name).is_some() { println!("Removed {name}"); } else { println!("{name} was not declared"); } }
    }
    save_project(&project)
}

fn list_dependencies() -> Result<(), Box<dyn std::error::Error>> {
    let project = load_project()?;
    if project.dependencies.is_empty() { println!("No dependencies declared."); }
    for (name, version) in project.dependencies { println!("{name} {version}"); }
    Ok(())
}

fn safe_version(value: &str) -> bool { !value.is_empty() && value.len() <= 100 && value != "." && value != ".." && value.chars().all(|c| c.is_ascii_alphanumeric() || matches!(c, '-' | '_' | '.' | '+')) }

fn digest_tree(root: &Path) -> Result<String, Box<dyn std::error::Error>> {
    let mut paths = Vec::new();
    collect_files(root, &mut paths)?;
    paths.sort();
    let mut hasher = Sha256::new();
    for path in paths {
        let rel = path.strip_prefix(root)?.to_string_lossy().replace('\\', "/");
        hasher.update(rel.as_bytes());
        hasher.update([0]);
        hasher.update(fs::read(&path)?);
        hasher.update([0]);
    }
    Ok(hex::encode(hasher.finalize()))
}

fn collect_files(dir: &Path, out: &mut Vec<PathBuf>) -> io::Result<()> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        let ty = entry.file_type()?;
        if ty.is_symlink() { return Err(io::Error::new(io::ErrorKind::InvalidData, "symlinks are not allowed in package trees")); }
        if ty.is_dir() { collect_files(&path, out)?; }
        else if ty.is_file() { out.push(path); }
    }
    Ok(())
}

fn install() -> Result<(), Box<dyn std::error::Error>> {
    let project = load_project()?;
    let registry = Path::new("registry/packages");
    let target_root = Path::new(".opaml/packages");
    fs::create_dir_all(target_root)?;
    let mut locked = Vec::new();

    for (name, version) in &project.dependencies {
        if !valid_name(name) || !safe_version(version) {
            return Err(format!("unsafe package name or version: {name} {version}").into());
        }
        let package_dir = registry.join(name).join(version);
        if !package_dir.is_dir() {
            return Err(format!("package {name} {version} not found at {}; only local registry packages are supported in this version", package_dir.display()).into());
        }
        let hash = digest_tree(&package_dir)?;
        let destination = target_root.join(name).join(version);
        if destination.exists() { fs::remove_dir_all(&destination)?; }
        if let Some(parent) = destination.parent() { fs::create_dir_all(parent)?; }
        copy_tree(&package_dir, &destination)?;
        locked.push(LockedPackage { name: name.clone(), version: version.clone(), sha256: hash, source: format!("registry/packages/{name}/{version}") });
        println!("Installed {name} {version}");
    }
    locked.sort_by(|a, b| a.name.cmp(&b.name));
    fs::write(LOCKFILE, serde_json::to_string_pretty(&Lockfile { lock_version: 1, packages: locked })?)?;
    println!("Wrote {LOCKFILE}");
    Ok(())
}

fn copy_tree(src: &Path, dst: &Path) -> io::Result<()> {
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let from = entry.path();
        let to = dst.join(entry.file_name());
        let ty = entry.file_type()?;
        if ty.is_symlink() { return Err(io::Error::new(io::ErrorKind::InvalidData, "symlinks are not allowed")); }
        if ty.is_dir() { fs::create_dir_all(&to)?; copy_tree(&from, &to)?; }
        else if ty.is_file() { fs::copy(&from, &to)?; }
    }
    Ok(())
}

fn verify() -> Result<(), Box<dyn std::error::Error>> {
    let raw = fs::read_to_string(LOCKFILE)
        .map_err(|e| io::Error::new(e.kind(), format!("cannot read {LOCKFILE}; run opaml install first: {e}")))?;
    let lock: Lockfile = serde_json::from_str(&raw)?;
    if lock.lock_version != 1 { return Err(format!("unsupported lockfile version: {}", lock.lock_version).into()); }
    let mut failures = 0usize;
    for package in lock.packages {
        if !valid_name(&package.name) || !safe_version(&package.version) {
            return Err("lockfile contains an unsafe package path".into());
        }
        let installed = Path::new(".opaml/packages").join(&package.name).join(&package.version);
        if !installed.is_dir() {
            eprintln!("MISSING {} {}", package.name, package.version);
            failures += 1;
            continue;
        }
        let actual = digest_tree(&installed)?;
        if actual != package.sha256 {
            eprintln!("MISMATCH {} {}", package.name, package.version);
            failures += 1;
        } else { println!("OK {} {}", package.name, package.version); }
    }
    if failures > 0 { return Err(format!("verification failed for {failures} package(s)").into()); }
    println!("All locked packages verified.");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn package_names_reject_path_traversal() {
        assert!(!valid_name("../outside"));
        assert!(!valid_name("a/b"));
        assert!(valid_name("liquid-core_2"));
    }
    #[test]
    fn digest_is_stable_for_same_tree() {
        let dir = tempfile::tempdir().unwrap();
        fs::write(dir.path().join("package.toml"), "name = 'demo'").unwrap();
        assert_eq!(digest_tree(dir.path()).unwrap(), digest_tree(dir.path()).unwrap());
    }
}
