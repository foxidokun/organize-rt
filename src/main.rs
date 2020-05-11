extern crate organize_rt;

use std::process;
use structopt::StructOpt;
use human_panic::setup_panic;

use organize_rt::{Options, CompiledRules, get_files, create_dirs, move_files, undo};



fn main() {
    // --INITIALIZATION--
    setup_panic!();
    let mut options = Options::from_args();

    //Don't use quiet with verbose flag
    if options.quiet && options.verbose {
        println!("Can't use quiet and verbose flags together");
        process::exit(1);
    }

    if options.undo {
        undo_mode(&options);
    } else {
        normal_mode(&mut options);
    }
      
}

fn undo_mode(options: &Options) {
    if !options.log_path.exists() {
        println!("Wrong log path");
        process::exit(1);
    }
    undo(&options);
}

fn normal_mode(options: &mut Options) {
    //Resolve & check input dir
    options.resolve();
    if !options.source.is_dir() {
        println!("Wrong source dir");
        process::exit(1);
    }

    //Load rules or panic
    let rules = if let Ok(rules) = CompiledRules::load(&options) { rules } else {
        println!("Can't load rules, please check config file. To restore default just remove it");
        process::exit(1);
     };


    // Get files to move

    options.verbose_print("Counting files...");
    let files = get_files(options.hidden, options.recursive, &options.source);
    options.verbose_print(format!("Counted {} files", files.len()).as_str());

    //Creating dirs to move

    if !options.dry_run {
        if let Err(e) = create_dirs(&options) {
            println!("Failed to create output dirs");
            panic!("{}", e);
        }
    }

    //Move files
    move_files(&files, &rules, &options);
}