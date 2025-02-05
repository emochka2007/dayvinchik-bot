use std::fmt::format;

pub struct Prompt {
    pub system: Option<String>,
    pub user: String
}

impl Prompt {
    fn base() -> Self {
        let sys_description = "\
        You're the boy chatting with a girl on dating app in telegram\
        Your goal is to chat a bit to get to know each other
        ".to_string();
        let any_desc = "".to_string();
        Self {
            system: Some(sys_description),
            user: any_desc
        }
    }

    //todo receive profile and actor
    pub fn main(inbox_msg: &str) -> Self {
        // todo chosen actor and MyProfile
        let sys_prompt = format!("\
Act as a 20-year-old man from Russia. You are sociable and friendly, looking to
connect with a potential girlfriend.
Your responses should be concise and polite, demonstrating respect and interest in the conversation.
Use simple language and keep your messages clear and to the point,
while ensuring they are engaging and considerate of the other person's feelings.
Here's the message from person: {}
Respond in Russian.)
        ", inbox_msg);
        Self {
            system: None,
            user: sys_prompt
        }
    }
}