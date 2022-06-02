//! A path type exposed from the configuration crate
//!
//! This type allows the user to specify paths as strings, with some
//! support for tab expansion and user directory support.

use std::path::{Path, PathBuf};

use directories::{BaseDirs, ProjectDirs};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

use tor_error::{ErrorKind, HasKind};

/// A path in a configuration file: tilde expansion is performed, along
/// with expansion of certain variables.
///
/// The supported variables are:
///   * `ARTI_CACHE`: an arti-specific cache directory.
///   * `ARTI_CONFIG`: an arti-specific configuration directory.
///   * `ARTI_SHARED_DATA`: an arti-specific directory in the user's "shared
///     data" space.
///   * `ARTI_LOCAL_DATA`: an arti-specific directory in the user's "local
///     data" space.
///   * `USER_HOME`: the user's home directory.
///
/// These variables are implemented using the `directories` crate, and
/// so should use appropriate system-specific overrides under the
/// hood. (Some of those overrides are based on environment variables.)
/// For more information, see that crate's documentation.
///
/// Alternatively, a `CfgPath` can contain literal `PathBuf`, which will not be expaneded.
#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
#[serde(transparent)]
pub struct CfgPath(PathInner);

/// Inner implementation of CfgPath
///
/// `PathInner` exists to avoid making the variants part of the public Rust API
#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
#[serde(untagged)]
enum PathInner {
    /// A path that should be used literally, with no expansion.
    Literal(LiteralPath),
    /// A path that should be expanded from a string using ShellExpand.
    Shell(String),
}

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
/// Inner implementation of PathInner:Literal
///
/// `LiteralPath` exists to arrange that `PathInner::Literal`'s (de)serialization
/// does not overlap with `PathInner::Shell`'s.
struct LiteralPath {
    ///
    literal: PathBuf,
}

/// An error that has occurred while expanding a path.
#[derive(thiserror::Error, Debug, Clone)]
#[non_exhaustive]
pub enum CfgPathError {
    /// The path contained a variable we didn't recognize.
    #[error("unrecognized variable {0}")]
    UnknownVar(String),
    /// We couldn't construct a ProjectDirs object.
    #[error("can't construct project directories")]
    NoProjectDirs,
    /// We couldn't construct a BaseDirs object.
    #[error("can't construct base directories")]
    NoBaseDirs,
    /// We couldn't convert a variable to UTF-8.
    ///
    /// (This is due to a limitation in the shellexpand crate, which should
    /// be fixed in the future.)
    #[error("can't convert value of {0} to UTF-8")]
    BadUtf8(String),
    /// We couldn't convert a string to a valid path on the OS.
    #[error("invalid path string: {0:?}")]
    InvalidString(String),
}

impl HasKind for CfgPathError {
    fn kind(&self) -> ErrorKind {
        use CfgPathError as E;
        use ErrorKind as EK;
        match self {
            E::UnknownVar(_) | E::InvalidString(_) => EK::InvalidConfig,
            E::NoProjectDirs | E::NoBaseDirs => EK::NoHomeDirectory,
            E::BadUtf8(_) => {
                // Arguably, this should be a new "unsupported config"  type,
                // since it isn't truly "invalid" to have a string with bad UTF8
                // when it's going to be used as a filename.
                //
                // With luck, however, this error will cease to exist when shellexpand
                // improves its character-set handling.
                EK::InvalidConfig
            }
        }
    }
}

impl CfgPath {
    /// Create a new configuration path
    pub fn new(s: String) -> Self {
        CfgPath(PathInner::Shell(s))
    }

    /// Construct a new `CfgPath` designating a literal not-to-be-expanded `PathBuf`
    pub fn new_literal<P: Into<PathBuf>>(path: P) -> Self {
        CfgPath(PathInner::Literal(LiteralPath {
            literal: path.into(),
        }))
    }

