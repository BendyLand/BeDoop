use rand::{self, seq::SliceRandom};
use regex::Regex;
use itertools::Itertools;

pub enum TextUtilOp {
    Defang,
    Refang,
    Deburr,
    ShuffleLines,
    SumAll,
    CountChars,
    CollapseLines,
    DedupLines,
    SortLines,
    Trim,
    NaturalSortLines,
    ReverseString,
    LoremIpsum,
    MdQuote,
    ReplaceSmartQuotes,
    Unknown,
}

fn str_to_encoding_op(arg: &str) -> TextUtilOp {
    return match arg {
        "defang" => TextUtilOp::Defang,
        "refang" => TextUtilOp::Refang,
        "deburr" => TextUtilOp::Deburr,
        "shuffle" => TextUtilOp::ShuffleLines,
        "sum" => TextUtilOp::SumAll,
        "count" => TextUtilOp::CountChars,
        "collapse" => TextUtilOp::CollapseLines,
        "dedup" => TextUtilOp::DedupLines,
        "sort" => TextUtilOp::SortLines,
        "trim" => TextUtilOp::Trim,
        "natural_sort" => TextUtilOp::NaturalSortLines,
        "reverse" => TextUtilOp::ReverseString,
        "lorem_ipsum" => TextUtilOp::LoremIpsum,
        "md_quote" => TextUtilOp::MdQuote,
        "replace_smart_quotes" => TextUtilOp::ReplaceSmartQuotes,
        _ => TextUtilOp::Unknown,
    }
}

pub fn select_text_util_option(args: &Vec<String>) -> TextUtilOp {
    let text_util_options: Vec<String> = vec!["defang", "refang", "deburr", "shuffle", "sum", "count", "collapse", "dedup", "sort", "trim", "natural_sort", "reverse", "lorem_ipsum", "md_quote", "replace_smart_quotes"].into_iter().map(|x| x.to_string()).collect();
    for arg in args {
        if text_util_options.contains(&arg.to_lowercase()) {
            return str_to_encoding_op(arg);
        }
    }
    return TextUtilOp::Unknown
}

pub fn handle_text_util_operation(text: &str, op: TextUtilOp) -> String {
    return match op {
        TextUtilOp::Defang => defang(text),
        TextUtilOp::Refang => refang(text),
        TextUtilOp::Deburr => deburr(text),
        TextUtilOp::ShuffleLines => shuffle_lines(text),
        TextUtilOp::SumAll => sum_all(text),
        TextUtilOp::CountChars => count_chars(text),
        TextUtilOp::CollapseLines => collapse_lines(text),
        TextUtilOp::DedupLines => dedup_lines(text),
        TextUtilOp::SortLines => sort_lines(text),
        TextUtilOp::Trim => trim_input(text),
        TextUtilOp::NaturalSortLines => natural_sort_lines(text),
        TextUtilOp::ReverseString => reverse_string(text),
        TextUtilOp::LoremIpsum => lorem_ipsum(),
        TextUtilOp::MdQuote => md_quote(text),
        TextUtilOp::ReplaceSmartQuotes => replace_smart_quotes(text),
        TextUtilOp::Unknown => panic!("Unknown text util operation specified."),
    };
}

fn defang(text: &str) -> String {
    return {
        text
            .replace(".", "[.]")
            .replace("http", "hxxp")
    };
}

fn refang(text: &str) -> String {
    return {
        text
            .replace("hxxp", "http")
            .replace("[.]", ".")
    };
}

fn deburr(text: &str) -> String {
    return deunicode::deunicode(text);
}

fn shuffle_lines(text: &str) -> String {
    let mut lines: Vec<String> = text.lines().map(|x| x.to_string()).collect();
    let mut rng = rand::rng();
    lines.shuffle(&mut rng);
    return lines.join("\n");
}

fn sum_all(text: &str) -> String {
    let pattern = Regex::new(r"[\s\n,;=]").unwrap();
    let tokens: Vec<String> = pattern.split(text).map(|x| x.to_string()).filter(|x| !x.is_empty()).collect();
    let nums: Vec<f32> = tokens.iter().map(|x| x.parse::<f32>().expect("Not a valid number.")).collect();
    let mut total = 0.0;
    for num in nums {
        total += num;
    }
    let result = format!("{}\n= {}", tokens.join("\n"), total);
    return result;
}

fn count_chars(text: &str) -> String {
    return text.chars().count().to_string();
}

fn collapse_lines(text: &str) -> String {
    return text.replace("\n", "").to_string();    
}

fn dedup_lines(text: &str) -> String {
    let lines: Vec<String> = text.lines().map(|x| x.to_string()).unique().collect();
    return lines.join("\n");
}

fn sort_lines(text: &str) -> String {
    let mut lines: Vec<String> = text.lines().map(|x| x.to_string()).collect();
    lines.sort();
    return lines.join("\n");
}

