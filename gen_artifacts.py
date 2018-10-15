import argparse
import os
import shutil
import subprocess


def run(*cmd):
    print("::", *cmd)
    subprocess.check_call(cmd)


def copy(src, dest):
    print(src, "->", dest)
    shutil.copy(src, dest)


def build_release():
    run("cargo", "build", "--release")


def populate_archive(archive_path, *, platform):
    if platform == "windows":
        ext = ".exe"
    else:
        ext = ""
    copy("target/release/ruplacer" + ext, archive_path)
    copy("README.md", archive_path)
    copy("CHANGELOG.md", archive_path)
    copy("LICENSE", archive_path)


def make_archive(archive_path, *, platform):
    if platform == "windows":
        archive_format = "zip"
    else:
        archive_format = "gztar"
    res = shutil.make_archive(
        archive_path, archive_format, root_dir=".", base_dir=archive_path
    )
    print(":: generated", res)
    return res


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--version", required=True)
    parser.add_argument("--platform", required=True)
    args = parser.parse_args()
    version = args.version
    platform = args.platform

    build_release()
    archive_path = "ruplacer-%s-%s" % (version, platform)
    os.makedirs(archive_path, exist_ok=True)

    populate_archive(archive_path, platform=platform)

    dist_path = "dist"
    os.makedirs(dist_path, exist_ok=True)
    archive = make_archive(archive_path, platform=platform)
    # So that we can simply glob dist/* in .travis.yml
    shutil.move(archive,  dist_path)


if __name__ == "__main__":
    main()
