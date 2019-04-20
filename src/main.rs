use clap::{App, Arg};
use regex::Regex;
use std::io::{self, BufRead};
use std::ops::Range;

#[derive(Debug)]
enum CutsError {
    NonIntegerSelection(String),
    MalformedSelection(String),
    InputClosedUnexpectedly,
}

fn main() -> Result<(), CutsError> {
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
    println!("parsed selections: {:?}", selections);

    let separator = Regex::new(r"\s+").unwrap();

    let stdin = io::stdin();
    let mut handle = stdin.lock();

    loop {
        let mut input = String::new();
        handle
            .read_line(&mut input)
            .map_err(|_| CutsError::InputClosedUnexpectedly)?;

        let fields = &separator.split(&input).collect::<Vec<_>>();

        let output = selections
            .iter()
            .flat_map(|selection| match selection {
                Selection::Single(column) => vec![fields[to_concrete_index(*column, fields.len())]],
                Selection::Range(start, end) => {
                    let start = start
                        .map(|index| to_concrete_index(index, fields.len()))
                        .unwrap_or(0);
                    let end = end
                        .map(|index| to_concrete_index(index, fields.len()))
                        .unwrap_or_else(|| fields.len());
                    let concrete_range = Range { start, end };
                    fields[concrete_range].to_vec()
                }
            })
            .collect::<Vec<&str>>()
            .join(" ");

        println!("{}", output);
    }
}

fn to_concrete_index(selection: isize, num_fields: usize) -> usize {
    if selection >= 0 {
        selection as usize
    } else {
        (num_fields as isize + selection).max(0) as usize
    }
}

#[derive(Debug)]
enum Selection {
    Single(isize),
    Range(Option<isize>, Option<isize>),
}

fn parse_selection(selection: &str) -> Result<Vec<Selection>, CutsError> {
    selection.split(',').map(|spec| parse_range(spec)).collect()
}

fn parse_range(selection: &str) -> Result<Selection, CutsError> {
    let parts = selection.split("..").collect::<Vec<_>>();

    match parts[..] {
        [index] => Ok(Selection::Single(parse_int(index)?)),
        [start, end] => Ok(Selection::Range(
            if start.is_empty() {
                None
            } else {
                Some(parse_int(start)?)
            },
            if end.is_empty() {
                None
            } else {
                Some(parse_int(end)?)
            },
        )),
        _ => Err(CutsError::MalformedSelection(selection.to_owned())),
    }
}

fn parse_int(string: &str) -> Result<isize, CutsError> {
    string
        .parse()
        .map_err(|_| CutsError::NonIntegerSelection(string.to_owned()))
}
