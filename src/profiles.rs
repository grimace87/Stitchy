
pub const PROFILE_FILE_NAME: &str = ".stitchyrc";

pub struct Profile {
    path: Option<std::path::PathBuf>
}

impl Profile {

    pub fn main() -> Profile {
        Profile { path: Self::get_profile_file() }
    }

    #[cfg(test)]
    pub fn test_file() -> Profile {
        let mut path = std::env::current_dir().unwrap();
        path.push(".testrc");
        Profile { path: Some(path) }
    }

    pub fn to_string(self) -> Option<String> {
        let path = self.path?;
        match std::fs::read_to_string(path) {
            Ok(json) => Some(json),
            Err(_) => None
        }
    }

    pub fn write_string(self, contents: String) {
        match self.path {
            Some(path) => {
                if let Err(e) = std::fs::write(path, contents) {
                    println!("Error writing user defaults: {:?}", e);
                }
            },
            None => {
                println!("The user defaults could not be determined.");
            }
        }
    }

    pub fn delete(self) {
        if let Some(path) = self.path {
            if std::fs::remove_file(path).is_err() {
                println!("User defaults were not deleted.");
            }
        }
    }

    fn get_profile_file() -> Option<std::path::PathBuf> {
        let mut buff = home::home_dir()?;
        buff.push(PROFILE_FILE_NAME);
        Some(buff)
    }
}

#[cfg(test)]
mod test {
    use super::Profile;

    #[test]
    fn text_written_reads_back() {
        let test_content = "{\"greeting\":\"Ahoy!\"}".to_owned();
        Profile::test_file().write_string(test_content.clone());
        let retrieved = Profile::test_file().to_string()
            .expect("Could not read written file");
        assert_eq!(test_content, retrieved);
    }
}
