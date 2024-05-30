
use std::fmt::{Display, Formatter, Result as FormatResult};
use std::fs::read_dir;
use regex::Regex;
use once_cell::sync::Lazy;


static POSIX_LANG_CONSTRUCTION_REGEX: Lazy<Regex> = Lazy::new( || {Regex::new(r"(?<language>[a-zA-Z]*)(?<territory>_..)?(?<encoding>\.(.*))?(?<modifier>\@([a-zA-Z0-9]*))?").unwrap()} );

pub struct PosixLanguage {
    language: String,
    territory: String,
    encoding: String,
    modifier: String,
}

impl PosixLanguage {
    pub fn new(string: String) -> Option<Self> {
        let captures = match Lazy::<Regex>::force(&POSIX_LANG_CONSTRUCTION_REGEX).captures(&string) {
            Some(thing) => thing,
            None => return None,
        };
        Some(PosixLanguage {
            language: (&captures["language"]).to_string(),
            territory: (&captures)
                .name("territory")
                .map_or("_xx", |m| &m.as_str())
                .strip_prefix("_")
                .unwrap() //regex would have failed if this unwrap fails
                .to_lowercase(),
            encoding: (&captures)
                .name("encoding")
                .map_or(".blank", |m| &m.as_str())
                .strip_prefix(".")
                .unwrap() //regex would have failed if this unwrap fails
                .to_lowercase(),
            modifier: (&captures)
                .name("modifier")
                .map_or("@blank", |m| &m.as_str())
                .strip_prefix("@")
                .unwrap() //regex would have failed if this unwrap fails
                .to_lowercase(),
        })
    }

    pub fn get_language(&self) -> &String {
        &self.language
    }
    pub fn get_territory(&self) -> &String {
        &self.territory
    }
    pub fn get_encoding(&self) -> &String {
        &self.encoding
    }
    pub fn get_modifier(&self) -> &String {
        &self.modifier
    }

    pub fn four_best_file_names(&self) -> Vec<String> {
        let mut file_names: Vec<String> = Vec::<String>::new();
        let territory: String = if self.get_territory() == "xx" {
            "".to_string()
        } else {
            "_".to_string() + &self.get_territory()
        };
        let modifier: String = if self.get_modifier() == "blank" {
            "".to_string()
        } else {
            "@".to_string() + &self.get_modifier()
        };

        file_names.push(self.get_language().to_owned() + &territory + &modifier + &".toml");
        file_names.push(self.get_language().to_owned() + &territory + &".toml");
        file_names.push(self.get_language().to_owned() + &modifier + &".toml");
        file_names.push(self.get_language().to_owned() + &".toml");
        file_names
    }

    pub fn get_best_file_path(&self, path: String) -> String {
        let four_best = self.four_best_file_names();

        for name in four_best.iter() {
            for option in read_dir(&path).expect(format!("there was no dir at {}", &path).as_str())
            {
                let dir = option.unwrap();
                if dir.file_type().unwrap().is_file() {
                    if dir.file_name() == name.as_str() {
                        return path + name;
                    }
                }
            }
        }
        for option in read_dir(&path).unwrap() {
            let dir = option.unwrap();
            match dir
                .file_name()
                .into_string()
                .unwrap()
                .strip_prefix(self.get_language())
            {
                Some(_) => return path + dir.file_name().to_str().unwrap(),
                None => todo!(),
            }
        }
        for option in read_dir(&path).unwrap() {
            let dir = option.unwrap();
            match dir.file_name().into_string().unwrap().strip_prefix("en") {
                Some(_) => return path + dir.file_name().to_str().unwrap(),
                None => todo!(),
            }
        }
        "failed_to_find_language_file".to_string()
    }
}

impl Display for PosixLanguage {
    fn fmt(&self, f: &mut Formatter<'_>) -> FormatResult {
        write!(
            f,
            "lang: {}, terr: {}, encd: {}, mod: {}",
            self.get_language(),
            self.get_territory(),
            self.get_encoding(),
            self.get_modifier()
        )
    }
}