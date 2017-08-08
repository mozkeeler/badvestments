extern crate oauth_client as oath;
extern crate rand;
extern crate twitter_api as twitter;

pub mod badvestments;
pub mod ast;

use oath::Token;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use std::collections::HashMap;
use rand::Rng;

struct SubstitutionMap {
    map: HashMap<String, Vec<Vec<String>>>
}

impl SubstitutionMap {
    fn new() -> SubstitutionMap {
        SubstitutionMap { map: HashMap::new() }
    }

    fn add_substitution(&mut self, substitution : ast::Substitution) {
        let mut insert_as_new = false;
        match self.map.get_mut(&substitution.s) {
            Some(ref mut substitutions) => {
                substitutions.push(substitution.v.clone());
            },
            None => {
                insert_as_new = true;
            }
        }
        if insert_as_new {
            self.map.insert(substitution.s, vec![substitution.v]);
        }
    }

    fn apply_substitutions(&self, symbols : Vec<String>) -> Vec<String> {
        let mut results: Vec<String> = vec![];
        for symbol in symbols {
            match self.map.get(&symbol) {
                Some(substitutions) => {
                    let chosen = rand::thread_rng().choose(&substitutions).unwrap();
                    for s in chosen.iter() {
                        results.push(s.clone());
                    }
                },
                None => {
                    results.push(symbol);
                }
            }
        }
        return results;
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
        let mut token = AccessToken { consumer_key: String::new(), consumer_secret: String::new(),
                                      access_key: String::new(), access_secret: String::new() };
        let file = match File::open("badvestments.keys") {
            Ok(f) => f,
            Err(_) => return None,
        };
        let reader = BufReader::new(&file);
        let mut lines = reader.lines();
        token.consumer_key = match lines.next() {
            Some(result) => match result {
                Ok(val) => val,
                Err(_) => return None,
            },
            None => return None,
        };
        token.consumer_secret = match lines.next() {
            Some(result) => match result {
                Ok(val) => val,
                Err(_) => return None,
            },
            None => return None,
        };
        token.access_key = match lines.next() {
            Some(result) => match result {
                Ok(val) => val,
                Err(_) => return None,
            },
            None => return None,
        };
        token.access_secret = match lines.next() {
            Some(result) => match result {
                Ok(val) => val,
                Err(_) => return None,
            },
            None => return None,
        };
        return Some(token);
    }
}

fn main() {
    let file = File::open("badvestments.rules").unwrap();
    let reader = BufReader::new(&file);
    let mut substitutions = SubstitutionMap::new();
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
    let mut badvestment = vec![String::from("Badvestment")];
    let mut next = substitutions.apply_substitutions(badvestment.clone());
    while badvestment != next {
        badvestment = next;
        next = substitutions.apply_substitutions(badvestment.clone());
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
