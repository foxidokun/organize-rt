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
organize-rt 1.0.0
Tool for organizing files in garbage dirs like 'Downloads'

USAGE:
    organize-rt [FLAGS] [OPTIONS] --output <output> --source <source>

FLAGS:
        --dry-run      Prints where the file would move, but does not move
    -h, --help         Prints help information
    -H, --hidden       Include hidden files/directories
    -q, --quiet        Quiet run, empty output
    -r, --recursive
    -u, --undo         Undo action (require log)
    -V, --version      Prints version information
    -v, --verbose      Show more info

OPTIONS:
        --log <log-path>     Path to save/load log [default: ./organize-rt.log]
    -o, --output <output>    Output directory
    -s, --source <source>    Directory to organize
```

Recommended mode: `organize-rt -rH`


If you have several file with the same name, program will save them like `file`, `file.COPY<id>`, `file.COPY<id>`..., where `id` isn't a copy number, but a unique number. Also, program will skip all file errors like `Bad permissions` and print about them (if --quiet flag isn't specified). 

When you run this program, __after all moves__ it will save its actions in log (--log option, default "./organize-rt.log").
If you want to discard changes, run with --undo option. For example `organize-rt --undo --log ./badrun.log` will discard changes, saved in 
badrun.log. Some important notes about undo:
* It use absolute paths, so you can run it from anywhere.
* Due to absolute paths, you can't undid changes if output dir was moved
* It will skip errors (deleted files from output dir)
* You can delete source directory, undo mode will restore it

## Example 
Normal mode:
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

Undo this action (# hide absolute path):
```
$ organize-rt --undo --dry-run
#/out/Video/avi.avi -> #/in/avi.avi
#/out/Compressed/compressed.tar.bz2 -> #/in/compressed.tar.bz2
#/out/Audio/ogg.ogg -> #/in/ogg.ogg
#/out/Configuration/conf.conf -> #/in/conf.conf
#/out/ISO/archlinux.iso -> #/in/archlinux.iso
#/out/Compressed/compressed.tar.gz -> #/in/compressed.tar.gz
#/out/Unsorted/unsorted.norule -> #/in/unsorted.norule
#/out/Audio/mp3.mp3 -> #/in/mp3.mp3
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

# Project status
The project is alive, but I have no ideas for new functionality. Waiting for your pull request, issue or email, if you don't have GitLab account.
