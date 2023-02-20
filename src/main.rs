use reqwest;
use serde::{Serialize, Deserialize};
use std::{env, time};
// Define the events that will be sent to the server
#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
enum EventType {
    PageView,
    PageReady,
    AutoPlay,
    UserPlay,
    PauseVideo,
    StopVideo,
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
struct Event {
    event_type: EventType,
    timestamp: u64,
}

#[tokio::main]
async fn main() {
    // Set up the configuration options
    let server_url:String = env::args().nth(1).expect("Missing server URL argument");
    // let server_url = "https://example.com/analytics";
    let num_users = env::args().nth(2).unwrap_or("10".to_string()).parse::<u32>().unwrap();
    let events_per_user = 5;

    // Start simulating users
    let mut handles = vec![];
    for user_id in 0..num_users {
        let server_url = server_url.clone();
        let handle = tokio::spawn(async move {
            // Simulate the user's activity
            for event_num in 0..events_per_user {
                // Wait for a random amount of time to simulate realistic user behavior
                let sleep_time = time::Duration::from_millis(rand::random::<u64>() % 1000);
                tokio::time::sleep(sleep_time).await;

                // Generate a random event
                let event = match event_num {
                    0 => Event { event_type: EventType::PageView, timestamp: get_timestamp() },
                    1 => Event { event_type: EventType::PageReady, timestamp: get_timestamp() },
                    2 => Event { event_type: EventType::AutoPlay, timestamp: get_timestamp() },
                    3 => Event { event_type: EventType::UserPlay, timestamp: get_timestamp() },
                    4 => Event { event_type: EventType::PauseVideo, timestamp: get_timestamp() },
                    _ => Event { event_type: EventType::StopVideo, timestamp: get_timestamp() },
                };

                // Send the event to the server
                match send_event_to_server(&server_url, event, user_id).await {
                    Ok(_) => println!("Sent event {:?} for user {} successfully", event.event_type, user_id),
                    Err(err) => println!("Failed to send event {:?} for user {}: {}", event.event_type, user_id, err),
                }
            }
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.await.unwrap();
    }
}

async fn send_event_to_server(server_url: &str, event: Event, user_id: u32) -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let url = format!("{}/user/{}/event", server_url, user_id);
    let res = client.post(&url)
        .json(&event)
        .send()
        .await?;

    if res.status().is_success() {
        Ok(())
    } else {
        Err("Failed to send event".into())
    }
}

fn get_timestamp() -> u64 {
    let now = time::SystemTime::now();
    now.duration_since(time::UNIX_EPOCH).unwrap().as_secs()
}
