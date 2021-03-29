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
}