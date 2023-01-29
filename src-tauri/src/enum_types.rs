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