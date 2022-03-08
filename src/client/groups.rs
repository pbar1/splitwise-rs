use crate::{
    client::Client,
    model::groups::{Group, GroupWrapper, GroupsWrapper},
};

#[derive(Debug)]
pub struct GroupsSvc<'c> {
    client: &'c Client,
}

impl<'c> GroupsSvc<'c> {
    pub fn new(client: &'c Client) -> Self {
        Self { client }
    }

    pub async fn get_groups(&self) -> Result<Vec<Group>, anyhow::Error> {
        let url = self.client.base_url.join("get_groups")?;
        let response: GroupsWrapper = self.client.get(url).await?;
        Ok(response.groups)
    }

    pub async fn get_group(&self, id: i64) -> Result<Group, anyhow::Error> {
        let url = self.client.base_url.join(&format!("get_group/{}", id))?;
        let response: GroupWrapper = self.client.get(url).await?;
        Ok(response.group)
    }
}

#[cfg(test)]
mod integration_tests {
    use test_log::test;

    use super::*;

    #[test(tokio::test)]
    async fn get_groups_works() {
        Client::default().groups().get_groups().await.unwrap();
    }

    #[test(tokio::test)]
    async fn get_group_works() {
        Client::default()
            .groups()
            .get_group(30331347)
            .await
            .unwrap();
    }
}
