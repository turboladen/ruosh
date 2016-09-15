mod file;
mod ssh;


#[no_mangle]
pub extern fn init_my_things() {
    file::init();
    ssh::init();
}

pub fn internal_init() {
    file::init();
    ssh::init();
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
