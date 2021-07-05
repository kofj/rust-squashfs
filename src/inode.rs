use super::*;
use std::io::Result;
use std::mem;

#[repr(u16)]
#[derive(Clone, SmartDefault, Debug)]
pub enum InodeType {
  #[default]
  BasicDirectory = 1,
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

impl InodeType {
  pub fn body_size(&self) -> usize {
    match self {
      InodeType::BasicDirectory => BASIC_DIRECTORY_BODY_SIZE,
      InodeType::BasicFile => BASIC_FILE_BODY_SIZE,
      InodeType::BasicSymlink => BASIC_SYMLINK_BODY_SIZE,
      InodeType::BasicBlockDevice => BASIC_BLOCK_BODY_SIZE,
      InodeType::BasicCharDevice => BASIC_CHAR_BODY_SIZE,
      InodeType::BasicFifo => BASIC_FIFO_BODY_SIZE,
      InodeType::BasicSocket => BASIC_SOCKET_BODY_SIZE,

      InodeType::ExtendedDirectory => EXTENDED_DIRECTORY_BODY_SIZE,
      InodeType::ExtendedFile => EXTENDED_FILE_BODY_SIZE,
      InodeType::ExtendedSymlink => EXTENDED_SYMLINK_BODY_SIZE,
      InodeType::ExtendedBlockDevice => EXTENDED_BLOCK_BODY_SIZE,
      InodeType::ExtendedCharDevice => EXTENDED_CHAR_BODY_SIZE,
      InodeType::ExtendedFifo => EXTENDED_FIFO_BODY_SIZE,
      InodeType::ExtendedSocket => EXTENDED_SOCKET_BODY_SIZE,
    }
  }
}

pub const BASIC_DIRECTORY_BODY_SIZE: usize = mem::size_of::<BasicDirectory>();
#[repr(C)]
#[derive(Clone, Default, Debug)]
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
impl_converter!(BasicDirectory);

impl InodeBody for BasicDirectory {}

pub const BASIC_FILE_BODY_SIZE: usize = mem::size_of::<BasicFile>();
#[repr(C)]
#[derive(Clone, Default, Debug)]
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
impl_converter!(BasicFile);

impl InodeBody for BasicFile {}

pub const BASIC_SYMLINK_BODY_SIZE: usize = mem::size_of::<BasicSymlink>();
#[repr(C)]
#[derive(Clone, Default, Debug)]
pub struct BasicSymlink {
  /// The number of hard links to this directory
  pub nlink: u32,

  /// The size in bytes of the `target_path` this symlink points to
  pub target_size: u32,
}
impl_converter!(BasicSymlink);

impl InodeBody for BasicSymlink {}

pub const BASIC_BLOCK_BODY_SIZE: usize = mem::size_of::<BasicBlockDevice>();
#[repr(C)]
#[derive(Clone, Default, Debug)]
pub struct BasicBlockDevice {
  /// The number of hard links to this directory
  pub nlink: u32,

  /// To extract the major device number, (device & 0xfff00) >> 8. To extract the minor device number, use (dev & 0xff) | ((dev >> 12) & 0xfff00)
  pub device: u32,
}

impl_converter!(BasicBlockDevice);

impl InodeBody for BasicBlockDevice {}

pub const BASIC_CHAR_BODY_SIZE: usize = mem::size_of::<BasicCharDevice>();
#[repr(C)]
#[derive(Clone, Default, Debug)]
pub struct BasicCharDevice {
  /// The number of hard links to this directory
  pub nlink: u32,

  /// To extract the major device number, (device & 0xfff00) >> 8. To extract the minor device number, use (dev & 0xff) | ((dev >> 12) & 0xfff00)
  pub device: u32,
}

impl_converter!(BasicCharDevice);

impl InodeBody for BasicCharDevice {}

pub const BASIC_FIFO_BODY_SIZE: usize = mem::size_of::<BasicFifo>();
#[repr(C)]
#[derive(Clone, Default, Debug)]
pub struct BasicFifo {
  /// The number of hard links to this directory
  pub nlink: u32,
}

impl_converter!(BasicFifo);

impl InodeBody for BasicFifo {}

pub const BASIC_SOCKET_BODY_SIZE: usize = mem::size_of::<BasicSocket>();
#[repr(C)]
#[derive(Clone, Default, Debug)]
pub struct BasicSocket {
  /// The number of hard links to this directory
  pub nlink: u32,
}

impl_converter!(BasicSocket);

impl InodeBody for BasicSocket {}

pub const EXTENDED_DIRECTORY_BODY_SIZE: usize = mem::size_of::<ExtendedDirectory>();
#[repr(C)]
#[derive(Clone, Default, Debug)]
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

impl_converter!(ExtendedDirectory);

impl InodeBody for ExtendedDirectory {}

pub const EXTENDED_FILE_BODY_SIZE: usize = mem::size_of::<ExtendedFile>();
#[repr(C)]
#[derive(Clone, Default, Debug)]
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

impl_converter!(ExtendedFile);

impl InodeBody for ExtendedFile {}

pub const EXTENDED_SYMLINK_BODY_SIZE: usize = mem::size_of::<ExtendedSymlink>();
#[repr(C)]
#[derive(Clone, Default, Debug)]
pub struct ExtendedSymlink {
  /// The number of hard links to this directory
  pub nlink: u32,

