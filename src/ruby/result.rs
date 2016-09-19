extern crate ruru;

use self::ruru::types::Argc;
use self::ruru::{AnyObject, Boolean, Class, Fixnum, NilClass, Object, VM};

use ruby::exceptions;

extern fn is_error(_: Argc, _: *const AnyObject, itself: AnyObject) -> Boolean {
    let object = itself.instance_variable_get("@object");
    let exception_class = Class::from_existing("Exception");

    let result = object.send("is_a?", vec![exception_class.to_any_object()]);

    unsafe { result.to::<Boolean>() }
}

extern fn initialize(argc: Argc, argv: *const AnyObject, mut itself: AnyObject) -> NilClass {
    let args = VM::parse_arguments(argc, argv);

    if args.len() != 2 {
        exceptions::raise_argument_error(2usize, args.len());
    }

    let ref object = args[0];
    let ref command_number = args[1];

    let object = unsafe { object.to::<AnyObject>() };
    itself.instance_variable_set("@object", object);

    let command_number = unsafe { command_number.to::<Fixnum>() };
    itself.instance_variable_set("@command_number", command_number);

    NilClass::new()
}

pub extern fn init() {
    Class::from_existing("Ruosh").define(|ruosh| {
        ruosh.define_nested_class("Result", None).define(|itself| {
            itself.def("initialize", initialize);
            itself.def("error?", is_error);
        });
    });
}
