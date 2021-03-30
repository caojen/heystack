use std::fs;
use std::io;
use std::io::prelude::*;

use crate::diskio::read_write;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct PhysicalFileItem {
  key: u32,         // unique key of file
  flag: bool,       // true if file valid,
  size: u64,        // filesize,
  data: Vec::<u8>,  // filedata,
}

impl PhysicalFileItem {
  pub fn get_from_index(index: &IndexFileItem, f: &mut fs::File) -> io::Result<Option<PhysicalFileItem>> {
    f.seek(io::SeekFrom::Start(index.offset))?;

    let key = read_write::read_struct_from_file(f)?;
    let flag = read_write::read_struct_from_file(f)?;
    let size = read_write::read_struct_from_file(f)?;
    if key.is_none() || flag.is_none() || size.is_none() {
      Ok(None)
    } else {
      let data = read_write::read_bytes_from_file(size.unwrap(), f)?;
      Ok(Some(PhysicalFileItem {
        key: key.unwrap(),
        flag: flag.unwrap(),
        size: size.unwrap(),
        data
      }))
    }
  }

  pub fn sync(index: &IndexFileItem, f: &mut fs::File) -> io::Result<()> {
    f.seek(io::SeekFrom::Start(index.offset))?;
    read_write::modify_struct_in_file(&index.key, f)?;
    read_write::modify_struct_in_file(&index.flag, f)?;

    Ok(())
  }

  /// OpenOption: write
  pub fn add_one_file(data: &Vec::<u8>, f: &mut fs::File) -> io::Result<IndexFileItem> {
    let offset = f.seek(io::SeekFrom::End(0))?;
    let size = data.len() as u64;
    let key = (offset / ::std::mem::size_of::<PhysicalFileItem>() as u64) as u32;
    let pfi = PhysicalFileItem {
      key,
      flag: true,
      size,
      data: vec![] // use data instead of this.data
    };
    read_write::append_struct_to_file(&pfi.key, f)?;
    read_write::append_struct_to_file(&pfi.flag, f)?;
    read_write::append_struct_to_file(&pfi.size, f)?;
    read_write::write_bytes_to_file(data, f)?;

    Ok(IndexFileItem {
      key,
      flag: true,
      size,
      offset
    })
  }

  fn read_struct_from_file_without_data(f: &mut fs::File) -> io::Result<Option<Self>> {

    let key: Option<u32> = read_write::read_struct_from_file(f)?;
    let flag: Option<bool> = read_write::read_struct_from_file(f)?;
    let size: Option<u64> = read_write::read_struct_from_file(f)?;

    if key.is_none() || flag.is_none() || size.is_none() {
      Ok(None)
    } else {
      f.seek(std::io::SeekFrom::Current(size.unwrap() as i64))?;
      Ok(Some(PhysicalFileItem {
        key: key.unwrap(),
        flag: flag.unwrap(),
        size: size.unwrap(),
        data: vec![] // will not return data
      }))
    }
  }

  // build the entire index file based on f
  // openoption: read
  pub fn build_index_file(f: &mut fs::File) -> io::Result<Vec<IndexFileItem>> {
    let mut r = vec![];
    while let Some(item) = PhysicalFileItem::read_struct_from_file_without_data(f)? {
      if item.flag {
        let ifi = IndexFileItem {
          key: item.key,
          flag: true,
          offset: f.stream_position()?,
          size: item.size
        };

        crate::loglnf!(ifi);
        r.push(ifi);
      }
    }
    Ok(r)
  }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct IndexFileItem {
  key: u32,         // unique key of file
  flag: bool,       // true if file valid
  offset: u64,      // use seek(offset) to find this file
  size: u64         // filesize
}

impl IndexFileItem {
  /// sync this index file item to physical file item
  /// i,e, (only) delete file will raise this function
  pub fn sync(&self, f: &mut fs::File) -> io::Result<()> {
    PhysicalFileItem::sync(self, f)?;
    Ok(())
  }

