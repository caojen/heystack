use actix_web::{ web, get };
use super::AppState;

#[get("/")]
pub async fn index(data: web::Data<AppState>) -> String {
  let indexes = data.indexes.lock().unwrap();
  format!("hello: {:?}", indexes.len())
}
