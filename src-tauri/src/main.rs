// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::process::{Command, Stdio, Child};
use dotenv::from_filename;
use std::env;
use std::path::Path;
use std::thread;
use std::sync::{Arc, Mutex};
use rusqlite::{Connection, Error as RusqliteError};

use db::{EditorDocument, Document, Folder, save_document, load_document, gen_side_bar_list, update_document,  load_documents, insert_new_folder, load_folders};
use tauri::command;
use error::AppError;
use std::fs;


mod db;
mod error;



// const DB_PATH: &str = "../sqlite_database/documents.db";
const DB_PATH: &str = "/Users/tim/Documents/Programming/Projects/J.A.R.V.I.S./database/database.db";

#[tauri::command]
fn fetch_documents_command() -> Result<Vec<Document>, String> {
    println!("Executing load document command");
    let conn = Connection::open(DB_PATH).map_err(|e| e.to_string())?;
    load_documents(&conn).map_err(|e| e.to_string())
}

#[tauri::command]
fn fetch_folders_command() -> Result<Vec<Folder>, String> {
    println!("Executing load folders command");
    let conn = Connection::open(DB_PATH).map_err(|e| e.to_string())?;
    load_folders(&conn).map_err(|e| e.to_string())
}

#[tauri::command]
fn save_document_command(doc: EditorDocument) -> Result<(), String> {
    println!("Executing save document command");
    let conn = Connection::open(DB_PATH).map_err(|e| e.to_string())?;
    save_document(&conn, &doc).map_err(|e| e.to_string())
}

#[tauri::command]
fn load_document_command(id: i64) -> Result<EditorDocument, String> {
    println!("Executing load document command");
    let conn = Connection::open(DB_PATH).map_err(|e| e.to_string())?;
    load_document(&conn, id).map_err(|e| e.to_string())
}

#[tauri::command]
fn gen_side_bar_list_command() -> Result<Vec<Document>, String> {
    println!("gen_side_bar_list command");
    let conn = Connection::open(DB_PATH).map_err(|e| e.to_string())?;
    gen_side_bar_list(&conn).map_err(|e| e.to_string())
}

#[tauri::command]
fn update_document_command(id: i64, doc: EditorDocument) -> Result<(), String> {
    println!("update_document_command");
    let conn = Connection::open(DB_PATH).map_err(|e| e.to_string())?;
    // let doc_json = serde_json::to_string(&doc)?;
    let doc_json = match serde_json::to_string(&doc) {  
    Ok(json) => json,
    Err(e) => e.to_string(),
    };
    println!("{}", &doc_json);

    let db_doc: Document = Document {
        id: id,
        title: ("TestTitle").to_string(),
        time: (&doc.time).to_string(),
        content: doc_json,
        folder_id: None,  // Set folder_id to None or provide the appropriate folder_id
    };
    update_document(&conn, id, &db_doc).map_err(|e| e.to_string())
}

#[tauri::command]
fn create_new_folder_command(name: String)-> Result<(), String>{
    println!("create new Folder command");
    // let conn = Connection::open(DB_PATH).map_err(AppError::SqliteError)?;
    let conn = Connection::open(DB_PATH).map_err(|e| e.to_string())?;
    insert_new_folder(&conn, &name).map_err(|e| e.to_string());
    Ok(())
}

fn initialize_database() -> Result<(), AppError> {
    let path = Path::new(DB_PATH).parent().unwrap();
    println!("Creating directory if it doesn't exist: {:?}", path);

    let conn = Connection::open(DB_PATH).map_err(AppError::SqliteError)?;

    println!("Creating documents and folders tables if they don't exist...");
    conn.execute(
        "CREATE TABLE IF NOT EXISTS folders (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL
        )",
        [],
    ).map_err(AppError::SqliteError)?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS documents (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            title TEXT NOT NULL,
            time TEXT NOT NULL,
            content TEXT NOT NULL,
            folder_id INTEGER,
            FOREIGN KEY(folder_id) REFERENCES folders(id) ON DELETE SET NULL
        )",
        [],
    ).map_err(AppError::SqliteError)?;

    println!("Database initialized successfully.");
    Ok(())
}


fn main() {
    if let Err(e) = initialize_database() {
        println!("Failed to initialize database: {:?}", e);
    }

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            save_document_command,
            load_document_command,
            gen_side_bar_list_command,
            update_document_command,
            create_new_folder_command,
            fetch_documents_command,
            fetch_folders_command
        ])  
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}





// fn main() {
//     // Load .env variables from the correct path
//     let env_path = Path::new("../.env");
//     from_filename(env_path).expect("Failed to load .env file");

//     // Define the path to the start.sh script for the python backend
//     let script_path = "../backend/start.sh";

//     // Use Arc and Mutex to safely share the child process handle between threads
//     let child_process: Arc<Mutex<Option<Child>>> = Arc::new(Mutex::new(None));
//     let child_process_clone = Arc::clone(&child_process);

//     // Start the script in a new thread to run it asynchronously
//     thread::spawn(move || {
//         let mut child = Command::new("bash")
//             .arg(script_path)
//             .envs(env::vars()) // Pass all environment variables, including those from .env
//             .stdout(Stdio::inherit())  // Inherit stdout to see script output in the console
//             .stderr(Stdio::inherit())  // Inherit stderr for errors
//             .spawn()  // Use spawn instead of output to run the process in the background
//             .expect("Failed to execute start.sh script");

//         // Store the child process handle in the Arc<Mutex<Option<Child>>>
//         *child_process_clone.lock().unwrap() = Some(child);
//     });

//     // Run the Tauri app
//     tauri::Builder::default()
//         .on_window_event(move |event| {
//             if let tauri::WindowEvent::CloseRequested { .. } = event.event() {
//                 // When the window is closed, terminate the child process
//                 if let Some(mut child) = child_process.lock().unwrap().take() {
//                     if let Err(e) = child.kill() {
//                         eprintln!("Failed to kill child process: {}", e);
//                     } else {
//                         println!("Child process for python backend terminated");
//                     }
//                 }
//             }
//         })
//         .run(tauri::generate_context!())
//         .expect("error while running tauri application");
// }


// fn main() {
//   tauri::Builder::default()
//     .run(tauri::generate_context!())
//     .expect("error while running tauri application");
// }