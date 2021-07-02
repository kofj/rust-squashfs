use crate::compress::Algorithm;
use crate::SqsIoReader;
use prettytable::Table;
use std::fmt;
use std::io::Result;
use std::mem::size_of;

pub const MAGIC_NUMBER: u32 = 0x7371_7368;
pub const VERSION_MAJOR: u16 = 4;
pub const VERSION_MINOR: u16 = 0;

macro_rules! impl_converter {
  ($T: ty) => {
    impl AsRef<[u8]> for $T {
      #[inline]
      fn as_ref(&self) -> &[u8] {
        let ptr = self as *const $T as *const u8;
        unsafe { &*std::slice::from_raw_parts(ptr, size_of::<$T>()) }
      }
    }

    impl AsMut<[u8]> for $T {
      #[inline]
      fn as_mut(&mut self) -> &mut [u8] {
        let ptr = self as *mut $T as *mut u8;
        unsafe { &mut *std::slice::from_raw_parts_mut(ptr, size_of::<$T>()) }
      }
    }
  };
}

macro_rules! has_flag {
  ($H: ident, $F: ident) => {
    #[inline]
    pub fn $H(&self) -> bool {
      self.contains(Flags::$F)
    }
  };
}

#[repr(C)]
#[derive(Debug, Default)]
pub struct Superblock {
  /// Must match the value of 0x73717368 to be considered a squashfs archive
  pub magic: u32,

  /// The number of inodes stored in the inode table
  pub inode_count: u32,

  /// The number of seconds (not counting leap seconds) since 00:00, Jan 1 1970 UTC when the archive was created (or last appended to). This is unsigned, so it expires in the year 2106 (as opposed to 2038).
  pub modification_time: u32,

  /// The size of a data block in bytes. Must be a power of two between 4096 and 1048576 (1 MiB)
  pub block_size: u32,

  /// The number of entries in the fragment table
  pub fragment_entry_count: u32,

  /// 1 - GZIP
  /// 2 - LZMA
  /// 3 - LZO
  /// 4 - XZ
  /// 5 - LZ4
  /// 6 - ZSTD
  pub compressor: Algorithm,

  /// The log2 of block_size. If block_size and block_log do not agree, the archive is considered corrupt
  pub block_log: u16,

  /// Superblock Flags, u16
  pub flags: Flags,

  /// The unique user or group IDs number of entries in the id lookup table
  pub id_count: u16,

  /// The major version of the squashfs file format. Should always equal 4
  pub version_major: u16,

  /// The minor version of the squashfs file format. Should always equal 0
  pub version_minor: u16,

  /// A reference to the inode of the root directory of the archive
  pub root_inode_ref: InodeRef, //u64,

  /// The number of bytes used by the archive. Because squashfs archives are often padded to 4KiB, this can often be less than the file size
  pub bytes_used: u64,

  /// The byte offset at which the id table starts
  pub id_table_start: u64,

  /// The byte offset at which the xattr id table starts
  pub xattr_id_table_start: u64,

  /// The byte offset at which the inode table starts
  pub inode_table_start: u64,

  /// The byte offset at which the directory table starts
  pub directory_table_start: u64,

  /// The byte offset at which the fragment table starts
  pub fragment_table_start: u64,

  /// The byte offset at which the export table starts
  pub export_table_start: u64,
}

impl_converter!(Superblock);

impl Superblock {
  pub fn new() -> Self {
    Self::default()
  }

  pub fn load(&mut self, r: &mut SqsIoReader) -> Result<()> {
    r.read_exact(self.as_mut())?;
    Ok(())
  }

  pub fn to_table(&self) -> Table {
    table!(
      ["Field", "Value"],
      ["magic", self.magic],
      ["inode_count", self.inode_count],
      ["modification_time", self.modification_time],
      ["block_size", self.block_size],
      ["fragment_entry_count", self.fragment_entry_count],
      ["compressor", self.compressor],
      ["block_log", self.block_log],
      ["flags", self.flags.to_table()],
      ["id_count", self.id_count],
      ["version_major", self.version_major],
      ["version_minor", self.version_minor],
      ["root_inode_ref", self.root_inode_ref],
      ["bytes_used", self.bytes_used],
      ["id_table_start", self.id_table_start],
      ["xattr_id_table_start", self.xattr_id_table_start],
      ["inode_table_start", self.inode_table_start],
      ["directory_table_start", self.directory_table_start],
      ["fragment_table_start", self.fragment_table_start],
      ["export_table_start", self.export_table_start]
    )
  }
}

