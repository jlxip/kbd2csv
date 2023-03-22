use pest::Parser;
use pest_derive::Parser;
use std::collections::BTreeMap;
use std::env;
use std::fs;
use std::path::Path;

type Keymap = BTreeMap<usize, [String; 5]>;

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct KbdParser;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("Usage: {} <path/to/keymap.map> <output.csv>", args[0]);
        return;
    }

    // Does path exist?
    let path: &str = &args[1];
    if !Path::new(path).exists() {
        println!("File not found: {}", path);
        return;
    }

    let mut keymap: Keymap = BTreeMap::new();
    scan(path, &mut keymap);

    // Finished!
    let mut out = String::new();
    for scancode in keymap.keys() {
        let joined = keymap[scancode].join(",");
        out += &format!("{},{}\n", scancode, joined);
    }

    fs::write(&args[2], out).unwrap();
}

fn scan(path: &str, keymap: &mut Keymap) {
    let data = fs::read(path).unwrap(); // bytes
    let data = String::from_utf8_lossy(&data); // str

    let parse = KbdParser::parse(Rule::file, &data).unwrap().next().unwrap();
    for pair in parse.into_inner() {
        match pair.as_rule() {
            Rule::include => {
                let orig_path = path;

                let filename = pair.into_inner().next().unwrap(); // quotepath
                let filename = filename.into_inner().next().unwrap(); // path
                let filename = filename.as_span().as_str();
                println!("Including: {}", filename);

                let target = search(orig_path, filename);
                scan(&target, keymap);
            }
            Rule::keycode => {
                let mut rule = pair.into_inner();
                let num = rule.next().unwrap().as_span().as_str();
                let num = num.parse::<usize>().unwrap();

                let mut codes: [String; 5] = Default::default();
                for (idx, i) in rule.enumerate() {
                    let code = i.as_span().as_str();
                    if code == "nul" {
                        continue;
                    }

                    codes[idx] = code.to_string();
                }
                keymap.insert(num, codes);
            }
            _ => unreachable!(),
        };
    }
}

fn search(orig_path: &str, filename: &str) -> String {
    // Given orig_path (e.g. "kbd/data/keymaps/i386/qwerty/us.map")
    // and filename (e.g. "compose.latin1")
    // Find the actual path for the filename
    // I do this by trying upwards in the directory hierarchy:
    // -> kbd/data/keymaps/i386/qwerty/include/compose.latin1 (wrong)
    // -> kbd/data/keymaps/i386/include/compose.latin1 (wrong)
    // -> kbd/data/keymaps/include/compose.latin1 (correct!)

    let mut buf = Path::new(orig_path).to_path_buf();
    buf.pop(); // Do not consider ".../qwerty/us.map/include/compose.latin1"
    loop {
        if buf.as_os_str().len() == 0 {
            panic!("Could not find {}", filename);
        }

        buf.push("include");
        for cand in search_candidates(filename) {
            buf.push(cand);
            if buf.as_path().is_file() {
                // Got it!
                return buf.as_os_str().to_string_lossy().to_string();
            }
            buf.pop();
        }
        buf.pop(); // include
        buf.pop(); // wrong directory
    }
}

fn search_candidates(filename: &str) -> Vec<String> {
    let mut ret: Vec<String> = vec![];
    ret.push(format!("{}.inc", filename));
    ret.push(filename.to_string());
    ret
}
