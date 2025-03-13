// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::os::unix::process::parent_id;
use std::process::{Command, Stdio, Child};
use dotenv::from_filename;
use std::env;
use std::path::Path;
use std::thread;
use std::sync::Arc;
use db::{EditorDocument, Document, Folder,  PythonBackendDocument, TimerSession, save_document,load_document, load_document_for_editor, gen_side_bar_list, update_document,  load_documents, insert_new_folder, load_folders, save_timer_session, extract_title};
use tauri::command;
use error::AppError;
use std::fs;
use libsql::{Connection, Builder};
use reqwest::Client;
use serde_json::json;
use serde_json::Value;
use tauri::async_runtime::spawn;
use tokio::sync::Mutex;

mod db;
mod error;

struct DbState {
    conn: Arc<Mutex<Connection>>,
}

// const DB_PATH: &str = "../sqlite_database/documents.db";
// const DB_PATH: &str = "/Users/tim/Documents/Programming/Projects/J.A.R.V.I.S./database/database.db";
// const DB_PATH: &str = "/Users/tim/Documents/Programming/Projects/personal_assistant/knowledge-graph/pa_db.db";
// const DB_PATH: &str = "/Users/tim/Documents/Programming/Projects/personal_assistant/knowledge-graph/test_data.db";
// TODO: currently gets foldername as input not filenames fix it to get file isa or file names 
#[tauri::command]
async fn folder_clicked(name: String, state: tauri::State<'_, DbState>) -> Result<EditorDocument, String> {
    println!("Retrieving doc command ");
    println!("{}", &name);
    
    // Attempt to parse the string into an i64
    let num = match name.parse::<i64>() {
        Ok(num) => {
            println!("Converted number: {}", num);
            num
        }
        Err(e) => return Err(format!("Failed to convert: {}", e)),
    };

    let conn_guard = state.conn.lock().await;
    let conn = &*conn_guard;
    
    // Call load_document_for_editor only if conversion succeeded
    load_document_for_editor(conn, num).await.map_err(|e| e.to_string())
}


#[tauri::command]
async fn fetch_documents_command(state: tauri::State<'_, DbState>) -> Result<Vec<Document>, String> {
    println!("Executing load document command");
    let conn_guard = state.conn.lock().await;
    let conn = &*conn_guard;
    
    load_documents(conn).await.map_err(|e| e.to_string())
}

#[tauri::command]
async fn fetch_folders_command(state: tauri::State<'_, DbState>) -> Result<Vec<Folder>, String> {
    println!("Executing load folders command");
    
    let conn_guard = state.conn.lock().await;
    let conn = &*conn_guard;
    
    load_folders(conn).await.map_err(|e| {
        eprintln!("Error loading folders: {}", e);
        e.to_string()
    })
}



#[tauri::command]
async fn create_new_folder_command(name: String, parent_id: Option<i64>, state: tauri::State<'_, DbState>) -> Result<(), String> {
    println!("Received in Rust -> name: '{}', parent_id: {:?}", name, parent_id);
    
    let conn_guard = state.conn.lock().await;
    let conn = &*conn_guard;
    
    insert_new_folder(conn, &name, parent_id).await.map_err(|e| e.to_string())?;
    
    Ok(())
}



#[tauri::command]
async fn save_document_command(doc: EditorDocument, folderId: i64, state: tauri::State<'_, DbState>) -> Result<(), String> {
    println!("Executing save document command");
    let conn_guard = state.conn.lock().await;
    let conn = &*conn_guard;
    
    save_document(conn, &doc, &folderId).await.map_err(|e| e.to_string())?;

    // Use async spawn to handle the asynchronous request to the Python backend
    
    Ok(())
}

#[tauri::command]
async fn load_document_command(id: i64, state: tauri::State<'_, DbState>) -> Result<EditorDocument, String> {
    println!("Executing load document command");
    let conn_guard = state.conn.lock().await;
    let conn = &*conn_guard;
    
    load_document_for_editor(conn, id).await.map_err(|e| e.to_string())
}

#[tauri::command]
async fn gen_side_bar_list_command(state: tauri::State<'_, DbState>) -> Result<Vec<Document>, String> {
    println!("gen_side_bar_list command");
    let conn_guard = state.conn.lock().await;
    let conn = &*conn_guard;
    
    gen_side_bar_list(conn).await.map_err(|e| e.to_string())
}

