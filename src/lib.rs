#![allow(non_camel_case_types)]

pub mod test {
  /// Simple test function
  /// please show up in analyzer
  /// # Arguments
  /// * `none` - aint nuttin here
  /// # Examples
  /// ```
  ///   yippie();
  /// ```
  pub fn yippie() {
    println!("Hey it's working");
  }
}

mod inputs;
mod movement;


pub mod prelude {
  pub use crate::test::*;
  pub use crate::inputs::*;
  pub use crate::movement::*;
}
