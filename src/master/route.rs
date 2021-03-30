use actix_web::{ web, get, post, put, delete, Responder, HttpResponse, Error };
use super::AppState;
use futures::StreamExt;

#[put("/sync")]
pub async fn sync_index_file(data: web::Data<AppState>) -> impl Responder {
  let index_file = data.index_file.lock().unwrap();
  match index_file.store_into_file() {
    Err(e) => {
      println!("{:?}", e);
      HttpResponse::InternalServerError()
        .body("Something went wrong")
    },
    _ => {
      HttpResponse::Ok()
        .body("done")
    }
  }

}

#[get("/file/{key}")]
pub async fn get_file(data: web::Data<AppState>, web::Path(key): web::Path<u32>) -> impl Responder {
  let mut index_file = data.index_file.lock().unwrap();
  match index_file.get_data(key) {
    Err(_) => {
      HttpResponse::InternalServerError()
        .body("Something went wrong")
    },
    Ok(r) => match r {
      None => {
        HttpResponse::NotFound()
          .body("Resource Not Found")
      },
      Some(t) => {
        HttpResponse::Ok()
          .body(t)
      }
    }
  }
}

#[post("/file")]
pub async fn upload_file(mut body: web::Payload, data: web::Data<AppState>) -> Result<HttpResponse, Error> {
  let mut bytes = web::BytesMut::new();
  while let Some(item) = body.next().await {
    let item = item?;
    bytes.extend_from_slice(&item);
  }

  let d = &bytes[..];
  let mut index_file = data.index_file.lock().unwrap();
  match index_file.add_item(&d.to_vec()) {
    Err(_) => {
      Ok(HttpResponse::InternalServerError()
        .body("Something went wrong"))
    },
    Ok(ifi) => {
      Ok(HttpResponse::Ok()
        .json(ifi))
    }
  }
}

#[delete("/file")]
pub async fn delete_file() -> String {
  format!("delete file")
}

#[put("/file")]
pub async fn update_file() -> String {
  format!("update file")
}