extern crate ruru;

pub mod exceptions;
pub mod file;
pub mod file_system;
pub mod result;

use self::ruru::VM;

pub fn init_ruby() {
    VM::init();
    VM::require("/Users/sloveless/Development/projects/ruosh/lib/ruosh/runner");

    result::init();
    file_system::init();
    file::init();
}
