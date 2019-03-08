Change log
==========

[0.3.1] - 2019-03-08
--------------------

### Fixed

- File names were quoted due to the stringification of serde_json Values
- Empty string now considered to be 0 when converted to a number

[0.3.0] - 2019-03-08
--------------------

### Changed

- `--numeric-arrays` is now just `--arrays` and `-n` becomes `-a`
- `--dimensional-separator` changed its short notation from `-d` to `-D`

### Fixed

- Bug where items that only contain empty items were not cleared away correctly, even when their
  children were.

### Added

- `--boolean`, `-b` to identify columns that contain boolean values
- `--numeric`, `-n` to identify columns that contain numeric values
- `--delimiter`, `-d` to identify specify column delimiters other than `,`

### Known Issues

- Currently we can't support escaped characters for delimiter values, such as `-d \t`

[0.2.0] - 2019-03-06
--------------------

### Added

- `--numeric-arrays` - If an object only contains numeric keys, assume it should be an array
- `--remove-empty-strings` - Remove empty strings from arrays and objects (removes key from object)
- `--remove-empty-objects` - Remove empty objects from arrays and objects (removes key from object)
- `--out-dir <DIR>` - Write to a file in DIR instead of to stdout (uses same name as input file)
- `--out-name <TEMPLATE>` - Break the csv into multiple files using a TEMPLATE for the name

[0.1.1] - 2018-03-28
--------------------

### Changed

- Better documentation in crates.io

[0.1.0] - 2018-03-28
--------------------

Initial Release

### Added

- Basic functionality
