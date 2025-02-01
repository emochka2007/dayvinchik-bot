- Roadmap
    - TDlib setup
    - Interact through TDLib
    - ## not implemented yet
    - Get description of the profile and construct a "special_message" using LLM
    - Send message to LLM with prompt and image and profile description
    - Get the answer from LLM and log everything
    - Actors and communication.
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

-- features --
- analyze ignored chats (no answer for more than 24 hrs)
- analyze any new message
- multiple variants to start (love ur style, nyashka, wanna go for a walk) conside