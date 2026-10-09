# OPAML + Liquidsoap radio example

This example keeps a Liquidsoap runtime script beside an OPAML project manifest.

## Layout

- `Radio.liq` — playlist source, silence fallback, and Icecast output.
- `music.m3u` — create this file locally; list one audio file path or stream URL per line.
- `opaml.toml` — project metadata for OPAML.

## Prerequisites

1. Install a compatible Liquidsoap 2.x build and an Icecast server separately.
2. Create `music.m3u` with paths/URLs to audio tracks that you are authorized to stream.
3. Edit the Icecast host, port, mount, and password in `Radio.liq`. Do not commit real passwords.
4. Run Liquidsoap from this directory, for example:

```sh
liquidsoap Radio.liq
```

The output connects to the configured Icecast server at `/radio.mp3`. The `fallback` source uses silence if the playlist is unavailable or exhausted, so the output can remain connected.

## OPAML status

The manifest demonstrates the project metadata format. OPAML's current prototype uses a local package registry and does not yet install or manage the Liquidsoap executable itself. This sample does not claim that a Liquidsoap runtime or Icecast server is bundled.

## Security notes

- Replace `CHANGE_ME` before use and keep production credentials in your deployment's secret-management system.
- Bind Icecast to an appropriate interface and firewall its admin/source ports.
- Stream only media you have permission to broadcast.
