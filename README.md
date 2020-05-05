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

# Usage
```
$ organize-rt --help
orginize-rt 0.9.0
Tool for organizing files in garbage dirs like 'Downloads'

USAGE:
    orginize-rt [FLAGS] --output <output> --source <source>

FLAGS:
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

## Writing own rules
Just edit ~/.config/organize-rt/rules.toml. File structure:
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
With this rule, file, that match REGEX rule, but *didn't match previous rules* will move to OUTPUT_DIR/OUTPUT_SUBDIR,
where OUTPUT_DIR is --output option.
# Like it?
Star this repo, please.
# Bad code?
Waiting for your pull request or issue