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
        "Tinky"
    ]
  }
]
```
