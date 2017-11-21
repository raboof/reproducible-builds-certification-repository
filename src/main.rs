#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;

use std::io;
use std::fs;
use std::env;
use std::path::{Path, PathBuf};

use rocket::Data;
use rocket::Request;
use rocket::Response;
use rocket::http::Status;
use rocket::response::NamedFile;
use rocket::response::Responder;

enum RetrievedData {
  Certification(NamedFile),
  Index(String),
}
use RetrievedData::*;

impl<'r> Responder<'r> for RetrievedData {

  fn respond_to(self, request: &Request) -> Result<Response<'r>, Status> {
    match self {
      Certification(file) => file.respond_to(request),
      Index(listing) => listing.respond_to(request),
    }
  }
}

#[get("/<file..>")]
fn files(file: PathBuf) -> Result<RetrievedData, String> {
    if file.is_dir() {
      match fs::read_dir(file) {
        Ok(dir) => {
          let mut s = "Is a directory, ".to_owned();
          for entry in dir {
            match entry {
              Ok(e) => match e.path().to_str() {
                Some(p) => s.push_str(p),
                None => ()
              },
              Err(_) => ()
            }
          };
          Ok(Index(s.to_string()))
        },
        _e => Err("Could not open".to_string())
      }
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

#[post("/<file..>", data = "<body>")]
fn posts(file: PathBuf, body: Data) -> io::Result<String> {
    let loc = Path::new("data/").join(file);
    if loc.exists() {
        // TODO rocket turns this into a 404, find out how to return
        // a better error message
        Err(io::Error::new(io::ErrorKind::Other, "Already exists"))
    } else {
        body.stream_to_file(loc).map(|_| "OK".to_string())
    }
}

fn main() {
    // TODO error responses are HTML by default, perhaps something more
    // machinereadable?
    env::set_current_dir(&Path::new("data"));

    rocket::ignite()
        .mount("/", routes![files, posts])
        .launch();
}
