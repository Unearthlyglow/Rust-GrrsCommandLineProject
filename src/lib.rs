//! Andrea, your family has too much money. Shut it up!! - Award

// Find Matches does....
pub fn find_matches(
    content: &str,
    pattern: &str,
    mut writer: impl std::io::Write,
) -> Result<(), anyhow::Error> {
    for line in content.lines() {
        if line.contains(pattern) {
            writeln!(writer, "{}", line)?;
        }
    }
    Ok(())
}

pub mod ModuleExample {
    pub fn module_example() {
        println!("This is a module example");
    }
}

///Testing Out some documentation stuff AND MORE
pub fn pizza_roo() {
    todo!()
}

pub struct Jubilee {
    name: String,
    age: u32,
}

///An example of ~~strikethrough text~~.
///This is an example of a footnote[^note].

///[^note]: This text is the contents of the footnote, which will be rendered
//towards the bottom.
pub enum EnumExample {
    Happy,
    Sad,
}

/// Example
/// ```rust
/// # main() -> Result<(), std::num::ParseIntError> {
/// let fortytwo = "42".parse::<u32>()?;
/// println!("{} + 10 = {}", fortytwo, fortytwo+10);
/// #     Ok(())
/// # }
/// ```
pub fn example() {
    todo!()
}
