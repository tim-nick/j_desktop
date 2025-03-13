use libsql::{params, Connection};

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

pub async fn save_document(conn: &Connection, doc: &EditorDocument, folderId: &i64) -> Result<(), AppError> {
    let doc_json = serde_json::to_string(&doc)?;
    println!("The folder id : {}", folderId);
    println!("{}", doc_json);
    let title = extract_title(&doc_json);
    let title_str = match title {
        Some(t) => t,
        None => "No title found".to_string(),
    };
    println!("Found title: {:?}", &title_str);
    conn.execute(
        "INSERT INTO documents (title, time, content, folder_id) VALUES (?1, ?2, ?3, ?4)",
        params![title_str.as_str(), doc.time, doc_json.as_str(), *folderId]
    ).await.map_err(AppError::LibsqlError)?;
    println!("Saved doc");
    Ok(())
}

pub async fn load_document_for_editor(conn: &Connection, id: i64) -> Result<EditorDocument, AppError> {
    println!("Loading Doc with id: {} ...", id);

    let mut rows = conn.query("SELECT id, title, time, content, folder_id FROM documents WHERE id = ?1", params![id])
        .await
        .map_err(AppError::LibsqlError)?;

    if let Some(row) = rows.next().await.map_err(AppError::LibsqlError)? {
        let content_json: String = row.get(3).map_err(AppError::LibsqlError)?;
        println!("Got following data from db {}", content_json);

        let content: EditorDocument = serde_json::from_str(&content_json)
            .map_err(|e| AppError::Custom(e.to_string()))?;
        println!("Returning data from load command");
        Ok(content)
    } else {
        Err(AppError::Custom("Document not found".to_string()))
    }
}

pub async fn load_documents(conn: &Connection) -> Result<Vec<Document>, AppError> {
    let mut rows = conn.query("SELECT id, title, time, content, folder_id FROM documents", ())
        .await
        .map_err(AppError::LibsqlError)?;

    let mut docs = Vec::new();

    while let Some(row) = rows.next().await.map_err(AppError::LibsqlError)? {
        let content_json: String = row.get(3).map_err(AppError::LibsqlError)?;
        docs.push(Document {
            id: row.get(0).map_err(AppError::LibsqlError)?,
            title: row.get(1).map_err(AppError::LibsqlError)?,
            time: row.get(2).map_err(AppError::LibsqlError)?,
            content: content_json,
            folder_id: row.get(4).map_err(AppError::LibsqlError)?,
        });
    }

    Ok(docs)
}

pub async fn load_document(conn: &Connection, id: i64) -> Result<Document, AppError> {
    let mut rows = conn.query("SELECT id, title, time, content, folder_id FROM documents WHERE id = ?1", params![id])
        .await
        .map_err(AppError::LibsqlError)?;

    if let Some(row) = rows.next().await.map_err(AppError::LibsqlError)? {
        let content_json: String = row.get(3).map_err(AppError::LibsqlError)?;
        Ok(Document {
            id: row.get(0).map_err(AppError::LibsqlError)?,
            title: row.get(1).map_err(AppError::LibsqlError)?,
            time: row.get(2).map_err(AppError::LibsqlError)?,
            content: content_json,
            folder_id: row.get(4).map_err(AppError::LibsqlError)?,
        })
    } else {
        Err(AppError::Custom("Document not found".to_string()))
    }
}

pub async fn load_folders(conn: &Connection) -> Result<Vec<Folder>, AppError> {
    let mut rows = conn.query("SELECT id, name, parent_id FROM folders", ())
        .await
        .map_err(AppError::LibsqlError)?;

    let mut folders = Vec::new();

    while let Some(row) = rows.next().await.map_err(AppError::LibsqlError)? {
        let id: i64 = row.get(0).map_err(AppError::LibsqlError)?;
        let name: String = row.get(1).map_err(AppError::LibsqlError)?;
        let parent_id: Option<i64> = row.get(2).map_err(AppError::LibsqlError)?;

        let mut doc_rows = conn.query("SELECT id FROM documents WHERE folder_id = ?1", params![id])
            .await
            .map_err(AppError::LibsqlError)?;
        let mut document_ids = Vec::new();
        while let Some(doc_row) = doc_rows.next().await.map_err(AppError::LibsqlError)? {
            document_ids.push(doc_row.get(0).map_err(AppError::LibsqlError)?);
        }

        folders.push(Folder {
            id,
            name,
            parent_id,
            documents: document_ids,
        });
    }

    Ok(folders)
}

pub async fn gen_side_bar_list(conn: &Connection) -> Result<Vec<Document>, AppError> {
    let mut rows = conn.query("SELECT id, title, time, content, folder_id FROM documents", ())
        .await
        .map_err(AppError::LibsqlError)?;

    let mut docs = Vec::new();

    while let Some(row) = rows.next().await.map_err(AppError::LibsqlError)? {
        let content_json: String = row.get(3).map_err(AppError::LibsqlError)?;
        docs.push(Document {
            id: row.get(0).map_err(AppError::LibsqlError)?,
            title: row.get(1).map_err(AppError::LibsqlError)?,
            time: row.get(2).map_err(AppError::LibsqlError)?,
            content: content_json,
            folder_id: row.get(4).map_err(AppError::LibsqlError)?,
        });
    }

    Ok(docs)
}

pub async fn update_document(conn: &Connection, id: i64, new_doc: &Document) -> Result<(), AppError> {
    println!("Updating document with ID: {}", id);
    println!("New title: {}", &new_doc.title);
    println!("New time: {}", &new_doc.time);
    println!("New content: {}", &new_doc.content);
    println!("New folder_id: {:?}", &new_doc.folder_id);

    let sql_query = if new_doc.folder_id.is_some() {
        "UPDATE documents SET title = ?1, time = ?2, content = ?3, folder_id = ?4 WHERE id = ?5"
    } else {
        "UPDATE documents SET title = ?1, time = ?2, content = ?3 WHERE id = ?4"
    };

    let rows_affected = if let Some(folder_id) = new_doc.folder_id {
        conn.execute(sql_query, params![new_doc.title.as_str(), new_doc.time.as_str(), new_doc.content.as_str(), folder_id, id])
            .await
            .map_err(AppError::LibsqlError)?
    } else {
        conn.execute(sql_query, params![new_doc.title.as_str(), new_doc.time.as_str(), new_doc.content.as_str(), id])
            .await
            .map_err(AppError::LibsqlError)?
    };

    println!("Rows affected: {}", rows_affected);

    if rows_affected == 0 {
        println!("Warning: No document found with ID: {}", id);
    } else {
        println!("Document updated successfully.");
    }

    Ok(())
}

pub async fn insert_new_folder(conn: &Connection, name: &str, parent_id: Option<i64>) -> Result<(), AppError> {
    conn.execute(
        "INSERT INTO folders (name, parent_id) VALUES (?1, ?2)",
        params![name, parent_id]
    ).await.map_err(AppError::LibsqlError)?;
    Ok(())
}

pub async fn save_timer_session(conn: &Connection, session: &TimerSession) -> Result<(), AppError> {
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
            session.start_time_work.as_str(),
            session.stop_time_work.as_str(),
            session.start_time_break.as_ref().map(|s| s.as_str()),
            session.stop_time_break.as_ref().map(|s| s.as_str()),
            session.extended,
            session.extended_start_time.as_ref().map(|s| s.as_str()),
            session.extended_stop_time.as_ref().map(|s| s.as_str())
        ]
    ).await.map_err(AppError::LibsqlError)?;

    println!("Timer session saved successfully.");
    Ok(())
}