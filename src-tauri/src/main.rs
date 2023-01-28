#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::generate_context;
use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, Debug)]
enum State {
    Loaded,
    Loading
}

#[derive(Serialize, Deserialize, Clone, Debug)]
enum LoadErr {
    FileErr,
    FormatErr
}

#[derive(Serialize, Deserialize, Clone, Debug)]

struct AppState {
    loaded: State,
    pub agoras: Vec<Agora>,
    pub users: Vec<User>,
    pub comments: Vec<Comment>
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct User {}
#[derive(Serialize, Deserialize, Clone, Debug)]
struct Agora {}
#[derive(Serialize, Deserialize, Clone, Debug)]
struct Comment {
    user_id : Uuid, // v4
    convo_id: Uuid, // v7
    content: String
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(generate_context!())
        .expect("error while running tauri application");
}
