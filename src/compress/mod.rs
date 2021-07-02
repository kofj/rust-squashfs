use flate2::read::GzDecoder;
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

pub fn decompress(raw: &[u8], output: &mut [u8], algorithm: Algorithm) -> Result<usize> {
  match algorithm {
    Algorithm::Gzip => {
      let mut gzd = GzDecoder::new(raw);
      gzd.read_exact(output)?;

      Ok(output.len())
    }
    _ => Ok(output.len()),
  }
}
