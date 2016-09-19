// pub mod ssh;
pub mod ruby;

#[no_mangle]
pub extern fn init_my_things() {
    ruby::init_ruby();
    // ssh::init();
}

pub fn internal_init() {
    ruby::init_ruby();
    // ssh::init();
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
