use flate2::bufread::{GzDecoder, ZlibDecoder};
use flate2::write::GzEncoder;
use flate2::Compression;
use std::borrow::Cow;
use std::fmt;
use std::io::{Read, Result, Write};

#[repr(u16)]
#[derive(Clone, Copy, Debug, PartialEq, Serialize, SmartDefault)]
pub enum Algorithm {
  #[default]
  None = 0,
  Gzip,
  Lzma,
  Lzo,
  Xz,
  Lz4,
  Zstd,
}

impl fmt::Display for Algorithm {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", format!("{:?}", self))?;
    Ok(())
  }
}

pub fn compress(raw: &[u8], algorithm: Algorithm) -> Result<(Cow<[u8]>, bool)> {
  if raw.len() == 0 {
    return Ok((Cow::Borrowed(raw), false));
  }

  let compressed = match algorithm {
    Algorithm::None => return Ok((Cow::Borrowed(raw), false)),
    Algorithm::Gzip => {
      let out: Vec<u8> = Vec::new();
      let mut gz = GzEncoder::new(out, Compression::default());
      gz.write_all(raw)?;
      gz.finish()?
    }
    // dont compress if algorithm not support
    _ => return Ok((Cow::Borrowed(raw), false)),
  };

  Ok((Cow::Owned(compressed), true))
}

/// ZLIB/GZIP headers
/// Level | ZLIB  | GZIP
///  1   | 78 01 | 1F 8B
///  2   | 78 5E | 1F 8B
///  3   | 78 5E | 1F 8B
///  4   | 78 5E | 1F 8B
///  5   | 78 5E | 1F 8B
///  6   | 78 9C | 1F 8B
///  7   | 78 DA | 1F 8B
///  8   | 78 DA | 1F 8B
///  9   | 78 DA | 1F 8B
pub fn decompress(raw: &[u8], output: &mut [u8], algorithm: Algorithm) -> Result<usize> {
  match algorithm {
    Algorithm::Gzip => {
      trace!(
        "[decompress] Gzip header={:X?} isZlib={}",
        &raw[0..1],
        &raw[0..1] == &[0x78]
      );
      match raw[0..1] {
        [0x78] => {
          let mut zlib = ZlibDecoder::new(raw);
          let size = zlib.read(output)?;
          Ok(size)
        }
        _ => {
          let mut gzd = GzDecoder::new(raw);
          gzd.read_exact(output)?;
          Ok(output.len())
        }
      }
    }
    _ => Ok(output.len()),
  }
}
