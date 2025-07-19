pub mod lib;
pub use crate::lib::execute;

fn main() {
  execute();
  print!("Hello, world!");
}