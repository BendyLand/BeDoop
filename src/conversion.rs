use std::collections::HashMap;
use serde_json::Value;
use std::collections::BTreeSet;

pub enum ConversionOp {
    AsciiToHex, HexToAscii, YamlToJson, JsonToYaml, DateToTimestamp, DateToUtc,
    BinaryToDecimal, DecimalToBinary, JsonToQueryString, QueryStringToJson, DecimalToHex,
    HexToDecimal, JsonToCsv, CsvToJson, HexToRgb, FishPathHexConv, Unknown,
}

fn str_to_conversion_op(arg: &str) -> ConversionOp {
    return match arg {
       "ascii_to_hex" => ConversionOp::AsciiToHex,
       "hex_to_ascii" => ConversionOp::HexToAscii,
       "yaml_to_json" => ConversionOp::YamlToJson,
       "json_to_yaml" => ConversionOp::JsonToYaml,
       "date_to_timestamp" => ConversionOp::DateToTimestamp,
       "date_to_utc" => ConversionOp::DateToUtc,
       "binary to_decimal" => ConversionOp::BinaryToDecimal,
       "decimal_to_binary" => ConversionOp::DecimalToBinary,
       "json_to_query_string" => ConversionOp::JsonToQueryString,
       "query_string_to_json" => ConversionOp::QueryStringToJson,
       "decimal_to_hex" => ConversionOp::DecimalToHex,
       "hex_to_decimal" => ConversionOp::HexToDecimal,
       "json_to_csv" => ConversionOp::JsonToCsv,
       "csv_to_json" => ConversionOp::CsvToJson,
       "hex_to_rgb" => ConversionOp::HexToRgb,
       "fish_path_hex_conv" => ConversionOp::FishPathHexConv,
        _ => ConversionOp::Unknown,
    }
}

pub fn select_conversion_option(args: &Vec<String>) -> ConversionOp {
    let conversion_options: Vec<String> = vec!["ascii_to_hex", "hex_to_ascii", "yaml_to_json", "json_to_yaml", "date_to_timestamp", "date_to_utc", "binary_to_decimal", "decimal_to_binary", "json_to_query_string", "query_strong_to_json", "decinal_to_hex", "hex_to_decimal", "json_to_csv", "csv_to_json", "hex_to_rgb", "fish_path_hex_conv"].into_iter().map(|x| x.to_string()).collect();
    for arg in args {
        if conversion_options.contains(&arg.to_lowercase()) {
            return str_to_conversion_op(arg);
        }
    }
    return ConversionOp::Unknown
}

pub fn handle_conversion_operation(text: &str, op: &ConversionOp) -> String {
    return match op {
        ConversionOp::AsciiToHex => ascii_to_hex(text),
        ConversionOp::HexToAscii => hex_to_ascii(text),
        ConversionOp::YamlToJson => yaml_to_json(text),
        ConversionOp::JsonToYaml => json_to_yaml(text),
        ConversionOp::DateToTimestamp => date_to_timestamp(text),
        ConversionOp::DateToUtc => date_to_utc(text),
        ConversionOp::BinaryToDecimal => binary_to_decimal(text),
        ConversionOp::DecimalToBinary => decimal_to_binary(text),
        ConversionOp::JsonToQueryString => json_to_query_string(text),
        ConversionOp::QueryStringToJson => query_string_to_json(text),
        ConversionOp::DecimalToHex => decimal_to_hex(text),
        ConversionOp::HexToDecimal => hex_to_decimal(text),
        ConversionOp::JsonToCsv => json_to_csv(text),
        ConversionOp::CsvToJson => csv_to_json(text),
        ConversionOp::HexToRgb => hex_to_rgb(text),
        ConversionOp::FishPathHexConv => fish_path_hex_conv(text),
        ConversionOp::Unknown => panic!("Unknown conversion operation specified."),
    };
}

fn ascii_to_hex(text: &str) -> String {
    return hex::encode(text);
}

fn hex_to_ascii(text: &str) -> String {
    if let Ok(temp) = hex::decode(text) {
        let res = String::from_utf8(temp);
        match res {
            Err(e) => panic!("Unable to convert hex to UTF-8: {}", e),
            Ok(result) => return result,
        }
    }
    panic!("Invalid hex provided.");
}

fn yaml_to_json(text: &str) -> String {
    match serde_yaml::from_str::<serde_json::Value>(text) {
        Ok(value) => match serde_json::to_string_pretty(&value) {
            Ok(json) => json,
            Err(e) => format!("Failed to convert to JSON: {}", e),
        },
        Err(e) => format!("Failed to parse YAML: {}", e),
    }
}

