use anyhow::Result;
use async_std::task;
use tide::log;

mod gp;

async fn run() -> Result<()> {
  let mut app = tide::new();
  app.at("/gp/:s").get(gp::get_global_property);
  app.at("/gp/keys").get(gp::get_keys);
  app.listen("0.0.0.0:6000").await?;
  Ok(())
}

fn main() -> Result<()> {
  log::start();
  task::block_on(run())
}