fn trim_input(text: &str) -> String {
    return text.trim().to_string();    
}

fn natural_sort_lines(text: &str) -> String {
    let mut lines: Vec<String> = text.lines().map(|x| x.to_string()).collect();
    lines.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));
    return lines.join("\n");
}

fn reverse_string(text: &str) -> String {
    return text.chars().rev().collect();
}

const LOREM_SECTIONS: &[&str] = &[
    "Lorem ipsum dolor sit amet consectetur adipiscing elit. Blandit quis suspendisse aliquet nisi sodales consequat magna. Sem placerat in id cursus mi pretium tellus. Finibus facilisis dapibus etiam interdum tortor ligula congue. Sed diam urna tempor pulvinar vivamus fringilla lacus. Porta elementum a enim euismod quam justo lectus. Nisl malesuada lacinia integer nunc posuere ut hendrerit. Imperdiet mollis nullam volutpat porttitor ullamcorper rutrum gravida. Ad litora torquent per conubia nostra inceptos himenaeos.", 
    "Ornare sagittis vehicula praesent dui felis venenatis ultrices. Dis parturient montes nascetur ridiculus mus donec rhoncus. Potenti ultricies habitant morbi senectus netus suscipit auctor. Maximus eget fermentum odio phasellus non purus est. Platea dictumst lorem ipsum dolor sit amet consectetur. Dictum risus blandit quis suspendisse aliquet nisi sodales. Vitae pellentesque sem placerat in id cursus mi. Luctus nibh finibus facilisis dapibus etiam interdum tortor.", 
    "Eu aenean sed diam urna tempor pulvinar vivamus. Tincidunt nam porta elementum a enim euismod quam. Iaculis massa nisl malesuada lacinia integer nunc posuere. Velit aliquam imperdiet mollis nullam volutpat porttitor ullamcorper. Taciti sociosqu ad litora torquent per conubia nostra. Primis vulputate ornare sagittis vehicula praesent dui felis. Et magnis dis parturient montes nascetur ridiculus mus. Accumsan maecenas potenti ultricies habitant morbi senectus netus. Mattis scelerisque maximus eget fermentum odio phasellus non.", 
    "Hac habitasse platea dictumst lorem ipsum dolor sit. Vestibulum fusce dictum risus blandit quis suspendisse aliquet. Ex sapien vitae pellentesque sem placerat in id. Neque at luctus nibh finibus facilisis dapibus etiam. Tempus leo eu aenean sed diam urna tempor. Viverra ac tincidunt nam porta elementum a enim. Bibendum egestas iaculis massa nisl malesuada lacinia integer. Arcu dignissim velit aliquam imperdiet mollis nullam volutpat. Class aptent taciti sociosqu ad litora torquent per. Turpis fames primis vulputate ornare sagittis vehicula praesent. Natoque penatibus et magnis dis parturient montes nascetur.", 
    "Feugiat tristique accumsan maecenas potenti ultricies habitant morbi. Nulla molestie mattis scelerisque maximus eget fermentum odio. Cubilia curae hac habitasse platea dictumst lorem ipsum. Mauris pharetra vestibulum fusce dictum risus blandit quis. Quisque faucibus ex sapien vitae pellentesque sem placerat. Ante condimentum neque at luctus nibh finibus facilisis. Duis convallis tempus leo eu aenean sed diam. Sollicitudin erat viverra ac tincidunt nam porta elementum. Nec metus bibendum egestas iaculis massa nisl malesuada.", 
    "Commodo augue arcu dignissim velit aliquam imperdiet mollis. Semper vel class aptent taciti sociosqu ad litora. Cras eleifend turpis fames primis vulputate ornare sagittis. Orci varius natoque penatibus et magnis dis parturient. Proin libero feugiat tristique accumsan maecenas potenti ultricies. Eros lobortis nulla molestie mattis scelerisque maximus eget. Curabitur facilisi cubilia curae hac habitasse platea dictumst. Efficitur laoreet mauris pharetra vestibulum fusce dictum risus. Adipiscing elit quisque faucibus ex sapien vitae pellentesque. Consequat magna ante condimentum neque at luctus nibh. Pretium tellus duis convallis tempus leo eu aenean. Ligula congue sollicitudin erat viverra ac tincidunt nam.", 
];

fn lorem_ipsum() -> String {
    let choice = rand::random_range(0..6);
    return LOREM_SECTIONS[choice].to_string();
}

fn md_quote(text: &str) -> String {
    let lines: Vec<String> = text.lines().map(|x| x.to_string()).collect();
    let result: Vec<String> = lines.into_iter().map(|x| format!(" > {}", x)).collect();
    return result.join("\n");
}

fn replace_smart_quotes(text: &str) -> String {
    return {
        text
            .replace("“", "\"")
            .replace("”", "\"")
            .replace("‘", "'")
            .replace("’", "'")
            .replace("–", "-")
            .replace("—", "-")
            .replace("…", "...")
    };
}

