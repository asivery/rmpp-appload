use appload_client::{AppLoad, AppLoadBackend, BackendReplier, Message, MSG_SYSTEM_NEW_COORDINATOR};
use async_trait::async_trait;

#[tokio::main]
async fn main() {
    AppLoad::new(&mut MyBackend).unwrap();
}

struct MyBackend;

#[async_trait]
impl AppLoadBackend for MyBackend {
    async fn handle_message(&mut self, functionality: &BackendReplier, message: Message) {
        match message.msg_type {
            MSG_SYSTEM_NEW_COORDINATOR => {
                println!("A frontend has connected")
            }
            1 => {
                println!("Backend got a message: {}", &message.contents);
                functionality.send_message(
                    101,
                    &format!("Backend responds to message {}", message.contents)
                ).unwrap();
            }
            _ => println!("Unknown message received.")
        }
    }
}
