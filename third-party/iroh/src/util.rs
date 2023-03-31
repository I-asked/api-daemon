//! Utility functions and types.
use anyhow::{ensure, Context, Result};
use base64::{engine::general_purpose, Engine as _};
use bytes::Bytes;
use derive_more::Display;
use postcard::experimental::max_size::MaxSize;
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use std::{
    fmt::{self, Display},
    io::{self, BufReader, Read, Seek},
    path::{Component, Path},
    result,
    str::FromStr,
};
use thiserror::Error;
use tokio::sync::mpsc;

/// Encode the given buffer into Base64 URL SAFE without padding.
pub fn encode(buf: impl AsRef<[u8]>) -> String {
    general_purpose::URL_SAFE_NO_PAD.encode(buf.as_ref())
}

/// Decode the given buffer from Base64 URL SAFE without padding.
pub fn decode(buf: impl AsRef<str>) -> Result<Vec<u8>, base64::DecodeError> {
    general_purpose::URL_SAFE_NO_PAD.decode(buf.as_ref())
}

/// Hash type used throught.
#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
pub struct Hash(blake3::Hash);

impl Hash {
    /// Calculate the hash of the provide bytes.
    pub fn new(buf: impl AsRef<[u8]>) -> Self {
        let val = blake3::hash(buf.as_ref());
        Hash(val)
    }
}

impl AsRef<[u8]> for Hash {
    fn as_ref(&self) -> &[u8] {
        self.0.as_bytes()
    }
}

impl From<Hash> for blake3::Hash {
    fn from(value: Hash) -> Self {
        value.0
    }
}

impl From<blake3::Hash> for Hash {
    fn from(value: blake3::Hash) -> Self {
        Hash(value)
    }
}

impl From<[u8; 32]> for Hash {
    fn from(value: [u8; 32]) -> Self {
        Hash(blake3::Hash::from(value))
    }
}

impl PartialOrd for Hash {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.0.as_bytes().cmp(other.0.as_bytes()))
    }
}

impl Ord for Hash {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.as_bytes().cmp(other.0.as_bytes())
    }
}

impl Display for Hash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", encode(self.0.as_bytes()))
    }
}

impl FromStr for Hash {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut arr = [0u8; 32];
        let val = decode(s)?;
        ensure!(
            val.len() == 32,
            "invalid byte length, expected 32, got {}",
            val.len()
        );
        arr.copy_from_slice(&val);
        let hash = blake3::Hash::from(arr);

        Ok(Hash(hash))
    }
}

impl Serialize for Hash {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_bytes(self.0.as_bytes())
    }
}

impl<'de> Deserialize<'de> for Hash {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_bytes(HashVisitor)
    }
}

struct HashVisitor;

impl<'de> de::Visitor<'de> for HashVisitor {
    type Value = Hash;

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "an array of 32 bytes containing hash data")
    }

    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let bytes: [u8; 32] = v.try_into().map_err(E::custom)?;
        Ok(Hash::from(bytes))
    }
}

impl MaxSize for Hash {
    const POSTCARD_MAX_SIZE: usize = 32;
}

/// A serializable error type for use in RPC responses.
#[derive(Serialize, Deserialize, Debug, Error)]
pub struct RpcError(serde_error::Error);

impl fmt::Display for RpcError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

impl From<anyhow::Error> for RpcError {
    fn from(e: anyhow::Error) -> Self {
        RpcError(serde_error::Error::new(&*e))
    }
}

/// A serializable result type for use in RPC responses.
#[allow(dead_code)]
pub type RpcResult<T> = result::Result<T, RpcError>;

/// Todo: gather more information about validation errors. E.g. offset
///
/// io::Error should be just the fallback when a more specific error is not available.
#[derive(Debug, Display, Error)]
pub(crate) enum BaoValidationError {
    /// Generic io error. We were unable to read the data.
    IoError(io::Error),
    // /// The hash of the data does not match the hash of the outboard.
    // HashMismatch,
    // /// The size of the data does not match the size of the outboard.
    // SizeMismatch,
}

impl From<io::Error> for BaoValidationError {
    fn from(e: io::Error) -> Self {
        BaoValidationError::IoError(e)
    }
}

