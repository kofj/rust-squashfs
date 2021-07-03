/// map_other_error: convert other error to io:Error
/// and ad file and line info to it.
#[macro_export]
macro_rules! map_other_error {
  ($E: expr) => {
    std::io::Error::new(
      std::io::ErrorKind::Other,
      format!("Error {} at {}:{}", $E, file!(), line!()),
    )
  };
}

/// map_error: add file and line info to io::Error;
#[macro_export]
macro_rules! map_error {
  ($E: expr) => {
    std::io::Error::new(
      $E.kind(),
      format!("Error {} at {}:{}", $E, file!(), line!()),
    )
  };
}
