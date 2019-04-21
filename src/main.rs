use clap::{App, Arg};
use regex::Regex;
use std::borrow::ToOwned;
use std::io::{self, BufRead};
use std::ops::Range;
use std::result::Result;

#[derive(Debug)]
enum CutsError {
    NonIntegerSelection(String),
    MalformedSelection(String),
    InvalidDelimiter(String),
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
        .arg(
            Arg::with_name("string_delimiter")
                .short("d")
                .takes_value(true)
                .help("use this delimiter instead of whitespace")
                .group("delimiter"),
        )
        .arg(
            Arg::with_name("regex_delimiter")
                .short("D")
                .takes_value(true)
                .help("use this regex as the delimiter instead of whitespace")
                .group("delimiter"),
        )
        .get_matches();

    let selections = matches.value_of("SELECTION").unwrap();
    let selections = parse_selection(selections)?;
    println!("parsed selections: {:?}", selections);

    let separator = matches
        .value_of("regex_delimiter")
        .map(ToOwned::to_owned)
        .or_else(|| matches.value_of("string_delimiter").map(regex::escape))
        .unwrap_or_else(|| r"\s+".to_owned());
    let split_regex = Regex::new(&separator).map_err(|_| CutsError::InvalidDelimiter(separator))?;

    io::stdin()
        .lock()
        .lines()
        .filter_map(Result::ok) // filter out lines with invalud utf-8
        .for_each(|line| {
            let fields = &split_regex.split(&line).collect::<Vec<_>>();

            let output = selections
                .iter()
                .map(|selection| to_concrete_range(selection, fields.len()))
                .flat_map(|range| fields[range].to_vec())
                .collect::<Vec<&str>>()
                .join(" ");

            println!("{}", output);
        });

    Ok(())
}

fn to_concrete_range(selection: &Selection, num_fields: usize) -> Range<usize> {
    #[allow(clippy::range_plus_one)]
    match selection {
        Selection::Single(column) => to_concrete_index(*column, num_fields)
            .map_or_else(|| 0..0, |index| index..index + 1),
        Selection::Range(start, end) => {
            let start = start
                .and_then(|index| to_concrete_index(index, num_fields))
                .unwrap_or(0);
            let end = end
                .and_then(|index| to_concrete_index(index, num_fields))
                .unwrap_or_else(|| num_fields);
            start..end
        }
    }
}

fn to_concrete_index(selection: isize, num_fields: usize) -> Option<usize> {
    let num_fields = num_fields as isize;
    if selection >= 0 && selection < num_fields {
        Some(selection as usize)
    } else if selection < 0 && num_fields + selection >= 0 {
        Some((num_fields + selection) as usize)
    } else {
        None
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
