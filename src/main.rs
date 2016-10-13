extern crate ruru;

extern crate rustyline;
extern crate rosh;

use rustyline::Editor;
use rustyline::completion::FilenameCompleter;
use rustyline::error::ReadlineError;
use rosh::*;
use ruru::{AnyObject, Class, Object, RString};

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

    let completer = FilenameCompleter::new();
    let mut rl = Editor::new().history_ignore_space(true);
    rl.set_completer(Some(completer));

    if let Err(_) = rl.load_history("history.txt") {
        println!("No previous history.");
    }

    let runner_instance = Class::from_existing("Rosh")
        .get_nested_class("Runner")
        .new_instance(vec![]);

    loop {
        let readline = rl.readline(PROMPT);

        let input = match readline {
            Ok(line) => {
                match line.trim() {
                    "exit" | "quit" => {
                        rl.save_history("history.txt").unwrap();
                        break
                    },
                    "" => continue,
                    l => {
                        rl.add_history_entry(&l);
                        l.to_string()
                    }
                }
            },
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                continue
            },
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break
            },
            Err(err) => {
                println!("Error: {:?}", err);
                continue
            }
        };

        if input == "```" {
            run_ruby_loop();
        } else {
            let ruby_result = reval(input, &runner_instance);
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
