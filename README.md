## Feature in progress

- Слишком много ❤️ за сегодня. stop bot
- Learning tool to educate model

## Altushka Bot

- Identifies emo-girls from chat bot using LLM with a score

### Current implementation

- send out to all matches MSG_STARTER
- all db entities in one folder

- Roadmap
    - TDlib setup
    - Interact through TDLib
    - ## not implemented yet
    - Get description of the profile and construct a "special_message" using LLM
    - Send message to LLM with prompt and image and profile description
    - Get the answer from LLM and log everything
    - Actors and communication.
    - Filters (looking for friends => skip)
- Entities
    - Actors. The behaviour of the actor depends on the chosen.
      Likes and dislikes also depends on the actor
      Each of the actors has different prompt, different behaviour
        - Like/Communicate actor
        - Custom actors for users
    - Stats.
        - Log each profile. Photo + name. assign to it `PROFILE_ID`  `uuid`
        - Save matches
        - Log each response from GPT and assign to `PROFILE_ID`
        - save each dialogue in a separate file

