use std::collections::HashMap;
use base64::{engine::general_purpose::STANDARD, Engine as _};
use sha1::{Sha1, Digest};
use sha2::Sha256;
use sha2::Sha512;
use percent_encoding::{utf8_percent_encode, percent_decode_str, AsciiSet, NON_ALPHANUMERIC, CONTROLS};

pub enum EncodingOp {
    Rot13, Base64Encode, Base64Decode, Md5, Sha1, Sha256, Sha512, HtmlEncode, HtmlDecode,
    HtmlEncodeAll, UrlEncode, UrlDecode, UrlEntityEncode, UrlEntitiesDecode, Unknown,
}

fn str_to_encoding_op(arg: &str) -> EncodingOp {
    return match arg {
        "rot13" => EncodingOp::Rot13,
        "base64_encode" => EncodingOp::Base64Encode,
        "base64_decode" => EncodingOp::Base64Decode,
        "md5" => EncodingOp::Md5,
        "sha1" => EncodingOp::Sha1,
        "sha256" => EncodingOp::Sha256,
        "sha512" => EncodingOp::Sha512,
        "html_encode" => EncodingOp::HtmlEncode,
        "html_decode" => EncodingOp::HtmlDecode,
        "html_encode_all" => EncodingOp::HtmlEncodeAll,
        "url_encode" => EncodingOp::UrlEncode, 
        "url_decode" => EncodingOp::UrlDecode, 
        "url_entity_encode" => EncodingOp::UrlEntityEncode, 
        "url_entities_decode" => EncodingOp::UrlEntitiesDecode, 
        _ => EncodingOp::Unknown,
    }
}

pub fn select_encoding_option(args: &Vec<String>) -> EncodingOp {
    let encoding_options: Vec<String> = vec!["rot13", "base64_encode", "base64_decode", "md5", "html_encode", "html_decode", "html_encode_all", "url_encode", "url_decode", "url_entity_encode", "url_entities_decode", "sha1", "sha256", "sha512"].into_iter().map(|x| x.to_string()).collect();
    for arg in args {
        if encoding_options.contains(&arg.to_lowercase()) {
            return str_to_encoding_op(arg);
        }
    }
    return EncodingOp::Unknown
}

pub fn handle_encoding_operation(text: &str, op: EncodingOp) -> String {
    return match op {
        EncodingOp::Rot13 => rot13(text),
        EncodingOp::Base64Encode => base64_encode(text),
        EncodingOp::Base64Decode => base64_decode(text),
        EncodingOp::Md5 => md5(text),
        EncodingOp::Sha1 => sha1(text),
        EncodingOp::Sha256 => sha256(text),
        EncodingOp::Sha512 => sha512(text),
        EncodingOp::HtmlEncode => html_encode(text),
        EncodingOp::HtmlDecode => html_decode(text),
        EncodingOp::HtmlEncodeAll => html_encode_all(text),
        EncodingOp::UrlEncode => url_encode(text),
        EncodingOp::UrlDecode => url_decode(text),
        EncodingOp::UrlEntityEncode => url_entity_encode(text),
        EncodingOp::UrlEntitiesDecode=> url_entities_decode(text),
        EncodingOp::Unknown => panic!("Unknown encoding operation specified."),
    };
}

fn rot13(text: &str) -> String {
    return {
        text.chars()
            .map(|c| match c {
                'A'..='Z' => (((c as u8 - b'A' + 13) % 26) + b'A') as char,
                'a'..='z' => (((c as u8 - b'a' + 13) % 26) + b'a') as char,
                _ => c,
            })
            .collect()
    };
}

fn base64_encode(text: &str) -> String {
    return STANDARD.encode(text);
}

fn base64_decode(text: &str) -> String {
    let res = STANDARD.decode(text).unwrap_or_default();
    if res.len() == 0 { return text.to_string(); }
    let result = String::from_utf8(res).unwrap_or(text.to_string());
    return result;
}

fn md5(text: &str) -> String {
    return format!("{:x}", md5::compute(text));
}

