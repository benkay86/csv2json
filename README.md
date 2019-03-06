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

### Dimensional Seperator

If your CSV contains multidimensional data, you can add use the dimensional separator argument `-d`

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

Setting the separator `-d .`:
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

### Numeric Arrays

You can use `--numeric-arrays` (or `-n`) with `-d` to break items into arrays

```csv
name,pets.1,pets.2
Daniel Mason,Yuki,Tinky
```

Without using numeric keys:
```json
[
  {
    "name": "Daniel Mason",
    "pets.1": "Yuki",
    "pets.2": "Tinky"
  }
]
```

With numeric keys (`-d . -n`):
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
$ csv2json --in test.csv -d . -n --remove-empty-strings
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
$ csv2json --in test.csv -d . -n --remove-empty-strings --remove-empty-objects
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
$ csv2json --in test.csv --out-dir . --out-name "{name.first}-{name.last}" -d . -n --remove-empty-strings --remove-empty-objects
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
