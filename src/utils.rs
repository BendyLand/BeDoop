use std::fs;

pub enum CommandFamily {
    Casing, Encoding, Format, TextUtils, Conversion, Unknown,
}

pub fn find_file_path(args: &Vec<String>) -> Option<String> {
    let flags = get_flags(args);
    if flags_contains(&flags, 'f') {
        let idx = get_flag_idx(&flags, 'f');
        return Some(args[idx].clone());
    }
    for arg in args {
        if arg.contains(".") {
            return Some(arg.clone());
        }
    }
    return None;
}

pub fn get_file_contents(args: &Vec<String>) -> String {
    let maybe_path = find_file_path(args);
    if let Some(path) = maybe_path {
        let contents = fs::read_to_string(path);
        return match contents {
            Ok(text) => text,
            Err(_) => panic!("Invalid file path."),
        };
    }
    eprintln!("Unable to find file path.\nIf using a file with no extension, please use the -f flag.\nIf using a string, please use the -s flag.");
    std::process::exit(1);
}

pub fn get_flags(args: &Vec<String>) -> Vec<(usize, char)> {
    let mut result = Vec::new();
    let valid_flags = vec!['s', 'f', 'i', 'h'];
    for (i, arg) in args.iter().enumerate() {
        if arg.starts_with("-") {
            let flags: Vec<char> = {
                arg
                    .chars()
                    .skip(1) // skip dash
                    .filter(|c| valid_flags.contains(c))
                    .collect()
            };
            result.extend_from_slice(
                &flags
                    .into_iter()
                    .map(|c| (i, c))
                    .collect::<Vec<(usize, char)>>()
            );
        }
    }
    return result;
}

pub fn get_flag_idx(flags: &Vec<(usize, char)>, target: char) -> usize {
    return flags.iter().find(|(_, c)| *c == target).map(|(i, _)| i + 1).expect("Flag not found");
}

pub fn flags_contains(flags: &Vec<(usize, char)>, target: char) -> bool {
    return flags.iter().map(|x| x.1).collect::<Vec<char>>().contains(&target); 
}

pub fn get_command_family(args: &Vec<String>) -> CommandFamily {
    let casing_options: Vec<String> = vec!["upper", "lower", "title", "sponge", "snake", "camel", "kebab"].into_iter().map(|x| x.to_string()).collect();
    let encoding_options: Vec<String> = vec!["rot13", "base64_encode", "base64_decode", "md5", "html_encode", "html_decode", "html_encode_all", "url_encode", "url_decode", "url_entity_encode", "url_entities_decode", "sha1", "sha256", "sha512"].into_iter().map(|x| x.to_string()).collect();
    let format_options: Vec<String> = vec!["add_slashes", "remove_slashes", "format_json", "format_sql", "format_css", "format_xml", "minify_json", "minify_sql", "minify_css", "minify_xml"].into_iter().map(|x| x.to_string()).collect();
    let text_util_options: Vec<String> = vec!["defang", "refang", "deburr", "shuffle", "sum", "count", "collapse", "dedup", "sort", "trim", "natural_sort", "reverse", "lorem_ipsum", "md_quote", "replace_smart_quotes"].into_iter().map(|x| x.to_string()).collect();
    let conversion_options: Vec<String> = vec!["ascii_to_hex", "hex_to_ascii", "yaml_to_json", "json_to_yaml", "date_to_timestamp", "date_to_utc", "binary_to_decimal", "decimal_to_binary", "json_to_query_string", "query_strong_to_json", "decimal_to_hex", "hex_to_decimal", "json_to_csv", "csv_to_json", "hex_to_rgb", "fish_path_hex_conv"].into_iter().map(|x| x.to_string()).collect();
    for arg in args {
        if casing_options.contains(&arg.to_lowercase()) {
            return CommandFamily::Casing;
        }
        else if encoding_options.contains(&arg.to_lowercase()) {
            return CommandFamily::Encoding;
        }
        else if format_options.contains(&arg.to_lowercase()) {
            return CommandFamily::Format;
        }
        else if text_util_options.contains(&arg.to_lowercase()) {
            return CommandFamily::TextUtils;
        }
        else if conversion_options.contains(&arg.to_lowercase()) {
            return CommandFamily::Conversion;
        }
    }
    return CommandFamily::Unknown;
}

pub fn print_commands() {
    println!("Available options:");
    let casing_options: Vec<String> = vec!["upper", "lower", "title", "sponge", "snake", "camel", "kebab"].into_iter().map(|x| x.to_string()).collect();
    let encoding_options: Vec<String> = vec!["rot13", "base64_encode", "base64_decode", "md5", "html_encode", "html_decode", "html_encode_all", "url_encode", "url_decode", "url_entity_encode", "url_entities_decode", "sha1", "sha256", "sha512"].into_iter().map(|x| x.to_string()).collect();
    let format_options: Vec<String> = vec!["add_slashes", "remove_slashes", "format_json", "format_sql", "format_css", "format_xml", "minify_json", "minify_sql", "minify_css", "minify_xml"].into_iter().map(|x| x.to_string()).collect();
    let text_util_options: Vec<String> = vec!["defang", "refang", "deburr", "shuffle", "sum", "count", "collapse", "dedup", "sort", "trim", "natural_sort", "reverse", "lorem_ipsum", "md_quote", "replace_smart_quotes"].into_iter().map(|x| x.to_string()).collect();
    let conversion_options: Vec<String> = vec!["ascii_to_hex", "hex_to_ascii", "yaml_to_json", "json_to_yaml", "date_to_timestamp", "date_to_utc", "binary_to_decimal", "decimal_to_binary", "json_to_query_string", "query_strong_to_json", "decimal_to_hex", "hex_to_decimal", "json_to_csv", "csv_to_json", "hex_to_rgb", "fish_path_hex_conv"].into_iter().map(|x| x.to_string()).collect();
    println!("CASING:");
    for option in casing_options {
        println!(" - {}", option);
    }
    println!("ENCODING:");
    for option in encoding_options {
        println!(" - {}", option);
    }
    println!("FORMAT:");
    for option in format_options {
        println!(" - {}", option);
    }
    println!("TEXT UTILS:");
    for option in text_util_options {
        println!(" - {}", option);
    }
    println!("CONVERSION:");
    for option in conversion_options {
        println!(" - {}", option);
    }
}
