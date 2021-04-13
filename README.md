# Docs

optparse
===
![downloads](https://img.shields.io/github/downloads/atom/atom/total.svg)
![build](https://img.shields.io/appveyor/ci/rootlou/optparse)

## Table of Contents

[TOC]

## License
This project is licensed under [GPLv3](https://www.gnu.org/licenses/gpl-3.0.html).

Installation
---
To add optparse to you Project by downloading it from [crates.io](https://crates.io/crates/optparse) or simply add it to your dependencies:
```toml
[dependencies]
optparse = ["0.1.0"]
```

Documentation
---
The documentation can be found 

Usage
---
```rust
// import the crate
use optparse;

// functions/closures must take String as their only parameter
// and not return anything
fn hello(arg: String) {
    println!("Hello {} \o/", arg);
}
fn main() {
    // create a new parser with a parser description
    let mut parser = optargs::Parser::new("Example Description");

    // register a flag and a corresponding function to be called
    // from the command line
    // register!(flag, command description, function/closure, parser);
    register!("-welcome", "Example Description", hello, parser);

    // run the parser using the arguments from std::env
    // parse!(arguments, arguments length, parser)
    let args = std::env::args().collect();
    parse!(args.clone(), args.len() as u8, parser);
}
```

Work in Progress
---
- [ ] aliasing
- [ ] single flag parsing
- [ ] multi parameter closures

:::info
**You want to contribute?** Feel free to leave a comment or create a pull request.
:::