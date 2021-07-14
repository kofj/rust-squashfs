# rust-squashfs 
[![](https://img.shields.io/crates/v/squashfs.svg)](https://crates.io/crates/squashfs) [![Docs](https://docs.rs/squashfs/badge.svg)](https://docs.rs/squashfs)

A pure rust implement of squashfs library.

## Roadmap

- [x] Parse squashfs `Superblock`.
- [x] Parse `fragment table`.
- [ ] Parse `xattrs table`.
- [x] Parse `uid/gid lookup table`.
- [ ] Parse `inode table`.
- [ ] Parse `directory table`.
- [ ] Parse `export table`.
- [ ] Multiple Compressors:
  - [x] `gzip` algorithm.
  - [ ] `lzma` algorithm.
  - [ ] `lzo` algorithm.
  - [ ] `xz` algorithm.
  - [ ] `lz4` algorithm.
  - [ ] `zstd` algorithm.
