use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = std::env::var("SPLITWISE_API_KEY").unwrap();

    let client = splitwise::Client::new_default_http_client(api_key);

    let cur_user = client.clone().users().get_current_user().await?;
    println!(
        "Current user: {} {}",
        cur_user.user.first_name, cur_user.user.last_name
    );

    let id = cur_user.user.id;

    let got_user = client.clone().users().get_user(id.to_string()).await?;
    println!(
        "Got user: {} {}",
        got_user.user.first_name, got_user.user.last_name
    );

    let update_user = splitwise::users::UpdateUserRequest {
        last_name: Some(got_user.clone().user.first_name),
        email: None,
        password: None,
        locale: None,
        first_name: Some(got_user.clone().user.last_name),
        default_currency: None,
    };
    let updated_user = client.clone().users().update_user(id, update_user).await?;
    println!(
        "Updated user: {} {}",
        updated_user.user.first_name, updated_user.user.last_name
    );

    Ok(())
}
