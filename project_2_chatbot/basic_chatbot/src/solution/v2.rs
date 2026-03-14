use kalosm::language::*;

#[allow(dead_code)]
pub struct ChatbotV2 {
    // What should you store inside your Chatbot type?
    // The model? The chat_session?
    chat_session:Chat<Llama>,
    }

impl ChatbotV2 {
    #[allow(dead_code)]
    pub fn new(model: Llama) -> ChatbotV2 {
        let mut chat_session = model
        .chat()
        .with_system_prompt("The assistant will act like a pirate");
        return ChatbotV2 {
           chat_session
        };
    }

    #[allow(dead_code)]
    pub async fn chat_with_user(&mut self, message: String) -> String {
        // Add your code for chatting with the agent while keeping conversation history here.
        let user_message = self.chat_session.add_message(message);
        let response = user_message.await;
        let result = response.map(|msg|msg.to_string()).unwrap();
        return result;
    }
}