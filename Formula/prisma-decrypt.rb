class PrismaDecrypt < Formula
  desc "CLI tool for super fast prisma-field-encryption column decryption (clock decryption)"
  homepage "https://github.com/sagefarrenholz/prisma-decrypt"
  url "https://github.com/sagefarrenholz/prisma-decrypt/releases/download/v0.1.2/prisma-decrypt-macos-intel.tar.gz"
  sha256 "e2dbb53cbf48b768e1a7000f47e3fc4bf07807b60eb432566e2dcce9dd1dcfc1"
  version "0.1.2"
  license "MIT OR Apache-2.0"

  on_arm do
    url "https://github.com/sagefarrenholz/prisma-decrypt/releases/download/v0.1.2/prisma-decrypt-macos-arm64.tar.gz"
    sha256 "366b047f666ad6713137c33dd9713a379d7788151b588a7c6ada3deb318749a6"
  end

  def install
    bin.install "prisma-decrypt"
  end

  test do
    system "#{bin}/prisma-decrypt", "--version"
  end
end