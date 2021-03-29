//! The main service of the program

use ::std::io;
use ::std::fs;
use ::std::sync::Mutex;

use actix_web::{web, App, HttpServer, Responder};

use crate::diskio::read_write::read_struct_from_file;
use crate::config::Config;
use crate::storage::{
  IndexFile,
  PhysicalFile
};
use crate::storage;

mod route;

/// share value in different route
pub struct AppState {
  pub indexes: Mutex<Vec::<storage::IndexFile>>
}

#[actix_web::main]
pub async fn service_start(config: Config) -> io::Result<()> {
  // 1. load index file
  // 1. currently, load all index
  let indexes = load_index_file(&config)?;

  // 2. use web-framework to start http listening
  // 2. share indexes
  crate::logln!("Raising HttpServer at ", config.service_port);
  let state = web::Data::new(AppState {
    indexes: Mutex::new(indexes)
  });

  HttpServer::new(move || {
    App::new()
      .app_data(state.clone())
      .service(route::index)
  })
    .bind(&format!("0.0.0.0:{}", config.service_port))?
    .run()
    .await
}

pub fn load_index_file(config: &Config) -> io::Result<Vec::<IndexFile>> {
  crate::logln!("loading index file");

  let mut v: Vec::<IndexFile> = vec![];
  let mut f = fs::File::open(&config.index_name)?;
  while let Some(item) = read_struct_from_file::<IndexFile>(&mut f)? {
    println!("read one index");
    v.push(item);
  }

  crate::logln!("load index file size: ", v.len());

  Ok(v)
}