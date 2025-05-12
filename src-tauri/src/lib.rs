// // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
// #[tauri::command]
// fn greet(name: &str) -> String {
//     format!("Hello, {}! You've been greeted from Rust!", name)
// }
//
// #[cfg_attr(mobile, tauri::mobile_entry_point)]
// pub fn run() {
//     tauri::Builder::default()
//         .plugin(tauri_plugin_opener::init())
//         .invoke_handler(tauri::generate_handler![greet])
//         .run(tauri::generate_context!())
//         .expect("error while running tauri application");
// }

use pdf::file::{File as PdfFile, FileOptions};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use std::sync::mpsc;
use tauri_plugin_clipboard_manager::ClipboardExt;
use tauri_plugin_dialog::DialogExt;
use tauri_plugin_dialog::FileDialogBuilder;
// use tauri::Manager;

#[derive(Serialize, Deserialize, Clone)]
pub struct BookInfo {
    pub file_name: String,
    pub title: String,
    pub author: Option<String>,
    pub genre: Option<String>,
    pub publish_year: Option<String>,
}

#[tauri::command]
fn pick_pdf(app: tauri::AppHandle) -> Result<String, String> {
    // Create a channel to communicate between the callback and main thread
    let (sender, receiver) = mpsc::channel();

    app.dialog()
        .file()
        .add_filter("PDF Files", &["pdf"])
        .pick_file(move |file_path| {
            // Send the result through the channel
            let _ = sender.send(file_path);
        });

    // Wait for the result (this blocks the current thread)
    match receiver.recv().unwrap() {
        Some(path) => Ok(path.to_string()),
        None => Err("No file selected".to_string()),
    }
}
// #[tauri::command]
// async fn pick_pdf(app: tauri::AppHandle) -> Result<String, String> {
//     let path = Option::<FilePath>::None;
//     app.dialog()
//         .file()
//         .add_filter("PDF Files", &["pdf"])
//         .pick_file(|file_path| {
//             path = file_path;
//         });
//
//     match path {
//         Some(path) => Ok(path.to_string()),
//         None => Err("No file selected".to_string()),
//     }
// }
#[tauri::command]
async fn extract_pdf_metadata(file_path: String) -> BookInfo {
    let path = Path::new(&file_path);

    // Extract file name without extension
    let file_name = path
        .file_stem()
        .and_then(|n| n.to_str())
        .unwrap_or("Untitled")
        .to_string();

    // Open PDF with caching for better performance
    let file = FileOptions::cached().open(path).unwrap_or_else(|_| {
        panic!("Failed to open PDF file");
    });
    // .map_err(|e| format!("Failed to open PDF: {}", e));

    // Get document info dictionary
    let info_dict = file.trailer.info_dict.as_ref();

    // Extract title (fallback to filename if not available)
    let title = match info_dict {
        Some(dict) => match dict.title.clone() {
            Some(str) => str.to_string_lossy(),
            None => file_name.clone(),
        },
        None => file_name.clone(),
    };

    // Extract author if available
    let author = info_dict.and_then(|dict| match dict.author.clone() {
        Some(str) => Some(str.to_string_lossy().to_string()),
        None => None,
    });

    return BookInfo {
        file_name,
        title,
        author,
        genre: None,        // PDF standard doesn't include genre
        publish_year: None, // Would need to parse from author/date fields
    };
}
// #[tauri::command]
// async fn extract_pdf_metadata(file_path: String) -> Result<BookInfo, String> {
//     let path = Path::new(&file_path);
//     let file_name = path
//         .file_name()
//         .map(|f| f.to_string_lossy().to_string())
//         .unwrap_or_default();
//
//     let pdf = PdfFile::open(path).map_err(|e| format!("Failed to open PDF: {}", e))?;
//     let info = pdf
//         .get_info()
//         .map_err(|e| format!("Failed to get PDF info: {}", e))?;
//
//     let title = info
//         .title
//         .as_ref()
//         .map(|t| t.to_string())
//         .unwrap_or(file_name.clone());
//
//     let book_info = BookInfo {
//         file_name,
//         title,
//         author: info.author.as_ref().map(|a| a.to_string()),
//         genre: None,
//         publish_year: None,
//     };
//
//     Ok(book_info)
// }

#[tauri::command]
async fn search_book_info(title: String) -> Result<BookInfo, String> {
    // Mock API call (replace with real API like Google Books)
    let book_info = BookInfo {
        file_name: "".to_string(),
        title,
        author: Some("Unknown Author".to_string()),
        genre: Some("Fiction".to_string()),
        publish_year: Some("2020".to_string()),
    };
    Ok(book_info)
}

// #[tauri::command]
// async fn export_to_json(
//     book_info: BookInfo,
//     app_handle: tauri::AppHandle,
// ) -> Result<String, String> {
//     let json = serde_json::to_string_pretty(&book_info)
//         .map_err(|e| format!("Failed to serialize JSON: {}", e))?;
//
//     let dialog = FileDialogBuilder::new()
//         .add_filter("JSON Files", &["json"])
//         .save_file()
//         .await;
//
//     match dialog {
//         Some(path) => {
//             fs::write(&path, &json).map_err(|e| format!("Failed to write JSON: {}", e))?;
//             Ok(json)
//         }
//         None => {
//             // If no file is selected, return JSON for clipboard
//             let mut clipboard = app_handle
//                 .get_plugin("clipboard-manager")
//                 .unwrap()
//                 .clipboard();
//             clipboard
//                 .write_text(json.clone())
//                 .map_err(|e| format!("Failed to copy to clipboard: {}", e))?;
//             Ok(json)
//         }
//     }
// }

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .invoke_handler(tauri::generate_handler![
            pick_pdf,
            extract_pdf_metadata,
            search_book_info,
            // export_to_json
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
