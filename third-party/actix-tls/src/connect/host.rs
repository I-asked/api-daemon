//! The [`Host`] trait.

/// An interface for types where host parts (hostname and port) can be derived.
///
/// The [WHATWG URL Standard] defines the terminology used for this trait and its methods.
///
/// ```plain
/// +------------------------+
/// |          host          |
/// +-----------------+------+
/// |    hostname     | port |
/// |                 |      |
/// | sub.example.com : 8080 |
/// +-----------------+------+
/// ```
///
/// [WHATWG URL Standard]: https://url.spec.whatwg.org/
pub trait Host: Unpin + 'static {
    /// Extract hostname.
    fn hostname(&self) -> &str;

    /// Extract optional port.
    fn port(&self) -> Option<u16> {
        None
    }
}

impl Host for String {
    fn hostname(&self) -> &str {
        str_split_once(self, ':')
            .map(|(hostname, _)| hostname)
            .unwrap_or(self)
    }

    fn port(&self) -> Option<u16> {
        str_split_once(self, ':').and_then(|(_, port)| port.parse().ok())
    }
}

impl Host for &'static str {
    fn hostname(&self) -> &str {
        str_split_once(self, ':')
            .map(|(hostname, _)| hostname)
            .unwrap_or(self)
    }

    fn port(&self) -> Option<u16> {
        str_split_once(self, ':').and_then(|(_, port)| port.parse().ok())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! assert_connection_info_eq {
        ($req:expr, $hostname:expr, $port:expr) => {{
            assert_eq!($req.hostname(), $hostname);
            assert_eq!($req.port(), $port);
        }};
    }

    #[test]
    fn host_parsing() {
        assert_connection_info_eq!("example.com", "example.com", None);
        assert_connection_info_eq!("example.com:8080", "example.com", Some(8080));
        assert_connection_info_eq!("example:8080", "example", Some(8080));
        assert_connection_info_eq!("example.com:false", "example.com", None);
        assert_connection_info_eq!("example.com:false:false", "example.com", None);
    }
}

// `str::split_once` is stabilized in 1.52.0
fn str_split_once(str: &str, delimiter: char) -> Option<(&str, &str)> {
    let mut splitn = str.splitn(2, delimiter);
    let prefix = splitn.next()?;
    let suffix = splitn.next()?;
    Some((prefix, suffix))
}
