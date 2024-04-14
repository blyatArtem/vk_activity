mod vk;

enum ActivityType
{
    Typing,
    AudioMessage,
    Photo,
    Video,
    VideoMessage,
}

impl ActivityType
{
    fn to_string(&self) -> String
    {
        match self
        {
            ActivityType::Typing => {"typing".to_owned() },
            ActivityType::AudioMessage => {"audiomessage".to_owned() },
            ActivityType::Photo => {"photo".to_owned() },
            ActivityType::Video => {"video".to_owned() },
            ActivityType::VideoMessage => {"videomessage".to_owned() },
        }
    }
}

#[tokio::main]
async fn main()
{
    println!("token:");
    let token = read_string();
    println!("user id:");
    let user_id = read_string();
    let activity_type = ActivityType::AudioMessage;

    loop
    {
        let mut params = Vec::new();
        params.push(["user_id".to_owned(), user_id.to_owned()]);
        params.push(["type".to_owned(), activity_type.to_string()]);
        if vk::request(&token, "messages".to_owned(), "setActivity".to_owned(), params).await
        {
            break;
        }
        // 5-10 сек в зависимости от платформы
        std::thread::sleep(std::time::Duration::from_millis(4500));
    }
}

fn read_string() -> String
{
    let mut token = String::new();
    std::io::stdin().read_line(&mut token).unwrap();
    token.trim().to_owned()
}