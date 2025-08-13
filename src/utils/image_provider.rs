use {
    crate::utils::images::{WCImage, WCImages},
    serde::Deserialize,
    serde_json,
    std::{
        fs,
        sync::{Arc, Mutex},
    },
};

#[derive(Debug, Deserialize)]
struct Message {
    filename: String,
    url: String,
}

#[derive(Debug, Deserialize)]
struct MessagesWrapper {
    messages: Vec<Message>,
}

pub fn get_image_tasks() -> WCImages {
    let data = fs::read_to_string("src/messages.json").unwrap();
    let msg_wrapper: MessagesWrapper = serde_json::from_str(&data).unwrap();

    let mut tasks: WCImages = vec![];

    for message in msg_wrapper.messages {
        tasks.push(Arc::new(Mutex::new(WCImage::new(
            message.filename.clone(),
            message.url.clone(),
        ))));
    }
    tasks
}
