use std::env;
use log::{error, warn, info, debug, trace};
use anyhow::{Context, Result, anyhow};

use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

// Applicaion error messages.
mod errors;
use errors::Errors;

/** This demonstration program explores how to integrate log4rs logging and error handling
 * using anyhow and thiserror libraries.  This is a first approximation solution that can be
 * improved over time (see below).  The current implementation shows how to:
 * 
 *  1. Define and use enummerated error messages annotated by thiserror macros.
 *  2. Log error messages of a specified format to output targets using log4rs.
 * 
 * See error.rs for error messages; see /resources/log4rs.yml for log configuration.
 * 
 * Execution
 * =========
 * Issue the following commands to generate specific errors:
 * 
 *  1. MissingArg:     cargo run
 *  2. FileNotFount:   cargo run xxx
 *  3. ReadError:      cargo run resources/stampede2-0717.jpg
 *  4. EmptySource     cargo run resources/inputempty.txt
 * 
 * No error execution: cargo run resources/input.txt
 * 
 * Future Work
 * ===========
 * 1. Better integration of log4rs with anyhow to reduce the code needed to handle the
 *    MissingArg example below.  
 * 
 *    Create a custom type that implements
 *    Debug and Display to be passed into anyhow!().  This type would write to the log4rs
 *    log at a specified level (error, warn, etc.) when given a string. The macro would 
 *    return an ad-how error like anyhow! currently does.  This idea is to reduce the
 *    code needed to handle the MissingArg example below.
 * 
 * 2. Better integration of log4rs with thiserror to implicitly log errors such as those
 *    handled by map_err() in the ReadError example below.
 * 
 *    Determine how to best map a Result<T, E> to Result<T, F> by applying a function to a 
 *    contained Err value.  The goal is to automatically log the error when transforming 
 *    the result.
 * 
 * 3. Generally, figure out how to get sufficient backtraces for efficient debugging.
 * 
 * 4. Figure out how to stop log4rs from creating empty log files for appenders that are 
 *    defined by not referenced in the log4rs.yml configuration file.
 */
fn main() -> Result<()> {
    println!("Starting logtest2");

    // Initialize log4rs.
    log4rs::init_file("resources/log4rs.yml", Default::default()).unwrap();

    // Log each type of message.
    error!("msg1");
    warn!("msg2");
    info!("msg3");
    debug!("msg4");
    trace!("msg5"); 

    // Get at least 1 command line argument. This shows how to get a record written
    // to the log capturing the line number and also return an error result.  
    if env::args().len() < 2 {
        let msg = format!("{}", Errors::MissingArg("filename".to_string()));
        error!("{}", msg);
        return Err(anyhow!(msg));
    }

    // Open the input file.
    let mut wordcount = 0;
    for filename in env::args().skip(1).collect::<Vec<String>>() {
        let mut file = File::open(&filename)
            .context(format!("{}", Errors::FileNotFound(filename.clone())))?;

        // Read the file.
        let reader = BufReader::new(&mut file);
        for line in reader.lines() {
            let line = line.map_err(|source| Errors::ReadError { source })?;
            for _word in line.split_whitespace() {
                wordcount += 1;
            }
        } 
        
        // We don't like empty files.
        if wordcount == 0 {
            return Err(anyhow!(Errors::EmptySource(filename)));
        }    

        // The success message return the number of words and the input filename.
        info!("{}", format!("{}", Errors::Success{count: wordcount, fname: filename}));
    }
           
    Ok(())
}
