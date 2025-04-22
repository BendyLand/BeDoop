use std::{env, fs};

mod utils;
mod encoding;
mod casing;
mod conversion;
mod format;
mod text_utils;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: bdp [flags] <filename>");
        return;
    }
    let flags = utils::get_flags(&args);
    let text: String;
    if utils::flags_contains(&flags, 'i') {
        // TODO
        // start_interactive_session()
        return;
    }
    else if utils::flags_contains(&flags, 'h') {
        utils::print_commands();
        return;
    }
    if utils::flags_contains(&flags, 's') {
        let idx = utils::get_flag_idx(&flags, 's');
        text = args[idx].clone();
    }
    else { text = utils::get_file_contents(&args); }
    let command_family = utils::get_command_family(&args);
    match command_family {
        utils::CommandFamily::Casing => {
            let case_op = casing::select_case_option(&args);
            let result = casing::handle_case_operation(&text, case_op);
            let path = utils::find_file_path(&args);
            handle_result(&result, path);
        },
        utils::CommandFamily::Encoding => {
            let encoding_op = encoding::select_encoding_option(&args);
            let result = encoding::handle_encoding_operation(&text, encoding_op);
            let path = utils::find_file_path(&args);
            handle_result(&result, path);
        },
        _ => println!("Operation not implemented yet.")
    }
}

fn handle_result(result: &String, path: Option<String>) {
    match path {
        None => println!("{}", result),
        _ => { 
            let res = fs::write(path.unwrap(), result);  
            match res {
                Ok(_) => println!("File updated!"),
                Err(e) => eprintln!("Unable to write file: {}", e),
            }
        }
    }
}

