use actix_web::{get, put, web, App, HttpResponse, HttpServer, Responder};
use std::collections::BTreeMap;
use std::sync::Mutex;

struct AppData {
    map: Mutex<BTreeMap<String, String>>,
}

#[put("/db/{key}")]
async fn put_route(
    data: web::Data<AppData>,
    path: web::Path<String>,
    req_body: String,
) -> impl Responder {
    let mut map = data.map.lock().unwrap();
    let key = path.into_inner();
    map.insert(key, req_body);
    HttpResponse::Ok()
}

#[get("/db/{key}")]
async fn get_route(
    data: web::Data<AppData>,
    path: web::Path<String>,
) -> impl Responder {
    let map = data.map.lock().unwrap();
    let key = path.into_inner();
    let value = match map.get(key.as_str()) {
        Some(value) => value,
        None => "",
    };
    HttpResponse::Ok().body(format!("{}", value))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let map: BTreeMap<String, String> = BTreeMap::new();
    let map_data = web::Data::new(AppData {
        map: Mutex::new(map),
    });

    HttpServer::new(move || {
        App::new().app_data(map_data.clone())
        .service(put_route)
        .service(get_route)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
