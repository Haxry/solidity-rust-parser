#![feature(try_blocks)]

use antlr_rust::common_token_stream::CommonTokenStream;
use antlr_rust::input_stream::InputStream;
use antlr_rust::parser_rule_context::ParserRuleContext;
use antlr_rust::tree::{ParseTreeListener, ParseTree};  // Import the ParseTree trait
use antlr_rust::lexer::Lexer;
use antlr_rust::parser::Parser;

mod antlr {
    pub mod gen_output {
        pub mod antlr {
            pub mod soliditylexer;
            pub mod solidityparser;
            pub mod soliditylistener;
        }
    }
}

use crate::antlr::gen_output::antlr::{soliditylexer, solidityparser};

#[macro_use]
extern crate lazy_static;

fn main() {
    // Input Solidity code
    let solidity_code = String::from(r#"
        pragma solidity ^0.8.0;
        contract SimpleStorage {
            uint256 public data;

            function set(uint256 x) public {
                data = x;
            }
        }
    "#);
    println!("programme started");

    // Convert the String to &str for InputStream
    let input_stream = InputStream::new(solidity_code.as_str());

    // Create a lexer from the input stream
    let lexer = soliditylexer::SolidityLexer::new(input_stream);

    // Create a token stream from the lexer
    let token_stream = CommonTokenStream::new(lexer);

    // Create a parser from the token stream
    let mut parser = solidityparser::SolidityParser::new(token_stream);

    // Parse the input to get the parse tree
    let parse_tree = parser.sourceUnit().expect("Failed to parse Solidity code");

    // Print a human-readable parse tree
    println!("parse tree is: {}", parse_tree.to_string_tree(&*parser));  // Use `to_string_tree` method

    println!("programme end");
}

