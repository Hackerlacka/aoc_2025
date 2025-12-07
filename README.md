# aoc_2025
## Setup
### Rust workspace
Create the top level Cargo.toml file manually by following this guide [link](https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html).

More about resolver versions can be found in [link](https://doc.rust-lang.org/cargo/reference/resolver.html#resolver-versions).
#### Add a new day/package
`cargo new <name>`
#### Add utils library
`cargo new <name> --lib`

#### Add a crate to a package
`cargo add -p <package> <crate>`

#### Add utils library as dependency
`cargo add -p <package> --path utils`

#### Add logging crate as dependency
`cargo add -p <package> log`

`cargo add -p <package> env_logger`

#### All
`cargo add -p dayX log env_logger anyhow`
`cargo add -p dayX --path utils`

## AOC
### Input files
Input files are put under the input folder, but never added to git.

The file names are named as follows:
- <day_number>_<task_number>_example_input.txt
- <day_number>_<task_number>_input.txt
- <day_number>_<task_number>_custom_input.txt (For made up files)

If there are multiple of these files, suffixes like "_\<number\>" are added

## Tips & Tricks
### VSCode
#### Preview markdown files
`Ctrl + Shift + V`
### Markdown
#### Cheat sheet
[link](https://www.markdownguide.org/cheat-sheet/)