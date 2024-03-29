use docopt::Docopt;
use git2::{Error, Repository, StatusOptions};
use serde_derive::Deserialize;
use std::fs::{metadata,read_dir};

#[derive(Debug)]
#[derive(Deserialize)]
struct Args {
    arg_path: Vec<String>,
}

// TODO: add tests.
// TODO: remove dependency from OpenSSL.
fn main() {
    // TODO: fix invalid arguments message.
    const USAGE: &str = "
    usage: npr [options] [<path>]
    Options:
        -h, --help                  show this message
    ";

    let args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());
    match run(&args) {
        Ok(()) => {}
        Err(e) => println!("error: {}", e),
    }
}

fn run(args: &Args) -> Result<(), Error> {
    for path in &args.arg_path {
        let md = metadata(path).unwrap();
        if !md.is_dir() {
            println!("{} it's a not directory", path);
            continue;
        }

        let sub_paths = read_dir(path).unwrap();

        for sub_path in sub_paths {
            let sub_path = sub_path.unwrap().path();

            let md = metadata(&sub_path).unwrap();
            if md.is_dir() {
                let repo = Repository::open(&sub_path);
                if repo.is_ok() {
                    let repo = repo.unwrap();
                    
                    if repo.is_bare() {
                        return Err(Error::from_str("cannot report status on bare repository"));
                    }

                    let mut opts = StatusOptions::new();
                    opts.include_untracked(true).recurse_untracked_dirs(true);
                    
                    let statuses = repo.statuses(Some(&mut opts))?;
                    for _entry in statuses.iter().filter(|e| e.status() != git2::Status::CURRENT) {
                        println!("{:?} is a not pushed repository", sub_path);
                        // TODO: add option to print not pushed items.
                        break;
                    }
                }
            }
        }        
    }
    return Ok(());
}
