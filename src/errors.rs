use thiserror::Error;

/// WordCountError enumerates all possible errors returned by this library.
#[derive(Error, Debug)]
pub enum Errors {
    /// A success message demonstrating the use of a structure
    /// for arguments.  This approach allows arbitrarily complex
    /// data to be squirrelled away in an error result. 
    #[error("SUCCESS!  We found {} words in {}.", .count, .fname)]
    Success{count: i32, fname: String},

    /// Represents an empty source. 
    #[error("Source file contains no data: {}", .0)]
    EmptySource(String),

    /// Invalid path name.
    #[error("File not found: {}", .0)]
    FileNotFound(String),

    /// Represents a failure to read from input.
    #[error("Read error")]
    ReadError { source: std::io::Error },

    // Formatted errors.
    //#[error("Argument `{}` is required.", .0)]
    // #[error("Argument!!!!!! -> {x}")]
    // MissingArg{x: String},
    #[error("A '{}' argument is required.", .0)]
    MissingArg(String),

    /// Represents all other cases of `std::io::Error`.
    #[error(transparent)]
    IOError(#[from] std::io::Error),
}