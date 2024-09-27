#![feature(try_blocks)]

use antlr_rust::common_token_stream::CommonTokenStream;
use antlr_rust::input_stream::InputStream;
use antlr_rust::int_stream::{IntStream, EOF};
use antlr_rust::parser_rule_context::ParserRuleContext;
use antlr_rust::token::Token;
use antlr_rust::token_stream::TokenStream;
use antlr_rust::tree::{ParseTreeListener, ParseTree};  // Import the ParseTree trait
use antlr_rust::lexer::Lexer;
use antlr_rust::parser::Parser;

mod antlr {
    pub mod gen_output {
        pub mod antlr {
            pub mod soliditylexer;
            pub mod solidityparser;
            pub mod soliditylistener;
            pub mod solidityvisitor;
            pub mod solidity_tokens;
            
        }
    }
}

mod ast{
    pub mod ASTBuilder;
    pub mod ast_types;
    pub mod types;
    pub mod tokens;
}
use crate::antlr::gen_output::antlr::{soliditylexer, solidityparser};

#[macro_use]
extern crate lazy_static;

// fn main() {
//     // Input Solidity code
//     let solidity_code = String::from(r#"
//         pragma solidity ^0.8.0;
//         contract SimpleStorage {
//             uint256 public data;

//             function set(uint256 x) public {
//                 data = x;
//             }
//         }
//     "#);
//     println!("programme started");

//     // Convert the String to &str for InputStream
//     let input_stream = InputStream::new(solidity_code.as_str());

//     // Create a lexer from the input stream
//     let lexer = soliditylexer::SolidityLexer::new(input_stream);

//     // Create a token stream from the lexer
//     let token_stream = CommonTokenStream::new(lexer);
    
//     let mut index: isize = 0;
//     loop{
//         let token = token_stream.get(index);
//         println!("token stream is {:?}",token);
//         if token.get_token_type()==EOF{
//             break;
//         }
        
//         index+=1;
//     }
//     //println!("token stream is {:?}",token_stream.get(index));

//     // Create a parser from the token stream
//     let mut parser = solidityparser::SolidityParser::new(token_stream);

//     // Parse the input to get the parse tree
//     let parse_tree = parser.sourceUnit().expect("Failed to parse Solidity code");

//     // Print a human-readable parse tree
//     println!("parse tree is: {}", parse_tree.to_string_tree(&*parser));  // Use `to_string_tree` method

//     println!("programme end");
// }

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
    println!("Programme started");

    // Convert the String to &str for InputStream
    let input_stream = InputStream::new(solidity_code.as_str());

    // Create a lexer from the input stream
    let lexer = soliditylexer::SolidityLexer::new(input_stream);

    // Create a token stream from the lexer
    let mut token_stream = CommonTokenStream::new(lexer);

    // let mut index: isize = 0;
    
    // // Iterate over tokens until we encounter EOF
    // loop {
    //     let token = token_stream.get(index);
        
    //     // Check if the token is EOF
    //     if token.get_token_type() == EOF {
    //         break;
    //     }

    //     // Print token information: type and text
    //     println!(
    //         "Token[Type: {:?}, Text: {:?}]",
    //         token.get_token_type(),
    //         token.get_text()
    //     );

    //     index += 1;
    // }
    let mut token = token_stream.lt(1).unwrap(); // Look at the first token
    while token.get_token_type() != EOF {
        // Print token information: type and text
        println!(
            "Token[Type: {:?}, Text: {:?}]",
            token.get_token_type(),
            token.get_text(),
            //add more methods wanted for tokens
        );

        // Move to the next token
        token_stream.consume();
        token = token_stream.lt(1).unwrap(); // Look at the next token
    }

    // Create a parser from the token stream
    let input_stream_parser = InputStream::new(solidity_code.as_str());

    // Create a lexer from the input stream
    let lexer_parser = soliditylexer::SolidityLexer::new(input_stream_parser);

    // Create a token stream from the lexer
    let  token_stream_parser = CommonTokenStream::new(lexer_parser);
    let mut parser = solidityparser::SolidityParser::new(token_stream_parser);

    // Parse the input to get the parse tree
    let parse_tree = parser.sourceUnit().expect("Failed to parse Solidity code");

    // Print a human-readable parse tree
    println!("Parse tree is: {}", parse_tree.to_string_tree(&*parser));  // Use `to_string_tree` method

    println!("Programme end");
}

