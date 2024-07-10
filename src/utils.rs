use std::ffi::OsString;

const PACKAGE_JSON: &str = "package.json";

#[derive(Debug, PartialEq, Eq)]
pub enum SpecialFile {
    PackageJSON,
}

pub fn is_special_file(file_name: &OsString) -> Option<SpecialFile> {
    if let Some(file_name) = file_name.to_str() {
        match file_name {
            PACKAGE_JSON => Some(SpecialFile::PackageJSON),
            _ => None,
        }
    } else {
        None
    }
}

#[cfg(test)]
mod test {
    use super::{is_special_file, SpecialFile, PACKAGE_JSON};
    use std::ffi::OsString;

    #[test]
    fn check_special_files() {
        assert_eq!(
            is_special_file(&OsString::from(PACKAGE_JSON)),
            Some(SpecialFile::PackageJSON)
        );
    }
}
