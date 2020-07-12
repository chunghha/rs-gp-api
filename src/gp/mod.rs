use anyhow::Result;
use ron::de::from_reader;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs::File};
use tide::{log, Body, Request, Response};

#[derive(Debug, Deserialize)]
struct Config {
  gp: HashMap<String, String>,
}

trait IConfig {
  fn get_global_properties() -> Config;
  fn get_keys_vec(&self) -> Vec<String>;
  fn get_value_by_key(&self, k: &String) -> String;
}

impl IConfig for Config {
  fn get_global_properties() -> Config {
    match get_config_file() {
      Ok(x) => x,
      Err(e) => {
        log::error!("Failed to load the config file: {}", e);
        std::process::exit(1);
      }
    }
  }

  fn get_keys_vec(&self) -> Vec<String> {
    self.gp.keys().map(|v| v.clone()).collect::<Vec<String>>()
  }

  fn get_value_by_key(&self, k: &String) -> String {
    self.gp.get(k).as_deref().unwrap_or(&"NOT_FOUND".to_owned()).to_string()
  }
}

#[derive(Deserialize, Serialize)]
struct GlobalProperty {
  key: String,
  value: String,
}

#[derive(Deserialize, Serialize)]
struct Keys {
  keys: Vec<String>,
}

pub async fn get_global_property(req: Request<()>) -> tide::Result<Response> {
  let k: String = req.param("s").unwrap_or_else(|_| "key".to_owned());

  respond(&GlobalProperty { key: k.clone(), value: Config::get_global_properties().get_value_by_key(&k) })
}

pub async fn get_keys(_req: Request<()>) -> tide::Result<Response> {
  respond(&Keys { keys: Config::get_global_properties().get_keys_vec() })
}

fn get_config_file() -> Result<Config> {
  Ok(from_reader(File::open(&format!("{}/ron/global_properties.ron", env!("CARGO_MANIFEST_DIR")))?)?)
}

fn respond(json: &impl Serialize) -> tide::Result<Response> {
  let mut res = Response::new(200);
  res.set_body(Body::from_json(json)?);

  Ok(res)
}
