extern crate ruru;

use std::process::Command;
use self::ruru::types::Argc;
use self::ruru::{AnyObject, Array, Class, Object, RString, VM};

use ruby::exceptions;

extern fn list_files(argc: Argc, argv: *const AnyObject, _: AnyObject) -> Array {
    let args = VM::parse_arguments(argc, argv);

    if args.len() != 0 {
        exceptions::raise_argument_error(0usize, args.len());
    }

    let output = Command::new("ls")
                    .output()
                    .ok()
                    .expect("ls command failed to start");

    let output_string = String::from_utf8_lossy(&output.stdout);

    let file_names: Array = output_string.split("\n")
                                .map(|f| RString::new(f).to_any_object())
                                .collect();

    file_names
}

pub extern fn init() {
    Class::from_existing("Rosh").define(|rosh| {
        rosh.define_nested_class("FileSystem", None).define(|itself| {
            itself.def("list", list_files);
        });
    });
}