    /// Return the path on disk designated by this `CfgPath`.
    pub fn path(&self) -> Result<PathBuf, CfgPathError> {
        match &self.0 {
            PathInner::Shell(s) => expand(s),
            PathInner::Literal(LiteralPath { literal }) => Ok(literal.clone()),
        }
    }

    /// If the `CfgPath` is a string that should be expaneded, return the (unexpanded) string,
    ///
    /// Before use, this string would have be to expanded.  So if you want a path to actually use,
    /// call `path` instead.
    ///
    /// Returns `None` if the `CfgPath` is a literal `PathBuf` not intended for expansion.
    pub fn as_unexpanded_str(&self) -> Option<&str> {
        match &self.0 {
            PathInner::Shell(s) => Some(s),
            PathInner::Literal(_) => None,
        }
    }

    /// If the `CfgPath` designates a literal not-to-be-expanded `Path`, return a reference to it
    ///
    /// Returns `None` if the `CfgPath` is a string which should be expanded, which is the
    /// usual case.
    pub fn as_literal_path(&self) -> Option<&Path> {
        match &self.0 {
            PathInner::Shell(_) => None,
            PathInner::Literal(LiteralPath { literal }) => Some(literal),
        }
    }
}

/// Helper: expand a directory given as a string.
#[cfg(feature = "expand-paths")]
fn expand(s: &str) -> Result<PathBuf, CfgPathError> {
    Ok(shellexpand::full_with_context(s, get_home, get_env)
        .map_err(|e| e.cause)?
        .into_owned()
        .into())
}

/// Helper: convert a string to a path without expansion.
#[cfg(not(feature = "expand-paths"))]
fn expand(s: &str) -> Result<PathBuf, CfgPathError> {
    s.try_into()
        .map_err(|_| CfgPathError::InvalidString(s.to_owned()))
}

/// Shellexpand helper: return the user's home directory if we can.
#[cfg(feature = "expand-paths")]
fn get_home() -> Option<&'static Path> {
    base_dirs().ok().map(BaseDirs::home_dir)
}

/// Shellexpand helper: Expand a shell variable if we can.
#[cfg(feature = "expand-paths")]
fn get_env(var: &str) -> Result<Option<&'static str>, CfgPathError> {
    let path = match var {
        "ARTI_CACHE" => project_dirs()?.cache_dir(),
        "ARTI_CONFIG" => project_dirs()?.config_dir(),
        "ARTI_SHARED_DATA" => project_dirs()?.data_dir(),
        "ARTI_LOCAL_DATA" => project_dirs()?.data_local_dir(),
        "USER_HOME" => base_dirs()?.home_dir(),
        _ => return Err(CfgPathError::UnknownVar(var.to_owned())),
    };

    match path.to_str() {
        // Note that we never return Ok(None) -- an absent variable is
        // always an error.
        Some(s) => Ok(Some(s)),
        // Note that this error is necessary because shellexpand
        // doesn't currently handle OsStr.  In the future, that might
        // change.
        None => Err(CfgPathError::BadUtf8(var.to_owned())),
    }
}

impl std::fmt::Display for CfgPath {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.0 {
            PathInner::Literal(LiteralPath { literal }) => write!(fmt, "{:?} [exactly]", literal),
            PathInner::Shell(s) => s.fmt(fmt),
        }
    }
}

/// Return a ProjectDirs object for the Arti project.
#[cfg(feature = "expand-paths")]
fn project_dirs() -> Result<&'static ProjectDirs, CfgPathError> {
    /// lazy cell holding the ProjectDirs object.
    static PROJECT_DIRS: Lazy<Option<ProjectDirs>> =
        Lazy::new(|| ProjectDirs::from("org", "torproject", "Arti"));

    PROJECT_DIRS.as_ref().ok_or(CfgPathError::NoProjectDirs)
}

/// Return a UserDirs object for the current user.
#[cfg(feature = "expand-paths")]
fn base_dirs() -> Result<&'static BaseDirs, CfgPathError> {
    /// lazy cell holding the BaseDirs object.
    static BASE_DIRS: Lazy<Option<BaseDirs>> = Lazy::new(BaseDirs::new);

    BASE_DIRS.as_ref().ok_or(CfgPathError::NoBaseDirs)
}

