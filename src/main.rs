use openai_api_rs::v1::api::Client;
use openai_api_rs::v1::chat_completion::{self, ChatCompletionRequest};
use openai_api_rs::v1::common::GPT4_VISION_PREVIEW;
use screenshots::Screen;
use std::fs::File;
use std::io::Read;
use base64::{encode};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let screen = Screen::from_point(0, 0).unwrap();
    let image = screen.capture_area(1350, 250, 550, 785).unwrap();
    let image_path = "target/capture_display_with_point.png";
    image.save(&image_path).unwrap();

    // 이미지 파일을 바이트 배열로 읽습니다.
    let mut file = File::open(&image_path).unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();

    // 바이트 배열을 Base64 문자열로 인코딩합니다.
    let base64_image = encode(&buffer);

    // Base64 문자열을 URL로 사용합니다.
    let image_url = format!("data:image/jpeg;base64,{}", base64_image);

    let client = Client::new("your_openai_key".to_string());

    let req = ChatCompletionRequest::new(
        GPT4_VISION_PREVIEW.to_string(),
        vec![chat_completion::ChatCompletionMessage {
            role: chat_completion::MessageRole::user,
            content: chat_completion::Content::ImageUrl(vec![
                chat_completion::ImageUrl {
                    r#type: chat_completion::ContentType::text,
                    text: Some(String::from("이 바둑의 현재 상황을 설명해줘.")),
                    image_url: None,
                },
                chat_completion::ImageUrl {
                    r#type: chat_completion::ContentType::image_url,
                    text: None,
                    image_url: Some(chat_completion::ImageUrlType {
                        url: String::from(
                            image_url,
                        ),
                    }),
                },
            ]),
            name: None,
        }],
    ).max_tokens(500);

    let result = client.chat_completion(req)?;
    println!("{:?}", result.choices[0].message.content);

    Ok(())
}