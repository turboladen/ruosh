extern crate ruru;

pub mod file;
// pub mod ssh;
pub mod result;
pub mod exceptions;

use ruru::VM;

#[no_mangle]
pub extern fn init_my_things() {
    file::init();
    // ssh::init();
    result::init();
}

pub fn internal_init() {
    VM::init();
    VM::require("/Users/sloveless/Development/projects/ruosh/lib/ruosh/runner");
    file::init();
    // ssh::init();
    result::init();
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
