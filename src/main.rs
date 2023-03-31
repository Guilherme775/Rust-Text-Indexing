use std::{collections::HashMap, thread, fs};

use rust_indexing::lexer::{Lexer, Token};

// fn single_thread_indexer(lexer: Lexer) -> HashMap<Token, usize> {
//     let mut hash_map = HashMap::new();

//     for token in lexer {
//         if let Some(quantity) = hash_map.get(&token) {
//             hash_map.insert(token, quantity + 1);
//         } else {
//             hash_map.insert(token, 1);
//         }
//     }

//     hash_map
// }

// First version
fn multi_thread_indexer(lexer: Lexer) -> HashMap<Token, usize> {
    let num_chunks = 10;

    let chunk_size = (lexer.tokens.len() as f32 / num_chunks as f32).ceil() as usize;
    let split_lexer = lexer.tokens.chunks(chunk_size).collect::<Vec<_>>();

    let mut handlers = vec![];

    for tokens in split_lexer {
        let tokens = tokens.to_vec();

        handlers.push(thread::spawn(move || {
            let mut hash_map = HashMap::new();

            for token in tokens {
                if let Some(quantity) = hash_map.get(&token) {
                    hash_map.insert(token, quantity + 1);
                } else {
                    hash_map.insert(token, 1);
                }
            }

            hash_map
        }));
    }

    let mut hash_map = HashMap::new();

    for handler in handlers {
        let handler_result = handler.join().unwrap();

        for (item, value) in handler_result {
            if let Some(quantity) = hash_map.get(&item) {
                hash_map.insert(item, value + quantity);
            } else {
                hash_map.insert(item, value);
            }
        }
    }

    hash_map
}

// Second Version (I did this because I thought it would be faster, but it didn't differ much from the first version)
// fn multi_thread_indexer(lexer: Lexer) -> HashMap<Token, usize> {
//     let num_threads = 10;

//     let mut handlers = vec![];

//     for i in 1 .. (num_threads + 1) {
//         let tokens = if i != 10 {
//             let quantity = (lexer.tokens.len() as f32 / num_threads as f32).floor() as usize;

//             let start = quantity * (i - 1);
//             let end = quantity * i;

//             lexer.tokens[start .. end].to_vec()
//         } else {
//             let quantity = (lexer.tokens.len() as f32 / num_threads as f32).floor() as usize;

//             let start = quantity * (i - 1);
//             let end = lexer.tokens.len() - 1;

//             lexer.tokens[start .. end].to_vec()
//         };

//         handlers.push(thread::spawn(move || {
//             let mut hash_map = HashMap::new();

//             for token in tokens {
//                 if let Some(quantity) = hash_map.get(&token) {
//                     hash_map.insert(token, quantity + 1);
//                 } else {
//                     hash_map.insert(token, 1);
//                 }
//             }

//             hash_map
//         }));
//     }

//     let mut hash_map = HashMap::new();

//     for handler in handlers {
//         let handler_result = handler.join().unwrap();

//         for (item, value) in handler_result {
//             if let Some(quantity) = hash_map.get(&item) {
//                 hash_map.insert(item, value + quantity);
//             } else {
//                 hash_map.insert(item, value);
//             }
//         }
//     }

//     hash_map
// }

fn main() {
    let content = fs::read_to_string("test/alice.txt").unwrap();

    let lexer = Lexer::tokenize(&content);
    // single_thread_indexer(lexer);
    multi_thread_indexer(lexer);
}