  /// check self.flag == true, if true, the file exists
  pub fn file_exists(&self) -> bool {
    self.flag
  }
}

#[derive(Debug, Clone)]
pub struct IndexFile {
  indexes: Vec<IndexFileItem>,
  max: usize,
  index_filename: String,
  physical_filename: String
}

impl IndexFile {
  pub fn new(
    indexes: Vec<IndexFileItem>,
    max: usize,
    index_filename: String,
    physical_filename: String
  ) -> Self {
    crate::logln!("Index File In Memory Build");
    crate::logln!("  Current: ", indexes.len());
    crate::logln!("  Max:     ", max);

    IndexFile {
      indexes,
      max,
      index_filename,
      physical_filename
    }
  }

  /// check index item exists
  pub fn _exists(&self, key: u32) -> (bool, Option<&IndexFileItem>) {
    let mut r = false;
    let mut option = None;
    for index in &self.indexes {
      if index.key == key && index.flag == true {
        r = true;
        option = Some(index);
        break;
      }
    }
    (r, option)
  }

  pub fn get_mut(&mut self, key: u32) -> Option<&mut IndexFileItem> {
    let mut option = None;
    for index in &mut self.indexes {
      if index.key == key && index.flag == true {
        option = Some(index);
        break;
      }
    }

    option
  }

  /// delete index file item
  /// return
  /// Ok(()), delete success
  /// Err(()), no such file
  pub fn delete_item(&mut self, key: u32) -> io::Result<()> {
    crate::logln!("delete item with key ", key);
    let physical_filename = self.physical_filename.clone();
    match self.get_mut(key) {
      None => io::Result::Err(io::Error::new(io::ErrorKind::Other, "No Such File")),
      Some(item) => {
        item.flag = false;
        item.sync(
          &mut fs::OpenOptions::new()
            .write(true)
            .read(true)
            .open(physical_filename)?
        )?;
        Ok(())
      }
    }
  }

  pub fn add_item(&mut self, data: &Vec::<u8>) -> io::Result<IndexFileItem> {
    let physical_filename = self.physical_filename.clone();
    let r = PhysicalFileItem::add_one_file(&data,
      &mut fs::OpenOptions::new()
        .write(true)
        .read(true)
        .open(physical_filename)?
    )?;
    self.indexes.push(r.clone());
    crate::logln!("adding new data with new key ", r.key);

    // test if out of memory
    if self.indexes.len() > self.max {
      crate::logln!("Out of memory. Store files count: ", self.indexes.len());
    }
    Ok(r)
  }

  pub fn get_data(&mut self, key: u32) -> io::Result<Option<Vec<u8>>> {
    crate::logln!("getting data with key ", key);
    let physical_filename = self.physical_filename.clone();
    match self.get_mut(key) {
      None => Ok(None),
      Some(ifi) => {
        match PhysicalFileItem::get_from_index(ifi, 
            &mut fs::File::open(physical_filename)?
          )? {
            None => Ok(None),
            Some(t) => Ok(Some(t.data))
        }
      }
    }
  }

  // store self.indexes into index_filename
  pub fn store_into_file(&self) -> io::Result<()> {
    crate::logln!("storing indexes into file");
    let index_filename = self.index_filename.clone();
    let mut f = fs::File::create(&index_filename)?;
    for index in &self.indexes {
      read_write::append_struct_to_file::<IndexFileItem>(&index, &mut f)?;
    }
    f.sync_all()?;

    // try to load file
    // let mut f = fs::File::open(&index_filename)?;
    // let mut v: Vec::<IndexFileItem> = vec![];
    // while let Some(item) = read_write::read_struct_from_file::<IndexFileItem>(&mut f)? {
    //   v.push(item);
    // }
    // println!("{:?}", v);
    Ok(())
  }

  // based on the given indexes, create index file and save it to that file
  pub fn create_index_file_and_save(path: &str, indexes: Vec::<IndexFileItem>) -> io::Result<()> {
    crate::logln!("create index file and save");
    let mut f = fs::File::create(path)?;
    for index in indexes {
      read_write::append_struct_to_file::<IndexFileItem>(&index, &mut f)?;
    }
    f.sync_all()?;

    Ok(())
  }
}