  /// The size in bytes of the `target_path` this symlink points to
  pub target_size: u32,

  /// An index into the xattr lookup table. Set to 0xFFFFFFFF if the inode has no extended attributes
  pub xattr_idx: u32,
}

impl_converter!(ExtendedSymlink);

impl InodeBody for ExtendedSymlink {}

pub const EXTENDED_BLOCK_BODY_SIZE: usize = mem::size_of::<ExtendedBlock>();
#[repr(C)]
#[derive(Clone, Default, Debug)]
pub struct ExtendedBlock {
  /// The number of hard links to this directory
  pub nlink: u32,

  /// To extract the major device number, (device & 0xfff00) >> 8. To extract the minor device number, use (dev & 0xff) | ((dev >> 12) & 0xfff00)
  pub device: u32,

  /// An index into the xattr lookup table. Set to 0xFFFFFFFF if the inode has no extended attributes
  pub xattr_idx: u32,
}

impl_converter!(ExtendedBlock);

impl InodeBody for ExtendedBlock {}

pub const EXTENDED_CHAR_BODY_SIZE: usize = mem::size_of::<ExtendedChar>();
#[repr(C)]
#[derive(Clone, Default, Debug)]
pub struct ExtendedChar {
  /// The number of hard links to this directory
  pub nlink: u32,

  /// To extract the major device number, (device & 0xfff00) >> 8. To extract the minor device number, use (dev & 0xff) | ((dev >> 12) & 0xfff00)
  pub device: u32,

  /// An index into the xattr lookup table. Set to 0xFFFFFFFF if the inode has no extended attributes
  pub xattr_idx: u32,
}

impl_converter!(ExtendedChar);

impl InodeBody for ExtendedChar {}

pub const EXTENDED_FIFO_BODY_SIZE: usize = mem::size_of::<ExtendedFifo>();
#[repr(C)]
#[derive(Clone, Default, Debug)]
pub struct ExtendedFifo {
  /// The number of hard links to this directory
  pub nlink: u32,

  /// An index into the xattr lookup table. Set to 0xFFFFFFFF if the inode has no extended attributes
  pub xattr_idx: u32,
}

impl_converter!(ExtendedFifo);

impl InodeBody for ExtendedFifo {}

pub const EXTENDED_SOCKET_BODY_SIZE: usize = mem::size_of::<ExtendedSocket>();
#[repr(C)]
#[derive(Clone, Default, Debug)]
pub struct ExtendedSocket {
  /// The number of hard links to this directory
  pub nlink: u32,

