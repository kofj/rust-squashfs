use std::fmt;

pub const MAGIC_NUMBER: u32 = 0x7371_7368;

#[derive(Debug)]
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
  pub compression_id: Compression, //u16,

  /// The log2 of block_size. If block_size and block_log do not agree, the archive is considered corrupt
  pub block_log: u16,

  /// Superblock Flags, u16
  pub flags: Flags,

  /// The number of entries in the id lookup table
  pub id_count: u16,

  /// The major version of the squashfs file format. Should always equal 4
  pub version_major: u16,

  /// The minor version of the squashfs file format. Should always equal 0
  pub version_minor: u16,

  /// A reference to the inode of the root directory of the archive
  pub root_inode_ref: u64,

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

bitflags! {
  // #[derive(Clone, Debug)]
  pub struct Flags: u16 {
    // Inodes are stored uncompressed. For backward compatibility reasons, UID/GIDs are also stored uncompressed.
    const UNCOMPRESSED_INODES	= 0x0001;

    // Data are stored uncompressed
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

    /// The compression options section is present
    const COMPRESSOR_OPTIONS	= 0x0400;

    /// UID/GIDs are stored uncompressed. Note that the UNCOMPRESSED_INODES flag also has this effect. If that flag is set, this flag has no effect. This flag is currently only available on master in git, no released version of squashfs yet supports it.s
    const UNCOMPRESSED_IDS	= 0x0800;
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

#[repr(u16)]
#[derive(Debug, Serialize)]
pub enum Compression {
  None = 0,
  Gzip,
  Lzma,
  Lzo,
  Xz,
  Lz4,
  Zstd,
}

#[cfg(test)]
mod tests {
  use std::io::Result;

  #[test]
  fn read_superblock() -> Result<()> {
    Ok(())
  }
}
