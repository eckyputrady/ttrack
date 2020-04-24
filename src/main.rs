mod core;
mod adapter;
mod controller;

use adapter::sqlite::SQLiteActivityRepo;
// use self::core::services;
// use self::core::types::*;
use self::controller::cli;

fn main() {
  let mut db = SQLiteActivityRepo::new().unwrap();
  cli::run(&mut db);
  
  // let mut db = SQLiteActivityRepo::new().unwrap();

  // let req = StartActivityRequest {
  //   name: "Test".to_string(),
  //   tags: vec![Tag { key: "key1".to_string(), val: "value1".to_string()}]
  // };
  // let result = db.start_activity(&req);
  // println!("{:?}", result);
  // let result = db.start_activity(&req);
  // println!("{:?}", result);

  // println!("Status:");
  // for activity in db.status() {
  //   println!("{:?}", activity);
  // }

  // let result = db.stop_activity();
  // println!("{:?}", result);
  // let result = db.stop_activity();
  // println!("{:?}", result);

  // let req = StartActivityRequest {
  //   name: "Test2".to_string(),
  //   tags: vec![]
  // };
  // let result = db.start_activity(&req);
  // println!("{:?}", result);

  // println!("Status:");
  // for activity in db.status() {
  //   println!("{:?}", activity);
  // }
}