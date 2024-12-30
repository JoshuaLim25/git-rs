#![allow(dead_code, unused_variables)]

use std::env;
use std::fs;
use std::io;
use std::path::Path;

fn cli_args() -> Vec<String> {
    let args: Vec<String> = env::args()
        .skip(1) // iterator @ idx 0 is the bin
        .inspect(|arg| println!("{arg}"))
        .collect();
    args
}

fn init_repo(args: &[String]) -> Result<(), io::Error> {
    assert!(args.is_empty(), "Expected a subcommand.");
    assert_eq!(
        args[0], "init",
        "Assertion failed: init expected, {} given.",
        args[0]
    );
    if let Ok(ref mut cwd) = env::current_dir() {
        // theres a better way lol
        cwd.push(".git/objects");
        fs::create_dir_all(&cwd)?;
        cwd.pop();
        cwd.push("refs");
        fs::create_dir_all(&cwd)?;
        fs::write(".git/HEAD", "ref: refs/heads/main")?;
    };
    Ok(())
}

fn main() {
    let args = cli_args();
    init_repo(&args).expect("Error creating certain directories");
}
