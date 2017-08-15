extern crate oauth_client as oath;
extern crate rand;
extern crate twitter_api as twitter;

pub mod ast;
pub mod badvestments;

use oath::Token;
use rand::Rng;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};

#[derive(Debug)]
struct SubstitutionsMap {
    map: HashMap<String, Substitutions>
}

struct TokenMap {
    map: HashMap<String, usize>
}

#[derive(Debug)]
struct Substitution {
    produces: Vec<String>,
}

#[derive(Debug)]
struct Substitutions {
    options: Vec<Substitution>
}

impl SubstitutionsMap {
    fn new() -> SubstitutionsMap {
        SubstitutionsMap { map: HashMap::new() }
    }

    fn add_substitution(&mut self, substitution : ast::Substitution) {
        let new_substitution = Substitution::new(substitution.v);
        if self.find_loop(&substitution.s, &new_substitution) {
            println!("can't add {} ::= {:?} - loop found", substitution.s, new_substitution);
            return;
        }
        match self.map.get_mut(&substitution.s) {
            Some(ref mut substitutions) => {
                substitutions.options.push(new_substitution);
                return;
            },
            None => {}
        };
        self.map.insert(substitution.s, Substitutions::new(new_substitution));
    }

    fn apply_substitutions(&self, token_map: &TokenMap, symbols : Vec<String>) -> Vec<String> {
        let mut results: Vec<String> = vec![];
        for symbol in symbols {
            match self.map.get(&symbol) {
                Some(substitutions) => {
                    let mut total_options = 0;
                    for sub in substitutions.options.iter() {
                        total_options += sub.get_count(token_map);
                    }
                    let mut rng = rand::thread_rng();
                    let chosen = rng.gen_range(0, total_options);
                    let mut current = 0;
                    for substitution in substitutions.options.iter() {
                        let count = substitution.get_count(token_map);
                        if current + count >= chosen {
                            for produced in substitution.produces.iter() {
                                results.push(produced.clone());
                            }
                            break;
                        }
                        current += count;
                    }
                },
                None => {
                    results.push(symbol);
                }
            }
        }
        results
    }

    fn find_loop(&self, symbol : &String, substitution : &Substitution) -> bool {
        for produced in substitution.produces.iter() {
            if symbol == produced {
                return true;
            }
            match self.map.get(produced) {
                Some(preexisting_substitutions) => {
                    for preexisting_substitution in preexisting_substitutions.options.iter() {
                        if self.find_loop(symbol, preexisting_substitution) {
                            return true;
                        }
                    }
                },
                None => {}
            }
        }
        false
    }

    fn count_for(&self, symbol : &String) -> usize {
        let mut count = 0;
        match self.map.get(symbol) {
            Some(substitutions) => {
                for substitution in substitutions.options.iter() {
                    let mut sub_count = 1;
                    for produced in substitution.produces.iter() {
                        sub_count *= self.count_for(produced);
                    }
                    count += sub_count;
                }
            },
            None => {
                count = 1;
            }
        }
        count
    }
}

impl TokenMap {
    fn new(substitutions_map : &SubstitutionsMap) -> TokenMap {
        let mut token_map = TokenMap { map: HashMap::new() };
        for token in substitutions_map.map.keys() {
            token_map.map.insert(token.clone(), substitutions_map.count_for(token));
        }
        token_map
    }

    fn count_for(&self, token : &String) -> usize {
        return match self.map.get(token) {
            Some(count) => *count,
            None => 1
        }
    }
}

impl Substitution {
    fn new(produces : Vec<String>) -> Substitution {
        Substitution { produces: produces }
    }

    fn get_count(&self, token_map : &TokenMap) -> usize {
        let mut count = 0;
        for token in self.produces.iter() {
            count += token_map.count_for(token);
        }
        count
    }
}

impl Substitutions {
    fn new(substitution : Substitution) -> Substitutions {
        Substitutions { options: vec![substitution] }
    }
}

fn prompt(msg: &str) -> String {
    print!("{}: ", msg);
    io::stdout().flush().unwrap();
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    return line.trim().to_string();
}

struct AccessToken {
    pub consumer_key: String,
    pub consumer_secret: String,
    pub access_key: String,
    pub access_secret: String,
}

impl AccessToken {
    pub fn read() -> Option<AccessToken> {
        let file = match File::open("badvestments.keys") {
            Ok(f) => f,
            Err(_) => return None,
        };
        let reader = BufReader::new(&file);
        let mut lines = reader.lines();
        let consumer_key = match lines.next() {
            Some(result) => match result {
                Ok(val) => val,
                Err(_) => return None,
            },
            None => return None,
        };
        let consumer_secret = match lines.next() {
            Some(result) => match result {
                Ok(val) => val,
                Err(_) => return None,
            },
            None => return None,
        };
        let access_key = match lines.next() {
            Some(result) => match result {
                Ok(val) => val,
                Err(_) => return None,
            },
            None => return None,
        };
        let access_secret = match lines.next() {
            Some(result) => match result {
                Ok(val) => val,
                Err(_) => return None,
            },
            None => return None,
        };
        return Some(AccessToken { consumer_key: consumer_key, consumer_secret: consumer_secret,
                                  access_key: access_key, access_secret: access_secret });
    }
}

fn main() {
    let file = File::open("badvestments.rules").unwrap();
    let reader = BufReader::new(&file);
    let mut substitutions = SubstitutionsMap::new();
    for line in reader.lines() {
        let l = line.unwrap();
        let result = badvestments::parse_Rule(l.as_str());
        match result {
            Ok(substitution) => {
                substitutions.add_substitution(substitution);
            },
            _ => {
                println!("(error parsing)");
            }
        }
    };
    let token_map = TokenMap::new(&substitutions);
    let mut badvestment = vec![String::from("Badvestment")];
    let mut next = substitutions.apply_substitutions(&token_map, badvestment.clone());
    while badvestment != next {
        badvestment = next;
        next = substitutions.apply_substitutions(&token_map, badvestment.clone());
    }
    let results: Vec<String> = badvestment.into_iter()
                                          .map(|s| { s.replace('"', "") })
                                          .collect();

    let tweet = results.join(" ");
    println!("{}", tweet);
    let token = match AccessToken::read() {
        Some(t) => t,
        None => {
            println!("log in to twitter and visit https://apps.twitter.com/app");
            println!("go to the 'Keys and Access Tokens' tab and click");
            println!("'(Re)generate My Access Token and Token Secret'");
            println!("Copy your Consumer Key, Consumer Secret, Access Token, and");
            println!("Access Token Secret and save them in badvestments.keys");
            panic!("couldn't read access tokens");
        },
    };
    let consumer = Token::new(token.consumer_key, token.consumer_secret);
    let access = Token::new(token.access_key, token.access_secret);

    let doit = prompt("tweet? y/n");
    match doit.as_ref() {
        "y" => {
            twitter::update_status(&consumer, &access, &tweet).unwrap();
        },
        _ => {
            println!("ok - didn't tweet that.");
        }
    }
}
