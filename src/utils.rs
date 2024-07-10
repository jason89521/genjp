use std::ffi::OsString;

const IGNORE_FILES: [&str; 2] = ["node_modules", "pnpm-lock.yaml"];
pub const PACKAGE_JSON: &str = "package.json";
pub const PNPM_WORKSPACE_YAML: &str = "pnpm-workspace.yaml";

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

pub fn should_ignore<T: PartialEq<str> + ?Sized>(file_name: &T) -> bool {
    if IGNORE_FILES
        .iter()
        .filter(|&&file| file_name == file)
        .count()
        == 0
    {
        false
    } else {
        true
    }
}

#[cfg(test)]
mod test {
    use crate::utils::{is_special_file, should_ignore, SpecialFile, IGNORE_FILES, PACKAGE_JSON};
    use std::ffi::OsString;

    #[test]
    fn check_special_files() {
        assert_eq!(
            is_special_file(&OsString::from(PACKAGE_JSON)),
            Some(SpecialFile::PackageJSON)
        );
    }

    #[test]
    fn ignore_files() {
        assert!(!should_ignore(PACKAGE_JSON));
        for item in IGNORE_FILES.iter() {
            assert!(should_ignore(&OsString::from(item)))
        }
    }
}
