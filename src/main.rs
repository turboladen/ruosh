extern crate rustyline;
extern crate ruru;
extern crate ruosh;

pub use rustyline::Editor;
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
    let mut rl = rustyline::Editor::<()>::new();

    loop {
        let readline = rl.readline(">> ");
        let input = match readline {
            Ok(line) => {
                println!("Line: {:?}", line);
                rl.add_history_entry(&line);
                line
            },
            Err(_)   => {
                println!("No input");
                break;
            },
        };

        if input == "quit" || input == "exit" {
            println!("Quitting...");
            break;
        }

        let ruby_result = reval(input);
        rputs(ruby_result);
    }
}
