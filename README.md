[![GitHub release](https://img.shields.io/github/release/apolitical/csv2json.svg)](https://github.com/apolitical/csv2json/releases)
[![GitHub license](https://img.shields.io/github/license/apolitical/csv2json.svg)](https://github.com/apolitical/csv2json/blob/master/LICENSE)
[![Crates.io](https://img.shields.io/crates/v/csv2json.svg)](https://crates.io/crates/csv2json)

csv2json
========

Turns a CSV into a JSON file

Installation:
-------------

```
$ cargo install csv2json
```

Usage:
------

```
$ csv2json --in <csv file> > <json file>
```

### CSV Delimiter

By default, the csv is split by commas. If your csv is delimited in a different way, you can
specify the character using the `--delimiter` or `-d` option

Eg:
```csv
colon:delimited
one:two
```
Without specifying:
```json
[
  {
    "colon:delimited": "one:two"
  }
]
```
Using `-d :`
```json
[
  {
    "colon": "one",
    "delimited": "two"
  }
]
```


### Dimensional Seperator

If your CSV contains multidimensional data, you can add use the dimensional separator argument, capital `-D`

Eg:
```csv
name.first,name.last,age
Daniel,Mason,not telling
```

Without using the separator:
```json
[
  {
    "age": "not telling",
    "name.first": "Daniel",
    "name.last": "Mason"
  }
]
```

Setting the separator `-D .`:
```json
[
  {
    "name": {
      "first": "Daniel",
      "last": "Mason"
    },
    "age": "not telling"
  }
]
```

### Arrays

You can use `--arrays` (or `-a`) with `-D` to break items into arrays

```csv
name,pets.1,pets.2
Daniel Mason,Yuki,Tinky
```

Without using arrays:
```json
[
  {
    "name": "Daniel Mason",
    "pets.1": "Yuki",
    "pets.2": "Tinky"
  }
]
```

With arrays (`-D . -a`):
```json
[
  {
    "name": "Daniel Mason",
    "pets": [
        "Yuki",
        "Tinky"
    ]
  }
]
```

**Note:** The number of the key is irrelevant, it only need be a number for example:

```csv
name,pets.45,pets.22
Daniel Mason,,Tinky
```

Will produce:

```json
[
  {
    "name": "Daniel Mason",
    "pets": [
        "",
        "Tinky"
    ]
  }
]
```

### Remove Empty Strings

You can remove empty strings from objects and arrays with the `--remove-empty-strings` flag.

**Note:** this happens for both objects and arrays, which may have undesirable affects.

```csv
name.first,name.last,age,pets.1,pets.2
daniel,,34,,
```

```shell
$ csv2json --in test.csv -d . -a --remove-empty-strings
```

```json
[
  {
    "age": "34",
    "name": {
      "first": "daniel"
    },
    "pets": []
  }
]
```

### Remove Empty Objects

You can remove empty objects from objects and arrays with the `--remove-empty-objects` flag.

**Note:** this happens for both objects and arrays, which may have undesirable affects.

```csv
name.first,name.last,pets.1.name,pets.1.type,pets.2.name,pets.2.type
james,smith,,,,
daniel,mason,yuki,cat,tinky,cat
```

```shell
$ csv2json --in test.csv -d . -a --remove-empty-strings --remove-empty-objects
```

```json
[
  {
    "name": {
      "first": "james",
      "last": "smith"
    },
    "pets": []
  },
  {
    "name": {
      "first": "daniel",
      "last": "mason"
    },
    "pets": [
      {
        "name": "yuki",
        "type": "cat"
      },
      {
        "name": "tinky",
        "type": "cat"
      }
    ]
  }
]
```

### Fold Output

By defaut, csv2json generates an array of json objects with one object for each row in the input file.  For example:

```
foo,bar
1,a
2,b
3,c
```
```
[{"foo": "1", "bar": "a"}, {"foo": "2", "bar": "b"}, {"foo": "3", "bar": "c"}]
```

By specifying the `--fold` or `-F` option, csv2json will fold the array of objects into an object of arrays:

```
{"foo": ["1", "2", "3"], "bar": ["a", "b", "c"]}
```

### JSONL output

Using the `--jsonl` flag will write out newline-delimited JSON.
So-called [JSONL or JSON Lines](https://jsonlines.org/). This flag is
ignored when outputting to files based on names.

### Output to directory

Using the `--out-dir <dir>` to write the `.json` file to the output dir. It will use the name of the
original file so `--in /some/dir/my-data.csv --out-dir /some/other/dir` will produce the file
`/some/other/dir/my-data.json`.

### Output to files based on names

Using the `--out-name <template>` with `--out-dir <dir>` to write multiple files of json using the
template to generate their name from the original data. For example

Given `test.csv`
```csv
name.first,name.last,pets.1.name,pets.1.type,pets.2.name,pets.2.type
james,smith,suki,cat,,
daniel,mason,yuki,cat,tinky,cat
```

Running csv2json with the following naming template
```shell
$ csv2json --in test.csv --out-dir . --out-name "{name.first}-{name.last}" -d . -a --remove-empty-strings --remove-empty-objects
```

Will produce the following files

`james-smith.json`
```json
{
  "name": {
    "first": "james",
    "last": "smith"
  },
  "pets": [
    {
      "name": "suki",
      "type": "cat"
    }
  ]
}
```
`daniel-mason.json`
```json
{
  "name": {
    "first": "daniel",
    "last": "mason"
  },
  "pets": [
    {
      "name": "yuki",
      "type": "cat"
    },
    {
      "name": "tinky",
      "type": "cat"
    }
  ]
}
```

### Types

#### Booleans

You can specify a column contains a boolean value by using the `--boolean` option

```csv
type,option.a,option.b,option.c,option.d
true,1,true,anything,TRUE
false,0,false,,FALSE
```

```shell
$ csv2json --in test.csv -d . --boolean option.a --boolean option.b --boolean option.c --boolean option.d
[
  {
    "option": {
      "a": true,
      "b": true,
      "c": true,
      "d": true
    },
    "type": "true"
  },
  {
    "option": {
      "a": false,
      "b": false,
      "c": false,
      "d": false
    },
    "type": "false"
  }
]
```

#### Numerics

You can specify a column contains a numeric value by using the `--numeric` option

```csv
number
0
1
-1
1.0
```

```shell
$ csv2json --in test.csv --numeric number
[
  {
    "number": 0
  },
  {
    "number": 1
  },
  {
    "number": -1
  },
  {
    "number": 1.0
  }
]
```
