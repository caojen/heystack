#[derive(Debug)]
pub struct PhysicalFile {
  key: u32,         // unique key of file
  flag: bool,       // true if file valid,
  cookie: u64,      // special cookie for key
  size: u64,        // filesize,
  data: Vec::<u8>,  // filedata,
  checksum: u8      // checksum of filedata
}

#[derive(Debug)]
pub struct IndexFile {
  key: u32,         // unique key of file
  flag: bool,       // true if file valid
  offset: u64,      // use seek(offset) to find this file
  size: u64         // the entire PhysicalFile's size, including key, flag, cookie etc
}
