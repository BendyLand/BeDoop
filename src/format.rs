use serde_json::Result;
use lightningcss::stylesheet::{StyleSheet, ParserOptions, MinifyOptions, PrinterOptions};
use sqlformat::{format, FormatOptions, QueryParams};
use xmltree::Element;
use quick_xml::events::Event;
use quick_xml::Reader;

pub enum FormatOp {
    AddSlashes,
    RemoveSlashes,
    FormatJson,
    FormatSql,
    FormatCss,
    FormatXml,
    MinifyJson,
    MinifySql,
    MinifyCss,
    MinifyXml,
    Unknown,
}

fn str_to_format_op(arg: &str) -> FormatOp {
    return match arg {
        "add_slashes" => FormatOp::AddSlashes,
        "remove_slashes" => FormatOp::RemoveSlashes,
        "format_json" => FormatOp::FormatJson,
        "format_sql" => FormatOp::FormatSql,
        "format_css" => FormatOp::FormatCss,
        "format_xml" => FormatOp::FormatXml,
        "minify_json" => FormatOp::MinifyJson,
        "minify_sql" => FormatOp::MinifySql,
        "minify_css" => FormatOp::MinifyCss,
        "minify_xml" => FormatOp::MinifyXml,
        _ => FormatOp::Unknown,
    }
}

pub fn select_format_option(args: &Vec<String>) -> FormatOp {
    let format_ops: Vec<String> = vec!["add_slashes", "remove_slashes", "format_json", "format_sql", "format_css", "format_xml", "minify_json", "minify_sql", "minify_css", "minify_xml"].into_iter().map(|x| x.to_string()).collect();
    for arg in args {
        if format_ops.contains(&arg.to_lowercase()) {
            return str_to_format_op(arg);
        }
    }
    return FormatOp::Unknown
}

pub fn handle_format_operation(text: &str, op: FormatOp) -> String {
    return match op {
        FormatOp::AddSlashes => add_slashes(text),
        FormatOp::RemoveSlashes => remove_slashes(text),
        FormatOp::FormatJson => format_json(text),
        FormatOp::MinifyJson => minify_json(text),
        FormatOp::FormatCss => format_css(text),
        FormatOp::MinifyCss => minify_css(text),
        FormatOp::FormatSql => format_sql(text),
        FormatOp::MinifySql => minify_sql(text),
        FormatOp::FormatXml => format_xml(text),
        FormatOp::MinifyXml => minify_xml(text),
        _ => panic!("Unknown format operation specified."),
    };
}

fn add_slashes(text: &str) -> String {
    return text.chars().map(|c| 
        if c == '\'' || c == '"' { format!("\\{}", c) }
        else { c.to_string() }
    ).collect();    
}

fn remove_slashes(text: &str) -> String {
    let mut result = text.replace("\\\"", "\"");
    result = result.replace("\\'", "'");
    return result;
}

fn format_json(text: &str) -> String {
    let val: Result<serde_json::Value> = serde_json::from_str(text);
    if let Ok(value) = val {
        let res = serde_json::to_string_pretty(&value);
        match res {
            Err(e) => panic!("Unable to format JSON: {}", e),
            Ok(result) => return result,
        }
    }
    else { panic!("Unable to read JSON data to string (format_json).") }
}

fn minify_json(text: &str) -> String {
    let val: Result<serde_json::Value> = serde_json::from_str(text);
    if let Ok(value) = val {
        let res = serde_json::to_string(&value);
        match res {
            Err(e) => panic!("Unable to minify JSON: {}", e),
            Ok(result) => return result,
        }
    }
    else { panic!("Unable to read JSON data to string (minify_json).") }
}

fn format_css(text: &str) -> String {
    let stylesheet = StyleSheet::parse(text, ParserOptions::default()).unwrap();
    let result = stylesheet.to_css(PrinterOptions::default()).unwrap();
    return result.code;
}

fn minify_css(text: &str) -> String {
    let mut stylesheet = StyleSheet::parse(text, ParserOptions::default()).unwrap();
    let _ = stylesheet.minify(MinifyOptions::default());
    let result = stylesheet.to_css(PrinterOptions::default()).unwrap();
    return result.code;
}

fn format_sql(sql: &str) -> String {
    return format(sql, &QueryParams::None, &FormatOptions::default());
}

fn minify_sql(sql: &str) -> String {
    sql.split_whitespace().collect::<Vec<_>>().join(" ")
}

pub fn format_xml(input: &str) -> String {
    let mut reader = Reader::from_str(input);
    reader.config_mut().trim_text(true);
    let mut buf = Vec::new();
    let mut output = String::new();
    let mut indent_level = 0;
    loop {
        let event = reader.read_event_into(&mut buf);
        match event {
            Ok(Event::Start(e)) => {
                output.push_str(&format!(
                    "{}<{}>\n",
                    "  ".repeat(indent_level),
                    match String::from_utf8(e.name().as_ref().to_vec()) {
                        Ok(name) => name,
                        Err(_) => "[invalid tag]".to_string(),
                    }
                ));
                indent_level += 1;
            }
            Ok(Event::End(e)) => {
                indent_level = indent_level.saturating_sub(1); // avoid underflow
                output.push_str(&format!(
                    "{}</{}>\n",
                    "  ".repeat(indent_level),
                    match String::from_utf8(e.name().as_ref().to_vec()) {
                        Ok(name) => name,
                        Err(_) => "[invalid tag]".to_string(),
                    }
                ));
            }
            Ok(Event::Text(e)) => {
                match e.unescape() {
                    Ok(text) => {
                        let trimmed = text.trim();
                        if !trimmed.is_empty() {
                            output.push_str(&format!("{}{}\n", "  ".repeat(indent_level), trimmed));
                        }
                    }
                    Err(_) => output.push_str(&format!(
                        "{}[invalid text]\n",
                        "  ".repeat(indent_level)
                    )),
                }
            }
            Ok(Event::Eof) => break,
            Err(_) => {
                output.push_str("[error reading XML stream]\n");
                break;
            }
            _ => {} // Ignore comments, declarations, etc.
        }
        buf.clear();
    }
    return output;
}

fn clean_output(output: &str) -> String {
    let lines: Vec<String> = output.lines().map(|x| x.to_string()).collect();
    let mut result = Vec::<String>::new();
    for line in lines {
        let temp = line.trim().to_string();
        result.push(temp);
    }
    return result.join("");
}

fn minify_xml(input: &str) -> String {
    let root_res = Element::parse(input.as_bytes());
    let mut output = Vec::new();
    let output = match root_res {
        Err(e) => panic!("Unable to parse input to XML data: {}", e),
        Ok(root) => {
            let _ = root.write(&mut output);
            let res = String::from_utf8(output);
            match res {
                Err(e) => panic!("Unable to convert XML data to string: {}", e),
                Ok(out) => out,
            }
        }
    };
    return clean_output(&output);
}

