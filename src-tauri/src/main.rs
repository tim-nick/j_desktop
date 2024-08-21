// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::process::{Command, Stdio, Child};
use dotenv::from_filename;
use std::env;
use std::path::Path;
use std::thread;
use std::sync::{Arc, Mutex};

fn main() {
  tauri::Builder::default()
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