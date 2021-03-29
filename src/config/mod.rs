//! deal with the config of the program

use ::std::io;
use ::std::process;
use ::std::fs;
use crate::diskio::read_write;

pub struct Config {
  pub cpid: u32,          // the pid of **this** running function
  pub tpid: u32,          // the pid of the service pid

  pub pid_file: String,  // where the tpid store
  
  pub config_port: u32,   // listen at for reload, stop
  pub service_port: u32,  // listen at for serve

  pub volumn_name: String, // the physical filename
  pub index_name: String,  // the index filename

  pub max_index_in_mem: u64, // the maxinum memory(bytes) can be used to storing index
}

impl Config {
  pub fn new() -> io::Result<Self> {

    let mut c = Config {
      cpid: process::id(),
      tpid: 0,

      pid_file: "heystack.pid".to_string(),

      config_port: 10001,
      service_port: 10002,

      volumn_name: "heystack.volumn".to_string(),
      index_name: "heystack.index".to_string(),

      max_index_in_mem: 1024 * 1024 * 20, // 20 Mb
    };

    c.get_pid_from_file()?;
    c.create_files()?;
    Ok(c)
  }

  /// get pid from self.pid_file
  /// if file does not exists(the program isn't starting)
  /// will set self.tpid = 0, otherwise, set self.tpid as expected
  fn get_pid_from_file(&mut self) -> io::Result<()>  {
    let pid_file = &self.pid_file;
    
    if let Ok(mut f) = fs::File::open(pid_file) {
      let pid: Option<u32> = read_write::read_struct_from_file(&mut f)?;
      match pid {
        Some(pid) => self.tpid = pid,
        None => self.tpid = 0
      }
    } else {
      self.tpid = 0;
    }
    
    Ok(())
  }

  /// try to create self.files if that file does not exists
  /// will not write anything in those files
  fn create_files(&self) -> io::Result<()> {
    let filenames: Vec::<&str> = vec![
      &self.pid_file,
      &self.volumn_name,
      &self.index_name
    ];

    Ok(())
  }
}