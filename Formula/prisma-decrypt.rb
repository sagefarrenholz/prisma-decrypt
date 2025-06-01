class PrismaDecrypt < Formula
  desc "CLI tool for super fast prisma-field-encryption column decryption (clock decryption)"
  homepage "https://github.com/sagefarrenholz/prisma-decrypt"
  url "https://github.com/sagefarrenholz/prisma-decrypt/releases/download/v0.1.0/prisma-decrypt-macos-intel.tar.gz"
  sha256 "UPDATE_WITH_ACTUAL_SHA256_HASH"
  version "0.1.2"
  license "MIT OR Apache-2.0"

  on_arm do
    url "https://github.com/sagefarrenholz/prisma-decrypt/releases/download/v0.1.0/prisma-decrypt-macos-arm64.tar.gz"
    sha256 "UPDATE_WITH_ACTUAL_ARM64_SHA256_HASH"
  end

  def install
    bin.install "prisma-decrypt"
  end

  test do
    system "#{bin}/prisma-decrypt", "--version"
  end
end