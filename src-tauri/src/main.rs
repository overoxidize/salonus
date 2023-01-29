#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::generate_context;
use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Default, Serialize, Deserialize, Clone, Debug)]
enum State {
    Loaded,
    #[default]
    Loading
}

#[derive(Serialize, Deserialize, Clone, Debug)]
enum LoadErr {
    FileErr,
}
#[derive(Serialize, Deserialize, Clone, Debug)]

enum SaveErr {
    DirErr,
    FileErr,
    WriteErr,

}

#[derive(Default, Serialize, Deserialize, Clone, Debug)]
enum AgoraMessage {
    ConversationAdded(Conversation),
    NameChanged(String, Uuid),
    DescChanged(String, Uuid),
    UserMessage(UserMessage),
    InputChanged(String),
    #[default]
    CreateComment,
    ConversationMessage(ConversationMessage)
}

#[derive(Serialize, Deserialize, Clone, Debug)]
enum ConversationMessage {
    CommentAdded(String),
    CommentDeleted(Uuid),
    UserAdded(User),
    UserExited(Uuid)
}

#[derive(Serialize, Deserialize, Clone, Debug)]
enum UserMessage {
    KindChange(String),
    UserNameChange(String),
    EmailChange(String),
    PasswordChange(String)
}
#[derive(Default, Serialize, Deserialize, Clone, Debug)]

struct AppState {
    loaded: State,
    pub agoras: Vec<Agora>,
    pub users: Vec<User>,
    pub comments: Vec<Comment>
}

#[derive(Default, Serialize, Deserialize, Clone, Debug)]
struct User {
    kind: String,
    user_name: String,
    password: String,
    conversations: Vec<Uuid>,
    user_id: Uuid,
    comments: Vec<String>
}

#[derive(Default, Serialize, Deserialize, Clone, Debug)]
struct Conversation {
    assembly: Vec<User>,
    presenter: User,
    comments: Vec<Comment>,
    agora_id: Uuid
}
#[derive(Default, Serialize, Deserialize, Clone, Debug)]
struct Agora {
    name: String,
    topic: String,
    desc: String,
    conversations: Vec<Conversation>,
    founder: User
}
#[derive(Default, Serialize, Deserialize, Clone, Debug)]
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
