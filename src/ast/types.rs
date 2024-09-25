use crate::ast::ast_types::{Location,Position};


// Define the `Node` trait as a Rust struct

pub struct Node {
    pub node_type: String,  // In Rust, we cannot use `type` as it's a reserved keyword, so renamed to `node_type`
}

// Define `TokenizeOptions` struct with optional fields using `Option`
pub struct TokenizeOptions {
    pub range: Option<bool>,  // Use tuple for range as in TypeScript's `[number, number]`
    pub loc: Option<bool>,               // Use a separate `Loc` struct to represent location information
}

// Define `ParseOptions` struct which extends `TokenizeOptions`
pub struct ParseOptions {
    pub tokenize_options: TokenizeOptions,  // Composition: include TokenizeOptions
    pub comments: Option<bool>,
    pub tokens: Option<bool>,
    pub tolerant: Option<bool>,
}

// Define the `Token` struct
pub struct Token {
    pub token_type: String,               // Same as above, renamed to `token_type`
    pub value: Option<String>,            // `value` can be `None` (undefined in TypeScript), so use `Option`
    pub range: Option<(usize, usize)>,    // Tuple for the range
    pub loc: Option<Location>,                 // Use `Loc` struct for location information
}

// Define the `Loc` struct which represents location information
// pub struct Loc {
//     pub start: Position,
//     pub end: Position,
// }

// // Define the `Position` struct for start and end position
// pub struct Position {
//     pub line: usize,
//     pub column: usize,
// }