/// Validate that the data matches the outboard.
pub(crate) fn validate_bao(
    hash: Hash,
    data_reader: impl Read + Seek,
    outboard: Bytes,
    progress: impl Fn(u64),
) -> result::Result<(), BaoValidationError> {
    let hash = blake3::Hash::from(hash);
    let outboard_reader = io::Cursor::new(outboard);
    let progress_reader = ProgressReader::new(data_reader, |p| {
        if let ProgressReaderUpdate::Progress(x) = p {
            progress(x)
        }
    });
    let buffered_reader = BufReader::with_capacity(1024 * 1024, progress_reader);
    let mut decoder = abao::decode::Decoder::new_outboard(buffered_reader, outboard_reader, &hash);
    // todo: expose chunk group size in abao, so people can allocate good sized buffers
    let mut buffer = vec![0u8; 1024 * 16 + 4096];
    loop {
        match decoder.read(&mut buffer) {
            Ok(0) => break,
            Ok(_) => {}
            Err(err) => {
                // todo: figure out exactly what went wrong
                return Err(BaoValidationError::IoError(err));
            }
        }
    }
    Ok(())
}

/// converts a canonicalized relative path to a string, returning an error if
/// the path is not valid unicode
///
/// this will also fail if the path is non canonical, i.e. contains `..` or `.`,
/// or if the path components contain any windows or unix path separators
pub fn canonicalize_path(path: impl AsRef<Path>) -> anyhow::Result<String> {
    let parts = path
        .as_ref()
        .components()
        .map(|c| {
            let c = if let Component::Normal(x) = c {
                x.to_str().context("invalid character in path")?
            } else {
                anyhow::bail!("invalid path component {:?}", c)
            };
            anyhow::ensure!(
                !c.contains('/') && !c.contains('\\'),
                "invalid path component {:?}",
                c
            );
            Ok(c)
        })
        .collect::<anyhow::Result<Vec<_>>>()?;
    Ok(parts.join("/"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash() {
        let data = b"hello world";
        let hash = Hash::new(data);

        let encoded = hash.to_string();
        assert_eq!(encoded.parse::<Hash>().unwrap(), hash);
    }

    #[test]
    fn test_canonicalize_path() {
        assert_eq!(canonicalize_path("foo/bar").unwrap(), "foo/bar");
    }
}

pub(crate) struct ProgressReader<R, F: Fn(ProgressReaderUpdate)> {
    inner: R,
    offset: u64,
    cb: F,
}

impl<R: Read, F: Fn(ProgressReaderUpdate)> ProgressReader<R, F> {
    pub fn new(inner: R, cb: F) -> Self {
        Self {
            inner,
            offset: 0,
            cb,
        }
    }
}

impl<R: Read, F: Fn(ProgressReaderUpdate)> Read for ProgressReader<R, F> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let read = self.inner.read(buf)?;
        self.offset += read as u64;
        (self.cb)(ProgressReaderUpdate::Progress(self.offset));
        Ok(read)
    }
}

impl<R, F: Fn(ProgressReaderUpdate)> Drop for ProgressReader<R, F> {
    fn drop(&mut self) {
        (self.cb)(ProgressReaderUpdate::Done);
    }
}

pub(crate) enum ProgressReaderUpdate {
    Progress(u64),
    Done,
}

pub struct Progress<T>(Option<mpsc::Sender<T>>);

impl<T> Clone for Progress<T> {
    fn clone(&self) -> Self {
        Progress(self.0.clone())
    }
}

impl<T: fmt::Debug + Send + Sync + 'static> Progress<T> {
    pub fn new(sender: mpsc::Sender<T>) -> Self {
        Self(Some(sender))
    }
    pub fn none() -> Self {
        Self(None)
    }
    pub fn try_send(&self, msg: T) {
        if let Some(progress) = &self.0 {
            progress.try_send(msg).ok();
        }
    }
    pub async fn send(&self, msg: T) -> anyhow::Result<()> {
        if let Some(progress) = &self.0 {
            progress.send(msg).await?;
        }
        Ok(())
    }
}