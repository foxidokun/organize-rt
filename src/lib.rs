#![allow(clippy::must_use_candidate)]
#![allow(clippy::missing_errors_doc)] //Don't have docs


use std::path::{PathBuf, Path};
use std::fs::{create_dir_all, rename, remove_dir_all, canonicalize, File};
use std::io::{Write, Read};
use structopt::StructOpt;
use serde::{Serialize, Deserialize};
use walkdir::{WalkDir, DirEntry};
use indicatif::ProgressBar;
use regex::{Regex, RegexBuilder};

mod default;


// --CLI ARGS SECTION--
#[derive(StructOpt)]
#[allow(clippy::struct_excessive_bools)] //Allow, because all bools are flags and I need them
///Tool for organizing files in garbage dirs like 'Downloads'. 
pub struct Options {
    #[structopt(short, long)]
    pub recursive: bool,
    
    #[structopt(short="H", long)]
    ///Include hidden files/directories
    pub hidden: bool,
    
    #[structopt(short, long)]
    ///Show more info
    pub verbose: bool,
    
    #[structopt(short, long)]
    ///Quiet run, empty output
    pub quiet: bool,

    #[structopt(long="dry-run")]
    ///Prints where the file would move, but does not move
    pub dry_run: bool,

    #[structopt(short, long)]
    ///Undo action (require log)
    pub undo: bool,

    #[structopt(long="log", parse(from_os_str), default_value = "./organize-rt.log")]
    ///Path to save/load log
    pub log_path: PathBuf,

    #[structopt(short = "s", long = "source", name="source", parse(from_os_str), required_unless = "undo")]
    ///Directory to organize
    source_raw: Option<PathBuf>,

    #[structopt(skip)]
    pub source: PathBuf,

    #[structopt(short = "o", long = "output", name="output", parse(from_os_str), required_unless = "undo")]
    ///Output directory
    output_raw:  Option<PathBuf>,

    #[structopt(skip)]
    pub output: PathBuf
}


impl Options {
    pub fn verbose_print(&self, text: &str){
        if self.verbose {
            println!("{}", text);
        }
    }

    pub fn default_print(&self, text: &str) {
        if !self.quiet {
            println!("{}", text);
        }
    }

    pub fn resolve(&mut self) {
        create_dir_all(&self.output_raw.as_ref().unwrap()).unwrap();
        self.output = self.output_raw.clone().unwrap();
        self.source = self.source_raw.clone().unwrap();
    }
}

// --REGEX RULES SECTION--

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
    fn compile(self, output_dir: &PathBuf) -> Result<CompiledRules, Box<dyn std::error::Error>> {
        let mut compiled_rules: Vec<(Regex, PathBuf)> = Vec::new();
        for (regex, dir_name) in self.rules {
            let regex = RegexBuilder::new(regex.as_str()).case_insensitive(true).build()?;
            let mut path = (*output_dir).clone();
            path.push(dir_name);
            compiled_rules.push((regex, path));
        }

        Ok(CompiledRules{
            rules: compiled_rules
        })
    }
}


pub struct CompiledRules {
    rules: Vec<(Regex, PathBuf)>
}

impl CompiledRules {
    pub fn iter(&self) -> std::slice::Iter<(Regex, PathBuf)> {
        self.rules.iter()
    }

    pub fn load (options: &Options) -> Result<CompiledRules, Box<dyn std::error::Error>> {
        let rawrules: RawRules = confy::load("organize-rt")?;
        Ok(rawrules.compile(&options.output)?)
    }
}

// --LOG SECTION--
#[derive(Serialize, Deserialize)]
struct Move {
    from: PathBuf,
    to: PathBuf
}

impl Move {
    //Resolve path into absolute
    fn new(from: PathBuf, to: PathBuf) -> Move{
        Move {
            from,
            to
        }
    }
}

// --NORMAL MAIN SECTION--

pub fn get_files(hidden: bool, recursive: bool, source: &PathBuf) -> Vec<PathBuf> {
    //Walker setup
    let mut walker = WalkDir::new(&source);
    if !recursive {
        walker = walker.max_depth(1);
    }
    let walker = walker.into_iter().filter_map(Result::ok)
        .filter(|e| (hidden || !is_hidden(e)) && !e.file_type().is_dir());


    //Walk
    let mut files: Vec<PathBuf> = Vec::new();
    for entry in walker
    {
            files.push(entry.into_path());
    }

    files

}

fn is_hidden(entry: &DirEntry) -> bool {
    entry.path()
         .to_str()
         .map_or(false, |s| s.contains("/."))

}

