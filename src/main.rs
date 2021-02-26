extern crate reqwest;
extern crate serde;
extern crate serde_json;
use anyhow::Result;
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use structopt::StructOpt;

const BASE_URL: &str = "https://api.groupme.com/v3";
const TOKEN: &str = "&token=";

#[derive(Debug, StructOpt)]
#[structopt(
    name = "burrito-tracker",
    about = "Goes through your groupme conversation and runs some analytics."
)]
struct Args {
    #[structopt(long, env = "GROUPME_API_TOKEN")]
    groupme_api_token: String,
    #[structopt(long, env = "GROUPME_CONVERSATION_ID")]
    groupme_conversation_id: u32,
    #[structopt(long)]
    notable_burrito_count: usize,
}

#[derive(Serialize, Deserialize, Debug, Clone, Hash, PartialEq, Eq)]
struct UserId(String);
#[derive(Serialize, Deserialize, Debug)]
struct GroupId(String);
#[derive(Serialize, Deserialize, Debug)]
struct MessageId(String);
#[derive(Serialize, Deserialize, Debug)]
struct Attachment {
    #[serde(rename = "type")]
    _type: Option<String>,
    url: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Message {
    attachments: Vec<Attachment>,
    avatar_url: Option<String>,
    created_at: u64,
    //    event: Option<Event>, // Enum of message type?
    favorited_by: Vec<UserId>,
    group_id: GroupId,
    id: MessageId,
    name: String,
    platform: Option<String>,
    sender_id: UserId,
    sender_type: String,
    source_guid: String,
    system: bool,
    text: Option<String>,
    user_id: UserId, // Prefer user over sender?
}

#[derive(Serialize, Deserialize, Debug)]
struct Meta {
    code: u64,
}

#[derive(Serialize, Deserialize, Debug)]
struct ServerResponse {
    meta: Meta,
    response: Option<Response>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Response {
    count: u64,
    messages: Vec<Message>,
}
#[derive(Serialize, Deserialize, Debug, Default)]
struct UserData {
    name: String,
    message_count: usize,
    message_with_image_count: usize,
    number_of_likes: usize,
}

// Using a client, builds up a request and returns the response
fn get_messages(
    client: &reqwest::Client,
    before_id: &Option<MessageId>,
    conversation_id: u32,
    api_token: &str,
) -> Result<reqwest::Response> {
    // welcome to the shadow realm
    let before_id = match before_id {
        Some(before_id) => format!("&before_id={}", before_id.0),
        None => "".to_string(),
    };
    let endpoint = format!("/groups/{}/messages?limit=99", conversation_id);
    let url = format!(
        "{}{}{}{}{}",
        BASE_URL, endpoint, before_id, TOKEN, api_token
    );
    let resp: reqwest::Response = client
        .get(&url)
        .header("Accept", "application/json")
        .send()?;
    // println!("Sending request to {}", url);
    return Ok(resp);
}

fn add_to_user_map(
    mut user_map: HashMap<UserId, UserData>,
    response: Response,
) -> (MessageId, HashMap<UserId, UserData>) {
    let mut end_at: MessageId = MessageId("".to_string());
    for message in response.messages {
        match user_map.get_mut(&message.user_id) {
            Some(mut user) => {
                // I wanna do this either way...
                if !message.attachments.is_empty() {
                    user.message_with_image_count += 1;
                }
                // Ratio of messages with images to text
                user.message_count += 1;
                user.number_of_likes += message.favorited_by.len();
            }
            None => {
                let mut user = UserData {
                    name: message.name,
                    ..UserData::default()
                };

                // I wanna do this either way...
                if !message.attachments.is_empty() {
                    user.message_with_image_count += 1;
                }
                // Ratio of messages with images to text
                user.message_count += 1;
                user.number_of_likes += message.favorited_by.len();
                user_map.insert(message.user_id.clone(), user);
            }
        }
        end_at = message.id;
    }
    return (end_at, user_map);
}

fn main() -> Result<()> {
    match dotenv() {
        Err(_) => {
            println!("No env file found. Expecting keys from commandline args");
        }
        Ok(_) => {}
    }
    let args = Args::from_args();
    let client = reqwest::Client::new();
    let mut user_data: HashMap<UserId, UserData> = HashMap::new();
    let mut cursor: Option<MessageId> = None;
    let mut status_code: u64 = 200;

    while status_code == 200 {
        let mut body = get_messages(
            &client,
            &cursor,
            args.groupme_conversation_id,
            &args.groupme_api_token,
        )?;
        match body.json::<ServerResponse>() {
            Ok(body) => match body.response {
                Some(response) => {
                    let (end_at, user_data_t) = add_to_user_map(user_data, response);
                    cursor = Some(end_at);
                    user_data = user_data_t;
                }
                None => {
                    status_code = 305;
                   //  println!("{:#?}", body);
                }
            },
            Err(_e) => {
                status_code = 304;
                // println!("Error {:?}, {:#?}", e, body);
            }
        };
    }
    for user in user_data.values() {
        if user.message_with_image_count > args.notable_burrito_count && user.name != "GroupMe".to_string() {
            println!("{:#?}", user);
        }
    }
    Ok(())
}
