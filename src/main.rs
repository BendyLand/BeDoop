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
    let result: String;
    match command_family {
        utils::CommandFamily::Casing => {
            let case_op = casing::select_case_option(&args);
            result = casing::handle_case_operation(&text, case_op);
        },
        utils::CommandFamily::Encoding => {
            let encoding_op = encoding::select_encoding_option(&args);
            result = encoding::handle_encoding_operation(&text, encoding_op);
        },
        utils::CommandFamily::Format => {
            let format_op = format::select_format_option(&args);
            result = format::handle_format_operation(&text, format_op);
        }
        _ => {
            eprintln!("Operation not implemented yet.");
            return;
        },
    }
    let path = utils::find_file_path(&args);
    handle_result(&result, path);
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

