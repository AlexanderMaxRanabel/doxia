use std::{
    io::{Write},
};
//Jesus said, “Do not tell lies, and do not do what you hate, for all things are plain in the sight of heaven. For nothing hidden will not become manifest, and nothing covered will remain without being uncovered.”

use reqwest::{Client, header};

use serde::{Serialize, Deserialize};

use tokio::fs;

#[derive(Serialize, Deserialize)]
struct Message_Payload {
    server_username: String,
    message: String,
}

pub async fn get_chat_history(server_ip: String) -> anyhow::Result<()> {
    Ok(())
}

pub async fn send_message(server_ip: String, doxia_message: String, doxia_username: String) -> anyhow::Result<()> {
    let client = Client::new();

    // Define the payload
    let payload = Message_Payload {
        server_username: doxia_username,
        message: doxia_message,
    };

    // Send the POST request
    let response = client
        .post(server_ip.as_str())
        .json(&payload)
        .send()
        .await?;

    Ok(())
}

pub async fn client_runtime(server_ip: String, user_conf_path: String) -> anyhow::Result<()> {
    let doxia_username = fs::read_to_string(user_conf_path)
        .await
        .expect("Failed to read the file");

    loop {
        print!("ω ");
        let mut doxia_message = String::new();
        std::io::stdin()
            .read_line(&mut doxia_message)
            .expect("Failed to read line");
        std::io::stdout().flush().unwrap();

        if doxia_message == "/exit" {
            break;
        }

        let _ = send_message(server_ip.clone(), doxia_message.clone(), doxia_username.clone()).await?;
    }
    Ok(())
}
