# gh-prj: An extension to browse [projects](https://docs.github.com/en/issues/organizing-your-work-with-project-boards/managing-project-boards/about-project-boards)

## Installation

```
gh extension install mh4gf/gh-prj
```

## Usage
```
USAGE:
    gh-prj <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    help    Prints this message or the help of the given subcommand(s)
    list    List Projects in this repository
    view    Display the information about a Project
```

## Development

```
cargo install --force cargo-make
cargo make build
```
