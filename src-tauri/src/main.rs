#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::error::Error;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Record {
    id: String,
    value: String
}

#[allow(non_snake_case)]
#[tauri::command]
fn add_record(recordName: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", recordName)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![add_record])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// Updates the database using the current entry 
fn updateDB (key: String, value: String) -> Result<(), Box<dyn Error>>{
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_path("/tmp/tpwman.db")?;
    

    for result in rdr.deserialize() {
        let record: Record = result?;
        println!("{:?}", record); // temporary print all records to see if it works
    }

    Ok(())
}

// Remove a record from the database
fn removeRecordDB (key: String) -> Result<(), Box<dyn Error>>{
    Ok(())
}