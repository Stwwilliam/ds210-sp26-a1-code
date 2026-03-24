use kalosm::language::*;
use file_chatbot::solution::file_library;

use crate::solution::Cache;

pub struct ChatbotV5 {
    model: Llama,
    cache: Cache<Chat<Llama>>,
}

impl ChatbotV5 {
    pub fn new(model: Llama) -> ChatbotV5 {
        return ChatbotV5 {
            model: model,
            cache: Cache::new(3),
        };
    }

    // student 2
    pub async fn chat_with_user(&mut self, username: String, message: String) -> String {
        let filename = &format!("{}.txt", username);
        let cached_chat = self.cache.get_chat(&username);

        let mut chat_session = match cached_chat {
            None => {
                println!("chat_with_user: {username} is not in the cache!");
                match file_library::load_chat_session_from_file(&filename) {
                    None => {
                        Chat::new(self.model.clone())
                            .with_system_prompt("The assistant will act like a pirate")
                    },
                    Some(session) => {
                        Chat::new(self.model.clone()).with_session(session)
                    }
                }
            },
            Some(chat_session) => {
                println!("chat_with_user: {username} is in the cache! Nice!");
                chat_session.clone()
            }
        };

        let output = chat_session.add_message(message).await;

        self.cache.insert_chat(username.clone(), chat_session.clone());

        let session = chat_session.session().unwrap().clone();
        file_library::save_chat_session_to_file(&filename, &session);

        return output.unwrap();
    }

    pub fn get_history(&mut self, username: String) -> Vec<String> {
        let filename = &format!("{}.txt", username);
        let cached_chat = self.cache.get_chat(&username);

        let chat_session = match cached_chat {
            None => {
                println!("get_history: {username} is not in the cache!");
<<<<<<< HEAD
                // TODO: The cache does not have the chat. What should you do?
                // Your code goes here.
                let mut chat_session = match file_library::load_chat_session_from_file(&filename) {
                    Some(session) => self.model.chat().with_session(session),
                    None => self.model.chat().with_system_prompt("The assistant will act like a pirate"),
                };

                self.cache.insert_chat(username.clone(), chat_session);
                let chat_session = self.cache.get_chat(&username).unwrap();
                let history = chat_session.session().unwrap().history();
                let output: Vec<String> = history.iter().filter_map(|message| {
                    match message.role() {
                        MessageType::UserMessage => Some(message.content().to_string()),
                        MessageType::ModelAnswer => Some(message.content().to_string()),
                        MessageType::SystemPrompt => None,
                    }
                }).collect();
                return output;
            }
            Some(chat_session) => {
=======
                match file_library::load_chat_session_from_file(&filename) {
                    Some(session) => Chat::new(self.model.clone()).with_session(session),
                    None => Chat::new(self.model.clone())
                        .with_system_prompt("The assistant will act like a pirate"),
                }
            },
            Some(session) => {
>>>>>>> 6dc3e77017d88545f7095884fb5a4a29b578b919
                println!("get_history: {username} is in the cache! Nice!");
                session.clone()
            }
        };

        self.cache.insert_chat(username.clone(), chat_session.clone());

        chat_session.session().unwrap().history().iter().filter_map(|message| {
            match message.role() {
                MessageType::UserMessage => Some(message.content().to_string()),
                MessageType::ModelAnswer => Some(message.content().to_string()),
                MessageType::SystemPrompt => None,
            }
        }).collect()
    }
}

