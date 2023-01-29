use serde::{Serialize, Deserialize};
use uuid::Uuid;
use ulid::{Generator, Ulid};

mod enum_types;

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
    email: String,
    conversations: Vec<Uuid>,
    user_id: Uuid,
    comments: Vec<String>
}

impl User {
    fn new(
        kind: String, 
        user_name: String,
        password: String, 
        email: String, 
        conversations: Vec<Uuid>, // Stateful
        comments: Vec<String>, // Stateful
        user_id: Uuid) -> User
        {
            User {
                kind,
                user_name,
                password,
                email,
                conversations,
                comments,
                user_id
            }
        }
    fn update(&mut self, message: UserMessage) {
        match message {
            UserMessage::KindChange(new_kind) => {
                self.kind = new_kind
            }
            UserMessage::UserNameChange(username) => {
                self.user_name = username
            }
            UserMessage::EmailChange(e_mail) => {
                self.email = e_mail
            }
            UserMessage::PasswordChange(new_pw) => {
                // we should use the uuid hashes or hash fn here, no plaintext!
                self.password = new_pw
            }
        }
    }
}

#[derive(Default, Serialize, Deserialize, Clone, Debug)]
struct Conversation {
    assembly: Vec<User>,
    presenter: User,
    comments: Vec<Comment>,
    agora_id: Uuid
}

impl Conversation {
    fn new(assembly: Vec<User>, presenter: User, comments: Vec<Comment>, agora_id: Uuid) -> Conversation {
        Conversation {
            assembly,
            presenter,
            comments,
            agora_id,
        }
    }

    fn update(&mut self, message: ConversationMessage) {
        match message {
            ConversationMessage::CommentAdded(comment) => {
                let mut new_comments = self.comments.clone();
                let new_comment = Comment {content: comment, user_id: Uuid::new_v4(), convo_id: Uuid::new_v4()};
                new_comments.push(new_comment);
                self.comments = new_comments;
            }
            ConversationMessage::CommentDeleted(uuid) => {

                let length = self.comments.len();
                let comments = self.comments.clone();

                for mut i in comments {

                    if  i.user_id == uuid {

                        i.content = String::from("This comment was deleted.");
                    }
                }
            }
            ConversationMessage::UserAdded(user) => {

                let mut new_assembly = self.assembly.clone();
                new_assembly.push(user);
                self.assembly = new_assembly;
                
            }
            ConversationMessage::UserExited(u_uuid) => {

                let mut new_assembly = self.assembly.clone();
                new_assembly.retain(|user| user.user_id != u_uuid);

            }
        }
    }
}

#[derive(Default, Serialize, Deserialize, Clone, Debug)]
struct Agora {
    name: String,
    topic: String,
    desc: String,
    conversations: Vec<Conversation>,
    founder: User
}

impl Agora {
    pub fn new (
        name: String, 
        topic: String, 
        desc: String, 
        conversations: Vec<Conversation>, 
        founder: User) -> Agora {
            Agora {
                name,
                topic,
                desc,
                conversations: conversations,
                founder: founder
            }
        }
    }
#[derive(Default, Serialize, Deserialize, Clone, Debug)]
struct Comment {
    user_id : Uuid, // v4
    convo_id: Uuid, // v7
    content: String,
}

impl Comment {
    fn new(content: String, user_id: Uuid, convo_id: Uuid) -> Comment {
        Comment {
            user_id,
            convo_id,
            content,
        }
    }
    fn make_comment() -> Comment {
        let gen = Generator::new();
        Comment {
        content: "Awesome".to_string(), 
        user_id: Uuid::new_v4(),
        convo_id: Uuid::new_v4(),
        }
    }
}




