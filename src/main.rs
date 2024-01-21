use clap::Parser;
use lazy_static::lazy_static;
use regex::Regex;

use std::sync::{Mutex};
use openai_api_rs::v1::api::Client;
use openai_api_rs::v1::chat_completion::{self, ChatCompletionRequest};
use openai_api_rs::v1::common::GPT4_VISION_PREVIEW;
use screenshots::Screen;
use std::fs::File;
use std::io::Read;
use base64::{encode};

use tts_rust::tts::GTTSClient;
use tts_rust::languages::Languages;

#[derive(Parser, Debug, Clone)]
struct Args {
    #[clap(long)]
    openaikey: String,
}

lazy_static! {
    static ref MESSAGES: Mutex<Vec<chat_completion::ChatCompletionMessage>> = Mutex::new(Vec::new());
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let mut input = String::new();
    let mut counter = 0;

    loop {
        std::io::stdin().read_line(&mut input).unwrap();
        println!("{}", "해설 생성 중..");

        let image_url = capture_and_encode_image().unwrap();

        let message = match counter {
            0 => "이미지 최하단의 바가 밝은 색이면 백이 유리하고, 어두운 색이면 흑이 유리합니다. 최하단의 바의 안쪽에는 텍스트로 몇집 유리한지 적혀있습니다. 위에는 바둑판이 있습니다. 바둑이 시작되고 흑이 첫 수를 두었습니다. 지금 흑이 둔 수는 검정색돌 안에 하얀색 ◯표시가 있습니다. 지금 흑이 둔 수가 구석이나 테두리에 가까울 수록 실리적인 작전입니다. 지금 흑이 둔 수가 중앙에 가까울 수록 세력적인 작전입니다. 지금은 백의 차례입니다. 숫자들은 그 지점에 둘 경우 형세가 어떻게 변화될지 나타냅니다. 다음 최선의 한 수는 파란색 동그라미로 표시돼있습니다. 이미지 최하단을 보고 흑과 백 중에 어느 선수가 몇집 유리한지 차근차근히 정확하게 보고, 바둑판에서 검정색돌 안에 하얀색 ◯표시가 있는 돌(지금 흑이 둔 수)의 위치를 차근차근히 정확하게 보고 200자이내로 짧게 요약해서 재미있게 바둑의 상황을 해설해주세요.",
            _ if counter % 2 == 0 => "이미지 최하단의 바가 밝은 색이면 백이 유리하고, 어두운 색이면 흑이 유리합니다. 최하단의 바의 안쪽에는 텍스트로 몇집 유리한지 적혀있습니다. 위에는 바둑판이 있습니다. 흑이 수를 두었습니다. 지금 흑이 둔 수는 검정색돌 안에 하얀색 ◯표시가 있습니다. 지금 흑이 둔 수가 구석이나 테두리에 가까울 수록 실리적인 작전입니다. 지금 흑이 둔 수가 중앙에 가까울 수록 세력적인 작전입니다. 지금 흑이 둔 수가 하얀색 돌에 가까울 수록 공격적인 수입니다. 바둑판의 숫자들은 그 지점에 둘 경우 형세가 어떻게 변화될지 나타냅니다. 지금은 백의 차례입니다. 백의 다음 최선의 한 수는 파란색 동그라미로 표시돼있습니다. 이미지 최하단을 보고 흑과 백 중에 어느 선수가 몇집 유리한지 차근차근히 정확하게 보고, 바둑판에서 검정색돌 안에 하얀색 ◯표시가 있는 돌(지금 흑이 둔 수)의 위치를 차근차근히 정확하게 보고 200자이내로 짧게 요약해서 재미있게 바둑의 상황을 해설해주세요.",
            _ => "이미지 최하단의 바가 밝은 색이면 백이 유리하고, 어두운 색이면 흑이 유리합니다. 최하단의 바의 안쪽에는 텍스트로 몇집 유리한지 적혀있습니다. 위에는 바둑판이 있습니다. 백이 수를 두었습니다. 지금 백이 둔 수는 하얀색돌 안에 검정색 ◯표시가 있습니다. 지금 백이 둔 수가 구석이나 테두리에 가까울 수록 실리적인 작전입니다. 지금 백이 둔 수가 중앙에 가까울 수록 세력적인 작전입니다. 지금 백이 둔 수가 검정색 돌에 가까울 수록 공격적인 수입니다. 바둑판의 숫자들은 그 지점에 둘 경우 형세가 어떻게 변화될지 나타냅니다. 지금은 흑의 차례입니다. 흑의 다음 최선의 한 수는 파란색 동그라미로 표시돼있습니다. 이미지 최하단을 보고 흑과 백 중에 어느 선수가 몇집 유리한지 차근차근히 정확하게 보고, 바둑판에서 하얀색돌 안에 검정색 ◯표시가 있는 돌(지금 백이 둔 수)의 위치를 차근차근히 정확하게 보고 200자이내로 짧게 요약해서 재미있게 바둑의 상황을 해설해주세요.",
        };

        let _ = process_chat_completion(args.openaikey.clone(), message.to_string(), image_url.to_string()).await;
        counter += 1;
    }
}

