use atty;
use getopts::{Fail, Options};

use std::env;
use std::io::{self, BufRead};
use std::process::exit;

pub fn print_help(opts: &Options) {
    print!(
        "{}",
        opts.usage("USAGE: rpt [-v] [-h] [-n] [-e] [-s <sep>] repetitions [string]")
    );
}

pub fn print_usage() {
    eprintln!("USAGE: rpt [-v] [-h] [-n] [-e] [-s <sep>] repetitions [string]\n");
    eprintln!("run 'rpt -h' for more information\n");
}

pub fn print_version() {
    println!("VERSION: {}", env!("CARGO_PKG_VERSION"));
}

pub fn reverse_string(string: &mut String) {
    *string = string.chars().rev().collect::<String>();
}

pub fn repl_esc_seq(r_str: &mut String) {
    let chars = r_str.clone();
    let mut found_slash = false;
    for (index, c) in chars.char_indices() {
        if found_slash {
            let _char_found = match c {
                'n' => Some('\n'),
                't' => Some('\t'),
                '\\' => Some('\\'),
                _ => None,
            };
            if let Some(e_c) = _char_found {
                r_str.replace_range(index - 1..index + 1, &e_c.to_string());
            }
            found_slash = false;
        } else if c == '\\' {
            found_slash = true;
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut opts = Options::new();
    opts.optopt("s", "", "optional separator string", "SEP");
    opts.optflag("n", "", "do not output the trailing newline\n");
    opts.optflag("v", "", "print version info");
    opts.optflag(
        "e",
        "",
        "interpret some few escape sequences (\\\\,\\t,\\n)",
    );
    opts.optflag("h", "help", "print this help menu");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => match f {
            Fail::ArgumentMissing(_) => {
                eprintln!("rpt error: option -s (separator) needs a string value\n");
                exit(1);
            }
            Fail::UnrecognizedOption(opt) => {
                eprintln!("rpt error: unknown option => '{}'", opt);
                exit(1);
            }
            _ => {
                println!("{}", f.to_string());
                exit(1);
            }
        },
    };

    if matches.opt_present("h") {
        print_help(&opts);
        return;
    }

    if matches.opt_present("v") {
        print_version();
        return;
    }

    let mut input = String::new();

    if atty::is(atty::Stream::Stdin) {
        if matches.free.len() < 2 {
            eprintln!("rpt error: too few arguments\n");
            print_usage();
            return;
        }
        input = matches.free[1..].join(" ");
    } else {
        if matches.free.len() < 1 {
            eprintln!("rpt error: too few arguments\n");
            print_usage();
            return;
        }
        let stdin = io::stdin();
        let mut stdin = stdin.lock();

        match stdin.read_line(&mut input) {
            Ok(len) => {
                if len == 0 {
                    return;
                } else {
                    input = input
                        .strip_suffix("\n")
                        .and_then(|s| Some(s.to_string()))
                        .unwrap_or(input);
                }
            }
            Err(error) => {
                eprintln!("rpt error: {}", error);
                return;
            }
        };
    }

    let mut seperator = matches.opt_str("s");
    let mut num_of_repititions = matches.free[0].parse::<i32>().unwrap_or_else(|_| {
        eprintln!("rpt error: repitition value is not a number");
        exit(1)
    });

    if matches.opt_present("e") {
        repl_esc_seq(&mut input);
        if seperator.is_some() {
            repl_esc_seq(seperator.as_mut().unwrap());
        }
    }

    if num_of_repititions < 0 {
        reverse_string(&mut input);
        num_of_repititions = num_of_repititions.abs();
    }

    let output = match seperator {
        Some(sep) => {
            input.push_str(&sep);
            input
                .repeat(num_of_repititions as usize)
                .strip_suffix(&sep)
                .unwrap()
                .to_owned()
        }
        None => input.repeat(num_of_repititions as usize).to_string(),
    };

    print!("{}", output);

    if !matches.opt_present("n") {
        println!("");
    }
}
