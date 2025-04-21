use std::fs;

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
    panic!("Unable to find file path.\nIf using a file with no extension, please use the -f flag.");
}

pub fn get_flags(args: &Vec<String>) -> Vec<(usize, char)> {
    let mut result = Vec::new();
    let valid_flags = vec!['s', 'f', 'i'];
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
    return flags.into_iter().filter(|x| x.1 == target).collect::<Vec<&(usize, char)>>()[0].0 + 1;
    
}

pub fn flags_contains(flags: &Vec<(usize, char)>, target: char) -> bool {
    return flags.iter().map(|x| x.1).collect::<Vec<char>>().contains(&target); 
}

