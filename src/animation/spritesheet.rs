use crate::prelude::*;

pub fn load_character_sprite_data(raw_path: &str) {
  let path = Path::new(raw_path);
  //let file = File::open(path);
  if let Ok(raw_string) = read_to_string(path) {
    let raw_slice = &raw_string[..]; 
    let v: Value = from_str(raw_slice).unwrap();
    println!("{}",v["test"]);
  }
}
