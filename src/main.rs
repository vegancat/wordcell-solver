use bip0039::Mnemonic;
use solana_sdk::{
    signature::{Keypair, Signer}
};
use std::fs::File;
use std::io::prelude::*;

fn filterByLength(wordlist: &Vec<String>, length: usize) -> Vec<String> {
    let mut filtered: Vec<String> = Vec::new();
    for word in wordlist {
        if word.len() == length {
            filtered.push(word.to_string());
        }
    }

    filtered
}

fn main() {
    let mut file = File::open("bip39Keywords.txt").expect("File not found");
    let mut data = String::new();
    file.read_to_string(&mut data)
        .expect("Error while reading file");

    let words = data
        .split_whitespace()
        .map(String::from)
        .collect::<Vec<String>>();

    let keys: Vec<Vec<String>> = vec![
        vec![String::from("speed"), String::from("cheap")],
        vec![String::from("history")],
        vec![String::from("adjust"), String::from("insane"), String::from("noodle")],
        vec![String::from("pitch"), String::from("news")],
        vec![
            String::from("anxiety"),
            String::from("wastage"),
            String::from("amateur"),
        ],
        vec![String::from("drop")],
        vec![
            // String::from("hear"),
            String::from("note"),
            String::from("mind"),
        ],
        vec![String::from("dignity")],
        vec![
            String::from("inherit"),
            String::from("supreme"),
            String::from("citizen"),
            String::from("amazing"),
            String::from("antique"),
            String::from("average"),
            String::from("captain"),
            String::from("culture"),
            String::from("erosion"),
            String::from("essence"),
            String::from("fantasy"),
            String::from("fiction"),
            String::from("general"),
            String::from("goddess"),
            String::from("illegal"),
            String::from("kingdom"),
            String::from("private"),
            String::from("pyramid"),
            String::from("special"),
            String::from("citizen"),
        ],
        vec![String::from("reveal")],
        vec![
            String::from("oblige"),
            String::from("govern"),
            String::from("attack"),
            String::from("arrest"),
            String::from("battle"),
            String::from("census"),
            String::from("clutch"),
            String::from("crunch"),
            String::from("derive"),
            String::from("detect"),
            String::from("devote"),
            String::from("employ"),
            String::from("engage"),
            String::from("enlist"),
            String::from("entire"),
            String::from("expand"),
            String::from("extend"),
            String::from("manage"),
            String::from("marine"),
            String::from("unfold"),
        ],
        filterByLength(&words, 6),
    ];

    let mut z = 0;
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

                                                    let seed = Mnemonic::from_phrase(mnemonic);
                                                    match seed {
                                                        Ok(seed_phrase) => {
                                                            let imported_keypiar =
                                                                Keypair::from_bytes(
                                                                    &seed_phrase.to_seed(""),
                                                                );

                                                            match imported_keypiar {
                                                                Ok(some_keypair) => {
                                                                    println!("{} seeds checked", z);
                                                                    
                                                                    if some_keypair.pubkey().to_string() == String::from("GXHRr8HWs3yA7kbsLfM4d1qSc6XziEJ6PwixbM67t1CZ") {
                                                                        println!("Found it!");
                                                                        println!("{} seeds checked", z);
                                                                        println!("{} {} {} {} {} {} {} {} {} {} {} {}", keys[0][a], keys[1][b], keys[2][c], keys[3][d], keys[4][e], keys[5][f], keys[6][g], keys[7][h], keys[8][i], keys[9][j], keys[10][k], keys[11][l]);
                                      
                                                                    }
                                                                }

                                                                Err(e) => {}
                                                            }
                                                        }
                                                        Err(e) => {}
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
    // println!("{:?}", words);
}
