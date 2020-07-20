use anyhow::Result;

mod gp;

async fn run() -> Result<()> {
  let mut app = tide::new();
  app.at("/gp/:s").get(gp::get_global_property);
  app.at("/gp/keys").get(gp::get_keys);
  app.listen("0.0.0.0:6000").await?;
  Ok(())
}

#[async_std::main]
async fn main() -> Result<()> {
  tide::log::start();
  Ok(run().await?)
}
