use super::*;
use byteorder::{ByteOrder, LittleEndian};
use std::io::{Read, Result, SeekFrom};
use std::mem;

///
#[derive(Default, Debug)]
pub struct FragmentsTab {
  pub entries: Vec<FragmentEntry>,
}

#[derive(Default, Debug)]
pub struct FragmentEntry {
  pub start: u64,
  pub size: u32,
  pub compressed: bool,
}

#[repr(C)]
#[derive(Default, Debug)]
struct FragmentEntryInternal {
  start: u64,
  size: u32,
  _padding: u32,
}

impl_converter!(FragmentEntryInternal);

pub const FRAGMENT_SIZE: usize = mem::size_of::<FragmentEntryInternal>();
pub const UNCOMPRESSED_FRAGMENT_FLAG: u32 = 0x1000_000;

pub fn read_fragment_table(r: &mut SqsIoReader, sb: Superblock) -> Result<FragmentsTab> {
  let mut blocks = sb.fragment_entry_count / 512;
  if sb.fragment_entry_count % 512 > 0 {
    blocks += 1;
  }

  let mut tab = FragmentsTab::default();
  r.seek(SeekFrom::Start(sb.fragment_table_start))?;
  let mut buf = [0u8; 8];
  while blocks > 0 {
    r.read_exact(&mut buf)?;

    trace!("block={} buf={:?}", blocks, buf);

    let offset = LittleEndian::read_u64(&buf);
    let (metadata, _) = read_meta_block(r, sb.compressor, offset)?;

    let total = metadata.len() / FRAGMENT_SIZE;
    let mut idx = 0;
    while idx < total {
      let start = idx * FRAGMENT_SIZE;
      let end = (idx + 1) * FRAGMENT_SIZE;
      let fragment = parse_fragment(&mut &metadata[start..end])?;
      tab.entries.push(fragment);
      idx = idx + 1;
    }

    debug!(
      "[read_fragment_table] total={}, parsed={}",
      total,
      tab.entries.len()
    );

    trace!("[read_fragment_table] parsed.fragment={:?}", tab.entries);

    blocks -= 1;
  }

  Ok(tab)
}

fn is_uncompressed_fragment(s: u32) -> bool {
  s & UNCOMPRESSED_FRAGMENT_FLAG == UNCOMPRESSED_FRAGMENT_FLAG
}

fn parse_fragment(metadata: &mut &[u8]) -> Result<FragmentEntry> {
  if metadata.len() != FRAGMENT_SIZE {
    return Err(invalid_error!("invalid fragment data, should has 16 bytes"));
  }
  trace!("[parse_fragment] bytes={:x?}, {:x?}", metadata, 1 << 24);
  let mut internal = FragmentEntryInternal::default();
  metadata.read_exact(&mut internal.as_mut())?;

  Ok(FragmentEntry {
    start: internal.start,
    size: internal.size,
    compressed: !is_uncompressed_fragment(internal.size),
  })
}

#[cfg(test)]
mod tests {
  use crate::tests::*;
  use crate::*;
  use std::io::Result;

  #[test]
  #[cfg_attr(not(feature = "gzip-sqs"), ignore)]
  fn test_read_fragment_table() -> Result<()> {
    prepare_tests()?;
    let (mut reader, sb) = prepare_tests()?;
    read_fragment_table(&mut reader, sb)?;

    Ok(())
  }
}
