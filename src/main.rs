use evaluator::eval_node;
use reader::read;
use rustyline::{error::ReadlineError, DefaultEditor};

pub mod evaluator;
pub mod nodes;
pub mod reader;

fn main() {
    let mut rl = DefaultEditor::new().unwrap();

    #[cfg(feature = "with-file-history")]
    if rl.load_history(config_dir()).is_err() {
        //
    }

    loop {
        let readline = rl.readline("> ");

        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str()).unwrap();
                let node = read(line).unwrap();
                println!("{:?}", eval_node(node));
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
}
