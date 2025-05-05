curl https://api.openai.com/v1/responses \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $OPENAI_API_KEY" \
  -d '{
  "model": "ft:gpt-4.1-mini-2025-04-14:personal:emochka007:BTeZ9uCW:ckpt-step-20",
  "input": [],
  "text": {
    "format": {
      "type": "text"
    }
  },
  "reasoning": {},
  "tools": [],
  "temperature": 1,
  "max_output_tokens": 2048,
  "top_p": 1,
  "store": true
}'