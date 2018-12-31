#![feature(proc_macro_hygiene, decl_macro)]

use std::io;
use std::io::Error;
use std::fs;
use std::fs::ReadDir;
use std::env;
use std::path::{Path, PathBuf};

#[macro_use] extern crate rocket;

use rocket::Data;
use rocket::Request;
use rocket::Response;
use rocket::http::Status;
use rocket::http::MediaType;
use rocket::response::NamedFile;
use rocket::response::Responder;

extern crate rocket_contrib;
use rocket_contrib::json::Json;
use rocket_contrib::templates::Template;

enum RetrievedData {
  Certification(NamedFile),
  Index(String, ReadDir),
}
use RetrievedData::*;

#[macro_use] extern crate serde_derive;

#[derive(Serialize)]
struct Dir {
  base_url: String,
  name: String,
  entries: Vec<DirEntry>,
}

#[derive(Serialize)]
struct DirEntry {
  path: String,
  name: String,
  #[serde(rename = "type")]
  type_: String,
}

impl<'r> Responder<'r> for RetrievedData {

  fn respond_to(self, request: &Request) -> Result<Response<'r>, Status> {
    match self {
      Certification(file) => file.respond_to(request),
      Index(name, dir) => {
        let mut entries = Vec::new();
        for entry in dir {
          match entry {
            Ok(e) => {
              let type_;
              if e.path().is_file() {
                type_ = "object"
              } else if e.path().is_dir() {
                type_ = "collection"
              } else {
                type_ = "unknown"
              }
              match e.path().to_str() {
                Some(p) => {
                  let name;
                  if p.starts_with("./") {
                    name = &p[2..];
                  } else {
                    name = p;
                  };
                  let file_name = e.path().file_name().map(|f| f.to_str()).unwrap_or(None).unwrap_or("").to_string();
                  entries.push(DirEntry {
                    path: name.to_string(),
                    name: file_name,
                    type_: type_.to_string(),
                  });
                },
                None => ()
              }
            },
            Err(_) => ()
          }
        };
        match request.accept() {
          Some(accept) if accept.preferred().media_type() == &MediaType::HTML => {
            let context = Dir {
              name: name,
              base_url: env::var("BASE_URL").unwrap_or("http://localhost:8000".to_string()),

              entries: entries,
            };
            Template::render("dir", context).respond_to(request)
          }
          _ =>
            Json(entries).respond_to(request),
        }
      }
    }
  }
}

#[get("/")]
fn root() -> Result<RetrievedData, Error> {
  fs::read_dir(".").map(|read| Index("/".to_string(), read))
}

#[get("/<file..>")]
fn files(file: PathBuf) -> Result<RetrievedData, String> {
    if file.is_dir() {
      let filename = file.to_str().unwrap_or("x").to_string();
      fs::read_dir(file).map(|entries| Index(filename, entries)).map_err(|_| "Could not open".to_string())
    } else {
      NamedFile::open(file).map(Certification).map_err(|e| {
        if e.kind() == io::ErrorKind::NotFound {
        "Does not exist".to_string()
      } else {
        "Other error".to_string()
      }
    })
    }
}

#[put("/<file..>", data = "<body>")]
fn puts(file: PathBuf, body: Data) -> io::Result<String> {
    fs::create_dir_all(file.clone().parent().unwrap())?;
    if file.exists() {
        // TODO check if contents are identical, if so, return 200
        // TODO rocket turns this into a 404, find out how to return
        // a better 4xx error message
        Err(io::Error::new(io::ErrorKind::PermissionDenied, "Already exists"))
    } else {
        body.stream_to_file(file).map(|_| "OK".to_string())
    }
}

fn main() {
    // TODO error responses are HTML by default, perhaps something more
    // machinereadable?

    let engine = rocket::ignite()
        .mount("/", routes![root, files, puts])
        .attach(Template::fairing());

    // TODO check response
    env::set_current_dir(&Path::new("data"));

    engine.launch();
}
