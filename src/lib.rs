#[macro_use]
extern crate ruru;

pub mod ruby;

#[no_mangle]
pub extern fn init_my_things() {
    ruby::init_ruby();
}

pub fn internal_init() {
    ruby::init_ruby();
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
