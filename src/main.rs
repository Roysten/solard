use clap::Parser;

mod app;
mod db_io;
mod protocol;
mod serial_io;
mod solar_data;

use crate::app::App;

#[derive(Parser)]
#[clap(version)]
struct Args {}

fn main() {
    let _args = Args::parse();
    let app = App::new("solar.sqlite3");
    match app {
        Ok(mut app) => app.run(),
        Err(x) => println!("{:?}", x),
    }
}
