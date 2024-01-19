# ai-go-commentator-rs
AI-Powered go commentator
I am aiming to develop go commentator, which works with AI.
This code is designed to take a Go game screen as input and generate responses using OpenAI's GPT-4 Vision.
The output of GPT-4 is outputted through tts_rust.


## View in other languages

[**English**](./README.md)

# Setup
Install dependencies
```
git clone https://github.com/GG-O-BP/ai-go-commentator-rs.git
cd ai-go-commentator-rs
cargo build
```

# Usage (Not working yet)
```
cargo run -- --openaikey "OpenAI key"
```

## Notes
"OpenAI key" should be replaced with the actual OpenAI key.


# Other

- [x] Create Screenshot
- [x] Send Image to GPT-4 Vision API
- [ ] Must explain the rules of Go
- [ ] Must be able to view the content of Go

# License
This program is under the [MIT license](/LICENSE) 
