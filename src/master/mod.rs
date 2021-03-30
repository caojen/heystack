//! The main service of the program

use ::std::io;
use ::std::fs;
use ::std::sync::Mutex;

use actix_web::{web, App, HttpServer, Responder};

use crate::diskio::read_write::read_struct_from_file;
use crate::config::Config;
use crate::storage::{
  IndexFile,
  IndexFileItem,
  PhysicalFileItem
};
use crate::storage;

mod route;

/// share value in different route
#[derive(Debug)]
pub struct AppState {
  pub index_file: Mutex<IndexFile>,
  pub config: Mutex<Config>
}

#[actix_web::main]
pub async fn service_start(config: Config) -> io::Result<()> {
  // 1. load index file
  // 1. currently, load all index
  let indexes = load_index_file(&config)?;
  let service_port = config.service_port;
  let max_index_in_mem = config.max_index_in_mem / std::mem::size_of::<IndexFileItem>() as u64;

  // 2. use web-framework to start http listening
  // 2. share indexes
  let index_file = Mutex::new(IndexFile::new(
    indexes,
    max_index_in_mem as usize,
    config.index_name.clone(),
    config.volume_name.clone()
  ));
  let state = web::Data::new(AppState {
    index_file,
    config: Mutex::new(config)
  });

  crate::logln!("Trying to bind port: ", service_port);
  HttpServer::new(move || {
    App::new()
      .app_data(state.clone())
      .service(route::sync_index_file)
      .service(route::get_file)
      .service(route::upload_file)
      .service(route::delete_file)
      .service(route::update_file)
  })
    .bind(&format!("0.0.0.0:{}", service_port))?
    .run()
    .await
}

pub fn load_index_file(config: &Config) -> io::Result<Vec::<IndexFileItem>> {
  let mut v: Vec::<IndexFileItem> = vec![];
  let mut f = fs::File::open(&config.index_name)?;

  let mut index_count = 0 as u32;
  while let Some(item) = read_struct_from_file::<IndexFileItem>(&mut f)? {
    if item.file_exists() == false {
      continue
    }
    println!("{} // loading index: {:?}", index_count, item);
    v.push(item);
    index_count += 1;
  }

  Ok(v)
}
