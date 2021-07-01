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

pub mod superblock;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
