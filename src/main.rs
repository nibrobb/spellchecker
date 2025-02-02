extern crate edit_distance;
use edit_distance::edit_distance;
use std::{
    fs::File,
    io::{self, BufRead, BufReader, Write},
    path::Path,
    process::ExitCode, env,
};

enum QReturns {
    ERROR,
    OK,
    QUIT,
    CLEAR,
}

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("No such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn prompt() {
    print!("> ");
    io::stdout().flush().unwrap();
}

fn read_query(input: &mut String) -> QReturns {
    input.clear();
    match io::stdin().read_line(input) {
        Ok(_n) => {
            let trimmed = input.trim();

            match trimmed.to_lowercase().as_str() {
                ":q" => return QReturns::QUIT,
                ":quit" => return QReturns::QUIT,
                ":c" => return QReturns::CLEAR,
                ":clear" => return QReturns::CLEAR,
                &_ => QReturns::OK,
            }
        }
        Err(error) => {
            eprintln!("ERROR: {error}");
            return QReturns::ERROR;
        }
    }
}

fn spellcheck(input: &String, wordlist: &Vec<&str>) {
    fn is_alpha_or_apostrophe(c: char) -> bool {
        c.is_alphabetic() || c == '\''
    }

    let lowercased_input = input.to_lowercase();

    let words: Vec<&str> = lowercased_input
        .split(',')
        .flat_map(|s| s.split_whitespace())
        .map(|s| s.trim_matches(|c| !is_alpha_or_apostrophe(c)))
        .filter(|s| !s.is_empty())
        .collect();

    let result = |word: &str| {
        let (min_dist, min_word) = wordlist
            .iter()
            .map(|&other_word| (edit_distance(word, other_word), other_word))
            .min_by_key(|&(distance, _)| distance)
            .unwrap();
        (min_dist, min_word)
    };

    for word in words {
        match wordlist.binary_search(&word) {
            Ok(_) => {},
            Err(_) => {
                let (dist, sug_word) = result(word);
                if dist > 0 {
                    println!("Suggestion: change \"{word}\" to \"{sug_word}\", distance: {dist}");
                }
            }
        }
    }
}

fn main() -> ExitCode {
    let args: Vec<_> = env::args().collect();

    let mut filename = "./data/words_alpha.txt";

    if args.len() == 2 {
        filename = &args[1];
    }

    let lines = lines_from_file(filename);

    let mut input = String::new();

    // TODO: Add option to turn loop off/on for either single pass taking stdin or acting as REPL
    loop {
        prompt();
        match read_query(&mut input) {
            QReturns::ERROR => return ExitCode::FAILURE,
            QReturns::OK => spellcheck(&input, &lines.iter().map(|s| s.as_str()).collect()),
            QReturns::QUIT => break, // if user typed ":q", then quit
            QReturns::CLEAR => print!("{}[2J", 27 as char), // Clear the terminal
        }
    }

    return ExitCode::SUCCESS;
}
