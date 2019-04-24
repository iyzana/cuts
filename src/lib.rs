use regex::Regex;
use std::io::{self, BufRead};
use std::ops::Range;

pub struct Config {
    pub selections: Vec<Selection>,
    pub delimiter: Regex,
    pub trimmed: bool,
    pub only_delimited: bool,
}

#[derive(Debug)]
pub enum Selection {
    Single(isize),
    Range(Option<isize>, Option<isize>),
}

pub fn cuts(config: &Config) {
    io::stdin()
        .lock()
        .lines()
        .filter_map(Result::ok) // filter out lines with invalid utf-8
        .map(|line| {
            if config.trimmed {
                line.trim().to_string()
            } else {
                line
            }
        })
        .for_each(|line| {
            let fields = &config.delimiter.split(&line).collect::<Vec<_>>();

            if fields.len() == 1 && config.only_delimited {
                return;
            }

            let output = config
                .selections
                .iter()
                .map(|selection| to_concrete_range(selection, fields.len()))
                .flat_map(|range| fields[range].to_vec())
                .collect::<Vec<&str>>()
                .join(" ");

            println!("{}", output);
        });
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