#[cfg(all(test, feature = "expand-paths"))]
mod test {
    #![allow(clippy::unwrap_used)]
    use super::*;

    #[test]
    fn expand_no_op() {
        let p = CfgPath::new("Hello/world".to_string());
        assert_eq!(p.to_string(), "Hello/world".to_string());
        assert_eq!(p.path().unwrap().to_str(), Some("Hello/world"));

        let p = CfgPath::new("/usr/local/foo".to_string());
        assert_eq!(p.to_string(), "/usr/local/foo".to_string());
        assert_eq!(p.path().unwrap().to_str(), Some("/usr/local/foo"));
    }

    #[cfg(not(target_family = "windows"))]
    #[test]
    fn expand_home() {
        let p = CfgPath::new("~/.arti/config".to_string());
        assert_eq!(p.to_string(), "~/.arti/config".to_string());

        let expected = dirs::home_dir().unwrap().join(".arti/config");
        assert_eq!(p.path().unwrap().to_str(), expected.to_str());

        let p = CfgPath::new("${USER_HOME}/.arti/config".to_string());
        assert_eq!(p.to_string(), "${USER_HOME}/.arti/config".to_string());
        assert_eq!(p.path().unwrap().to_str(), expected.to_str());
    }

    #[cfg(target_family = "windows")]
    #[test]
    fn expand_home() {
        let p = CfgPath::new("~\\.arti\\config".to_string());
        assert_eq!(p.to_string(), "~\\.arti\\config".to_string());

        let expected = dirs::home_dir().unwrap().join(".arti\\config");
        assert_eq!(p.path().unwrap().to_str(), expected.to_str());

        let p = CfgPath::new("${USER_HOME}\\.arti\\config".to_string());
        assert_eq!(p.to_string(), "${USER_HOME}\\.arti\\config".to_string());
        assert_eq!(p.path().unwrap().to_str(), expected.to_str());
    }

    #[cfg(not(target_family = "windows"))]
    #[test]
    fn expand_cache() {
        let p = CfgPath::new("${ARTI_CACHE}/example".to_string());
        assert_eq!(p.to_string(), "${ARTI_CACHE}/example".to_string());

        let expected = project_dirs().unwrap().cache_dir().join("example");
        assert_eq!(p.path().unwrap().to_str(), expected.to_str());
    }

    #[cfg(target_family = "windows")]
    #[test]
    fn expand_cache() {
        let p = CfgPath::new("${ARTI_CACHE}\\example".to_string());
        assert_eq!(p.to_string(), "${ARTI_CACHE}\\example".to_string());

        let expected = project_dirs().unwrap().cache_dir().join("example");
        assert_eq!(p.path().unwrap().to_str(), expected.to_str());
    }

    #[test]
    fn expand_bogus() {
        let p = CfgPath::new("${ARTI_WOMBAT}/example".to_string());
        assert_eq!(p.to_string(), "${ARTI_WOMBAT}/example".to_string());

        assert!(matches!(p.path(), Err(CfgPathError::UnknownVar(_))));
        assert_eq!(
            &p.path().unwrap_err().to_string(),
            "unrecognized variable ARTI_WOMBAT"
        );
    }

    #[test]
    fn literal() {
        let p = CfgPath::new_literal(PathBuf::from("${ARTI_CACHE}/literally"));
        // This doesn't get expanded, since we're using a literal path.
        assert_eq!(
            p.path().unwrap().to_str().unwrap(),
            "${ARTI_CACHE}/literally"
        );
        assert_eq!(p.to_string(), "\"${ARTI_CACHE}/literally\" [exactly]");
    }
}

#[cfg(test)]
mod test_serde {
    use super::*;
    use std::ffi::OsString;
    use std::fmt::Debug;

