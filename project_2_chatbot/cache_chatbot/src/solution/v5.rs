use kalosm::language::*;
use file_chatbot::solution::file_library::{self, load_chat_session_from_file};

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

    pub async fn chat_with_user(&mut self, username: String, message: String) -> String {
        let filename = &format!("{}.txt", username);
        let cached_chat = self.cache.get_chat(&username);

        match cached_chat {
            None => {
                println!("chat_with_user: {username} is not in the cache!");
                // The cache does not have the chat. What should you do?
                return String::from("Hello, I am not a bot (yet)!");
            }
            Some(chat_session) => {
                println!("chat_with_user: {username} is in the cache! Nice!");
                // The cache has this chat. What should you do?
                return String::from("Hello, I am not a bot (yet)!");

            }
        }
    }

    pub fn get_history(&mut self, username: String) -> Vec<String> {
        let filename = &format!("{}.txt", username);
        let cached_chat = self.cache.get_chat(&username);

        match cached_chat {
            None => {
                println!("get_history: {username} is not in the cache!");
                // TODO: The cache does not have the chat. What should you do?
                // Your code goes here.
                let mut chat_session = self
                .model
                .chat()
                .with_system_prompt("The assistant will act like a pirate");
            let load_file = load_chat_session_from_file(filename);
            match load_file{
                None => {},
                Some(session) => {
                    chat_session = chat_session.with_session(session);
                }
            }
                
                let history = chat_session
                .session()
                .unwrap()
                .history()
                .iter()
                .map(|msg: &ChatMessage| msg.content().to_string())
                .collect();

                self.cache.insert_chat(username, chat_session);
                return history;
            }
            Some(chat_session) => {
                println!("get_history: {username} is in the cache! Nice!");
                // TODO: The cache has this chat. What should you do?
                // Your code goes here.
                return chat_session
                .session()
                .unwrap()
                .history()
                .iter()
                .map(|msg| msg.content().to_string())
                .collect();
            }
        }
    }
}