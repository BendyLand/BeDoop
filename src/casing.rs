extern crate case;
use rand::prelude::*;

use case::CaseExt;

pub enum CaseOp {
    Title, Lower, Upper, Sponge, Snake, Camel, Kebab, Unknown
}

fn title_case(text: &str) -> String {
    let mut result = Vec::<String>::new();
    let lines: Vec<String>  = text.lines().map(|x| x.to_string()).collect();
    for line in lines {
        let mut temp_result = Vec::<String>::new();
        let words: Vec<String> = line.split_whitespace().map(|x| x.to_string()).collect();
        for word in words {
            let lower_word = word.to_lowercase();
            let first_char = lower_word.chars().nth(0).unwrap_or_default().to_uppercase().to_string();
            let temp = first_char + &lower_word[1..];
            temp_result.push(temp);
        }
        result.push(temp_result.join(" "));
    }
    return result.join("\n");
}

fn lower_case(text: &str) -> String {
    return text.to_lowercase();
}

fn upper_case(text: &str) -> String {
    return text.to_uppercase();
}

fn sponge_case(text: &str) -> String {
    let mut result = Vec::<String>::new();
    let lines: Vec<String>  = text.lines().map(|x| x.to_string()).collect();
    for line in lines {
        let mut temp_result = Vec::<String>::new();
        let words: Vec<String> = line.split_whitespace().map(|x| x.to_string()).collect();
        let mut rng = rand::rng();
        for word in words {
            let temp = word.chars().map(|c| 
                if rng.random_bool(0.5) { c.to_lowercase().nth(0).unwrap() } 
                else { c.to_uppercase().nth(0).unwrap() }
            ).collect::<String>();
            temp_result.push(temp);
        }
        result.push(temp_result.join(" "));
    }
    return result.join("\n");
}

fn camel_case(text: &str) -> String {
    return text.to_camel();
}

fn snake_case(text: &str) -> String {
    return text.to_snake();
}

fn kebab_case(text: &str) -> String {
    let temp = text.to_snake();
    return temp.to_dashed();
}

fn str_to_case_op(arg: &str) -> CaseOp {
    return match arg {
        "title" => CaseOp::Title,
        "upper" => CaseOp::Upper,
        "lower" => CaseOp::Lower,
        "sponge" => CaseOp::Sponge,
        "snake" => CaseOp::Snake,
        "camel" => CaseOp::Camel,
        "kebab" => CaseOp::Kebab,
        _ => CaseOp::Unknown
    }
}

pub fn select_case_option(args: &Vec<String>) -> CaseOp {
    let casing_options: Vec<String> = vec!["upper", "lower", "title", "sponge", "snake", "camel", "kebab"].into_iter().map(|x| x.to_string()).collect();
    for arg in args {
        if casing_options.contains(&arg.to_lowercase()) {
            return str_to_case_op(arg);
        }
    }
    return CaseOp::Unknown;
}

pub fn handle_case_operation(text: &str, op: CaseOp) -> String {
    return match op {
        CaseOp::Title => title_case(text),
        CaseOp::Lower => lower_case(text),
        CaseOp::Upper => upper_case(text),
        CaseOp::Sponge => sponge_case(text),
        CaseOp::Snake => snake_case(text),
        CaseOp::Camel => camel_case(text),
        CaseOp::Kebab => kebab_case(text),
        CaseOp::Unknown => panic!("Unknown case operation specified."),
    };
}


