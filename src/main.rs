extern crate readline;
extern crate mrusty;

use readline::*;
use mrusty::*;

fn main() {
    let mruby = Mruby::new();

    loop {
        let input = match readline("ruosh> ") {
            Some(input) => input,
            None => {
                println!("");
                break;
            },
        };

        if input == "quit" {
            break;
        }
        // add words that start with 'a' to the history to demonstrate
        else if input[0 .. 1] == "a".to_string() {
            readline::add_history(input.as_ref());
        }

        println!("Input: '{}'", input);
        let ruby_command = format!("x = {}; x.to_s", input);
        println!("rcmd: '{}'", ruby_command);
        let ruby_result = mruby.run(&ruby_command).unwrap();
        println!("Ruby: {}", ruby_result.to_str().unwrap());
    }
}
