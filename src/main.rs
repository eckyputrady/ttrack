mod core;
mod adapter;
mod controller;

use adapter::sqlite::SQLiteActivityRepo;
use self::controller::cli;

fn main() {
  let mut db = SQLiteActivityRepo::new().unwrap();
  cli::run(&mut db);
}