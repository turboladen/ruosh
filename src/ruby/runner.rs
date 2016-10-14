extern crate ruru;

use self::ruru::{Array, Class, Fixnum, NilClass, Object};

class!(Runner);

methods!(
    Runner,
    itself,

    fn initialize() -> Runner {
        let workspace = Class::from_existing("Rosh")
            .get_nested_class("Workspace");

        itself.instance_variable_set("@binding", workspace.send("get_binding", vec![]));
        itself.instance_variable_set("@command_number", Fixnum::new(0));
        itself.instance_variable_set("@last_error", NilClass::new());

        itself.instance_variable_set("@command_history", Array::new());
        itself.instance_variable_set("@input_history", Array::new());
        itself.instance_variable_set("@result_history", Array::new());

        itself
    }
);

pub extern fn init() {
    Class::from_existing("Rosh").define(|rosh| {
        rosh.define_nested_class("Runner", None).define(|itself| {
            itself.attr_reader("last_error");
            itself.attr_reader("command_history");
            itself.attr_reader("input_history");
            itself.attr_reader("result_history");
            itself.def("initialize", initialize);
        });
    });
}
