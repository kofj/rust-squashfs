use super::*;
use byteorder::{ByteOrder, LittleEndian};
use std::io::{Read, Result, SeekFrom};

const ID_ENTRY_SIZE: usize = 4;

/// read gid/uid lookup table.
/// To read gids/uids lookup table,
/// 1. Get start of the index from the `Superblock`;
/// 2. Calculate table size via `id_count*ID_ENTRY_SIZE`;
/// 3. Calculate meta blocks number;
/// 4. Read the indexs, they are uncompressed, one index per metablock of the table, 8 bytes each(u64);
/// 5. Read the table.
pub fn read_lookup_table(r: &mut SqsIoReader, sb: Superblock) -> Result<()> {
  if sb.id_count == 0 {
    return Ok(());
  }
  let table_size = ID_ENTRY_SIZE * sb.id_count as usize;
  let blocks = (table_size - 1) / METADATA_BLOCK_SIZE + 1;

  let mut buf = vec![0u8; blocks * 8];
  r.seek(SeekFrom::Start(sb.id_table_start))?;
  r.read_exact(&mut buf)?;
  trace!("{:#x?}", buf);

  let total = buf.len() / 8;
  let mut idx = 0;
  let mut data: Vec<u8> = Vec::new();
  while idx < total {
    let location = LittleEndian::read_u64(&buf[idx * 8..(idx + 1) * 8]);
    idx += 1;
    let uncompressed = read_meta_block(r, sb.compressor, location)?;
    trace!(
      "[lookup tab] location={}, uncompressed.data={:x?}",
      location,
      uncompressed
    );
    data.extend(uncompressed.iter());
  }

  parse_id_tab(&mut &*data)?;

  Ok(())
}

type IdTab = Vec<u32>;

pub fn parse_id_tab(raw: &mut &[u8]) -> Result<Vec<u32>> {
  let count = raw.len() / 4;
  let mut entries = IdTab::with_capacity(count);
  unsafe {
    entries.set_len(count);
  }

  let (_, mut data, _) = unsafe { (&mut entries).align_to_mut::<u8>() };

  raw.read_exact(&mut data)?;

  trace!("[parse_id_tab] entries={:?}", entries);

  Ok(entries)
}

#[cfg(test)]
mod tests {
  use crate::tests::*;
  use crate::*;
  use std::io::Result;

  #[test]
  #[cfg_attr(not(feature = "gzip.sqs"), ignore)]
  fn test_lookup_table() -> Result<()> {
    let (mut reader, sb) = prepare_tests()?;
    read_lookup_table(&mut reader, sb)?;
    Ok(())
  }

  #[test]
  fn test_parse_id_tab() -> Result<()> {
    let raw = vec![
      0x0, 0x0, 0x0, 0x0, 0xa, 0x0, 0x0, 0x0, 0x1, 0x0, 0x2, 0x2c, 0xe6, 0x2a, 0x85, 0x7f,
    ];

    let expected = vec![0, 10, 0x2c020001, 0x7f852ae6];

    let uidsgids = parse_id_tab(&mut &*raw)?;

    assert_eq!(uidsgids.len(), expected.len());
    assert_eq!(&uidsgids, &expected);

    Ok(())
  }
}
