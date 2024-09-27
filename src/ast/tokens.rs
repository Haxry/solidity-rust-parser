// //use crate::antlr::gen_output::antlr::Token as AntlrToken; // Adjust the import path as necessary
// use crate::ast::types::{Token, TokenizeOptions}; // Assuming Token and TokenizeOptions are defined in types.rs
// use crate::ast::ast_types::{Comment, Location,Position}; // Assuming Comment and Location are defined in ast-types.rs
// use crate::antlr::gen_output::antlr::solidity_tokens;
// use antlr_rust::token::Token as AntlrToken;

// use super::ast_types::CommentType; // Assuming you have a module for tokens

// const TYPE_TOKENS: [&str; 9] = [
//     "var",
//     "bool",
//     "address",
//     "string",
//     "Int",
//     "Uint",
//     "Byte",
//     "Fixed",
//     "UFixed",
// ];

// fn get_token_type(value: &str) -> &str {
//     match value {
//         "Identifier" | "from" => "Identifier",
//         "TrueLiteral" | "FalseLiteral" => "Boolean",
//         "VersionLiteral" => "Version",
//         "StringLiteral" => "String",
//         _ if TYPE_TOKENS.contains(&value) => "Type",
//         "NumberUnit" => "Subdenomination",
//         "DecimalNumber" => "Numeric",
//         "HexLiteral" => "Hex",
//         "ReservedKeyword" => "Reserved",
//         _ if value.chars().all(|c| !c.is_alphanumeric()) => "Punctuator",
//         _ => "Keyword",
//     }
// }

// fn range(token: &dyn AntlrToken<Data = str>) -> (usize, usize) {
//     (token.get_start() as usize, token.get_stop() as usize + 1)
// }

// // Get location information (line, column, etc.)
// fn loc(token: &dyn AntlrToken<Data = str>) -> Location {
//     let token_text = token.get_text();
//     let text_in_lines: Vec<&str> = token_text.split('\n').collect();
//     let number_of_new_lines = text_in_lines.len() - 1;

//     Location {
//         start: Position {
//             line: token.get_line() as usize,
//             column: token.get_column() as usize,
//         },
//         end: Position {
//             line: token.get_line() as usize + number_of_new_lines,
//             column: text_in_lines[number_of_new_lines].len() + if number_of_new_lines == 0 { token.get_column() as usize } else { 0 },
//         },
//     }
// }

// // Build the token list
// pub fn build_token_list(tokens_arg: &[Box<dyn AntlrToken<Data = str>>], options: &TokenizeOptions) -> Vec<Token> {
//     tokens_arg
//         .iter()
//         .map(|token| {
//             let token_type = get_token_type(token.get_text());
//             let mut node = Token {
//                 token_type: token_type.to_string(),
//                 value: Some(token.get_text().to_string()),
//                 range: None,
//                 loc: None,
//             };

//             if options.range.unwrap_or(false) {
//                 node.range = Some(range(token.as_ref()));
//             }
//             if options.loc.unwrap_or(false) {
//                 node.loc = Some(loc(token.as_ref()));
//             }

//             node
//         })
//         .collect()
// }
// // Build the comment list by filtering out tokens that belong to the comment channel
// pub fn build_comment_list(tokens_arg: &[Box<dyn AntlrToken<Data = str>>], comments_channel_id: isize, options: &TokenizeOptions) -> Vec<Comment> {
//     tokens_arg.iter().filter(|token| token.get_channel() == comments_channel_id).map(|token| {
//         let comment_text = token.get_text();
//         let comment = if comment_text.starts_with("//") {
//             Comment {
//                 r#type: CommentType::LineComment,
//                 value: comment_text[2..].to_string(),
//                 range: None,
//                 loc: None,
//             }
//         } else {
//             Comment {
//                 r#type: CommentType::BlockComment,
//                 value: comment_text[2..comment_text.len() - 2].to_string(),
//                 range: None,
//                 loc: None,
//             }
//         };

//         let mut comment_node = comment;
//         if options.range.unwrap_or(false) {
//             comment_node.range = Some(range(token.as_ref()));
//         }
//         if options.loc.unwrap_or(false) {
//             comment_node.loc = Some(loc(token.as_ref()));
//         }
//         comment_node
//     }).collect()
// }
