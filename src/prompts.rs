pub struct Prompt {
    pub system: Option<String>,
    pub user: String
}
pub enum _Genders {
    MAN(String),
    WOMAN(String)
}
impl Prompt {
    //todo receive profile
    pub fn analyze_alt() -> Self {
        // let sys_description = "What's on the photo?".to_string();
        let sys_description = "
First image to analyze. Second represents the alternative female character.
Analyze the provided image to determine if the character depicted is considered 'alternative' in style.
Please focus on the following aspects and conclude with a confidence score, ranging from 1 to 100, on how sure you are that the character fits the 'alternative' description:
Fashion Style: Look for distinctive clothing choices that align with alternative fashion, such as punk, goth, or other subculture styles.
Hairstyle and Color: Note any unconventional hair styles or colors that may indicate an alternative aesthetic.
Accessories: Observe any unique or non-traditional accessories, such as piercings, tattoos, and unusual jewelry.
Makeup: Examine the style and application of makeup to see if it fits within alternative beauty norms.
Overall Impression: Summarize your findings based on the visual elements observed.
After analyzing these elements, provide a confidence score from 1 to 100 on whether the character can be considered 'alternative'.
Please return just a number for the first photo, only number.
".to_string();
        let any_desc =sys_description.to_string();
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