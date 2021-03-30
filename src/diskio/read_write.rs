//! write a struct into ::std::fs::File (append)
//! read struct(s) from ::std::fs::File
//! help change a sturct of ::std::fs::File

use ::std::io;
use ::std::io::prelude::*;
use ::std::fs;
use ::std::mem;

use super::struct_slice;

/// OpenOption: append
pub fn append_struct_to_file<T: Sized>(s: &T, f: &mut fs::File) -> io::Result<()> {
  let slice = unsafe { struct_slice::struct_into_slice(s) };
  f.write_all(slice)?;

  Ok(())
}

/// read **one** struct from file
pub fn read_struct_from_file<T: Sized>(f: &mut fs::File) -> io::Result<Option<T>> {
  let size = mem::size_of::<T>();
  let mut vec = vec![0u8; size];
  match f.read(vec.as_mut_slice())? {
    0 => Ok(None),
    _ => {
      // println!("read slice: {:?}", vec);
      let s = unsafe { struct_slice::slice_info_struct(&vec[..])? };
      Ok(Some(s))
    }
  }
}

/// OpenOption: write
/// modify **one** struct in file
pub fn modify_struct_in_file<T: Sized>(s: &T, f: &mut fs::File) -> io::Result<()> {
  let slice = unsafe { struct_slice::struct_into_slice(s) };
  f.write_all(slice)?;

  Ok(())
}

/// read some bytes from file
pub fn read_bytes_from_file(size: u64, f: &mut fs::File) -> io::Result<Vec::<u8>> {
  let mut r: Vec::<u8> = vec![0u8; size as usize];
  let mut reader = io::BufReader::new(f);
  reader.read_exact(&mut r)?;
  Ok(r)
}

pub fn write_bytes_to_file(bytes: &Vec::<u8>, f: &mut fs::File) -> io::Result<()> {
  f.write_all(&bytes[..])
}

#[cfg(test)]
mod tests {
  use super::*;

  #[derive(Debug, Clone, Copy)]
  struct TestStruct {
    pub a: u8,
    pub b: u8
  }

  impl std::cmp::PartialEq for TestStruct {
    fn eq(&self, other: &Self) -> bool {
      return self.a == other.a && self.b == other.b;
    }
  }

  #[test]
  fn read_and_write_struct_test() -> io::Result<()> {
    let a = TestStruct { a: 1, b: 2 };
    let b = TestStruct { a: 3, b: 4 };
    let filename = "test";
    {
      let mut f = fs::File::create(filename)?;
      append_struct_to_file(&a, &mut f)?;
    }

    {
      let mut f = fs::File::open(filename)?;
      let s: Option<TestStruct> = read_struct_from_file(&mut f)?;
      assert_eq!(s, Some(a));
    }

    {
      let mut f = fs::OpenOptions::new().write(true).append(true).open(filename)?;
      append_struct_to_file(&b, &mut f)?;
    }

    {
      let mut f = fs::File::open(filename)?;
      let s: Option<TestStruct> = read_struct_from_file(&mut f)?;
      assert_eq!(s, Some(a));
      let s: Option<TestStruct> = read_struct_from_file(&mut f)?;
      assert_eq!(s, Some(b));
    }

    {
      let mut f = fs::File::open(filename)?;
      let size = mem::size_of::<TestStruct>();
      f.seek(io::SeekFrom::Start(size as u64))?;
      let s: Option<TestStruct> = read_struct_from_file(&mut f)?;
      assert_eq!(s, Some(b));
    }

    fs::remove_file(filename)?;

    Ok(())
  }

  #[test]
  fn modify_struct_test() -> io::Result<()> {
    let mut a = TestStruct { a: 1, b: 2 };
    let mut b = TestStruct { a: 10, b: 24 };
    let mut c = TestStruct { a: 127, b: 8 };

    let filename = "test1";
    {
      let mut f = fs::File::create(filename)?;
      append_struct_to_file(&a, &mut f)?;
      append_struct_to_file(&b, &mut f)?;
      append_struct_to_file(&c, &mut f)?;
    }
    {
      let mut f = fs::OpenOptions::new()
        .write(true)
        .open(filename)?;
      a.a = 10;
      modify_struct_in_file(&a, &mut f)?;
      b.b = 25;
      modify_struct_in_file(&b, &mut f)?;
      c.a = 126;
      c.b = 9;
      modify_struct_in_file(&c, &mut f)?;
    }
    {
      let mut f = fs::File::open(filename)?;
      let s: Option<TestStruct> = read_struct_from_file(&mut f)?;
      assert_eq!(s, Some(a));
      let s: Option<TestStruct> = read_struct_from_file(&mut f)?;
      assert_eq!(s, Some(b));
      let s: Option<TestStruct> = read_struct_from_file(&mut f)?;
      assert_eq!(s, Some(c));
    }
    fs::remove_file(filename)?;
    Ok(())
  }

  #[derive(Debug)]
  struct TestStruct2 {
    pub a: u8,
    pub b: u16,
    pub c: bool,
    pub d: u8,
    pub e: u64
  }

  impl TestStruct2 {
    pub fn new(a: u8, b: u16, c: bool, d: u8, e: u64) -> Self {
      TestStruct2 {
        a,b,c,d,e
      }
    }
  }

  impl std::cmp::PartialEq for TestStruct2 {
    fn eq(&self, other: &Self) -> bool {
      self.a == other.a && self.b == other.b && self.c == other.c && self.d == other.d && self.e == other.e
    }
  }

  #[test]
  pub fn test_read_and_write_complex_struct() -> io::Result<()> {
    let a = TestStruct2::new(1, 291, true, 0, 199992);
    let b = TestStruct2::new(98, 23123, false, 12, 22);
    let c = TestStruct2::new(26, 8, false, 0, 1277372);
    let filename = "test333";
    {
      let mut f = fs::File::create(filename)?;
      append_struct_to_file(&a, &mut f)?;
      append_struct_to_file(&b, &mut f)?;
      append_struct_to_file(&c, &mut f)?;
      f.sync_all()?;
    }
    {
      let mut f = fs::OpenOptions::new()
        .read(true)
        .open(filename)?;
      let aa: Option<TestStruct2> = read_struct_from_file(&mut f)?;
      let bb: Option<TestStruct2> = read_struct_from_file(&mut f)?;
      let cc: Option<TestStruct2> = read_struct_from_file(&mut f)?;
      let dd: Option<TestStruct2> = read_struct_from_file(&mut f)?;
      println!("{:?}, {:?}", aa, a);
      println!("{:?}, {:?}", bb, b);
      println!("{:?}, {:?}", cc, c);
      println!("{:?}", dd);
      assert_eq!(aa, Some(a));
      assert_eq!(bb, Some(b));
      assert_eq!(cc, Some(c));
      assert_eq!(dd, None);
    }

    fs::remove_file(filename)?;
    Ok(())
  }
}