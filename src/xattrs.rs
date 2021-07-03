use super::*;
use std::io::{Result, SeekFrom};

pub const XATTR_IDENTRY_SIZE: usize = 16;
pub const XATTR_HEADER_SIZE: usize = 16;
pub const NO_XATTR_INODE_FLAG: u32 = 0xffff_ffff;
pub const NO_XATTR_SUPERBLOCK_FLAG: u64 = 0xffff_ffff_ffff_ffff;

#[repr(C)]
#[derive(Debug, Default)]
pub struct OnDiskXAttrIdTable {
  /// Locaton of first meta block holding the kv pairs.
  pub location: u64,
  /// Number of descriptors.
  pub count: u32,
  /// padding
  _padding: u32,
}

impl_converter!(OnDiskXAttrIdTable);

pub struct XAttrIndex {
  /// Location of the 1th kv pair.
  pub location: u64,
  /// Number of kv pairs.
  pub count: u32,
  /// Total size of the uncompressed kv pairs in bytes
  pub size: u32,
}

pub struct XAttrTable {
  pub location: u64,
  pub list: Vec<XAttrIndex>,
  pub data: Vec<u8>,
}

/// Single xattr key.
pub struct XAttrEntry {
  /// Encodes the prefix of the key
  pub xtype: u16,

  pub size: u16,

  ///
  pub key: u8,
}

/// TODO
pub fn read_xattrs_table(r: &mut SqsIoReader, sb: Superblock) -> Result<()> {
  let mut header = OnDiskXAttrIdTable::default();
  if sb.flags.no_xattrs() {
    return Ok(());
  }
  warn!(
    "[read_xattrs_table] xattr_id_table_start={}",
    sb.xattr_id_table_start
  );
  r.seek(SeekFrom::Start(sb.xattr_id_table_start))
    .map_err(|e| map_error!(e))?;
  r.read_exact(&mut header.as_mut())
    .map_err(|e| map_error!(e))?;
  warn!("[read_xattrs_table] header={:?}", header);

  Ok(())
}

#[cfg(test)]
mod tests {
  use crate::tests::*;
  use crate::*;
  use std::io::Result;

  #[test]
  #[cfg_attr(not(feature = "gzip.sqs"), ignore)]
  fn test_read_xattrs_table() -> Result<()> {
    let (mut reader, sb) = prepare_tests().map_err(|e| map_error!(e))?;

    read_xattrs_table(&mut reader, sb).map_err(|e| map_error!(e))?;

    Ok(())
  }
}
