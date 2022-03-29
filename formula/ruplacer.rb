class Ruplacer < Formula
  desc "Find and replace text in source files"
  homepage "https://github.com/dmerejkowsky/ruplacer/"
  url "https://github.com/dmerejkowsky/ruplacer/archive/v0.6.4.tar.gz"
  sha256 "4f66e8970942e55dc287c585eef7a21394aefa49df746cef429f9e5bc6714c7a"
  head "https://github.com/dmerejkowsky/ruplacer"

  depends_on "rust" => :build

  def install
    system "cargo", "install", "--root", prefix, "--path", "."
  end

  test do
    (testpath/"foo.txt").write("this is foo")
    system "#{bin}/ruplacer", "foo", "bar", testpath
  end
end
