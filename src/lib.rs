extern crate regex;

use std::fs;
use std::path::{Path, PathBuf};

use regex::Regex;

mod metadata;

fn is_gopro_file<S>(file_name: S) -> bool
where
    S: AsRef<str>,
{
    let re = Regex::new("^G[HX]\\d{6}.(mp|MP)4").unwrap();

    re.is_match(file_name.as_ref())
}

pub fn fix<P>(path: P) -> Option<PathBuf>
where
    P: AsRef<Path>,
{
    let path = path.as_ref();
    let file_name = path.file_name().unwrap().to_str().unwrap();

    if !is_gopro_file(file_name) {
        return None;
    }

    let meta = metadata::from_file_name(file_name);
    let enc_char = match meta.encoding() {
        metadata::Encoding::Avc => "H",
        metadata::Encoding::Hevc => "X",
    };
    let fixed_name = format!(
        "GP{:04}{:02}{}.mp4",
        meta.file_number(),
        meta.chapter(),
        enc_char,
    );
    let orig_path: &Path = path.as_ref();
    let parent = orig_path.parent().unwrap().to_path_buf();
    let fixed_path = parent.join(fixed_name);

    fs::rename(&orig_path, &fixed_path).unwrap();

    Some(fixed_path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_gopro_file_match() {
        let matches = vec!["GH011234.mp4", "GX011234.mp4", "GH021234.mp4"];

        for m in matches {
            println!("testing {}", m);
            assert!(is_gopro_file(m));
        }
    }
}
