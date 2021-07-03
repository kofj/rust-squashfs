/// https://www.kernel.org/doc/Documentation/filesystems/squashfs.txt
/// > A squashfs filesystem consists of a maximum of nine parts, packed together on a byte alignment:
///  ---------------
/// |  superblock   |
/// |---------------|
/// |  compression  |
/// |    options    |
/// |---------------|
/// |  datablocks   |
/// |  & fragments  |
/// |---------------|
/// |  inode table  |
/// |---------------|
/// |   directory   |
/// |     table     |
/// |---------------|
/// |   fragment    |
/// |    table      |
/// |---------------|
/// |    export     |
/// |    table      |
/// |---------------|
/// |    uid/gid    |
/// |  lookup table |
/// |---------------|
/// |     xattr     |
/// |     table     |
///  ---------------
///

#[macro_use]
extern crate bitflags;
// #[macro_use]
// extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate smart_default;
#[macro_use]
extern crate prettytable;
#[macro_use]
extern crate log;

use std::fs::File;
use std::io::{Read, Seek};

pub mod compress;
pub mod layout;

pub trait SqsIoRead: Read + Seek {}

pub type SqsIoReader = Box<dyn SqsIoRead>;

impl SqsIoRead for File {}

pub fn set_logging(level: LevelFilter) -> Result<()> {
    Logger::try_with_env_or_str("trace")
        .unwrap()
        .format(colored_opt_format)
        .start()
        .map_err(|e| map_other_error!(e))?;

    log::set_max_level(level);

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
