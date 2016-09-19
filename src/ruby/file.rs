extern crate ruru;

use std::fs::metadata;
use self::ruru::types::Argc;
use self::ruru::{AnyObject, Boolean, Class, Object, NilClass, RString, VM};

use ruby::exceptions;

extern fn is_directory(argc: Argc, argv: *const AnyObject, _: AnyObject) -> Boolean {
    let args = VM::parse_arguments(argc, argv);

    if args.len() != 1 {
        exceptions::raise_argument_error(1usize, args.len());
    }

    let file_name = args[0].try_convert_to::<RString>().unwrap().to_string();

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
    let mut rfile = Class::new("RFile", None);
    rfile.def_self("directory?", is_directory);

    Class::from_existing("RFile").define(|itself| {
        itself.def("meow", meow);
    });

    true
}
