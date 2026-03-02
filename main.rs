use chrono::{Local, Timelike, Duration as ChronoDuration};
use std::env;
use std::io::{self, Write};

fn main() {
    let n: u64 = 987654323;
    let e_base: u64 = 17;
    let args: Vec<String> = env::args().collect();

    let commandes = [
        ("PLAY/PAUSE", 100),
        ("NEXT", 200),
        ("PREVIOUS", 300),
        ("VOL_UP", 400),
        ("VOL_DOWN", 500),
    ];

    // --- 1. REFERENCE MODE (When you run it manually in Termux) ---
    if args.len() < 2 {
        let now = Local::now();
        let time_seed = (now.hour() as u64 * 100) + now.minute() as u64;
        let e_dynamic = e_base + time_seed;
        
        println!("--- CODES RÉFÉRENCE ({:02}:{:02}) ---", now.hour(), now.minute());
        println!("Seed actuelle: {}", time_seed);
        
        for (nom, m) in commandes.iter() {
            let code = mod_pow(*m, e_dynamic, n);
            println!("{:<12} : {:09}", nom, code);
        }
        return;
    }

    // --- 2. MACRODROID MODE (When called with a message) ---
    let input = &args[1];
    let code_recu_str: String = input.chars().filter(|c| c.is_digit(10)).collect();
    let code_recu: u64 = code_recu_str.parse().unwrap_or(0);

    let now = Local::now();
    // On vérifie Minute -1, Minute Actuelle, et Minute +1
    let times_to_check = [
        now - ChronoDuration::minutes(1),
        now, 
        now + ChronoDuration::minutes(1)
    ];

    let mut stdout = io::stdout();

    for time in times_to_check.iter() {
        let time_seed = (time.hour() as u64 * 100) + time.minute() as u64;
        let e_dynamic = e_base + time_seed;

        for (nom, m) in commandes.iter() {
            if code_recu == mod_pow(*m, e_dynamic, n) {
                // On écrit SEULEMENT le nom pour MacroDroid
                write!(stdout, "{}", nom).unwrap();
                stdout.flush().unwrap();
                return; 
            }
        }
    }

    // Si rien ne correspond
    write!(stdout, "NOMATCH").unwrap();
    stdout.flush().unwrap();
}

fn mod_pow(base: u64, mut exp: u64, modulus: u64) -> u64 {
    let mut res: u128 = 1;
    let mut b: u128 = (base as u128) % (modulus as u128);
    let m: u128 = modulus as u128;
    while exp > 0 {
        if exp % 2 == 1 { res = (res * b) % m; }
        b = (b * b) % m;
        exp /= 2;
    }
    res as u64
}
