pub mod parser;
pub mod sun_lib;
pub mod tokenizer;
pub mod utils;
pub mod vm;

use utils::run::run;

fn main() {
    run();
}
