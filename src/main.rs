use clap::{App, Arg};
use regex::Regex;
use std::fs::{read_to_string, write};

#[cfg(windows)]
const LINE_ENDING: &'static str = "\r\n";
#[cfg(not(windows))]
const LINE_ENDING: &'static str = "\n";

fn main() -> std::io::Result<()> {
    let app = App::new("whitespace")
        .version("0.1.0")
        .author("Payson Wallach <payson@paysonwallach.com>")
        .about("Trim trailing whitespace")
        .arg(Arg::with_name("paths").multiple(true).required(true))
        .get_matches();
    let paths = app.values_of("paths").unwrap().collect::<Vec<_>>();
    let re = Regex::new(r"\s+$").unwrap();

    for path in paths {
        write(
            path,
            read_to_string(path).unwrap().lines().fold(String::new(), |mut text, line| {
                let mut new_line: String = re.replace(&line, "").into();
                if new_line.len() > 0 {
                    new_line.push_str(LINE_ENDING);
                    text.push_str(&new_line);
                }
                text
            })
        ).unwrap();
    }

    Ok(())
}
