extern crate readline;
extern crate ruru;
extern crate ruosh;

use readline::*;
use ruosh::*;
use ruru::{Class, AnyObject, RString};
use ruru::traits::Object;

fn reval(code: String) -> AnyObject {
    let code = RString::new(code.as_str()).to_any_object();

    Class::from_existing("Kernel").send("eval", vec![code])
}

fn rputs(ruby_result: AnyObject) {
    let inspected = ruby_result.send("inspect", vec![]);
    Class::from_existing("Kernel").send("puts", vec![inspected]);
}

fn main() {
    internal_init();

    loop {
        let input = match readline("ruosh> ") {
            Some(input) => input,
            None => {
                println!("");
                break;
            },
        };

        if input == "quit" || input == "exit" {
            println!("Quitting...");
            break;
        }

        readline::add_history(input.as_ref());
        let ruby_result = reval(input);
        rputs(ruby_result);
    }
}
