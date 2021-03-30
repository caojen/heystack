//! deal with the config of the program

use ::std::io;
use ::std::process;
use ::std::fs;
use crate::diskio::read_write;

#[derive(Debug)]
pub struct Config {
  pub cpid: u32,          // the pid of **this** running function
  pub tpid: u32,          // the pid of the service pid

  pub pid_file: String,  // where the tpid store
  
  pub config_port: u32,   // listen at for reload, stop
  pub service_port: u32,  // listen at for serve

  pub volume_name: String, // the physical filename
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

      volume_name: "heystack.volume".to_string(),
      index_name: "heystack.index".to_string(),

      max_index_in_mem: 1024 * 1024 * 1024, // 1024 Mb
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
        Some(pid) => {
          if self.test_pid_is_running(pid) {
            self.tpid = pid;
          } else {
            fs::remove_file(&self.pid_file)?;
            self.tpid = 0;
          }
        },
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
      &self.volume_name,
      &self.index_name
    ];

    for filename in filenames {
      // test file if is exists
      if let Err(_) = fs::File::open(filename) {
        // that file isn't exists
        // try to create it, but write nothing
        fs::File::create(filename)?;
      }
    }

    Ok(())
  }

  fn test_pid_is_running(&self, pid: u32) -> bool {
    let output = process::Command::new("/bin/bash")
      .arg("-c")
      .arg(format!("ps -elf | grep {:?}", pid))
      .output()
      .unwrap();
    let stdout = ::std::str::from_utf8(&output.stdout).unwrap();
    stdout.contains(&::std::env::args().collect::<Vec<String>>()[0])
  }

  // to test service is started
  pub fn is_started(&self) -> bool {
    self.tpid != 0
  }

  /// the preparation of main service
  pub fn as_main_service(&mut self) -> io::Result<()> {
    self.tpid = self.cpid;
    crate::logln!("Write Pid: ", self.tpid);
    let mut f = fs::OpenOptions::new()
      .write(true)
      .open(&self.pid_file)?;
    read_write::modify_struct_in_file(&self.tpid, &mut f)
  }

  pub fn reload_index_file(&mut self) -> io::Result<()> {
    let mut f = fs::File::open(&self.volume_name)?;
    let r = crate::storage::PhysicalFileItem::build_index_file(&mut f)?;

    // remove the current index file
    fs::remove_file(&self.index_name)?;
    // write to file

    crate::storage::IndexFile::create_index_file_and_save(&self.index_name, r)?;

    Ok(())
  }
}
