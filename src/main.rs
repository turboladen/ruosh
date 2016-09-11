extern crate readline;
extern crate ruosh;
extern crate ruru;

use readline::*;
use ruosh::*;

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

        // println!("Input: '{}'", input);
        // let ruby_command = format!("x = {}; x.to_s", input);
        // println!("rcmd: '{}'", ruby_command);
        // let ruby_result = mruby.run(&ruby_command).unwrap();
        // println!("Ruby: {}", ruby_result.to_str().unwrap());
    }
}
