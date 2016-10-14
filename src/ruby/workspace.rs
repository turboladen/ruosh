extern crate ruru;

use self::ruru::{AnyObject, Class, Object};

class!(Workspace);

methods!(
    Workspace,
    _itself,

    fn get_binding() -> AnyObject {
        Class::from_existing("Kernel").send("binding", vec![])
    }
);

pub extern fn init() {
    Class::from_existing("Rosh").define(|rosh| {
        rosh.define_nested_class("Workspace", None).define(|itself| {
            itself.def_self("get_binding", get_binding);
        });
    });
}
