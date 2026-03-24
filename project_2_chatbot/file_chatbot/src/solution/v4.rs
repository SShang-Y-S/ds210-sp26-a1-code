use kalosm::language::*;
use crate::solution::file_library::{self, load_chat_session_from_file, save_chat_session_to_file};
pub struct ChatbotV4 {
    model: Llama,
}

impl ChatbotV4 {
    pub fn new(model: Llama) -> ChatbotV4 {
        return ChatbotV4 {
            model: model,
        };
    }

    pub async fn chat_with_user(&mut self, username: String, message: String) -> String {
        let filename = &format!("{}.txt", username);
            let mut chat_session: Chat<Llama> = self.model
                .chat()
                .with_system_prompt("The assistant will act like a pirate");

            // TODO: You have to implement the rest:
            // You need to load the chat session from the file using file_library::load_chat_session_from_file(...).
            // Think about what needs to happen if the function returns None vs Some(session).
            // Hint: look at https://docs.rs/kalosm/latest/kalosm/language/struct.Chat.html#method.with_session
        let load_file = load_chat_session_from_file(filename);
        match load_file{
            None => {}
            Some(session) => {
                chat_session = chat_session.with_session(session);
            }
        }
        let response = chat_session.add_message(message).await.unwrap();
        let current_session = chat_session.session().unwrap();
        file_library::save_chat_session_to_file(filename, &*current_session);

        return response.to_string();
    }

    pub fn get_history(&self, username: String) -> Vec<String> {
        let filename = &format!("{}.txt", username);

        match file_library::load_chat_session_from_file(&filename) {
            None => {
                return Vec::new();
            },
            Some(session) => {
                session.history()
                // personal notes to understand!! : gets the list of messages from loaded session
                    .iter()
                    // this iterates over each message
                    .filter(|msg|msg.role() != MessageType::SystemPrompt)
                    // an extra call to filter 
                    .map(|msg| msg.content().to_string())
                    // this converts each message into a string
                    .collect()
                    // this puts the strings to return
            }
        }
    }
}