bitflags! {
  pub struct Flags: u16 {
    /// Inodes are stored uncompressed. For backward compatibility reasons, UID/GIDs are also stored uncompressed.
    const UNCOMPRESSED_INODES	= 0x0001;

    /// Data are stored uncompressed
    const UNCOMPRESSED_DATA	= 0x0002;

    /// Unused in squashfs 4+. Should always be unset
    const CHECK	= 0x0004;

    /// Fragments are stored uncompressed
    const UNCOMPRESSED_FRAGMENTS	= 0x0008;

    /// Fragments are not used. Files smaller than the block size are stored in a full block.
    const NO_FRAGMENTS	= 0x0010;

    /// If the last block of a file is smaller than the block size, it will be instead stored as a fragment
    const ALWAYS_FRAGMENTS	= 0x0020;

    /// Identical files are recognized, and stored only once
    const DUPLICATES	= 0x0040;

    /// Filesystem has support for export via NFS (The export table is populated)
    const EXPORTABLE	= 0x0080;

    /// Xattrs are stored uncompressed
    const UNCOMPRESSED_XATTRS	= 0x0100;

    /// Xattrs are not stored
    const NO_XATTRS	= 0x0200;

    /// The compressor options section is present
    const COMPRESSOR_OPTIONS	= 0x0400;

    /// UID/GIDs are stored uncompressed. Note that the UNCOMPRESSED_INODES flag also has this effect. If that flag is set, this flag has no effect. This flag is currently only available on master in git, no released version of squashfs yet supports it.s
    const UNCOMPRESSED_IDS	= 0x0800;
  }
}

impl Flags {
  has_flag!(uncompressed_inodes, UNCOMPRESSED_INODES);
  has_flag!(uncompressed_data, UNCOMPRESSED_DATA);
  has_flag!(check, CHECK);
  has_flag!(uncompressed_fragments, UNCOMPRESSED_FRAGMENTS);
  has_flag!(no_fragments, NO_FRAGMENTS);
  has_flag!(always_fragments, ALWAYS_FRAGMENTS);
  has_flag!(duplicates, DUPLICATES);
  has_flag!(exportable, EXPORTABLE);
  has_flag!(uncompressed_xattrs, UNCOMPRESSED_XATTRS);
  has_flag!(no_xattrs, NO_XATTRS);
  has_flag!(compressor_options, COMPRESSOR_OPTIONS);
  has_flag!(uncompressed_ids, UNCOMPRESSED_IDS);

  pub fn to_table(&self) -> Table {
    table!(
      ["Flag", "Exist"],
      [Flags::UNCOMPRESSED_INODES, self.uncompressed_inodes()],
      [Flags::UNCOMPRESSED_DATA, self.uncompressed_data()],
      [Flags::CHECK, self.check()],
      [Flags::UNCOMPRESSED_FRAGMENTS, self.uncompressed_fragments()],
      [Flags::NO_FRAGMENTS, self.no_fragments()],
      [Flags::ALWAYS_FRAGMENTS, self.always_fragments()],
      [Flags::DUPLICATES, self.duplicates()],
      [Flags::EXPORTABLE, self.exportable()],
      [Flags::UNCOMPRESSED_XATTRS, self.uncompressed_xattrs()],
      [Flags::NO_XATTRS, self.no_xattrs()],
      [Flags::COMPRESSOR_OPTIONS, self.compressor_options()],
      [Flags::UNCOMPRESSED_IDS, self.uncompressed_ids()]
    )
  }
}

impl Default for Flags {
  fn default() -> Self {
    Flags::empty()
  }
}

impl fmt::Display for Flags {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", format!("{:?}", self))?;
    Ok(())
  }
}

#[repr(C)]
#[derive(Debug, Default)]
pub struct InodeRef {
  pub offset: u16,
  pub block: u16,
  padding: u32,
}

impl fmt::Display for InodeRef {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", format!("{:?}", self))?;
    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use crate::layout::Superblock;
  use crate::layout::{MAGIC_NUMBER, VERSION_MAJOR, VERSION_MINOR};
  use crate::SqsIoReader;
  use std::fs::File;
  use std::io::Result;
  use std::mem::size_of;

  #[test]
  fn read_superblock() -> Result<()> {
    println!("Superblock size: {}", size_of::<Superblock>());
    let f = File::open("tests/data/gzip.sqs")?;
    let mut reader = Box::new(f.try_clone().unwrap()) as SqsIoReader;

    let mut sb = Superblock::new();
    sb.load(&mut reader)?;

    sb.to_table().printstd();

    assert_eq!(sb.magic, MAGIC_NUMBER);
    assert_eq!(sb.version_major, VERSION_MAJOR);
    assert_eq!(sb.version_minor, VERSION_MINOR);

    Ok(())
  }
}
