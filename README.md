[![pipeline status](https://gitlab.com/FixFromDarkness/organize-rt/badges/master/pipeline.svg)](https://gitlab.com/FixFromDarkness/organize-rt/-/commits/master)

> Organize files based on regex rules, file extensions by default.

# Installation
## Using cargo (require Rust)

Install it by running
```bash
$ cargo install organize-rt
```

It will download & compile binary to ~/.cargo/bin. Add this directory to your path
or copy binary to /usr/bin/

## Compiling from source (require Rust)
Just clone this repo and build this project:
```bash
$ git clone --depth=1 https://gitlab.com/FixFromDarkness/organize-rt.git
$ cd organize-rt
$ cargo build --release
```

After compiling, you can use binary `./target/release/organize-rt`. For example, you could copy it to /usr/bin.

## Using pre-build binaries

Download binaries from GitLab release section.

## Arch Linux
Clone repo and use makepkg:
```bash
$ git clone --depth=1 https://gitlab.com/FixFromDarkness/organize-rt.git
$ makepkg -sri
```

## Fedora/CentOS
Available in [COPR](https://copr.fedorainfracloud.org/coprs/atim/organize-rt/):
```
sudo dnf copr enable atim/organize-rt -y
sudo dnf install organize-rt
```

# Usage
```
$ organize-rt --help
organize-rt 0.9.1
Tool for organizing files in garbage dirs like 'Downloads'

USAGE:
    organize-rt [FLAGS] --output <output> --source <source>

FLAGS:
        --dry-run      Prints where the file would move, but does not move
    -h, --help         Prints help information
    -H, --hidden       Include hidden files/directories
    -q, --quiet        Quiet run, empty output
    -r, --recursive
    -V, --version      Prints version information
    -v, --verbose      Show more info

OPTIONS:
    -o, --output <output>    Output directory
    -s, --source <source>    Directory to organize
```

Recommended mode: `organize-rt -rH`

## Example 
```
$ organize-rt -s in -o out -rH --dry-run
in/avi.avi -> out/Video/avi.avi
in/compressed.tar.bz2 -> out/Compressed/compressed.tar.bz2
in/ogg.ogg -> out/Audio/ogg.ogg
in/conf.conf -> out/Configuration/conf.conf
in/archlinux.iso -> out/ISO/archlinux.iso
in/compressed.tar.gz -> out/Compressed/compressed.tar.gz
in/dir/document.docx -> out/Documents/document.docx
in/dir/image.png -> out/Images/image.png
in/unsorted.norule -> out/Unsorted/unsorted.norule
in/.hidden.conf -> out/Configuration/.hidden.conf
in/mp3.mp3 -> out/Audio/mp3.mp3
```

## Writing own rules
Just edit ~/.config/organize-rt/organize-rt.toml. This file will appear after the first run, you can also use organize-rt.toml.default from this repository. File structure:
```toml
rules = [
#...
    [
    'REGEX',
    'OUTPUT_SUBDIR',
]
#...
]
``` 
With this rule, file, that match REGEX rule, but __didn't match previous rules__ will move to OUTPUT_DIR/OUTPUT_SUBDIR,
where OUTPUT_DIR is --output option.

# Like it?
Star this repo, please.

# Bad code?
Waiting for your pull request or issue
