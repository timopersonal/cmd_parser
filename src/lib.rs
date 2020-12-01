//! # Command Parser
//!
//! `cmd_parser` contains a simple struct that makes the parsing of command line commands much easier.
//! # Example
//! ```
//! fn echo(args: Vec<&str>) {
//!     println!("{}", args[1..].concat());
//! }
//!
//! let mut parser = cmd_parser::CommandParser::new();
//! parser.insert("echo", echo);
//! parser.call("echo This is an example!");
//! ```

pub struct CommandParser {
    cmd_map: std::collections::HashMap<String, fn(Vec<&str>)>,
}

impl CommandParser {
    /// Creates a new instance of CommandParser.
    ///
    /// # Example
    /// ```
    /// let mut parser = cmd_parser::CommandParser::new();
    /// ```
    pub fn new() -> Self {
        Self {
            cmd_map: std::collections::HashMap::new(),
        }
    }

    /// Adds a function to the system.
    ///
    /// # Example
    /// ```
    /// fn echo(args: Vec<&str>) {
    ///     println!("{}", args[1..].concat());
    /// }
    ///
    /// let mut parser = cmd_parser::CommandParser::new();
    /// parser.insert("echo", echo);
    /// ```
    pub fn insert(&mut self, name: &str, cmd: fn(Vec<&str>)) -> Result<(), &'static str> {
        // TODO: Make sure no whitespaces are present.
        if let Some(_) = self.cmd_map.get(name) {
            return Err("This command already exists!");
        }
        self.cmd_map.insert(String::from(name), cmd);
        Ok(())
    }

    /// Calls a function with a command string.
    ///
    /// # Example
    /// ```
    /// let mut parser = cmd_parser::CommandParser::new();
    /// parser.insert("echo", |x| println!("{}", x[1..].concat()));
    ///
    /// parser.call("echo This is an example!");
    /// ```
    pub fn call(&mut self, cmd: &str) -> Result<(), &'static str> {
        let args: Vec<&str> = cmd.split_whitespace().filter(|x| x.len() > 0).collect();
        self.call_args(args)
    }

    /// Calls a function with an array of components of the command.
    ///
    /// # Example
    /// ```
    /// let mut parser = cmd_parser::CommandParser::new();
    /// parser.insert("echo", |x| println!("{}", x[1..].concat()));
    ///
    /// parser.call_args(vec!["echo", "This is an example"]);
    /// ```
    pub fn call_args(&mut self, cmd: Vec<&str>) -> Result<(), &'static str> {
        if cmd.len() == 0 {
            return Err("Please specify a command");
        }
        let func = self.cmd_map.get(&String::from(cmd[0]));
        if let Some(f) = func {
            f(cmd);
        } else {
            return Err("Unknown command");
        }

        Ok(())
    }
}
