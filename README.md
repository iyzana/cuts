# cuts

cuts is a gnu cut clone with improved field selection.
cuts defaults to any amount of any whitespace as separator and trims each line before processing.
Currently cuts does not support input files, only stdin is read.

## Installation

```sh
git clone https://github.com/succcubbus/cuts
cargo install --path cuts
```

By default the binary will be installed under `~/.cargo/bin/`

## Examples

example input:

```
0 1 2
 0 1 2
0,1,2
0 1 2 3 4
```

access is zero-indexed

```
$ cuts 0
0
0
0,1,2
0
```

negative indicies access fields from the end

```
$ cuts -- -1
2
2
0,1,2
4
```

you can select field ranges

```
$ cuts ..-1 # everything but the last field
0 1
0 1
0,1,2
0 1 2 3
```

not specifing a range bound is equivalent to the extreme value for that line
