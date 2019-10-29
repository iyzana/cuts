use regex::Regex;
use std::io::StdinLock;
use std::io::{self, BufRead};
use std::ops::Range;
use std::io::prelude::*;

pub struct Config {
    pub selections: Vec<Selection>,
    pub delimiter: Regex,
    pub out_delimiter: String,
    pub trimmed: bool,
    pub only_delimited: bool,
    pub selection_type: SelectionType,
}

#[derive(Debug)]
pub enum Selection {
    Single(isize),
    Range(Option<isize>, Option<isize>),
}

pub enum SelectionType {
    Fields,
    Bytes,
    Characters,
}

pub fn cuts(config: &Config) {
    let stdin = io::stdin();
    let items = match config.selection_type {
        SelectionType::Fields => fields(config, stdin.lock()),
        SelectionType::Bytes => bytes(config, stdin.lock()),
        SelectionType::Characters => chars(config, stdin.lock()),
    };
    items.for_each(|elements| {
        let output = config
            .selections
            .iter()
            .map(|selection| to_concrete_range(selection, elements.len()))
            .flat_map(|range| elements[range].to_vec())
            .collect::<Vec<_>>();

        let stdout = std::io::stdout();
        let mut a = stdout.lock();
        for line in output {
            a.write_all(&line).unwrap();
            a.write_all(config.out_delimiter.as_bytes()).unwrap();
        }
        a.write_all("\n".as_bytes()).unwrap();
    });
}

pub fn lines<'a>(config: &'a Config, stdin: StdinLock<'a>) -> impl Iterator<Item = String> + 'a {
    stdin.lines().filter_map(Result::ok).map(move |line| {
        if config.trimmed {
            line.trim().to_string()
        } else {
            line
        }
    })
}

pub fn fields<'a>(
    config: &'a Config,
    stdin: StdinLock<'a>,
) -> Box<dyn Iterator<Item = Vec<Vec<u8>>> + 'a> {
    Box::new(lines(config, stdin).flat_map(move |line| {
        let fields = config
            .delimiter
            .split(&line)
            .map(ToString::to_string)
            .map(String::into_bytes)
            .collect::<Vec<_>>();

        if fields.len() == 1 && config.only_delimited {
            None
        } else {
            Some(fields)
        }
    }))
}

pub fn chars<'a>(
    config: &'a Config,
    stdin: StdinLock<'a>,
) -> Box<dyn Iterator<Item = Vec<Vec<u8>>> + 'a> {
    Box::new(lines(config, stdin).map(|line| {
        line.chars()
            .map(|c| {
                let mut buffer = [0; 8];
                c.encode_utf8(&mut buffer).to_string().into_bytes()
            })
            .collect()
    }))
}

pub fn bytes<'a>(
    _config: &'a Config,
    stdin: StdinLock<'a>,
) -> Box<dyn Iterator<Item = Vec<Vec<u8>>> + 'a> {
    Box::new(vec![stdin.bytes().filter_map(Result::ok).map(|b| vec![b]).collect()].into_iter())
}

fn to_concrete_range(selection: &Selection, num_fields: usize) -> Range<usize> {
    #[allow(clippy::range_plus_one)]
    match selection {
        Selection::Single(column) => {
            to_concrete_index(*column, num_fields).map_or_else(|| 0..0, |index| index..index + 1)
        }
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
