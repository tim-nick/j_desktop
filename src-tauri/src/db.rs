use rusqlite::{params, Connection, Result};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::error::AppError;
// use chrono::{DateTime, Utc};
use std::time::{Instant};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct EditorDocument {
    pub time: i64,
    pub blocks: Vec<Block>,
    pub version: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Block {
    pub id: String,
    pub r#type: String, // Use `r#type` to avoid the Rust keyword conflict
    pub data: serde_json::Value,
}

#[derive(Debug ,Serialize, Deserialize)]
pub struct Document {
    pub id: i64,
    pub title: String,
    pub time: String,
    pub content: String,
    pub folder_id: Option<i64>,  // Optional since a document may not belong to a folder
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Folder {
    pub id: i64,
    pub name: String,
    pub parent_id: Option<i64>,
    pub documents: Vec<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PythonBackendDocument {
    pub collection_name: String,
    pub name: String,
    pub title: String,
    pub filename: String,
    pub content: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct TimerSession {
    pub work_duration: i32,             // Work session duration in seconds
    pub break_duration: i32,            // Break session duration in seconds
    pub start_time_work: String,        // ISO 8601 formatted start time
    pub stop_time_work: String,         // ISO 8601 formatted stop time
    pub start_time_break: Option<String>, // Optional ISO 8601 start time for break
    pub stop_time_break: Option<String>,  // Optional ISO 8601 stop time for break
    pub extended: bool,                 // Indicates if the session was extended
    pub extended_start_time: Option<String>, // Optional ISO 8601 start time for the extension
    pub extended_stop_time: Option<String>,  // Optional ISO 8601 stop time for the extension
}

// #[derive(Serialize, Deserialize, Debug)]
// pub struct Calendar {

// }

pub fn extract_title(doc: &str) -> Option<String> {
    let json: Value = serde_json::from_str(doc).ok()?; // Parse JSON
    let first_block = json.get("blocks")?.get(0)?; // Get first block (index 0)
    let title = first_block.get("data")?.get("text")?.as_str()?; // Extract "text" field

    Some(title.to_string()) // Convert &str to String
}



pub fn create_python_document(doc: &Document) -> PythonBackendDocument {
    PythonBackendDocument {
        collection_name: "my_notes".to_string(), // You can adjust this to be dynamic if needed
        name: doc.title.clone(), // Use the title of the document as the name
        title: doc.title.clone(), // Use the title of the document
        filename: format!("{}.json", doc.title), // Create a filename based on the title
        content: doc.content.clone(), // Use the content directly from the Document
    }
}


pub fn save_document(conn: &Connection, doc: &EditorDocument, folderId: &i64) -> Result<(), AppError> {
    let doc_json = serde_json::to_string(&doc)?;
    // TODO call a get title fucntion fr
    println!("The fodler id : {}", folderId);
    println!("{}", doc_json);
    let title = extract_title(&doc_json);
    let title_str = match title {
        Some(t) => t, // Extract the value
        none => "No title found".to_string(), // Provide a default string
    };
    println!("Found title: {:?}", &title_str);
    conn.execute(
        "INSERT INTO documents (title, time, content, folder_id) VALUES (?, ?, ?, ?)",
        params![&title_str, &doc.time, &doc_json, &folderId],
    )?;
    println!("Saved doc");
    Ok(())
}

pub fn load_document_for_editor(conn: &Connection, id: i64) -> Result<EditorDocument, AppError> {
    println!("Loading Doc with id: {} ...", id);
    // Prepare the statement
    let mut stmt = conn.prepare("SELECT id, title, time, content, folder_id FROM documents WHERE id = ?1")?;
    
    // Execute query and retrieve the row
    let doc: EditorDocument = stmt.query_row(params![id], |row| {
        let content_json: String = row.get(3)?;
        println!("Got follwing data from db {}", content_json);
        
        // Deserialize JSON string into EditorDocument
        let content: EditorDocument = serde_json::from_str(&content_json).map_err(|e| {
            rusqlite::Error::FromSqlConversionFailure(0, rusqlite::types::Type::Text, Box::new(e))
        })?;
        println!("Returning data from load command ");
        Ok(content)
    }).map_err(|e| {
        println!("Error executing query: {}", e);
        AppError::SqliteError(e)
    })?;
    println!("Returned data to load command ");
    // Return the document
    Ok(doc)
}

pub fn load_documents(conn: &Connection) -> Result<Vec<Document>, AppError> {
    let mut stmt = conn.prepare("SELECT id, title, time, content, folder_id FROM documents")?;
    let docs = stmt.query_map([], |row| {
        // Assuming the Document struct fields align with the query results
        let content_json: String = row.get(3)?;
        Ok(Document {
            id: row.get(0)?,
            title: row.get(1)?,
            time: row.get(2)?,
            content: content_json,
            folder_id: row.get(4)?,
        })
    })?
    .collect::<Result<Vec<Document>, rusqlite::Error>>()?;
    
    Ok(docs)
}


pub fn load_document(conn: &Connection, id: i64) -> Result<Document, AppError> {
    let mut stmt = conn.prepare("SELECT id, title, time, content, folder_id FROM documents WHERE id = ?1")?;
    
    let doc = stmt.query_row([id], |row| {
        let content_json: String = row.get(3)?;
        Ok(Document {
            id: row.get(0)?,
            title: row.get(1)?,
            time: row.get(2)?,
            content: content_json,
            folder_id: row.get(4)?,
        })
    })?;

    Ok(doc)
}


pub fn load_folders(conn: &Connection) -> Result<Vec<Folder>, rusqlite::Error> {
    let mut stmt = conn.prepare("SELECT id, name, parent_id FROM folders")?; // Fixed query to include parent_id

    let folders = stmt
        .query_map([], |row| {
            let id: i64 = row.get(0)?;
            let name: String = row.get(1)?;
            let parent_id: Option<i64> = row.get(2)?; // Handle NULL values properly

            // Fetch document IDs associated with this folder
            let mut doc_stmt = conn.prepare("SELECT id FROM documents WHERE folder_id = ?")?;
            let document_ids = doc_stmt
                .query_map([id], |doc_row| Ok(doc_row.get(0)?))?
                .collect::<Result<Vec<i64>, rusqlite::Error>>()?;

            Ok(Folder {
                id,
                name,
                parent_id,
                documents: document_ids,
            })
        })?
        .collect::<Result<Vec<Folder>, rusqlite::Error>>()?;

    Ok(folders)
}

pub fn gen_side_bar_list(conn: &Connection) -> Result<Vec<Document>, AppError> {
    let mut stmt = conn.prepare("SELECT id, title, time, content, folder_id FROM documents")?;
    let docs = stmt.query_map([], |row| {
        let content_json: String = row.get(3)?;
        Ok(Document {
            id: row.get(0)?,
            title: row.get(1)?,
            time: row.get(2)?,
            content: content_json,
            folder_id: row.get(4)?,  // Add this line to include folder_id
        })
    })?
    .collect::<Result<Vec<Document>, rusqlite::Error>>()?;

    Ok(docs)
}

pub fn update_document(conn: &Connection, id: i64, new_doc: &Document) -> Result<(), AppError> {
    // Print the new document content for debugging
    println!("Updating document with ID: {}", id);
    println!("New title: {}", &new_doc.title);
    println!("New time: {}", &new_doc.time);
    println!("New content: {}", &new_doc.content);
    println!("New folder_id: {:?}", &new_doc.folder_id);

    // Convert folder_id properly for SQLite (if present)
    let folder_id_value = new_doc.folder_id.map(|v| v as i64);

    let sql_query = if folder_id_value.is_some() {
        // If `folder_id` is `Some`, include it in the update
        "UPDATE documents SET title = ?, time = ?, content = ?, folder_id = ? WHERE id = ?"
    } else {
        // If `folder_id` is `None`, do NOT update it (omit `folder_id` from the query)
        "UPDATE documents SET title = ?, time = ?, content = ? WHERE id = ?"
    };

    let rows_affected = if let Some(folder_id) = folder_id_value {
        // Execute query with folder_id when it's Some(value)
        conn.execute(sql_query, params![&new_doc.title, &new_doc.time, &new_doc.content, folder_id, &id])?
    } else {
        // Execute query without folder_id when it's None
        conn.execute(sql_query, params![&new_doc.title, &new_doc.time, &new_doc.content, &id])?
    };

    // Log the number of rows affected
    println!("Rows affected: {}", rows_affected);

    // Check if the update was successful
    if rows_affected == 0 {
        println!("Warning: No document found with ID: {}", id);
    } else {
        println!("Document updated successfully.");
    }

    Ok(())
}



// Function to insert a new folder with an optional parent_id
pub fn insert_new_folder(conn: &Connection, name: &str, parent_id: Option<i64>) -> Result<(), rusqlite::Error> {
    conn.execute(
        "INSERT INTO folders (name, parent_id) VALUES (?, ?)",
        params![name, parent_id], // Corrected query
    )?;
    Ok(())
}

pub fn save_timer_session(conn: &Connection, session: &TimerSession) -> Result<(), AppError> {
    conn.execute(
        "INSERT INTO timer_sessions (
            work_duration, 
            break_duration, 
            start_time_work, 
            stop_time_work, 
            start_time_break, 
            stop_time_break, 
            extended, 
            extended_start_time, 
            extended_stop_time
        ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
        params![
            session.work_duration,
            session.break_duration,
            session.start_time_work,
            session.stop_time_work,
            session.start_time_break,
            session.stop_time_break,
            session.extended,
            session.extended_start_time,
            session.extended_stop_time
        ],
    ).map_err(AppError::SqliteError)?;

    println!("Timer session saved successfully.");
    Ok(())
}