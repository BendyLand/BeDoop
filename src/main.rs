use std::env;

mod utils;

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
    if utils::flags_contains(&flags, 's') {
        let idx = utils::get_flag_idx(&flags, 's');
        text = args[idx].clone();
    }
    else { text = utils::get_file_contents(&args); }
    dbg!(text);
}

/*
rot13
base64 encode
base64 decode
start case
snake case
kebab case
upper case
down case
sponge case
camel case
add slashes
remove slashes
markdown quote (add > to line starts)
defang (dangerous URLs and other IOCs)
refang (remove defanging)
replace smart quotes (with their simpler values)
shuffle lines (randomize line order)
ascii to hex (convert ascii chars to hex codes)
hex to ascii (convert hex values to ascii chars)
fish PATH hex converter (espcapes terminal characters)
yaml to json
json to yaml
date to timestamp (convert date to unix timestamp)
date to utc (convert dates and timestamps to UTC dates)
sum all (sums a list of numbers)
format json
md5 checksum (computes the checksum of the text (hex encoded))
android strings to ios localizables
ios localizables to android strings
binary to decimal
decimal to binary
deburr (convert text to basic latin characters)
count characters
json to query string (convert json to URL query string)
query string to json (convert URL query string to json)
collapse lines
remove duplicate lines
sort lines
decimal to hex
sha1 hash
sha256 hash
sha512 hash
trim
natural sort lines (smart handling of numbers)
php unserialize (convert php serialized data to json)
hex to decimal
eval javascript
reverse string
format sql
lorem ipsum
format css
format xml
minify css
html encode all characters
minify json
minify xml
minify sql
hex to rgb
url entity encode
url entities decode
json to csv
csv to json
*/

