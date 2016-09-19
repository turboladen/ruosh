extern crate rustyline;
extern crate ruru;
extern crate rosh;

pub use rustyline::Editor;
use rosh::*;
use ruru::{AnyObject, Class, Object, RString, Symbol};

fn reval(code: String, shell: &AnyObject) -> AnyObject {
    let code = RString::new(code.as_str()).to_any_object();

    shell.send("run", vec![code])
}

fn rputs(ruby_result: AnyObject) {
    let inspected = ruby_result.send("inspect", vec![]);
    Class::from_existing("Kernel").send("puts", vec![inspected]);
}

static PROMPT: &'static str = "\x1b[1;32mrosh>>\x1b[0m ";

fn main() {
    internal_init();
    let mut rl = rustyline::Editor::<()>::new();
    let runner = Class::from_existing("Rosh")
        .send("const_get", vec![Symbol::new("Runner").to_any_object()]);
    let main_thing = unsafe { runner.to::<Class>().new_instance(vec![]) };

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
            run_ruby_loop();
        } else if input.starts_with("require ") {
            let words: Vec<&str> = input.split_whitespace().collect();

            if words.len() == 2 {
                let require_file = words[1];
                require_file.replace("\"", "");
                require_file.replace("'", "");

                println!("requiring: {}", require_file);
                // VM::require(require_file);
            } else {
                println!("Invalid line for require");
            }
        } else {
            let ruby_result = reval(input, &main_thing);
            rputs(ruby_result);
        }
    }
}

fn run_ruby_loop() {
    let mut rl = rustyline::Editor::<()>::new();
    let mut ruby_code = String::new();
    let sub_shell = Class::from_existing("Rosh").new_instance(vec![]);

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
            reval(ruby_code, &sub_shell);
            return
        } else {
            ruby_code = ruby_code + " " + &input + "\n";
        }
    }
}