  /// An index into the xattr lookup table. Set to 0xFFFFFFFF if the inode has no extended attributes
  pub xattr_idx: u32,
}

impl_converter!(ExtendedSocket);

impl InodeBody for ExtendedSocket {}

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

#[repr(C)]
#[derive(Clone, Default, Debug)]
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

trait InodeBody {}

impl_converter!(InodeHeader);

pub fn get_inode(
  r: &mut SqsIoReader,
  sb: Superblock,
  block: u32,
  offset: u32,
  inode_type: InodeType,
) -> Result<()> {
  // get the block
  let size = inode_type.body_size();

  // read inode header
  let uncompressed = read_metadata(r, sb.compressor, sb.inode_table_start, block, offset, size)?;
  trace!("[get_inode]size={}, uncompressed={:x?}", size, uncompressed);
  let _inode_header = parse_inode_header(uncompressed[0..INODE_HEADER_SIZE].to_vec())?;

  // read inode body
  parse_inode_body(
    (&uncompressed[INODE_HEADER_SIZE..]).to_vec(),
    sb.block_size,
    inode_type,
  )?;

  Ok(())
}

const INODE_HEADER_SIZE: usize = 16;
fn parse_inode_header(data: Vec<u8>) -> Result<InodeHeader> {
  if data.len() < INODE_HEADER_SIZE {
    return Err(invalid_error!("input data must great than 15 bytes"));
  }
  let mut data = &*data;
  let mut header = InodeHeader::default();
  data.read_exact(header.as_mut())?;

  debug!("[parse_inode_header] header={:?}", header);

  Ok(header)
}

fn parse_inode_body(data: Vec<u8>, block_size: u32, itype: InodeType) -> Result<()> {
  let _body: Box<dyn InodeBody> = match itype {
    InodeType::BasicDirectory => parse_basic_directory(data)?,
    InodeType::BasicFile => parse_basic_file(data, block_size)?,
    InodeType::BasicSymlink => parse_basic_symlink()?,
    InodeType::BasicBlockDevice | InodeType::BasicCharDevice => parse_basic_block_device()?,
    InodeType::BasicFifo | InodeType::BasicSocket => parse_basic_ipc()?,

    InodeType::ExtendedDirectory => parse_extened_directory()?,
    InodeType::ExtendedFile => parse_extened_file()?,
    InodeType::ExtendedSymlink => parse_extened_symlink()?,
    InodeType::ExtendedBlockDevice | InodeType::ExtendedCharDevice => parse_extened_block_device()?,
    InodeType::ExtendedFifo | InodeType::ExtendedSocket => parse_extened_ipc()?,
  };

  Ok(())
}

fn parse_basic_directory(data: Vec<u8>) -> Result<Box<BasicDirectory>> {
  if data.len() < BASIC_DIRECTORY_BODY_SIZE {
    return Err(invalid_error!(format!(
      "invalid BasicDirectory body size({} bytes), must >= {} bytes",
      data.len(),
      BASIC_DIRECTORY_BODY_SIZE
    )));
  }
  let mut body = BasicDirectory::default();
  (&*data).read(body.as_mut())?;
  Ok(Box::new(body))
}

fn parse_basic_file(data: Vec<u8>, block_size: u32) -> Result<Box<BasicFile>> {
  // TODO
  if data.len() < BASIC_FILE_BODY_SIZE {
    return Err(invalid_error!(format!(
      "invalid BasicFile body size({} bytes), must >= {} bytes",
      data.len(),
      BASIC_FILE_BODY_SIZE
    )));
  }

  let mut body = BasicFile::default();
  (&*data).read(body.as_mut())?;

  let mut block_list_size = body.size / block_size;

  if body.size % block_size > 0 && body.fragment_block_idx != 0xffffffff {
    block_list_size += 1
  }

  let extra = block_list_size * 4;

  while data[BASIC_FILE_BODY_SIZE..].len() >= extra as usize {}

  Ok(Box::new(body))
}

fn parse_basic_symlink() -> Result<Box<BasicSymlink>> {
  // TODO: parse BasicSymlink body.
  let body = BasicSymlink::default();
  Ok(Box::new(body))
}

fn parse_basic_block_device() -> Result<Box<BasicBlockDevice>> {
  // TODO: parse BasicBlockDevice body.
  let body = BasicBlockDevice::default();
  Ok(Box::new(body))
}

fn parse_basic_ipc() -> Result<Box<BasicSocket>> {
  // TODO: parse BasicSocket/BasicFifo body.
  let body = BasicSocket::default();
  Ok(Box::new(body))
}

fn parse_extened_directory() -> Result<Box<ExtendedDirectory>> {
  // TODO: parse ExtendedDirectory body.
  let body = ExtendedDirectory::default();
  Ok(Box::new(body))
}

fn parse_extened_file() -> Result<Box<ExtendedFile>> {
  // TODO: parse ExtendedFile body.
  let body = ExtendedFile::default();
  Ok(Box::new(body))
}

fn parse_extened_symlink() -> Result<Box<ExtendedSymlink>> {
  // TODO: parse ExtendedSymlink body.
  let body = ExtendedSymlink::default();
  Ok(Box::new(body))
}

fn parse_extened_block_device() -> Result<Box<ExtendedBlock>> {
  // TODO: parse ExtendedChar/ExtendedBlock body.
  let body = ExtendedBlock::default();
  Ok(Box::new(body))
}

fn parse_extened_ipc() -> Result<Box<ExtendedSocket>> {
  // TODO: parse ExtendedSocket/ExtendedFifo body.
  let body = ExtendedSocket::default();
  Ok(Box::new(body))
}

#[cfg(test)]
mod tests {
  use crate::tests::*;
  use crate::*;
  use std::io::Result;

  #[test]
  // #[cfg_attr(not(feature = "gzip.sqs"), ignore)]
  fn test_get_inode() -> Result<()> {
    // TODO
    let (mut reader, sb) = prepare_tests()?;

    let block = sb.root_inode_ref.block;
    let offset = sb.root_inode_ref.offset;

    get_inode(
      &mut reader,
      sb,
      block as u32,
      offset as u32,
      InodeType::BasicDirectory,
    )?;

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
