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

use flexi_logger::{colored_opt_format, Logger};
use std::fs::File;
use std::io::{Read, Result, Seek};

pub mod compress;
pub mod fragment;
pub mod layout;
pub mod metadata;
pub mod uidgids;
pub mod utils;
pub mod xattrs;

pub use fragment::*;
pub use layout::*;
pub use log::LevelFilter;
pub use metadata::*;
pub use uidgids::*;
pub use utils::errors::*;
pub use xattrs::*;

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
    use super::*;
    use std::env;
    use std::fs::File;
    use std::io::Result;
    use std::sync::Once;

    static TEST_LOGGER_INIT: Once = Once::new();

    pub fn prepare_tests() -> Result<(SqsIoReader, Superblock)> {
        TEST_LOGGER_INIT.call_once(move || {
            if let Ok(level) = env::var("RUST_LOG") {
                if level == "trace" {
                    set_logging(LevelFilter::Trace).unwrap();
                }
            } else {
                set_logging(LevelFilter::Debug).unwrap();
            }
        });

        let test_sqs_file =
            if let Ok(env) = std::env::var("TEST_SQS_FILE").map_err(|e| map_other_error!(e)) {
                env
            } else {
                String::from("tests/data/gzip.sqs")
            };
        let f = File::open(test_sqs_file).map_err(|e| map_error!(e))?;
        let mut reader = Box::new(f.try_clone()?) as SqsIoReader;

        let mut sb = Superblock::new();
        sb.load(&mut reader)?;

        Ok((reader, sb))
    }
}
