use kalosm::language::*;
use std::collections::HashMap;

#[allow(dead_code)]
pub struct ChatbotV3 {
    // What should you store inside your Chatbot type?
    // The model? The chat_session?
    // Storing a single chat session is not enough: it mixes messages from different users
    // together!
    // Need to store one chat session per user.
    // Think of some kind of data structure that can help you with this.
    model: Llama,
    chat_session: HashMap<String, Chat<Llama>>,
}

impl ChatbotV3 {
    #[allow(dead_code)]
    pub fn new(model: Llama) -> ChatbotV3 {
        return ChatbotV3 {
            // Make sure you initialize your struct members here
            model,
            chat_session: HashMap::new(),
        };
    }

    #[allow(dead_code)]
    pub async fn chat_with_user(&mut self, username: String, message: String) -> String {
        // Add your code for chatting with the agent while keeping conversation history here.
        // Notice, you are given both the `message` and also the `username`.
        // Use this information to select the correct chat session for that user and keep it
        // separated from the sessions of other users.
        if !self.chat_session.contains_key(&username){
            let chat_session = self
            .model
            .chat()
            .with_system_prompt("This assistant will act like a pirate");

            self.chat_session.insert(username.clone(), chat_session);
        }
        let user_session = self.chat_session.get_mut(&username).unwrap(); //must use get_mut to get an reference without making an error
        let response = user_session.add_message(message).await; //user_session is borrowed as mutable, therefore must use get_mut
        return response.unwrap();
    }

    #[allow(dead_code)]
    pub fn get_history(&self, username: String) -> Vec<String> {
    match self.chat_session.get(&username) {
        Some(session) => {
            session.session().expect("session error").history()
                .iter()
                .map(|msg: &ChatMessage| msg.to_string())
                .collect()
        }
        None => Vec::new(),
    }
}
}