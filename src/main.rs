extern crate clap;
extern crate gpnf;
extern crate walkdir;

use std::path::Path;

use clap::{App, Arg};
use walkdir::WalkDir;

fn main() {
    let matches = App::new("gpnf")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Christoph Wurst")
        .about("GoPro Name Fixer")
        .arg(
            Arg::with_name("path")
                .help("The file/directory to fix")
                .required(true),
        )
        .get_matches();

    let path_str = matches.value_of_os("path").unwrap();
    let path = Path::new(path_str);

    for entry in WalkDir::new(path) {
        let entry = entry.unwrap();

        match gpnf::fix(entry.path()) {
            Some(fixed) => println!("{} -> {:?}", entry.path().display(), fixed),
            None => println!("Ignoring {}", entry.path().display()),
        }
    }
}
