extern crate rand;

pub mod badvestments;
pub mod ast;

use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
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

fn main() {
    let file = File::open("badvestments.rules").unwrap();
    let reader = BufReader::new(&file);
    let mut substitutions = SubstitutionMap::new();
    for line in reader.lines() {
        let l = line.unwrap();
        let result = badvestments::parse_Rule(l.as_str());
        println!("{:?}", result);
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

    println!("{}", results.join(" "));
}
