## todo:

- todo Fix download file system
- Dynamic superlike prompt from LLM
- Superlike generation
- Auto-responder

## Altushka Bot

- Identifies altushka from chat bot using LLM with a score, and will talk for u

### Current implementation

- Callback executor with a queue
- move all global states to PG
- vectorize data in pg
- educating tool to store only correct data
- send out to all matches MSG_STARTER
- multiple variants to start (love ur style, nyashka, wanna go for a walk) conside
- iterate over cities with one account.
- all db entities in one folder
- vectorize all of my chats and store in pg

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

-- features --

- Random timestamp between answers
- Goals and checks in chat, getting info about girl (name, age, activity, favourite animal)
- analyze ignored chats (no answer for more than 24 hrs)
- Find altushka from the photo and identify.
-

## Done

- Detect unread messages
- list all active matches
- Get the match message
- interact with db
- Store the currently reviewing profi to send all photos to gpt to analuze
- Insert each review inside the db with the status `PENDING`, 'COMPLETED'

- Custom errors
 