#[tauri::command]
async fn update_document_command(id: i64, doc: EditorDocument, folderId: Option<i64>, state: tauri::State<'_, DbState>) -> Result<(), String> {
    println!("update_document_command");
    
    // Open database connection
    let conn_guard = state.conn.lock().await;
    let conn = &*conn_guard;
    
    // Convert `doc` to JSON
    let doc_json = match serde_json::to_string(&doc) {  
        Ok(json) => json,
        Err(e) => return Err(format!("Failed to serialize document: {}", e)),
    };
    
    println!("{}", &doc_json);
    
    // Extract title from JSON
    let extracted_title = extract_title(&doc_json).unwrap_or_else(|| "Untitled".to_string());

    // Create a `Document` struct instance with extracted title and provided folder ID
    let db_doc: Document = Document {
        id: id,
        title: extracted_title,  // Dynamically extracted from JSON
        time: doc.time.to_string(),
        content: doc_json,
        folder_id: folderId,  // Passed in from function argument
    };

    // Call `update_document` function
    update_document(conn, id, &db_doc).await.map_err(|e| e.to_string())
}

#[tauri::command]
async fn save_timer_session_command(session: TimerSession, state: tauri::State<'_, DbState>) -> Result<(), String> {
    println!("Executing save timer session command");
     // Log the incoming session data for debugging
     println!("Save Session: {:?}", session);
    let conn_guard = state.conn.lock().await;
    let conn = &*conn_guard;
    
    save_timer_session(conn, &session).await.map_err(|e| e.to_string())?;
    println!("Timer session saved successfully.");
    Ok(())
}


// New asynchronous version of initialize_database using libsql

async fn initialize_database(conn: &Connection) -> Result<(), Box<dyn std::error::Error>> {
    println!("Creating database tables if they don't exist...");
    
    // Enable foreign key support
    conn.execute("PRAGMA foreign_keys = ON", ()).await?;
    
    // Create the folders table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS folders (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            parent_id INTEGER
        )",
        ()
    ).await?;
    
    // Create the documents table with a foreign key to folders(id)
    conn.execute(
        "CREATE TABLE IF NOT EXISTS documents (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            title TEXT NOT NULL,
            time TEXT NOT NULL,
            content TEXT NOT NULL,
            folder_id INTEGER,
            FOREIGN KEY(folder_id) REFERENCES folders(id) ON DELETE SET NULL
        )",
        ()
    ).await?;
    
    // Create the timer_sessions table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS timer_sessions (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            work_duration INTEGER NOT NULL,
            break_duration INTEGER NOT NULL,
            start_time_work TEXT NOT NULL,
            stop_time_work TEXT NOT NULL,
            start_time_break TEXT,
            stop_time_break TEXT,
            extended BOOLEAN NOT NULL,
            extended_start_time TEXT,
            extended_stop_time TEXT
        )",
        ()
    ).await?;
    
    println!("Database initialized successfully.");
    Ok(())
}

fn main() {
    // Create the connection asynchronously
    let conn = tauri::async_runtime::block_on(async {
        let primary_url = "http://127.0.0.1:9090";
        let auth_token = "";

        // TODO: implened snyc mechnism between local  an online mode with 
        
        // Always create a remote replica connection with offline queuing support
        let db = Builder::new_remote_replica(
            "/Users/tim/Documents/Programming/Projects/jarvis_note/j_desktop/client_db/local.db",
            primary_url.to_string(),
            auth_token.to_string()
        )
        .sync_interval(std::time::Duration::from_secs(60))
        .build()
        .await
        .unwrap_or_else(|e| {
            println!("Error building remote replica: {:?}. Falling back to local-only database.", e);
            // Log the error and fallback to a local-only connection if remote replica creation fails
            tauri::async_runtime::block_on(async {
                Builder::new_local("/Users/tim/Documents/Programming/Projects/jarvis_note/j_desktop/client_db/local.db")
                    .build()
                    .await
                    .expect("Failed to build local database")
            })
        });

        // Establish connection to the database
        let conn = db.connect().expect("Failed to connect to db");
        println!("CREATED DATABASE!");

        // Initialize the database schema (creates tables if they don't exist)
        if let Err(e) = initialize_database(&conn).await {
            // Log any schema initialization errors (including WriteDelegation errors in offline mode) without interrupting connection creation
            println!("Schema initialization error (possibly due to offline mode): {:?}", e);
        }

        conn
    });

    println!("Database connection established and initialized.");

    // Wrap the connection in Arc and Mutex as before
    let db_state = DbState {
        conn: Arc::new(Mutex::new(conn)),
    };

    tauri::Builder::default()
        .manage(db_state)  // Register the shared state
        .invoke_handler(tauri::generate_handler![
            save_document_command,
            load_document_command,
            gen_side_bar_list_command,
            update_document_command,
            create_new_folder_command,
            fetch_documents_command,
            fetch_folders_command,
            save_timer_session_command,
            folder_clicked
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}