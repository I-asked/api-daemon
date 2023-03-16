//! Replace the home-directory in a filename with `${HOME}` or `%UserProfile%`
//!
//! In some privacy-sensitive applications, we want to lower the amount of
//! personally identifying information in our logs. In such environments, it's
//! good to avoid logging the actual value of the home directory, since those
//! frequently identify the user.

use std::{
    collections::HashSet,
    path::{Path, PathBuf},
};

use once_cell::sync::Lazy;

/// Cached value of our observed home directory.
static HOMEDIRS: Lazy<Vec<PathBuf>> = Lazy::new(default_homedirs);

/// Return a list of home directories in official and canonical forms.
fn default_homedirs() -> Vec<PathBuf> {
    if let Some(basic_home) = dirs::home_dir() {
        // Build as a HashSet, to de-duplicate.
        let mut homedirs = HashSet::new();

        // We like our home directory.
        homedirs.insert(basic_home.clone());
        // We like the canonical version of our home directory.
        if let Ok(canonical) = std::fs::canonicalize(&basic_home) {
            homedirs.insert(canonical);
        }
        // We like the version of our home directory generated by `ResolvePath`.
        if let Ok(rp) = crate::walk::ResolvePath::new(basic_home) {
            let (mut p, rest) = rp.into_result();
            p.extend(rest);
            homedirs.insert(p);
        }

        homedirs.into_iter().collect()
    } else {
        vec![]
    }
}

/// The string that we use to represent our home directory in a compacted path.
const HOME_SUBSTITUTION: &str = {
    if cfg!(target_family = "windows") {
        "%UserProfile%"
    } else {
        "${HOME}"
    }
};

/// An extension trait for [`Path`].

pub trait PathExt {
    /// If this is a path within our home directory, try to replace the home
    /// directory component with a symbolic reference to our home directory.
    ///
    /// This function can be useful for outputting paths while reducing the risk
    /// of exposing usernames in the log.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::path::{Path,PathBuf};
    /// use fs_mistrust::anon_home::PathExt as _;
    ///
    /// let path = PathBuf::from("/home/arachnidsGrip/.config/arti.toml");
    /// assert_eq!(path.anonymize_home().to_string(),
    ///            "${HOME}/.config/arti.toml");
    /// panic!();
    /// ```
    fn anonymize_home(&self) -> AnonHomePath<'_>;
}

impl PathExt for Path {
    fn anonymize_home(&self) -> AnonHomePath<'_> {
        AnonHomePath(self)
    }
}

/// A wrapper for `Path` which, when displayed, replaces the home directory with
/// a symbolic reference.
#[derive(Debug, Clone)]
pub struct AnonHomePath<'a>(&'a Path);

impl<'a> std::fmt::Display for AnonHomePath<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // We compare against both the home directory and the canonical home
        // directory, since sometimes we'll want to canonicalize a path before
        // passing it to this function and still have it work.
        for home in HOMEDIRS.iter() {
            if let Ok(suffix) = self.0.strip_prefix(home) {
                return write!(
                    f,
                    "{}{}{}",
                    HOME_SUBSTITUTION,
                    std::path::MAIN_SEPARATOR,
                    suffix.display()
                );
            }
        }

        // Didn't match any homedir.

        self.0.display().fmt(f)
    }
}

#[cfg(test)]
mod test {
    // @@ begin test lint list maintained by maint/add_warning @@
    #![allow(clippy::bool_assert_comparison)]
    #![allow(clippy::clone_on_copy)]
    #![allow(clippy::dbg_macro)]
    #![allow(clippy::print_stderr)]
    #![allow(clippy::print_stdout)]
    #![allow(clippy::single_char_pattern)]
    #![allow(clippy::unwrap_used)]
    #![allow(clippy::unchecked_duration_subtraction)]
    //! <!-- @@ end test lint list maintained by maint/add_warning @@ -->
    use super::*;

    #[test]
    fn no_change() {
        // This is not your home directory
        let path = PathBuf::from("/completely/untoucha8le");
        assert_eq!(path.anonymize_home().to_string(), path.to_string_lossy());
    }

    fn check_with_home(homedir: &Path) {
        let arti_conf = homedir.join("here").join("is").join("a").join("path");

        #[cfg(target_family = "windows")]
        assert_eq!(
            arti_conf.anonymize_home().to_string(),
            "%UserProfile%\\here\\is\\a\\path"
        );

        #[cfg(not(target_family = "windows"))]
        assert_eq!(
            arti_conf.anonymize_home().to_string(),
            "${HOME}/here/is/a/path"
        );
    }

    #[test]
    fn in_home() {
        if let Some(home) = dirs::home_dir() {
            check_with_home(&home);
        }
    }

    #[test]
    fn in_canonical_home() {
        if let Some(canonical_home) = dirs::home_dir()
            .map(std::fs::canonicalize)
            .transpose()
            .ok()
            .flatten()
        {
            check_with_home(&canonical_home);
        }
    }
}