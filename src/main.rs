use structopt;

use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "git", about = "the stupid content tracker")]
enum Git {
    #[structopt(name = "add")]
    Add {
        #[structopt(short = "i")]
        interactive: bool,
        #[structopt(short = "p")]
        patch: bool,
        #[structopt(parse(from_os_str))]
        files: Vec<PathBuf>,
    },
    #[structopt(name = "fetch")]
    Fetch {
        #[structopt(long = "dry-run")]
        dry_run: bool,
        #[structopt(long = "all")]
        all: bool,
        repository: Option<String>,
    },
    #[structopt(name = "commit")]
    Commit {
        #[structopt(short = "m")]
        message: Option<String>,
        #[structopt(short = "a")]
        all: bool,
    },
}

fn main() {
    match Git::from_args() {
        Git::Add {
            interactive,
            patch,
            files,
        } => println!("{:?} {:?} {:?}", interactive, patch, files),
        Git::Commit { message, all } => println!("{:?} {:?}", message, all),
        Git::Fetch {
            dry_run,
            all,
            repository,
        } => println!("{:?} {:?} {:?}", dry_run, all, repository),
    }
}
