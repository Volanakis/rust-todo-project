// Colored is a crate that allows you to colorize your terminal output
use colored::*;

// std::fs is a module that allows you to read and write files
use std::fs;
// OpenOptions is a struct that allows you to open files with different options
use std::fs::OpenOptions;
// std::io::prelude is a module that allows you to read and write files
// the difference between std::fs and std::io::prelude is that std::fs is a module that allows you to read and write files, 
// while std::io::prelude is a module that allows you to read and write files with different options
use std::io::prelude::*;
use std::io::{self, BufReader, Write};
// Path is a struct that allows you to work with file paths
use std::path::Path;
// std::env is a module that allows you to work with environment variables
use std::env;

// Entry is a struct that represents a todo entry
pub struct EntryOfTodo {
    pub todo_entry: String,
    pub done: bool,
}

// TodoList is a struct that represents a todo list
pub struct TodoList {
    pub todo: Vec<String>,
    pub todo_path: String,
    pub todo_backup_path: String,
    pub no_backup: bool,
}