pub fn create_dirs(options: &Options) -> Result<(), Box<dyn std::error::Error>>{
    options.verbose_print("Creating dirs...");
    
    create_dir_all(Path::new(&(options.output.to_str().unwrap().to_owned() + "/Audio")))?;
    create_dir_all(Path::new(&(options.output.to_str().unwrap().to_owned() + "/Compressed")))?;
    create_dir_all(Path::new(&(options.output.to_str().unwrap().to_owned() + "/Garbage")))?;
    create_dir_all(Path::new(&(options.output.to_str().unwrap().to_owned() + "/Downloads")))?;
    create_dir_all(Path::new(&(options.output.to_str().unwrap().to_owned() + "/Code")))?;
    create_dir_all(Path::new(&(options.output.to_str().unwrap().to_owned() + "/Documents")))?;
    create_dir_all(Path::new(&(options.output.to_str().unwrap().to_owned() + "/Images")))?;
    create_dir_all(Path::new(&(options.output.to_str().unwrap().to_owned() + "/ISO")))?;
    create_dir_all(Path::new(&(options.output.to_str().unwrap().to_owned() + "/Configuration")))?;
    create_dir_all(Path::new(&(options.output.to_str().unwrap().to_owned() + "/Encrypted")))?;
    create_dir_all(Path::new(&(options.output.to_str().unwrap().to_owned() + "/Video")))?;
    create_dir_all(Path::new(&(options.output.to_str().unwrap().to_owned() + "/Unsorted")))?;
    create_dir_all(Path::new(&(options.output.to_str().unwrap().to_owned() + "/REMOVE")))?;
    options.verbose_print("Done!");

    Ok(())
}

pub fn move_files(files: &[PathBuf], rules: &CompiledRules, options: &Options) {
    let progressbar = ProgressBar::new(files.len() as u64);
    let mut actions: Vec<Move> = Vec::new();

    for (id, file) in files.iter().enumerate() {
        for (regex, out_dir) in rules.iter() {
            if regex.is_match(&file.file_name().unwrap().to_str().unwrap()) {
                let mut file_out = out_dir.clone();
                file_out.push(file.file_name().unwrap());

                if options.dry_run {

                    options.default_print(format!("{} -> {}", file.to_str().unwrap(), file_out.to_str().unwrap()).as_str());

                } else {
                    let file = canonicalize(&file).unwrap();

                    //Check if file already exists
                    if file_out.exists() {
                        //and change it name
                        file_out.pop();
                        file_out.push(format!("{}.COPY{}", file.file_name().unwrap().to_str().unwrap(), id));
                    }

                    //Skip errors
                    if let Err(e) = rename(&file, &file_out) {
                        options.default_print(format!("Failed to move file {} with error {}", file.to_str().unwrap(), e).as_str());
                    } else {
                        let file_out = canonicalize(&file_out).unwrap();
                        actions.push(Move::new(file, file_out));
                    }
                }

                break;
            }
        }

        if !options.quiet  && !options.dry_run {
        progressbar.inc(1);
        }
    }

    //Save log
    let serialised_log = serde_json::to_string(&actions).unwrap();

    // - Create log dir
    let mut log_dir = options.log_path.clone();
    log_dir.pop();
    create_dir_all(log_dir).unwrap();

    // - Write log
    let mut file = File::create(&options.log_path).unwrap();
    file.write_all(serialised_log.as_bytes()).unwrap();


    //Delete `REMOVE` dir
    if !options.dry_run {
        options.verbose_print("Removing REMOVE dir...");
        remove_dir_all(options.output.to_str().unwrap().to_owned() + "/REMOVE").unwrap();
        options.verbose_print("Done!");
    }
}

// --UNDO MAIN SECTION-- 

pub fn undo(options: &Options) {
    let mut file = File::open(&options.log_path).unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();

    let actions:Vec<Move> = serde_json::from_str(&content).unwrap();

    for action in actions {
        let mut from_dir = action.from.clone();
        from_dir.pop();
        create_dir_all(from_dir).unwrap();
        if options.dry_run {
            options.default_print(format!("{} -> {}", action.to.to_str().unwrap(), action.from.to_str().unwrap()).as_str());
        } else if let Err(e) = rename(&action.to, &action.from) {
            options.default_print(format!("Failed to move {} back to {} with error '{}' (skipped it)", 
                action.to.to_str().unwrap(), action.from.to_str().unwrap(), e).as_str());
            
        }
    }
}