fn sha1(text: &str) -> String {
    let mut hasher = Sha1::new();
    hasher.update(text);
    let result = hasher.finalize();
    return format!("{:x}", result);
}

fn sha256(text: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(text);
    let result = hasher.finalize();
    return format!("{:x}", result);
}

fn sha512(text: &str) -> String {
    let mut hasher = Sha512::new();
    hasher.update(text);
    let result = hasher.finalize();
    return format!("{:x}", result);
}

fn html_encode(text: &str) -> String {
    let codes = HashMap::from([
        ('<', "&#x3C;"),
        ('>', "&#x3E;"),
        ('&', "&#x26;"),
        ('\'', "&#x22;"),
        ('"', "&#x27;"),
    ]);
    return text.chars().map(|c| {
        if let Some(code) = codes.get(&c) { code.to_string() } 
        else { c.to_string() }
    }).collect();
}

fn html_decode(text: &str) -> String {
    let codes = HashMap::from([
        ("&#x3C;", '<'),
        ("&#x3E;", '>'),
        ("&#x26;", '&'),
        ("&#x22;", '\''),
        ("&#x27;", '"'),
    ]);
    let mut result = String::new();
    let mut i = 0;
    let chars: Vec<char> = text.chars().collect();
    while i < chars.len() {
        if chars[i] == '&' {
            if let Some(semi_idx) = (i + 1..chars.len()).find(|&j| chars[j] == ';') {
                let entity: String = chars[i..=semi_idx].iter().collect();
                if let Some(decoded_char) = codes.get(entity.as_str()) {
                    result.push(*decoded_char);
                    i = semi_idx + 1;
                    continue;
                }
            }
        }
        result.push(chars[i]);
        i += 1;
    }
    return result;
}

fn html_encode_all(text: &str) -> String {
    return {
        text
            .chars()
            .map(|c| format!("&#{};", c as u32))
            .collect()
    };
}

fn url_encode(text: &str) -> String {
    utf8_percent_encode(text, NON_ALPHANUMERIC).to_string()
}


fn url_decode(text: &str) -> String {
    percent_decode_str(text).decode_utf8_lossy().to_string()
}


/// Define a custom set that matches nothing = encode everything
const ENCODE_ALL: &AsciiSet = &CONTROLS
    .add(b' ').add(b'!').add(b'"').add(b'#').add(b'$').add(b'%')
    .add(b'&').add(b'\'').add(b'(').add(b')').add(b'*').add(b'+')
    .add(b',').add(b'-').add(b'.').add(b'/').add(b'0').add(b'1')
    .add(b'2').add(b'3').add(b'4').add(b'5').add(b'6').add(b'7')
    .add(b'8').add(b'9').add(b':').add(b';').add(b'<').add(b'=')
    .add(b'>').add(b'?').add(b'@').add(b'A').add(b'B').add(b'C')
    .add(b'D').add(b'E').add(b'F').add(b'G').add(b'H').add(b'I')
    .add(b'J').add(b'K').add(b'L').add(b'M').add(b'N').add(b'O')
    .add(b'P').add(b'Q').add(b'R').add(b'S').add(b'T').add(b'U')
    .add(b'V').add(b'W').add(b'X').add(b'Y').add(b'Z').add(b'[')
    .add(b'\\').add(b']').add(b'^').add(b'_').add(b'`').add(b'a')
    .add(b'b').add(b'c').add(b'd').add(b'e').add(b'f').add(b'g')
    .add(b'h').add(b'i').add(b'j').add(b'k').add(b'l').add(b'm')
    .add(b'n').add(b'o').add(b'p').add(b'q').add(b'r').add(b's')
    .add(b't').add(b'u').add(b'v').add(b'w').add(b'x').add(b'y')
    .add(b'z').add(b'{').add(b'|').add(b'}').add(b'~');

fn url_entity_encode(text: &str) -> String {
    utf8_percent_encode(text, ENCODE_ALL).to_string()
}

fn url_entities_decode(text: &str) -> String {
    url_decode(text)  
}

