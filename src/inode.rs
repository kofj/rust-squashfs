use super::*;
use std::io::Result;
use std::mem;

#[repr(u16)]
#[derive(Clone, SmartDefault)]
pub enum InodeType {
  #[default]
  BasicDirectory = 0,
  BasicFile,
  BasicSymlink,
  BasicBlockDevice,
  BasicCharDevice,
  BasicFifo,
  BasicSocket,
  ExtendedDirectory,
  ExtendedFile,
  ExtendedSymlink,
  ExtendedBlockDevice,
  ExtendedCharDevice,
  ExtendedFifo,
  ExtendedSocket,
}

pub const BASIC_DIRECTORY_BODY_SIZE: usize = mem::size_of::<BasicDirectory>();
pub struct BasicDirectory {
  /// The index of the block in the Directory Table where the directory entry information starts
  pub block_idx: u32,

  /// The number of hard links to this directory
  pub nlink: u32,

  /// Total (uncompressed) size in bytes of the entries in the Directory Table, including headers
  pub size: u16,

  /// The (uncompressed) offset within the block in the Directory Table where the directory entry information starts
  pub offset: u16,

  /// The inode_number of the parent of this directory. If this is the root directory, this will be 1
  pub parent_inode: u32,
}

pub const BASIC_FILE_BODY_SIZE: usize = mem::size_of::<BasicFile>();
pub struct BasicFile {
  /// The offset from the start of the archive where the data blocks are stored
  pub block_idx: u32,

  /// The index of a fragment entry in the fragment table which describes the data block the fragment of this file is stored in. If this file does not end with a fragment, this should be 0xFFFFFFFF
  pub fragment_block_idx: u32,

  /// The (uncompressed) offset within the fragment data block where the fragment for this file. Information about the fragment can be found at `fragment_block_index`. The size of the fragment can be found as `file_size % superblock.block_size` If this file does not end with a fragment, the value of this field is undefined (probably zero)
  pub offset: u32,

  /// The (uncompressed) size of this file
  pub size: u32,
}

pub const BASIC_SYMLINK_BODY_SIZE: usize = mem::size_of::<BasicSymlink>();
pub struct BasicSymlink {
  /// The number of hard links to this directory
  pub nlink: u32,

  /// The size in bytes of the `target_path` this symlink points to
  pub target_size: u32,
}

pub const BASIC_BLOCK_BODY_SIZE: usize = mem::size_of::<BasicBlockDevice>();
pub struct BasicBlockDevice {
  /// The number of hard links to this directory
  pub nlink: u32,

  /// To extract the major device number, (device & 0xfff00) >> 8. To extract the minor device number, use (dev & 0xff) | ((dev >> 12) & 0xfff00)
  pub device: u32,
}

pub const BASIC_CHAR_BODY_SIZE: usize = mem::size_of::<BasicCharDevice>();
pub struct BasicCharDevice {
  /// The number of hard links to this directory
  pub nlink: u32,

  /// To extract the major device number, (device & 0xfff00) >> 8. To extract the minor device number, use (dev & 0xff) | ((dev >> 12) & 0xfff00)
  pub device: u32,
}

pub const BASIC_FIFO_BODY_SIZE: usize = mem::size_of::<BasicFifo>();
pub struct BasicFifo {
  /// The number of hard links to this directory
  pub nlink: u32,
}

pub const BASIC_SOCKET_BODY_SIZE: usize = mem::size_of::<BasicSocket>();
pub struct BasicSocket {
  /// The number of hard links to this directory
  pub nlink: u32,
}

pub const EXTENDED_DIRECTORY_BODY_SIZE: usize = mem::size_of::<ExtendedDirectory>();
pub struct ExtendedDirectory {
  /// The number of hard links to this directory
  pub nlink: u32,

  /// Total (uncompressed) size in bytes of the entries in the Directory Table, including headers
  pub size: u32,

  /// The index of the block in the Directory Table where the directory entry information starts
  pub block_idx: u32,

  /// The inode_number of the parent of this directory. If this is the root directory, this will be 1
  pub parent_inode: u32,

  /// The number of directory index entries following the inode structure
  pub inodex_count: u16,

  /// The (uncompressed) offset within the block in the Directory Table where the directory entry information starts
  pub offset: u16,

  /// An index into the xattr lookup table. Set to 0xFFFFFFFF if the inode has no extended attributes
  pub xattr_idx: u32,
}

pub const EXTENDED_FILE_BODY_SIZE: usize = mem::size_of::<ExtendedFile>();
pub struct ExtendedFile {
  /// The offset from the start of the archive where the data blocks are stored
  pub block_idx: u64,

  /// The (uncompressed) size of this file
  pub size: u64,

  /// The number of bytes saved by omitting blocks of zero bytes. Used in the kernel for sparse file accounting
  pub sparse: u64,

  /// The number of hard links to this node
  pub nlink: u32,

  /// The index of a fragment entry in the fragment table which describes the data block the fragment of this file is stored in. If this file does not end with a fragment, this should be 0xFFFFFFFF
  pub fragment_block_idx: u32,

  /// The (uncompressed) offset within the fragment data block where the fragment for this file. Information about the fragment can be found at `fragment_block_index`. The size of the fragment can be found as `file_size % superblock.block_size` If this file does not end with a fragment, the value of this field is undefined (probably zero)
  pub offset: u32,

  /// An index into the xattr lookup table. Set to 0xFFFFFFFF if the inode has no extended attributes
  pub xattr_idx: u32,
}

