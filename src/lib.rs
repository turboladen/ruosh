extern crate ruru;

mod file;

use ruru::VM;

#[no_mangle]
pub extern fn init_my_things() {
    file::init();
}

pub fn internal_init() {
    VM::init();
    file::init();
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
