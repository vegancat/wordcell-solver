use std::time::{Instant};
use bip0039::Mnemonic;
use solana_sdk::{
    signature::{Keypair, Signer}
};
use std::fs::File;
use std::io::prelude::*;

use regex::Regex;

fn filterByLength(wordlist: &Vec<String>, length: usize) -> Vec<String> {
    let mut filtered: Vec<String> = Vec::new();
    for word in wordlist {
        if word.len() == length {
            filtered.push(word.to_string());
        }
    }

    filtered
}



fn filter_by_pattern(wordlist: &Vec<String>, pattern: Regex, word_length: usize) -> Vec<String> {

    let new_word_list = filterByLength(wordlist, word_length);

    let mut filtered: Vec<String> = Vec::new();
    for word in new_word_list {

        if pattern.is_match(&word) {
            filtered.push(word);
        }
    }

    filtered
}

fn find_max_tries(keys: Vec<Vec<String>>) -> u128 {
    let mut max_tries: u128 = 1;
    for key in keys {
        max_tries *= key.len() as u128;
    }

    max_tries
}

fn main() {
    let start = Instant::now();
    let mut file = File::open("bip39Keywords.txt").expect("File not found");
    let mut data = String::new();
    file.read_to_string(&mut data)
        .expect("Error while reading file");

    let words = data
        .split_whitespace()
        .map(String::from)
        .collect::<Vec<String>>();

    let mnemonic_length = 24;
    let target_pub_key = String::from("FMR2u5HHJDadnDYaHoWGNjVHaNEoSwYohcwMvnLYxm2X");

    let keys: Vec<Vec<String>> = vec![
        filter_by_pattern(&words, Regex::new(r"^[cC].*$").unwrap(), 7),
        filter_by_pattern(&words, Regex::new(r"^[aA].*[rR]$").unwrap(), 3),
        filter_by_pattern(&words, Regex::new(r"^[cC].*$").unwrap(), 8),
        filter_by_pattern(&words, Regex::new(r"^[oO].*[nN]$").unwrap(), 3),
        filter_by_pattern(&words, Regex::new(r"^[pP].*$").unwrap(), 7),
        filter_by_pattern(&words, Regex::new(r"^[lL].*$").unwrap(), 4),
        filter_by_pattern(&words, Regex::new(r"^[cC].*$").unwrap(), 5),
        filter_by_pattern(&words, Regex::new(r"^[sS].*$").unwrap(), 4),
        filter_by_pattern(&words, Regex::new(r"^[tT].*$").unwrap(), 5),
        filter_by_pattern(&words, Regex::new(r"^[bB].*$").unwrap(), 5),
        filter_by_pattern(&words, Regex::new(r"^[rR].*$").unwrap(), 4),
        filter_by_pattern(&words, Regex::new(r"^[jJ].*$").unwrap(), 5),
        filter_by_pattern(&words, Regex::new(r"^[mM].*$").unwrap(), 5),
        filter_by_pattern(&words, Regex::new(r"^[sS].*$").unwrap(), 6),
        filter_by_pattern(&words, Regex::new(r"^[jJ].*$").unwrap(), 5),
        filter_by_pattern(&words, Regex::new(r"^[rR].*$").unwrap(), 7),
        filter_by_pattern(&words, Regex::new(r"^[pP].*$").unwrap(), 7),
        filter_by_pattern(&words, Regex::new(r"^[tT].*$").unwrap(), 6),
        filter_by_pattern(&words, Regex::new(r"^[sS].*$").unwrap(), 5),
        filter_by_pattern(&words, Regex::new(r"^[dD].*$").unwrap(), 6),
        filter_by_pattern(&words, Regex::new(r"^[tT].*[yY]$").unwrap(), 3),
        filter_by_pattern(&words, Regex::new(r"^[pP].*$").unwrap(), 7),
        filter_by_pattern(&words, Regex::new(r"^[mM].*$").unwrap(), 7),
        filter_by_pattern(&words, Regex::new(r"^[bB].*[xX]$").unwrap(), 3),
    ];

    let max_tries = find_max_tries(keys.clone());
    println!("Max tries: {}", max_tries);

    let avg_time_for_100000_tries = 68.305335004;
    let time_per_seed = avg_time_for_100000_tries / 100000.0;

    // estimated time in hours
    let estimated_time = time_per_seed * max_tries as f64  / 3600.0;

    println!("Estimated time: {} hours", estimated_time);
    let mut z = 0;

    if mnemonic_length == 12 {
        for a in 0..keys[0].len() {
            for b in 0..keys[1].len() {
                for c in 0..keys[2].len() {
                    for d in 0..keys[3].len() {
                        for e in 0..keys[4].len() {
                            for f in 0..keys[5].len() {
                                for g in 0..keys[6].len() {
                                    for h in 0..keys[7].len() {
                                        for i in 0..keys[8].len() {
                                            for j in 0..keys[9].len() {
                                                for k in 0..keys[10].len() {
                                                    for l in 0..keys[11].len() {

                                                        z += 1;

                                                        let mnemonic = format!(
                                                            "{} {} {} {} {} {} {} {} {} {} {} {}",
                                                            keys[0][a],
                                                            keys[1][b],
                                                            keys[2][c],
                                                            keys[3][d],
                                                            keys[4][e],
                                                            keys[5][f],
                                                            keys[6][g],
                                                            keys[7][h],
                                                            keys[8][i],
                                                            keys[9][j],
                                                            keys[10][k],
                                                            keys[11][l],
                                                        );
    
                                                        let seed = Mnemonic::from_phrase(&mnemonic);
                                                        match seed {
                                                            Ok(seed_phrase) => {
                                                                let imported_keypiar =
                                                                    Keypair::from_bytes(
                                                                        &seed_phrase.to_seed(""),
                                                                    );
    
                                                                match imported_keypiar {
                                                                    Ok(some_keypair) => {
                                                                        
                                                                        if some_keypair.pubkey().to_string() == target_pub_key {
                                                                            println!("Found it!");
                                                                            println!("{} seeds checked", z);
                                                                            println!("{}", mnemonic);
                                                                            
                                                                            let duration = start.elapsed();
                                                                            println!("Time elapsed in main() is: {:?}", duration);
                                                                            return;
                                                                        }

                                                                        println!("{} seeds checked", z);
                                                                    }
    
                                                                    Err(_e) => {}
                                                                }
                                                            }
                                                            Err(_e) => {}
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    } else if mnemonic_length == 24 {
        for a in 0..keys[0].len() {
            for b in 0..keys[1].len() {
                for c in 0..keys[2].len() {
                    for d in 0..keys[3].len() {
                        for e in 0..keys[4].len() {
                            for f in 0..keys[5].len() {
                                for g in 0..keys[6].len() {
                                    for h in 0..keys[7].len() {
                                        for i in 0..keys[8].len() {
                                            for j in 0..keys[9].len() {
                                                for k in 0..keys[10].len() {
                                                    for l in 0..keys[11].len() {
                                                        for m in 0..keys[12].len() {
                                                            for n in 0..keys[13].len() {
                                                                for o in 0..keys[14].len() {
                                                                    for p in 0..keys[15].len() {
                                                                        for q in 0..keys[16].len() {
                                                                            for r in 0..keys[17].len() {
                                                                                for s in 0..keys[18].len() {
                                                                                    for t in 0..keys[19].len() {
                                                                                        for u in 0..keys[20].len() {
                                                                                            for v in 0..keys[21].len() {
                                                                                                for w in 0..keys[22].len() {
                                                                                                    for x in 0..keys[23].len() {
                                                                                                                                                                                                                
                                                                                                        z += 1;
                                                                                                        let mnemonic = format!(
                                                                                                            "{} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {}",
                                                                                                            keys[0][a],
                                                                                                            keys[1][b],
                                                                                                            keys[2][c],
                                                                                                            keys[3][d],
                                                                                                            keys[4][e],
                                                                                                            keys[5][f],
                                                                                                            keys[6][g],
                                                                                                            keys[7][h],
                                                                                                            keys[8][i],
                                                                                                            keys[9][j],
                                                                                                            keys[10][k],
                                                                                                            keys[11][l],
                                                                                                            keys[12][m],
                                                                                                            keys[13][n],
                                                                                                            keys[14][o],
                                                                                                            keys[15][p],
                                                                                                            keys[16][q],
                                                                                                            keys[17][r],
                                                                                                            keys[18][s],
                                                                                                            keys[19][t],
                                                                                                            keys[20][u],
                                                                                                            keys[21][v],
                                                                                                            keys[22][w],
                                                                                                            keys[23][x],
                                                                                                        );

                                                                                                        let seed = Mnemonic::from_phrase(&mnemonic);
                                                                                                        match seed {
                                                                                                            Ok(seed_phrase) => {
                                                                                                                let imported_keypiar =
                                                                                                                    Keypair::from_bytes(
                                                                                                                        &seed_phrase.to_seed(""),
                                                                                                                    );
                                                    
                                                                                                                match imported_keypiar {
                                                                                                                    Ok(some_keypair) => {
                                                                                                                        println!("{} seeds checked", z);
                                                                                                                        
                                                                                                                        if some_keypair.pubkey().to_string() == target_pub_key {
                                                                                                                            println!("Found it!");
                                                                                                                            println!("{} seeds checked", z);
                                                                                                                            println!("{}", mnemonic);

                                                                                                                            let duration = start.elapsed();
                                                                                                                            println!("Time elapsed in main() is: {:?}", duration);
                                                                                                                            return;
                                                                                          
                                                                                                                        }
                                                                                                                    }
                                                    
                                                                                                                    Err(_e) => {}
                                                                                                                }
                                                                                                            }
                                                                                                            Err(_e) => {}
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

                                                                                                        
    let duration = start.elapsed();
    println!("Time elapsed in main() is: {:?}", duration);
                                                                                    
}