    #[derive(Serialize, Deserialize, Eq, PartialEq, Debug)]
    struct TestConfigFile {
        p: CfgPath,
    }

    fn deser_json(json: &str) -> CfgPath {
        dbg!(json);
        let TestConfigFile { p } = serde_json::from_str(json).expect("deser json failed");
        p
    }
    fn deser_toml(toml: &str) -> CfgPath {
        dbg!(toml);
        let TestConfigFile { p } = toml::from_str(toml).expect("deser toml failed");
        p
    }
    fn deser_toml_cfg(toml: &str) -> CfgPath {
        dbg!(toml);
        let cfg = config::File::from_str(toml, config::FileFormat::Toml);
        let cfg = config::Config::builder()
            .add_source(cfg)
            .build()
            .expect("parse toml failed");
        dbg!(&cfg);
        let TestConfigFile { p } = cfg.try_deserialize().expect("deser cfg failed");
        p
    }

    #[test]
    fn test_parse() {
        fn desers(toml: &str, json: &str) -> Vec<CfgPath> {
            vec![deser_toml(toml), deser_toml_cfg(toml), deser_json(json)]
        }

        for cp in desers(r#"p = "string""#, r#"{ "p": "string" }"#) {
            assert_eq!(cp.as_unexpanded_str(), Some("string"));
            assert_eq!(cp.as_literal_path(), None);
        }

        for cp in desers(
            r#"p = { literal = "lit" }"#,
            r#"{ "p": {"literal": "lit"} }"#,
        ) {
            assert_eq!(cp.as_unexpanded_str(), None);
            assert_eq!(cp.as_literal_path(), Some(&*PathBuf::from("lit")));
        }
    }

    fn non_string_path() -> PathBuf {
        #[cfg(target_family = "unix")]
        {
            use std::os::unix::ffi::OsStringExt;
            return PathBuf::from(OsString::from_vec(vec![0x80_u8]));
        }

        #[cfg(target_family = "windows")]
        {
            use std::os::windows::ffi::OsStringExt;
            return PathBuf::from(OsString::from_wide(&[0xD800_u16]));
        }

        #[allow(unreachable_code)]
        // Cannot test non-Stringy Paths on this platform
        PathBuf::default()
    }

    fn test_roundtrip_cases<SER, S, DESER, E, F>(ser: SER, deser: DESER)
    where
        SER: Fn(&TestConfigFile) -> Result<S, E>,
        DESER: Fn(&S) -> Result<TestConfigFile, F>,
        S: Debug,
        E: Debug,
        F: Debug,
    {
        let case = |easy, p| {
            let input = TestConfigFile { p };
            let s = match ser(&input) {
                Ok(s) => s,
                Err(e) if easy => panic!("ser failed {:?} e={:?}", &input, &e),
                Err(_) => return,
            };
            dbg!(&input, &s);
            let output = deser(&s).expect("deser failed");
            assert_eq!(&input, &output, "s={:?}", &s);
        };

        case(true, CfgPath::new("string".into()));
        case(true, CfgPath::new_literal(PathBuf::from("nice path")));
        case(true, CfgPath::new_literal(PathBuf::from("path with ✓")));

        // Non-UTF-8 paths are really hard to serialize.  We allow the serializsaton
        // to fail, and if it does, we skip the rest of the round trip test.
        // But, if they did serialise, we want to make sure that we can deserialize.
        // Hence this test case.
        case(false, CfgPath::new_literal(non_string_path()));
    }

    #[test]
    fn roundtrip_json() {
        test_roundtrip_cases(
            |input| serde_json::to_string(&input),
            |json| serde_json::from_str(json),
        );
    }

    #[test]
    fn roundtrip_toml() {
        test_roundtrip_cases(|input| toml::to_string(&input), |toml| toml::from_str(toml));
    }

    #[test]
    fn roundtrip_mpack() {
        test_roundtrip_cases(
            |input| rmp_serde::to_vec(&input),
            |mpack| rmp_serde::from_slice(mpack),
        );
    }
}