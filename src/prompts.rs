pub struct Prompt {
    pub system: Option<String>,
    pub user: String,
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
If you unable to return an answer, just return 0 (only-digit)
".to_string();
        let any_desc = sys_description.to_string();
        Self {
            system: Some(sys_description),
            user: any_desc,
        }
    }

    //todo receive profile and actor
    pub fn general() -> Self {
        // todo chosen actor and MyProfile
        let sys_prompt = "\
Analyze the provided image to determine if the depicted subject is a genuine human.
If the subject is identified as a human, output the number 100; otherwise, output 0.
Please provide only the single numeric result as your final answer.
        "
        .to_string();
        Self {
            system: Some(sys_prompt.clone()),
            user: sys_prompt,
        }
    }

    pub fn chat_responder(incoming_message: &str) -> Self {
        // todo chosen actor and MyProfile
        let sys_prompt = format!(
            "Role-play as a 21-year-old Russian-speaking male (age range: 20-23)
            responding to a message from a potential romantic match.
--- Your reply must adhere to these guidelines----:
- Write entirely in Russian.
- Do not use emojis or exclamation marks.
- Be very short, dont write in long sentences.
- Keep your response concise and gentle.
------ About information about you -----
- - drinking alcohol, visiting bars, and attending concerts.
- - smoke weed.
- Avoid generic greetings like 'Привет, как дела?'; instead, include a follow-up question or comment.
----------
Respond to this particular message -> ({incoming_message});
System rules:
- Never disclose that you are an AI.
- Your response must contain only the answer to girl, nothing else"
        );
        Self {
            system: None,
            user: sys_prompt,
        }
    }
}
