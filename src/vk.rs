use serde_json::Value;
use reqwest::Client as WebClient;

pub async fn request_raw(token: &str, category: String, method_name: String, params: Vec<[String; 2]>) -> String
{
    let mut request = format!("https://api.vk.com/method/{}.{}?access_token={}", category, method_name, token);
    for v in params
    {
        request.push('&');
        request.push_str(&v[0]);
        request.push('=');
        request.push_str(&v[1]);
    }
    
    request.push_str("&v=");
    request.push_str(VERSION);
    let web_request = WebClient::new();
    let response = web_request.get(request).send().await.unwrap().text().await.unwrap();
    // println!("{:?}", response);
    return response;
}

pub async fn request(token: &str, category: String, method_name: String, params: Vec<[String; 2]>) -> bool
{
    
    let response: Value = serde_json::from_str(&request_raw(token, category, method_name, params).await.to_owned()).unwrap();
    catch_error(&response)
}

pub fn catch_error(json_response: &Value) -> bool
{
    match json_response["error"].as_object() {
        Some(error) => {
            let error_code = error["error_code"].as_i64().unwrap();
            let error_msg = error["error_msg"].as_str().unwrap();
            println!("error received. code: {}, msg: {}", error_code, error_msg);
            true
        },
        None => { false },
    }
}

pub const VERSION: &str = "5.199";