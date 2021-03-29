use ::std::io;
use ::std::io::prelude::*;
use ::std::mem;
use ::std::slice;

pub unsafe fn struct_into_slice<T: Sized>(s: &T) -> &[u8] {
  ::std::slice::from_raw_parts(
    (s as *const T) as *const u8,
    mem::size_of::<T>()
  )
}

pub unsafe fn slice_info_struct<T: Sized>(s: &[u8]) -> io::Result<T> {
  let size = mem::size_of::<T>();

  assert_eq!(s.len(), size);

  let mut r: T = mem::zeroed();
  let rr: *mut T =&mut r;
  let buffer = slice::from_raw_parts_mut(rr as *mut u8, size);
  
  let mut reader = io::BufReader::new(s);
  reader.read_exact(buffer)?;

  Ok(r)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[derive(Debug)]
  struct TestStruct {
    pub a: u8,
    pub b: u16,
    pub c: u32,
    pub d: [u8; 64]
  }

  impl ::std::cmp::PartialEq for TestStruct {
    fn eq(&self, other: &TestStruct) -> bool {
      return self.a == other.a && self.b == other.b && self.c == other.c && self.d == other.d;
    }
  }

  #[test]
  fn read_into_file() {
    let t = TestStruct { a: 1, b: 12, c: 1024, d: [12; 64]};
    unsafe { struct_into_slice(&t) };
  }

  #[test]
  fn load_from_slice() {
    let t = TestStruct { a: 1, b: 12, c: 1024, d: [12; 64]};
    let r = unsafe { struct_into_slice(&t) };
    let p = unsafe { slice_info_struct::<TestStruct>(r) };
    let p = p.unwrap();
    assert_eq!(t, p);
  }
}