fn json_to_yaml(text: &str) -> String {
    match serde_json::from_str::<serde_json::Value>(text) {
        Ok(value) => match serde_yaml::to_string(&value) {
            Ok(yaml) => yaml,
            Err(e) => format!("Failed to convert to YAML: {}", e),
        },
        Err(e) => format!("Failed to parse JSON: {}", e),
    }
}

fn date_to_timestamp(text: &str) -> String {
    match chrono::DateTime::parse_from_rfc3339(text) {
        Ok(dt) => dt.timestamp().to_string(),
        Err(e) => format!("Failed to parse date: {}", e),
    }
}

fn date_to_utc(text: &str) -> String {
    match chrono::DateTime::parse_from_rfc3339(text) {
        Ok(dt) => dt.with_timezone(&chrono::Utc).to_rfc3339(),
        Err(e) => format!("Failed to parse date: {}", e),
    }
}

fn binary_to_decimal(text: &str) -> String {
    return i32::from_str_radix(text, 2).expect("Not a binary number!").to_string();
}

fn decimal_to_binary(text: &str) -> String {
    return format!("{:b}", text.parse::<i32>().expect("Invalid number passed (decimal_to_binary)."));
}

fn json_to_query_string(text: &str) -> String {
    let obj: serde_json::Map<String, serde_json::Value> = match serde_json::from_str(text) {
        Ok(val) => val,
        Err(e) => return format!("Invalid JSON: {}", e),
    };
    match serde_urlencoded::to_string(obj) {
        Ok(s) => s,
        Err(e) => format!("Error encoding query string: {}", e),
    }
}

fn query_string_to_json(text: &str) -> String {
    match serde_urlencoded::from_str::<serde_json::Value>(text) {
        Ok(val) => serde_json::to_string_pretty(&val).unwrap(),
        Err(e) => format!("Failed to parse query string: {}", e),
    }
}

fn decimal_to_hex(text: &str) -> String {
    return format!("{:x}", text.parse::<i32>().expect("Invalid number passed (decimal_to_hex)."));
}

fn hex_to_decimal(text: &str) -> String {
    if let Ok(result) = u32::from_str_radix(text, 16) {
        return result.to_string()
    }
    panic!("Unable to convert hex to decimal.");
}


fn json_to_csv(text: &str) -> String {
    let records: Value = match serde_json::from_str(text) {
        Ok(val) => val,
        Err(e) => return format!("Invalid JSON: {}", e),
    };
    let array = match records.as_array() {
        Some(arr) => arr,
        None => return "JSON must be an array of objects".to_string(),
    };
    let mut keys = BTreeSet::new();
    for record in array {
        if let Some(obj) = record.as_object() {
            for key in obj.keys() {
                keys.insert(key.clone());
            }
        } 
        else {
            return "Each item in the JSON array must be an object.".to_string();
        }
    }
    let mut wtr = csv::Writer::from_writer(vec![]);
    // Write header
    let header: Vec<&str> = keys.iter().map(String::as_str).collect();
    if let Err(e) = wtr.write_record(&header) {
        return format!("Failed to write header: {}", e);
    }
    // Write rows
    for record in array {
        let obj = record.as_object().unwrap(); 
        let row: Vec<String> = keys.iter()
            .map(|k| obj.get(k).map_or("".to_string(), |v| v.to_string()))
            .collect();
        if let Err(e) = wtr.write_record(row) {
            return format!("Failed to write row: {}", e);
        }
    }
    if let Ok(s) = String::from_utf8(wtr.into_inner().unwrap()) {
        return s;
    }
    panic!("Unable to convert JSON to CSV");
}

fn csv_to_json(text: &str) -> String {
    let mut rdr = csv::Reader::from_reader(text.as_bytes());
    let mut records = vec![];
    for result in rdr.deserialize::<HashMap<String, String>>() {
        match result {
            Ok(map) => records.push(Value::Object(
                map.into_iter().map(|(k, v)| (k, Value::String(v))).collect()
            )),
            Err(e) => return format!("Failed to parse CSV row: {}", e),
        }
    }
    return match serde_json::to_string_pretty(&Value::Array(records)) {
        Ok(json) => json,
        Err(e) => format!("Failed to serialize JSON: {}", e),
    };
}

fn hex_to_rgb(text: &str) -> String {
    let clean = text.trim_start_matches(|c| c == '#' || c == '0' && c == 'x');
    if clean.len() < 6 {
        panic!("Hex string must be at least 6 characters for RGB.");
    }
    let bytes = hex::decode(&clean[..6]).expect("Invalid hex for RGB conversion.");
    if bytes.len() < 3 {
        panic!("Decoded hex does not contain enough bytes for RGB.");
    }
    return format!("({}, {}, {})", bytes[0], bytes[1], bytes[2]);
}

fn fish_path_hex_conv(text: &str) -> String {
    return {
        text
            .bytes()
            .map(|b| format!("\\x{:02x}", b))
            .collect::<String>()
    };
}

