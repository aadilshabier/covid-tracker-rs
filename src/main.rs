mod functions;

use chrono::Local;
use error_chain::error_chain;
use io::Write;
use functions::{best_string_match, reorder_data};
use std::collections::HashMap;
use std::fs;
use std::io;

error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
        Serde(serde_json::Error);
    }
}

const URL: &str = "https://api.covid19india.org/states_daily.json";

const CACHEDIR: &str = "cache/cache.json";
const DATADIR: &str = "cache/states_daily.json";

fn main() -> Result<()> {
    let today = Local::today().format("%Y-%m-%d").to_string();

    // load cache from the cache directory or load a default
    let contents = fs::read_to_string(CACHEDIR)?;
    let mut cache: HashMap<String, String> = serde_json::from_str(&contents).unwrap_or_else(|_| {
        let mut result = HashMap::new();
        result.insert("last_loaded".to_string(), "".to_string());
        result
    });

    // load from the internet only if today's value isn't available
    let states_daily: HashMap<String, Vec<HashMap<String, String>>>;

    if cache["last_loaded"] == today {
        // Downloads json if file not found
        let contents = fs::read_to_string(DATADIR)?;
        states_daily = serde_json::from_str(&contents)
            .unwrap_or_else(|_| reqwest::blocking::get(URL).unwrap().json().unwrap());
    } else {
        states_daily = reqwest::blocking::get(URL)?.json()?;
    }

    let length = (&states_daily["states_daily"]).len();

    // Creates new Vec with only todays data and drops the rest
    let todays_cases = (&states_daily["states_daily"][length - 3..]).to_vec();
    drop(states_daily);

    let last_loaded = (&todays_cases[0]["dateymd"]).clone();
    *cache
        .entry("last_loaded".to_string())
        .or_insert("".to_string()) = last_loaded.clone();
    fs::write(CACHEDIR, serde_json::to_string_pretty(&cache).unwrap())?;

    let todays_cases = reorder_data(todays_cases);

    let mut buffer = String::new();

    let stdin = io::stdin();
    let mut stdout = io::stdout();
    loop {
        println!("\nEnter choice (q to quit):");
        print!("> ");
        stdout.flush()?;

        stdin.read_line(&mut buffer)?;
        println!();

        buffer = buffer.trim().to_lowercase();
        if buffer == "q" {
            println!("Stay safe!\nThank you :)");
            return Ok(());
        } else {
            let best_match = best_string_match(&buffer, todays_cases.keys());

            let result = todays_cases.get(&best_match).unwrap();
            println!("{} on {}:", best_match, last_loaded);
            println!("  Confirmed: {}", result[0]);
            println!("  Recovered: {}", result[1]);
            println!("  Deceased: {}", result[2]);
        }

        stdout.flush()?;
        stdin.read_line(&mut buffer)?;
        buffer.clear();
    }
}
