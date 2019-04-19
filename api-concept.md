# cuts api concept

// selecting
// default is reading stdin
// default delimiter is the regex \s+
cuts 0,3,1,-3 # take the 1st, 4th, 2nd fields and the 3rd from the back

// bounds
// inclusive..exclusive
// -1 is last field
cuts 1.. # take all but first field
cuts ..-1 # take all but last field
cuts ..4 # take first 4 fields
cuts -4.. # take last 4 fields
cuts 2..7 # take fields 2 through 6

// flags
cuts -d + // take a string that overrides the delimiter to use
cuts -D \d+ // take a regex that overrides the delimiter to use
cuts -b // slice bytes instead of fields
cuts -c // slice characters instead of fields
cuts -o --only-delimited // only return lines containing the delimiter
cuts -T --no-trim // do not trim preceeding and trailing whitespace

// files
cuts .. file.csv // take all fields from file.csv

cuts [-d <delimiter> | -D <regex_delimiter>] [-b | -c] [-o / --only-delimited] [-T / --no-trim] <selection> [<path>...]
