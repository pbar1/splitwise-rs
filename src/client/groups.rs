use crate::{
    client::Client,
    model::groups::{
        AddUserToGroupRequest, AddUserToGroupResponse, CreateGroupRequest, DeleteGroupResponse,
        Group, GroupWrapper, GroupsWrapper, RemoveUserFromGroupRequest,
        RemoveUserFromGroupResponse, RestoreGroupResponse,
    },
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

    pub async fn create_group(&self, request: CreateGroupRequest) -> Result<Group, anyhow::Error> {
        let url = self.client.base_url.join("create_group")?;
        let response: GroupWrapper = self.client.post(url, &request).await?;
        Ok(response.group)
    }

    pub async fn delete_group(&self, id: i64) -> Result<DeleteGroupResponse, anyhow::Error> {
        let url = self.client.base_url.join(&format!("delete_group/{}", id))?;
        let response: DeleteGroupResponse = self.client.post_no_body(url).await?;
        Ok(response)
    }

    pub async fn restore_group(&self, id: i64) -> Result<RestoreGroupResponse, anyhow::Error> {
        let url = self
            .client
            .base_url
            .join(&format!("undelete_group/{}", id))?;
        let response: RestoreGroupResponse = self.client.post_no_body(url).await?;
        Ok(response)
    }

    pub async fn add_user_to_group(
        &self,
        request: AddUserToGroupRequest,
    ) -> Result<AddUserToGroupResponse, anyhow::Error> {
        let url = self.client.base_url.join("add_user_to_group")?;
        let response: AddUserToGroupResponse = self.client.post(url, &request).await?;
        Ok(response)
    }

    pub async fn remove_user_from_group(
        &self,
        request: RemoveUserFromGroupRequest,
    ) -> Result<RemoveUserFromGroupResponse, anyhow::Error> {
        let url = self.client.base_url.join("remove_user_from_group")?;
        let response: RemoveUserFromGroupResponse = self.client.post(url, &request).await?;
        Ok(response)
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

    #[test(tokio::test)]
    async fn create_delete_restore_group_works() {
        let client = Client::default();
        let group = client
            .groups()
            .create_group(CreateGroupRequest {
                name: "fake-group-1".to_string(),
                group_type: Some("apartment".to_string()),
                simplify_by_default: Some(true),
                users: None,
            })
            .await
            .unwrap();
        let id = group.id.unwrap();
        client.groups().delete_group(id).await.unwrap();
        client.groups().restore_group(id).await.unwrap();
        client.groups().delete_group(id).await.unwrap();
    }

    // TODO: Re-enable this test with a user that the test user is friends with
    // #[test(tokio::test)]
    // async fn add_remove_user_from_group_works() {
    //     let client = Client::default();
    //     let group_id = 30331347;
    //     let user_id = 9239275;
    //
    //     let _add_resp = client
    //         .groups()
    //         .add_user_to_group(AddUserToGroupRequest {
    //             group_id,
    //             user_id: Some(user_id),
    //             ..AddUserToGroupRequest::default()
    //         })
    //         .await
    //         .unwrap();
    //     let _remove_resp = client
    //         .groups()
    //         .remove_user_from_group(RemoveUserFromGroupRequest { group_id, user_id })
    //         .await
    //         .unwrap();
    // }
}
