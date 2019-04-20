use clap::{App, Arg};
use regex::Regex;
use std::io::Error;
use std::io::{self, BufRead};
use std::ops::Range;

fn main() -> Result<(), Error> {
    let matches = App::new("cuts")
        .version("0.1.0")
        .author("succcubbus <jannis.kaiser2@gmail.com>")
        .about("A unix cut clone with improved field selection")
        .arg(
            Arg::with_name("SELECTION")
                .help("Specifies the selection/ranges to print")
                .required(true)
                .index(1),
        )
        .get_matches();

    let selections = matches.value_of("SELECTION").unwrap();
    let selections = parse_selection(selections)?;
    println!("{:?}", selections);

    let separator = Regex::new(r"\s+").unwrap();

    let stdin = io::stdin();
    let mut handle = stdin.lock();

    loop {
        let mut input = String::new();
        handle.read_line(&mut input)?;

        let fields = &separator.split(&input).collect::<Vec<_>>();

        let output = selections
            .iter()
            .flat_map(|selection| match selection {
                Selection::Single(column) => vec![fields[to_positive(*column, fields.len())]],
                Selection::Range(range) => {
                    let start = to_positive(range.start, fields.len());
                    let end = to_positive(range.end, fields.len());
                    let true_range = Range { start, end };
                    fields[true_range].to_vec()
                }
            })
            .collect::<Vec<&str>>()
            .join(" ");

        println!("{}", output);
    }
}

fn to_positive(selection: isize, num_fields: usize) -> usize {
    if selection >= 0 {
        selection as usize
    } else {
        num_fields + selection as usize
    }
}

#[derive(Debug)]
enum Selection {
    Single(isize),
    Range(Range<isize>),
}

fn parse_selection(selection: &str) -> Result<Vec<Selection>, Error> {
    selection
        .split(',')
        .map(|spec| {
            let indicies: Vec<isize> = spec.split("..").map(|s| s.parse()).collect()?;

            Ok(match indicies.len() {
                1 => Selection::Single(indicies[0]),
                2 => Selection::Range(indicies[0]..indicies[2]),
            })
        })
        .collect()
}
