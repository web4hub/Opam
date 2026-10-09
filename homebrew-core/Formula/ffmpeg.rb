class Ffmpeg < Formula
  desc "Play, record, convert, and stream select audio and video codecs"
  homepage "https://ffmpeg.org/"
  url "https://ffmpeg.org/releases/ffmpeg-8.1.1.tar.xz"
  sha256 "b6863adde98898f42602017462871b5f6333e65aec803fdd7a6308639c52edf3"
  license "GPL-3.0-or-later"

  def install
    system "./configure", "--prefix=#{prefix}", "--enable-shared"
    system "make", "install"
  end
end
