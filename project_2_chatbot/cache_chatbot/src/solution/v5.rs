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
        // this is the feature where we chat with the user

        match cached_chat {
            None => {
                println!("chat_with_user: {username} is not in the cache!");
                let mut chat_session = self.model
                // notes for self : this is when the user is not in cache
                    .chat()
                    .with_system_prompt("The assistant will act like a pirate");
                    // makes chatbot act like a pirate (ARUGH MATEY!!!)
                let load_file = load_chat_session_from_file(filename);
                // loads the file from their old conversation
                match load_file {
                    None => {},
                    // if its unable to find it, we start fresh
                    Some(session)=> {
                        chat_session = chat_session.with_session(session);
                        // if found, its picked up where they were left off
                    }
                }
                let response = chat_session.add_message(message).await.unwrap();
                // this sends a message and gets a response 
                let current_session = chat_session.session().unwrap();
                file_library::save_chat_session_to_file(filename, &*current_session);
                // saves the file as backup 
                self.cache.insert_chat(username,chat_session);
                // puts the convo into cache for next time
                return response.to_string();
                // returns the response for chatbot
          
            }
            Some(chat_session) => {
                // this is for when the user is in cache 
                println!("chat_with_user: {username} is in the cache! Nice!");
                // the conversations is already in the cache 
                let response = chat_session.add_message(message).await.unwrap();
                // sends messages directly and gets response
                let current_session = chat_session.session().unwrap();
                file_library::save_chat_session_to_file(filename, &*current_session);
                return response.to_string();
                // returns chatbot response

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