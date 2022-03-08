use splitwise::client::Client;

#[tokio::main]
async fn main() {
    let user = Client::default().users().get_current_user().await.unwrap();
    println!("Current user ID: {}", user.id.unwrap())
}
