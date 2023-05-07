#![allow(unused)]
use anyhow::{Context, Result};
use clap::Parser;
use std::io::{self, Write};

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    path: std::path::PathBuf,
}

#[derive(Debug)]
struct CustomError(String);

fn main() -> Result<(), anyhow::Error> {
    let args = Cli::parse();
    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("could not read file `{}`", args.path.display()))?;

    grrs::find_matches(&content, &args.pattern, &mut std::io::stdout());

    Ok(())
}

#[test]
fn find_a_match() {
    let mut result = Vec::new();
    grrs::find_matches("lorem ipsum\ndolor sit amet", "lorem", &mut result);
    assert_eq!(result, b"lorem ipsum\n");
}

//Code Breakdown

#[derive(Parser)]
{
    // Attributes available to this derive:
    #[clap]
    #[structopt]
    #[command]
    #[arg]
    #[group]
}

The derive attribute allows new items to be automatically generated for data structures. The derive attribute is used to specify which traits to automatically implement for the data structure. The derive attribute is used like this: #[derive(Trait1, Trait2, ...)].

The following:

#[derive(PartialEq, Clone)]
struct Foo<T> {
    a: i32,
    b: T,
}

is equivalent to:

impl<T: PartialEq> PartialEq for Foo<T> {
    fn eq(&self, other: &Foo<T>) -> bool {
        self.a == other.a && self.b == other.b
    }
}


//Notes::



//Dev Dependencies vs Dependencies in Rust.

//A note on printing performance
// Printing to the terminal is surprisingly slow! If you call things like println! in a loop, it can easily become a bottleneck in an otherwise fast program. To speed this up, there are two things you can do.

// First, you might want to reduce the number of writes that actually “flush” to the terminal. println! tells the system to flush to the terminal every time, because it is common to print each new line. If you don’t need that, you can wrap your stdout handle in a BufWriter which by default buffers up to 8 kB. (You can still call .flush() on this BufWriter when you want to print immediately.)

// use std::io::{self, Write};

// let stdout = io::stdout(); // get the global stdout entity
// let mut handle = stdout.lock(); // acquire a lock on it
// writeln!(handle, "foo: {}", 42); // add `?` if you care about errors here

// Showing a progress bar
// Some CLI applications run less than a second, others take minutes or hours. If you are writing one of the latter types of programs, you might want to show the user that something is happening. For this, you should try to print useful status updates, ideally in a form that can be easily consumed.

// Using the indicatif crate, you can add progress bars and little spinners to your program. Here’s a quick example:

// fn main() {
//     let pb = indicatif::ProgressBar::new(100);
//     for i in 0..100 {
//         do_hard_work();
//         pb.println(format!("[+] finished #{}", i));
//         pb.inc(1);
//     }
//     pb.finish_with_message("done");
// }

// Logging
// To make it easier to understand what is happening in our program, we might want to add some log statements. This is usually easy while writing your application. But it will become super helpful when running this program again in half a year. In some regard, logging is the same as using println!, except that you can specify the importance of a message. The levels you can usually use are error, warn, info, debug, and trace (error has the highest priority, trace the lowest).

// To add simple logging to your application, you’ll need two things: The log crate (this contains macros named after the log level) and an adapter that actually writes the log output somewhere useful. Having the ability to use log adapters is very flexible: You can, for example, use them to write logs not only to the terminal but also to syslog, or to a central log server.

// Since we are right now only concerned with writing a CLI application, an easy adapter to use is env_logger. It’s called “env” logger because you can use an environment variable to specify which parts of your application you want to log (and at which level you want to log them). It will prefix your log messages with a timestamp and the module where the log messages come from. Since libraries can also use log, you easily configure their log output, too.

// Here’s a quick example:

// use log::{info, warn};

// fn main() {
//     env_logger::init();
//     info!("starting up");
//     warn!("oops, nothing implemented!");
// }
// Assuming you have this file as src/bin/output-log.rs, on Linux and macOS, you can run it like this:

// $ env RUST_LOG=info cargo run --bin output-log
//     Finished dev [unoptimized + debuginfo] target(s) in 0.17s
//      Running `target/debug/output-log`
// [2018-11-30T20:25:52Z INFO  output_log] starting up
// [2018-11-30T20:25:52Z WARN  output_log] oops, nothing implemented!

// RUST_LOG is the name of the environment variable you can use to set your log settings. env_logger also contains a builder so you can programmatically adjust these settings, and, for example, also show info level messages by default.

// There are a lot of alternative logging adapters out there, and also alternatives or extensions to log. If you know your application will have a lot to log, make sure to review them, and make your users’ life easier.

// Tip: Experience has shown that even mildly useful CLI programs can end up being used for years to come. (Especially if they were meant as a temporary solution.) If your application doesn’t work and someone (e.g., you, in the future) needs to figure out why, being able to pass --verbose to get additional log output can make the difference between minutes and hours of debugging. The clap-verbosity-flag crate contains a quick way to add a --verbose to a project using clap.

//Testing
// One easy way to do that is to write a README file that describes what your program should do. And when you feel ready to make a new release, go through the README and ensure that the behavior is still as expected. You can make this a more rigorous exercise by also writing down how your program should react to erroneous inputs.

//In Notion I need to write up a Notion::Rust::Testing page that details all the notes I have on testing that I have uncovered through all my reading.

//What to test?
// While it can certainly be fun to write integration tests, it will also take some time to write them, as well as to update them when your application’s behavior changes. To make sure you use your time wisely, you should ask yourself what you should test.

// In general it’s a good idea to write integration tests for all types of behavior that a user can observe. That means that you don’t need to cover all edge cases: It usually suffices to have examples for the different types and rely on unit tests to cover the edge cases.

// It is also a good idea not to focus your tests on things you can’t actively control. It would be a bad idea to test the exact layout of --help as it is generated for you. Instead, you might just want to check that certain elements are present.

// Depending on the nature of your program, you can also try to add more testing techniques. For example, if you have extracted parts of your program and find yourself writing a lot of example cases as unit tests while trying to come up with all the edge cases, you should look into proptest. If you have a program which consumes arbitrary files and parses them, try to write a fuzzer to find bugs in edge cases.