use std::path::{PathBuf, Path};
use std::fs::{create_dir_all, rename, remove_dir};
use structopt::StructOpt;
use confy;
use serde::{Serialize, Deserialize};
use walkdir::{WalkDir, DirEntry};
use regex::{Regex, RegexBuilder};
use indicatif::ProgressBar;
use human_panic::setup_panic;

mod default;

#[derive(StructOpt)]
#[doc(hidden)]
///Tool for organizing files in garbage dirs like 'Downloads'. 
struct Options {
    #[structopt(short, long)]
    recursive: bool,
    
    #[structopt(short="H", long)]
    ///Include hidden files/directories
    hidden: bool,
    
    #[structopt(short, long)]
    ///Show more info
    verbose: bool,
    
    #[structopt(short, long)]
    ///Quiet run, empty output
    quiet: bool,


    #[structopt(long="dry-run")]
    ///Prints where the file would move, but does not move
    dry_run: bool,

    #[structopt(short, long, parse(from_os_str))]
    ///Directory to organize
    source: PathBuf,

    #[structopt(short, long, parse(from_os_str))]
    ///Output directory
    output:  PathBuf

}

#[derive(Serialize, Deserialize)]
struct RawRules {
    rules: Vec<(String, String)>
}

impl Default for RawRules {
    fn default() -> RawRules {
        let mut rules = Vec::new();
        default::rules(&mut rules);
        RawRules {
            rules
        }
    }
}

impl RawRules {
    fn compile(self, output_dir: &PathBuf) -> CompiledRules {
        let mut compiled_rules: Vec<(Regex, PathBuf)> = Vec::new();
        for (regex, dir_name) in self.rules.into_iter() {
            let regex = RegexBuilder::new(regex.as_str()).case_insensitive(true).build().unwrap();
            let mut path = (*output_dir).clone();
            path.push(dir_name);
            compiled_rules.push((regex, path));
        }

        CompiledRules{
            rules: compiled_rules
        }
    }
}


struct CompiledRules {
    rules: Vec<(Regex, PathBuf)>
}

impl CompiledRules {
    fn iter(&self) -> std::slice::Iter<(Regex, PathBuf)> {
        self.rules.iter()
    }
}

fn is_hidden(entry: &DirEntry) -> bool {
    entry.path()
         .to_str()
         .map(|s| s.contains("/."))
         .unwrap_or(false)
}

fn main() {
    setup_panic!();
    let options = Options::from_args();

    //Don't use quiet with verbose flag
    if options.quiet && options.verbose {
        println!("Can't use quiet and verbose flags together");
        return ();
    }


    let rules: RawRules = confy::load("organize-rt").unwrap();

    if !options.source.is_dir()  {
        println!("Wrong source directory");
        return ();
    }
    
    if options.verbose {
        println!("Compiling rules...");
    }
    let rules = rules.compile(&options.output);
    if options.verbose {
        println!("Rules compiled");
    }

    //Walker setup
    let mut walker = WalkDir::new(&options.source);
    if !options.recursive {
        walker = walker.max_depth(1);
    }
    let walker = walker.into_iter().filter_map(|e| e.ok())
        .filter(|e| (options.hidden || !is_hidden(e)) && !e.file_type().is_dir());
    

    // Get files to move
    let mut files: Vec<PathBuf> = Vec::new();
    for entry in walker
    {
            files.push(entry.into_path());
    }
    
    if options.verbose {
        println!("Counted {} files", files.len());
    }
    

    //Creating dirs to move
    if options.verbose && !options.dry_run {
        println!("Creating dirs...");
    }

    if !options.dry_run {
        create_dir_all(Path::new(&(options.output.to_str().unwrap().to_owned() + "/Audio"))).unwrap();
        create_dir_all(Path::new(&(options.output.to_str().unwrap().to_owned() + "/Compressed"))).unwrap();
        create_dir_all(Path::new(&(options.output.to_str().unwrap().to_owned() + "/Garbage"))).unwrap();
        create_dir_all(Path::new(&(options.output.to_str().unwrap().to_owned() + "/Downloads"))).unwrap();
        create_dir_all(Path::new(&(options.output.to_str().unwrap().to_owned() + "/Code"))).unwrap();
        create_dir_all(Path::new(&(options.output.to_str().unwrap().to_owned() + "/Documents"))).unwrap();
        create_dir_all(Path::new(&(options.output.to_str().unwrap().to_owned() + "/Images"))).unwrap();
        create_dir_all(Path::new(&(options.output.to_str().unwrap().to_owned() + "/ISO"))).unwrap();
        create_dir_all(Path::new(&(options.output.to_str().unwrap().to_owned() + "/Configuration"))).unwrap();
        create_dir_all(Path::new(&(options.output.to_str().unwrap().to_owned() + "/Encrypted"))).unwrap();
        create_dir_all(Path::new(&(options.output.to_str().unwrap().to_owned() + "/Video"))).unwrap();
        create_dir_all(Path::new(&(options.output.to_str().unwrap().to_owned() + "/Unsorted"))).unwrap();
        create_dir_all(Path::new(&(options.output.to_str().unwrap().to_owned() + "/REMOVE"))).unwrap();
    }

    //Move files
    let progressbar = ProgressBar::new(files.len() as u64);
    for file in files {
        for (regex, out_dir) in rules.iter() {
            if regex.is_match(&file.file_name().unwrap().to_str().unwrap()) {
                let mut file_out = out_dir.clone();
                file_out.push(file.file_name().unwrap());
                if !options.dry_run {
                    rename(&file, &file_out).unwrap();
                } else if !options.quiet {
                    println!("{} -> {}", file.to_str().unwrap(), file_out.to_str().unwrap());
                }
                break;
            }
        }
        if !options.quiet  && !options.dry_run {
        progressbar.inc(1);
        }
    }
    

    //Remove `REMOVE` dir
    if options.verbose && !options.dry_run {
        println!("Removing REMOVE dir...");
    }
    
    if !options.dry_run {
        remove_dir(options.output.to_str().unwrap().to_owned() + "/REMOVE").unwrap();
    }
}
