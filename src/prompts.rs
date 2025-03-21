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
    pub fn image_description() -> String {
        let prompt = "Analyze the image I provide with a detailed focus on the following aspects:\n
Subject Identification: Determine if the subject is female and estimate her approximate age.\n
Hair Characteristics: Identify if the hair is long or short, its color, and style.\n
Eyes: Determine the eye color if visible.\n
Interests & Activities: Look for visual clues (e.g., accessories, background elements, or props) that might suggest interests such as music, art, or other hobbies. \n
Cultural & National Identity: If possible, infer hints about the subject’s nationality or cultural background based on facial features or style. \n
Overall Impression: Provide a comprehensive description that ties together physical details with possible personality traits and lifestyle interests. \n".to_string();
        prompt.to_string()
    }
}
pub const EMO_GIRL_DESCRIPTION: &str = "The emo girl style is a distinctive and expressive look that blends dark aesthetics with a touch of punk and alternative influences. It’s not just about clothing—it’s an overall attitude that reflects emotion, individuality, and sometimes a hint of vulnerability. Here’s a comprehensive breakdown of the key elements:";
