INSERT INTO teloxide_dialogues VALUES ($1, $2)
ON CONFLICT(chat_id) DO UPDATE SET dialogue=excluded.dialogue