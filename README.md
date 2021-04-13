# Docs

optparse
===
![downloads](https://img.shields.io/github/downloads/atom/atom/total.svg)
![build](https://img.shields.io/appveyor/build/Rootlou/optparse)

## License
This project is licensed under [GPLv3](https://www.gnu.org/licenses/gpl-3.0.html).

Installation
---
Add optparse to your Project by downloading it from [crates.io](https://crates.io/crates/optparse) or simply adding it to your dependencies:
```toml
[dependencies]
optparse = ["0.1.0"]
```

Documentation
---
The documentation can be found [here]().

Usage
---
```rust
// import the crate
use optparse;

// functions/closures must take String as their only parameter
// and not return anything
fn hello(arg: String) {
    println!("Hello {} \\o/", arg);
}
fn main() {
    // create a new parser with a parser description
    let mut parser = optparse::Parser::new("Example Description");

    // register a flag and a corresponding function to be called
    // from the command line
    // register!(flag, command description, function/closure, parser);
    optparse::register!("-welcome", "Example Description", hello, parser);

    // run the parser using the arguments from std::env
    // parse!(arguments, arguments length, parser)
    let args: Vec<String> = std::env::args().collect();
    optparse::parse!(args.clone(), args.len() as u8, parser);
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