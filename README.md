[![FOSSA Status](https://app.fossa.com/api/projects/git%2Bgithub.com%2FCKingX%2Fhaveibeenpwned.svg?type=shield)](https://app.fossa.com/projects/git%2Bgithub.com%2FCKingX%2Fhaveibeenpwned?ref=badge_shield)

![https://img.shields.io/crates/v/haveibeenpwned](https://crates.io/crates/haveibeenpwned)

# haveibeenpwned

haveibeenpwned is a command-line application that uses [HaveIBeenPwned](https://haveibeenpwned.com/) service and can create and use Binary Fuse filter (which is smaller than Bloom filter or Cuckoo filter for the same false positive ratio) for efficient query at cost of false positives.

## Features
- Interactively check compromised password using HIBP API (requires internet) with `haveibeenpwed interactive-online`
- Download password file using HaveIBeenPwned queries. This can be more up to date than downloading passwords directly from HaveIBeenPwned website. According to Troy Hunt, passwords from ingestions are not included since a password version release in the download version. However, querying the password does contain the ingested passwords. In practice, this contained 3 more passwords than version 8 as of June 7 (847,223,405 vs Version 8's 847,223,402). You can download with `haveibeenpwned downloader [path to output file]`
- Downloads can be resumed if given the same argument
- Can interactively check compromised password using filter with `haveibeenpwned interactive-filter [path to filter file]`
- Can create filter (of 3 sizes) that allows you to query offline while consuming a fraction of the space. Does require existing downloaded password file (either from website or by using this tool) to create with. Filters can be created with `haveibeenpwned create-filter [path to password file] [output path for filter file]` Be advised that creating a filter requires a significant amount of RAM. Testing on the downloaded passwords file (35 GB) for creating a small filter alone used 11 GB on Windows. The Small filter has a false positive rate of <0.4%, while the Medium filter has a false positivity rate of 0.0016% and the Large filter has a false positivity rate of <1 in 4 billion.
- Check list of passwords in a file (using a filter) to see how many are compromised with `haveibeenpwned file-check [path to file with passwords to test] [path to filter]` (with optional -p command to print compromised passwords from the file)

## Prebuilt Filters
Prebuilt filters are available ([small](https://mega.nz/file/l5JwgTgR#fUtrkSzuItzO_ED_WWxAJOfvld9TnuHrDhEwW2ToMcg), [medium](https://mega.nz/file/wgYUiQwQ#JJLJ-QPLdJ0YCRXulLPjq0tVQG69kMQ8IkEIjdZYllk), [large](https://mega.nz/file/ApZVXRxL#PUSdijeY1wyQdyBHLqWtZ2yB0PfnNZLwTX-VhTew9HU)). The Small filter has a size of 909 MiB (with false positivity rate of ~0.4%). The Medium filter has a size of 1.77 GiB (with false positivity rate of 0.0016%) and the Large filter has a size of 3.55 GiB (with false positivity rate of <1 in 4 billion).

## Compatibility
As haveibeenpwned was in alpha, the design of the filter wasis not final at the time. Therefore, filter file compatibility was not maintained between versions until now. Filter created by version 0.4.0-alpha is not compatible with 0.5.0 (and version 0.5.0 has smaller filters than version 0.4.0). However, compatibility from v0.5.0 onwards (current version) will be maintained.

## Install
haveibeenpwned can be downloaded from [Releases](https://github.com/CKingX/haveibeenpwned/releases) page for Ubuntu .deb package for 18.04 and later, generic linux executable for 64-bit Intel systems (You may need to run `chmod +x <path to binary>`), and Windows releases. If you have rustup installed (see Build Guide), you can install by running:
```
cargo intall haveibeenpwned
```

Currently, macOS builds are not provided as I do not have a Mac. I will also work on creating a flatpak version of haveibeenpwned

## Upgrade Instructions
If you use the deb file on Ubuntu, uninstall the deb package with:
```
sudo apt remove haveibeenpwned
```
Finally, install with the newer deb file.

For Windows, just replace the older haveibeenpwned.exe with the newer version.

If you used the haveibeenpwned linux binary, just replace it with newer one (you may need to run `chmodm +x <path to haveibeenpwned>` again)

## Build Guide
We can use cargo to build haveibeenpwned. We first need to install rustup and build tools (instructions for those can be found [here](https://www.rust-lang.org/tools/install)). Then, we can build with:
```
git clone https://github.com/CKingX/haveibeenpwned.git
cd ./haveibeenpwned
cargo install --path ./
```
Now you can run by typing haveibeenpwned in terminal. Upgrading can be done with cargo install command again. If you would just like to build the binary, you can build the debug binary with:
```
cargo build
```
Release binary can be built with:
```
cargo build --release
```

The output of the build command will be in ./target/{debug/release}/haveibeenpwned

## License
haveibeenpwned is licensed as AGPL 3.0. However, there will eventually be an MPL library that can use a filter to check passwords in other programs.

[![FOSSA Status](https://app.fossa.com/api/projects/git%2Bgithub.com%2FCKingX%2Fhaveibeenpwned.svg?type=large)](https://app.fossa.com/projects/git%2Bgithub.com%2FCKingX%2Fhaveibeenpwned?ref=badge_large)