fn capture_and_encode_image() -> Result<String, Box<dyn std::error::Error>> {
    let screen = Screen::from_point(0, 0)?;
    let image = screen.capture_area(1200, 245, 705, 787)?;
    let image_path = "capture_display_with_point.png";
    image.save(&image_path)?;

    // 이미지 파일을 바이트 배열로 읽습니다.
    let mut file = File::open(&image_path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    // 바이트 배열을 Base64 문자열로 인코딩합니다.
    let base64_image = encode(&buffer);

    // Base64 문자열을 URL로 사용합니다.
    let image_url = format!("data:image/jpeg;base64,{}", base64_image);

    Ok(image_url)
}

async fn process_chat_completion(openaikey: String, content: String, image_url: String) -> Result<(), openai_api_rs::v1::error::APIError> {

    let client = Client::new(openaikey);

    // let message = chat_completion::ChatCompletionMessage {
    //     role: chat_completion::MessageRole::user,
    //     content: chat_completion::Content::ImageUrl(vec![
    //         chat_completion::ImageUrl {
    //             r#type: chat_completion::ContentType::text,
    //             text: Some(String::from(content)),
    //             image_url: None,
    //         },
    //         chat_completion::ImageUrl {
    //             r#type: chat_completion::ContentType::image_url,
    //             text: None,
    //             image_url: Some(chat_completion::ImageUrlType {
    //                 url: String::from(
    //                     image_url,
    //                 ),
    //             }),
    //         },
    //     ]),
    //     name: None,
    // };
    // {
    //     let mut messages = MESSAGES.lock().unwrap();
    //     messages.push(message);
    // }

    let messages = MESSAGES.lock().unwrap().to_vec();

    let req = ChatCompletionRequest::new(
        GPT4_VISION_PREVIEW.to_string(),
        // messages,
        vec![chat_completion::ChatCompletionMessage {
            role: chat_completion::MessageRole::user,
            content: chat_completion::Content::ImageUrl(vec![
                chat_completion::ImageUrl {
                    r#type: chat_completion::ContentType::text,
                    text: Some(String::from(content)),
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
    ).max_tokens(3000);

    let result = client.chat_completion(req)?;
    println!("{:?}", result.choices[0].message.content);
    // narrate_message(result.choices[0].message.content);

    Ok(())
}

fn narrate_message(message: &str) {
    let re = Regex::new(r"[^\w\s.,?!\-]").unwrap();
    let cleaned_message = re.replace_all(message, "");
    let parts: Vec<&str> = cleaned_message.split(|c| c == '.' || c == ',' || c == '!' || c == '?').collect();

    let narrator: GTTSClient = GTTSClient {
        volume: 1.0, 
        language: Languages::Korean,
        tld: "com"
    };
    for part in parts {
        let _ = narrator.speak(part);
    }
}