pub const EXTENDED_SYMLINK_BODY_SIZE: usize = mem::size_of::<ExtendedSymlink>();
pub struct ExtendedSymlink {
  /// The number of hard links to this directory
  pub nlink: u32,

  /// The size in bytes of the `target_path` this symlink points to
  pub target_size: u32,

  /// An index into the xattr lookup table. Set to 0xFFFFFFFF if the inode has no extended attributes
  pub xattr_idx: u32,
}

pub const EXTENDED_BLOCK_BODY_SIZE: usize = mem::size_of::<ExtendedBlock>();
pub struct ExtendedBlock {
  /// The number of hard links to this directory
  pub nlink: u32,

  /// To extract the major device number, (device & 0xfff00) >> 8. To extract the minor device number, use (dev & 0xff) | ((dev >> 12) & 0xfff00)
  pub device: u32,

  /// An index into the xattr lookup table. Set to 0xFFFFFFFF if the inode has no extended attributes
  pub xattr_idx: u32,
}

pub const EXTENDED_CHAR_BODY_SIZE: usize = mem::size_of::<ExtendedChar>();
pub struct ExtendedChar {
  /// The number of hard links to this directory
  pub nlink: u32,

  /// To extract the major device number, (device & 0xfff00) >> 8. To extract the minor device number, use (dev & 0xff) | ((dev >> 12) & 0xfff00)
  pub device: u32,

  /// An index into the xattr lookup table. Set to 0xFFFFFFFF if the inode has no extended attributes
  pub xattr_idx: u32,
}

pub const EXTENDED_FIFO_BODY_SIZE: usize = mem::size_of::<ExtendedFifo>();
pub struct ExtendedFifo {
  /// The number of hard links to this directory
  pub nlink: u32,

  /// An index into the xattr lookup table. Set to 0xFFFFFFFF if the inode has no extended attributes
  pub xattr_idx: u32,
}

pub const EXTENDED_SOCKET_BODY_SIZE: usize = mem::size_of::<ExtendedSocket>();
pub struct ExtendedSocket {
  /// The number of hard links to this directory
  pub nlink: u32,

  /// An index into the xattr lookup table. Set to 0xFFFFFFFF if the inode has no extended attributes
  pub xattr_idx: u32,
}

///
/// Inodes
///
#[derive(Clone, Default)]
pub struct InodeTab {
  pub(crate) data: Vec<InodeHeader>,
}
impl_converter!(InodeTab);

impl InodeTab {
  pub fn new() -> Self {
    Self::default()
  }

  pub fn load(&mut self, r: &mut SqsIoReader) -> Result<()> {
    r.read_exact(self.as_mut())?;
    Ok(())
  }
}

#[derive(Clone, Default)]
pub struct InodeHeader {
  /// The type of item described by the inode which follows this header.
  pub inode_type: InodeType,

  /// A bitmask representing the permissions for the item described by the inode. The values match with the permission values of mode_t (the mode bits, not the file type)
  pub permissions: u16,

  /// The index of the user id in the UID/GID Table
  pub uid_idx: u16,

  /// The index of the group id in the UID/GID Table
  pub gid_idx: u16,

  /// The unsigned number of seconds (not counting leap seconds) since 00:00, Jan 1 1970 UTC when the item described by the inode was last modified
  pub modified_time: u32,

  /// The position of this inode in the full list of inodes. Value should be in the range [1, inode_count](from the superblock) This can be treated as a unique identifier for this inode, and can be used as a key to recreate hard links: when processing the archive, remember the visited values of inode_number. If an inode number has already been visited, this inode is hardlinked
  pub inode_number: u32,
}

impl_converter!(InodeHeader);

pub fn get_inode(_block: u32, _offset: u32, _inode_type: u16) -> Result<()> {
  Ok(())
}

#[cfg(test)]
mod tests {
  // use crate::tests::*;
  use crate::*;
  use std::io::Result;

  #[test]
  #[cfg_attr(not(feature = "gzip.sqs"), ignore)]
  fn test_get_inode() -> Result<()> {
    // TODO
    // get_inode(block: u32, offset: u32, inode_type: u16)
    Ok(())
  }

  #[test]
  fn test_inode_type_struct_size() -> Result<()> {
    assert_eq!(BASIC_DIRECTORY_BODY_SIZE, 16);
    assert_eq!(BASIC_FILE_BODY_SIZE, 16);
    assert_eq!(BASIC_SYMLINK_BODY_SIZE, 8);
    assert_eq!(BASIC_BLOCK_BODY_SIZE, 8);
    assert_eq!(BASIC_CHAR_BODY_SIZE, 8);
    assert_eq!(BASIC_FIFO_BODY_SIZE, 4);
    assert_eq!(BASIC_SOCKET_BODY_SIZE, 4);

    assert_eq!(EXTENDED_DIRECTORY_BODY_SIZE, 24);
    assert_eq!(EXTENDED_FILE_BODY_SIZE, 40);
    assert_eq!(EXTENDED_SYMLINK_BODY_SIZE, 12);
    assert_eq!(EXTENDED_BLOCK_BODY_SIZE, 12);
    assert_eq!(EXTENDED_CHAR_BODY_SIZE, 12);
    assert_eq!(EXTENDED_FIFO_BODY_SIZE, 8);
    assert_eq!(EXTENDED_SOCKET_BODY_SIZE, 8);

    Ok(())
  }
}
