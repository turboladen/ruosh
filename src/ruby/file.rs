extern crate ruru;

use std::error::Error;
use std::fs::metadata;
use self::ruru::{Boolean, Class, Object, RString, VM};

class!(File);

methods!(
    File,
    itself,

    fn is_directory(path: RString) -> Boolean {
        if let Err(ref error) = path {
            VM::raise(error.to_exception(), error.description());
        }

        let file_name = path.unwrap().to_string();

        let md = match metadata(file_name) {
            Ok(file_name) => file_name,
            Err(_) => {
                return Boolean::new(false)
            }
        };

        Boolean::new(md.is_dir())
    }
);


pub extern fn init() {
    Class::from_existing("Rosh").define(|rosh| {
        rosh.define_nested_class("File", None).define(|itself| {
            itself.def_self("directory?", is_directory);
            // itself.def("initialize", initialize);
        });
    });
}
