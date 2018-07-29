use regex::Regex;

pub enum Encoding {
    Avc,
    Hevc,
}

pub struct MetaData {
    encoding: Encoding,
    chapter: usize,
    file_number: usize,
}

impl MetaData {
    pub fn encoding(&self) -> &Encoding {
        &self.encoding
    }
    pub fn chapter(&self) -> usize {
        self.chapter
    }
    pub fn file_number(&self) -> usize {
        self.file_number
    }
}

pub fn from_file_name(file_name: &str) -> MetaData {
    let re = Regex::new("^G(?P<enc>[HX])(?P<chap>\\d{2})(?P<num>\\d{4}).(mp|MP)4").unwrap();
    let caps = re.captures(file_name).unwrap();

    let enc = match &caps["enc"] {
        "H" => Encoding::Avc,
        "X" => Encoding::Hevc,
        _ => panic!("unexpected encoding {}", &caps["enc"]),
    };
    let chap = caps["chap"].parse::<usize>().unwrap();
    let num = caps["num"].parse::<usize>().unwrap();

    MetaData {
        encoding: enc,
        chapter: chap,
        file_number: num,
    }
}
