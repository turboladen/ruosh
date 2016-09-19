extern crate ruru;

use self::ruru::{Class, VM};

pub fn raise_argument_error(expected_count: usize, got_count: usize) {
    let message = format!("wrong number of arguments ({} for {})", expected_count, got_count);
    VM::raise(Class::from_existing("ArgumentError"), message.as_str());
}
