extern crate ruru;

use std::fs::metadata;
use ruru::types::Argc;
use ruru::{AnyObject, Boolean, Class, NilClass, RString, VM};

extern fn is_directory(argc: Argc, argv: *const AnyObject, _: AnyObject) -> Boolean {
    let argv = VM::parse_arguments(argc, argv);
    let file_name = argv[0].to::<RString>().to_string();
    let md = match metadata(file_name) {
        Ok(file_name) => file_name,
        Err(_) => {
            return Boolean::new(false)
        }
    };

    Boolean::new(md.is_dir())
}

extern fn meow(_: Argc, _: *const AnyObject, _: AnyObject) -> NilClass {
    println!("MEOWWWWWWWWWWWWWWWWW");
    NilClass::new()
}

pub extern fn init() -> bool {
    let mut rfile = Class::new("RFile");
    rfile.def_self("directory?", is_directory);

    Class::new("RFile").define(|itself| {
        itself.def("meow", meow);
    });

    true
}
