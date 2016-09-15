extern crate rustyline;
extern crate mrusty;
extern crate ruosh;

pub use rustyline::Editor;
use ruosh::*;
use mrusty::*;
use std::rc::Rc;
use std::cell::RefCell;

static PROMPT: &'static str = "\x1b[1;32m>>\x1b[0m ";

fn run_ruby_loop(mruby: &Rc<RefCell<Mruby>>) {
    let mut rl = rustyline::Editor::<()>::new();
    let mut ruby_code = String::new();

    loop {
        let ruby_readline = rl.readline("rubby>> ");

        let input = match ruby_readline {
            Ok(line) => {
                rl.add_history_entry(&line);
                line
            },
            Err(_)   => {
                println!("No input; breaking");
                break;
            },
        };

        // Done with Ruby input, so eval it
        if input == "```" {
            println!("code: {}", ruby_code);
            run_ruby(&mruby, ruby_code.as_str());
            return
        } else {
            ruby_code = ruby_code + " " + &input + "\n";
        }
    }
}

fn main() {
    // internal_init();
    let mut rl = rustyline::Editor::<()>::new();
    let mruby = Mruby::new();

    loop {
        let readline = rl.readline(PROMPT);

        let input = match readline {
            Ok(line) => {
                rl.add_history_entry(&line);
                line
            },
            Err(_)   => {
                println!("No input");
                break;
            },
        };

        if input.is_empty() { continue };

        if input == "quit" || input == "exit" {
            println!("Quitting...");
            break;
        }

        if input == "```" {
            run_ruby_loop(&mruby);
        } else {
            run_ruby(&mruby, input.as_str());
        }
    }
}

fn run_ruby(mruby: &Rc<RefCell<Mruby>>, code: &str) {
    let result = mruby.run(code);

    match result {
        Ok(value) => println!("Debug: {:?}", value),
        Err(ex) => println!("BOOM: {:?}", ex)
    };
}
