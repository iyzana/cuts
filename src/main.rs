extern crate cuts;

use crate::cuts::{cuts, Config, Selection, SelectionType};
use clap::{App, Arg};
use regex::Regex;
use std::borrow::ToOwned;
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
                .help("Specifies the selection/ranges to print, for more information try --help")
                .long_help(
                    "\
Comma separated, zero-based indicies or ranges. Any index can be negative so
that -1 selects the last element. Ranges are provided in the form start..end
where start is inclusive and end is exclusive. When a range bound is ommitted
it is assumed to be the extreme. Negative range bounds are allowed. If starting
with a negative index, be sure to add '--' (end of options) before it.

Examples:
  1,2,-2 # select the 2nd, 3rd and second from last field
  1..4,7 # select fields 1 2 3 and 7
  ..4    # select the first 4 fields
  ..-1   # select all but the last field
  -3..   # select the last three fields",
                )
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("string_delimiter")
                .short("d")
                .long("delimiter")
                .takes_value(true)
                .help("Use this delimiter instead of whitespace")
                .group("delimiter"),
        )
        .arg(
            Arg::with_name("regex_delimiter")
                .short("r")
                .long("regex")
                .takes_value(true)
                .help("Use this regex as the delimiter instead of whitespace")
                .group("delimiter"),
        )
        .arg(
            Arg::with_name("no_trim")
                .short("t")
                .long("no-trim")
                .help("Do not trim the lines before applying the delimiter"),
        )
        .arg(
            Arg::with_name("only_delimited")
                .short("o")
                .long("only-delimited")
                .help("Only print lines containing the delimiter at least once"),
        )
        .arg(
            Arg::with_name("bytes")
                .short("b")
                .long("bytes")
                .help("Slice bytes instead of fields")
                .group("selection_type")
                .conflicts_with_all(&["string_delimiter", "regex_delimiter"]),
        )
        .arg(
            Arg::with_name("characters")
                .short("c")
                .long("chars")
                .help("Slice characters instead of fields")
                .group("selection_type")
                .conflicts_with_all(&["string_delimiter", "regex_delimiter"]),
        )
        .get_matches();

    let selections = parse_selections(matches.value_of("SELECTION").unwrap())?;

    let regex = matches
        .value_of("regex_delimiter")
        .map(ToOwned::to_owned)
        .or_else(|| matches.value_of("string_delimiter").map(regex::escape))
        .unwrap_or_else(|| r"\s+".to_owned());
    let delimiter = Regex::new(&regex).map_err(|_| CutsError::InvalidDelimiter(regex))?;

    let selection_type = if matches.is_present("bytes") {
        SelectionType::Bytes
    } else if matches.is_present("characters") {
        SelectionType::Characters
    } else {
        SelectionType::Fields
    };

    let config = Config {
        selections,
        delimiter,
        trimmed: !matches.is_present("no_trim"),
        only_delimited: matches.is_present("only_delimited"),
        selection_type,
    };

    cuts(&config);

    Ok(())
}

fn parse_selections(selections: &str) -> Result<Vec<Selection>, CutsError> {
    selections.split(',').map(parse_selection).collect()
}

fn parse_selection(selection: &str) -> Result<Selection, CutsError> {
    let parts = selection.split("..").collect::<Vec<_>>();

    match parts.as_slice() {
        [index] => Ok(Selection::Single(parse_int(index)?)),
        [start, end] => Ok(Selection::Range(
            parse_range_bound(start)?,
            parse_range_bound(end)?,
        )),
        _ => Err(CutsError::MalformedSelection(selection.to_owned())),
    }
}

fn parse_int(string: &str) -> Result<isize, CutsError> {
    string
        .parse()
        .map_err(|_| CutsError::NonIntegerSelection(string.to_owned()))
}

fn parse_range_bound(string: &str) -> Result<Option<isize>, CutsError> {
    Ok(if string.is_empty() {
        None
    } else {
        Some(parse_int(string)?)
    })
}
