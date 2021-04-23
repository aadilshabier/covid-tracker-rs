mod functions;

use chrono::Local;
use error_chain::error_chain;
use functions::{best_string_match, reorder_data};
use io::Write;
use std::collections::HashMap;
use std::fs;
use std::io;
use std::path::PathBuf;

error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
        Serde(serde_json::Error);
    }
}

const URL: &str = "https://api.covid19india.org/states_daily.json";
const HOME: &str = std::env!("HOME");

fn main() -> Result<()> {
    // create dir if it does not exists
    let cache_home: PathBuf = [HOME, ".cache", "covid"].iter().collect();
    if !cache_home.is_dir() {
        fs::create_dir(&cache_home)?;
    };

    let cachedir = cache_home.join("cache.json");
    let datadir = cache_home.join("states_daily.json");

    let today = Local::today().format("%Y-%m-%d").to_string();

    // load cache from the cache directory or load a default
    let mut contents =
        fs::read_to_string(&cachedir).unwrap_or("{ \"last_loaded\" : \"\" }".to_string());
    let mut cache: HashMap<String, String> = serde_json::from_str(&contents)?;

    // load from the internet only if today's value isn't available
    let states_daily: HashMap<String, Vec<HashMap<String, String>>>;

    if cache["last_loaded"] == today {
        // Downloads json if file not found
        contents = fs::read_to_string(&datadir).unwrap_or_else(|_| {
            let contents = reqwest::blocking::get(URL)
                .expect("Could not connect to API")
                .text()
                .unwrap();
            fs::write(&datadir, &contents).unwrap();
            contents
        });
    } else {
        contents = reqwest::blocking::get(URL)
            .expect("Could not connect to API")
            .text()
            .unwrap();
        fs::write(&datadir, &contents).unwrap();
    }
    // TODO: Make safe
    states_daily = serde_json::from_str(&contents)?;

    let length = (&states_daily["states_daily"]).len();

    // Creates new Vec with only todays data, drops the rest and reorder data
    let todays_cases = (&states_daily["states_daily"][length - 3..]).to_vec();
    drop(states_daily);

    // Get the latest date from states_daily.json and save to cache.json
    let last_loaded = (&todays_cases[2]["dateymd"]).clone();
    *cache
        .entry("last_loaded".to_string())
        .or_insert("".to_string()) = last_loaded.clone();
    fs::write(&cachedir, serde_json::to_string_pretty(&cache).unwrap())?;

    // reorder data to a better format
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

        buffer = buffer.trim().to_string();
        if buffer.to_lowercase() == "q" {
            println!("Stay safe!\nThank you :)");
            return Ok(());
        } else if buffer.len() != 0 {
            let best_match = best_string_match(&buffer, todays_cases.keys());

            let result = todays_cases.get(&best_match).unwrap();
            println!("{} on {}:", best_match, last_loaded);
            println!("  Confirmed: {}", result[0]);
            println!("  Recovered: {}", result[1]);
            println!("  Deceased: {}", result[2]);
            stdin.read_line(&mut buffer)?;
            buffer.clear();
        }
    }
}
