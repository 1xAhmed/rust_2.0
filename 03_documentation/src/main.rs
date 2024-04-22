// Lets learn how to document your code

// Documentation is only useful if its readable, Heres a command which generates a good looking website for your documentation 
// `cargo doc`

// But i always use `cargo doc --no-deps --open` --no-deps means you only generates the documentation of your library without the documentation of all your dependencies.
// `--open` autiomatically opens the index page of your documentation, otherwise you have to open manually from this path `target/doc/packagename/index/html` whic is the waste of time


// Lets sayyou want to document this constant in your library. This constant is public, so its documentation will be included in our library, you can document private things as well but they won't be included in your website, unless you pass special argument to your cargo doc command.

// Documentation commant uses markdown syntax. The feature are pretty much is same as markdown but with a couple of exeptions
    // - 1 notable execption is intradoc link, which means hyperlinks to other parts of the documentation, we have as example `[`PUZZLE_PIECES`]` Whenever there is an item in scope at this point, you can simple put [] and backticks [``] and the correct link will be constructed for you. You can also use normal markdown link syntax as well.

// If the thing you want isn't in scope, you can use absolute path as well

    // This is the outer documentation comment
/// Number of pieces in the puzzle
/// 
/// This is a separate paragraph
/// 
/// # History
/// 
/// This is a separate paragraph.
/// - Clickable link: [`PUZZLE_PIECES`]
/// - We tried `7`, but this is better
/// [Spawn a thread](std::thread::spawn)
pub const PUZZLE_PIECES: u32 = 42;

// Rust documentation should always start from the description, then you put your sections of headers

fn main() {

    // You can use this to document things you're inside of. An inner documentation comment is exactly the same, only you add ! instead if another / at the end of comment 
    // like //! or /*! !*/
    println!("Hello, world!");

    // You use inner documentation comments for libraries and modules but for pretty much everything else, you use outer documentation comments


    // For structs, you document the struct as whole at the top butyou document the fields separately.
    // This will work in sections like the struct documentation will be at the top, and fields documentation will be in the sections

    /// This is a Puzzle!
    pub struct Puzzle {
        /// Number of pieces
        pub num_pieces: u32,
        /// Descriptive name
        pub name: String,
    }

    // We dont usally document the impl block itself because there usally isn't anything to say about the group of functions and methods as a whole that you wouldn't already said in the struct documentation.
    // If you did add the documentation to the impl block, it would appear below the list of fields and above the list of functions & methods
    // YOu do want to definately document each associated function and method by adding a documentation comment above the defination of each function & method inside the impl block
    
    impl Puzzle {
        /// Make a new puzzle!
        pub fn new() -> Self {
            Self {
                num_pieces: PUZZLE_PIECES,
                name: "Forest Lake".into(),
            }
        }
    }

}
