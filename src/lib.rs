extern crate ruru;

mod file;
mod ssh;

use ruru::VM;

#[no_mangle]
pub extern fn init_my_things() {
    file::init();
    ssh::init();
}

pub fn internal_init() {
    VM::init();
    file::init();
    ssh::init();
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
