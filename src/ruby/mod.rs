extern crate ruru;

pub mod exceptions;
pub mod file;
pub mod file_system;
pub mod result;
pub mod runner;
pub mod workspace;

use self::ruru::VM;

pub fn init_ruby() {
    VM::init();
    VM::require("/Users/sloveless/Development/projects/ruosh/lib/rosh/runner");

    runner::init();
    workspace::init();
    result::init();
    file_system::init();
    file::init();
}
