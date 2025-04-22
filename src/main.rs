use std::{env, fs};

mod utils;
mod encoding;
mod casing;
mod conversion;
mod format;
mod text_utils;

enum PathChange {
    JsonCsv, CsvJson, JsonYaml, YamlJson, NoChange,
}

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
    let change: PathChange;
    let result: String;
    match command_family {
        utils::CommandFamily::Casing => {
            let case_op = casing::select_case_option(&args);
            result = casing::handle_case_operation(&text, case_op);
            change = PathChange::NoChange;
        },
        utils::CommandFamily::Encoding => {
            let encoding_op = encoding::select_encoding_option(&args);
            result = encoding::handle_encoding_operation(&text, encoding_op);
            change = PathChange::NoChange;
        },
        utils::CommandFamily::Format => {
            let format_op = format::select_format_option(&args);
            result = format::handle_format_operation(&text, format_op);
            change = PathChange::NoChange;
        },
        utils::CommandFamily::TextUtils => {
            let format_op = text_utils::select_text_util_option(&args);
            result = text_utils::handle_text_util_operation(&text, format_op);
            change = PathChange::NoChange;
        },
        utils::CommandFamily::Conversion => {
            let conversion_op = conversion::select_conversion_option(&args);
            result = conversion::handle_conversion_operation(&text, &conversion_op);
            match conversion_op {
                conversion::ConversionOp::YamlToJson => change = PathChange::YamlJson,
                conversion::ConversionOp::JsonToYaml => change = PathChange::JsonYaml,
                conversion::ConversionOp::CsvToJson=> change = PathChange::CsvJson,
                conversion::ConversionOp::JsonToCsv => change = PathChange::JsonCsv,
                _ => change = PathChange::NoChange,
            }
        },
        _ => {
            eprintln!("Operation not implemented yet.");
            return;
        },
    }
    let path = utils::find_file_path(&args);
    handle_result(&result, path, change);
}

fn update_path(path: &str, change: PathChange) -> String {
    let idx = path.rfind(".").unwrap_or(path.len()-1);
    let prefix = path[..idx].to_string();
    return match change {
        PathChange::JsonCsv => format!("{}.csv", prefix),
        PathChange::CsvJson => format!("{}.json", prefix),
        PathChange::JsonYaml => format!("{}.yaml", prefix),
        PathChange::YamlJson => format!("{}.json", prefix),
        _ => path.to_string(),
    }
}

fn handle_result(result: &String, path: Option<String>, change: PathChange) {
    match path {
        None => println!("{}", result),
        _ => { 
            match change {
                PathChange::NoChange => {
                    let res = fs::write(path.unwrap(), result);  
                    match res {
                        Ok(_) => println!("File updated!"),
                        Err(e) => eprintln!("Unable to write file: {}", e),
                    }
                },
                _ => {
                    let new_path = update_path(path.as_ref().unwrap(), change);
                    let res = fs::write(new_path, result);  
                    match res {
                        Ok(_) => println!("File updated!"),
                        Err(e) => eprintln!("Unable to write file: {}", e),
                    }
                }
            }
        }
    }
}

