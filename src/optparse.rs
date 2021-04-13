// Simple parser for std::env by rootlou
// Copyright (C) 2021  rootlou
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.
// 
// Contact: rootl0u@protonmail.com

use std::io;
use std::collections::HashMap;

/**
 * parse arguments from std::env
 */
#[macro_export]
macro_rules! parse {
    ($args: expr, $arglen: expr, $parser: expr) => {
        let args: Vec<String> = $args;
        let arglen: u8 = $arglen;
        $parser.parse(args, arglen).unwrap();
    }
}

/**
 * simplified registration of flags and functions
*/
#[macro_export]
macro_rules! register {
    ($flag: expr, $description: expr, $func: expr, $parser: expr) => {
        let function = $func;
        let flag: &str = $flag;
        let desc: &str = $description;
        $parser.register(flag, desc, function);
    }
}

/**
 * holds a description of each action and the corresponding function
 */
pub struct Func<'a> {
    pub func_closure: Box<dyn Fn(String) + 'a>,
    pub func_description: String
}

impl <'a>Func<'a> {
    /**
     * Creates a new instance of Func holding a boxed function and a short description
    */
    pub fn new(func: Box<dyn Fn(String) + 'a>, desc: String) -> Func<'a> {
        Func {
            func_closure: func,
            func_description: desc
        }
    }
}

/**
 * Handle saving and parsing flags and options
 */
pub struct Parser<'a> {
    pub argmap: HashMap<String, Func<'a>>,
    pub description: String
}

impl <'a>Parser<'a> {

    /**
     *  crate a new instance of Parser 
    */
    pub fn new(desc: &str) -> Parser<'a> {
        Parser {
            argmap: HashMap::new(),
            description: String::from(desc)
        }
    }

    /**
     *  register a new flag with a description and a function
    */
    pub fn register<F: Fn(String) + 'a>(&mut self, flag: &str, desc: &str, func: F) {
        self.argmap.insert(String::from(flag), Func::new(Box::new(func), String::from(desc)));
    }

    /**
     * Execute a given function
    */
    fn execute<F>(&self, arg: String, f: F) where F: Fn(String) {
        f(arg);
    }

    /**
     * Parses the input vector, resolves all flags and executes the according functions
    */
    pub fn parse(&self, args: Vec<String>, arglen: u8) -> Result<(), io::Error>{

        if (arglen-1)%2 != 0 {                                                              // check if we can split the arguments up in pairs
            eprintln!("[ERROR] Invalid argument length: {}\n[ERROR] Exiting...", arglen-1);
            std::process::exit(0x0100);                                                     // return Err(io::Error::new(io::ErrorKind::Other, "Invalid argcount"))
        }
        for x in (1..arglen).step_by(2) {
            let flag: String;
            flag = args[x as usize].to_owned();
            let arg: String;
            arg = args[(x+1) as usize].to_owned();
            let function = &self.resolve_func(flag.clone()).unwrap().func_closure;

            self.execute(arg, function);        // resolve the flag to a "Func" struct and access the "func_closure" field inside of it
        }
        Ok(())
    }

    /**
     * resolve flag to function
    */
    fn resolve_func(&self, flag: String) -> Result<&Func<'a>, ()>{
        match self.argmap.get(&flag) {
            Some(func) => Ok(func), 
            None => {
                eprintln!("[ERROR] Unknown argument flag: {}\n[ERROR] Exiting...\n", flag);
                std::process::exit(0x0100);
            }
        }
    }    
}

#[cfg(test)]
mod test {
    use super::*;

    fn test(_arg: String) {
        println!("test")
    }

    #[test]
    fn test_parsing() {
        let mut parser: Parser = Parser::new("test");
        register!("-test", "test", test, parser);
        
        let args: Vec<String> = vec![String::from("binary"), String::from("-test"), String::from("test")];
        parse!(args.clone(), args.len() as u8, parser);
    }
}