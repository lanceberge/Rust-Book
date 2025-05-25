// Make a project with cargo new <project_name>

mod module;
mod subdir_mod;

use crate::garden::vegetables::Asparagus;

pub mod garden;

fn main() {
    module::say_hello();
    subdir_mod::say_hello::say_hello();
    let plant = Asparagus {};

    println!("I'm growing {plant:?}!");
}
