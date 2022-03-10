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

    pub async fn list_groups(&self) -> Result<Vec<Group>, anyhow::Error> {
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
    use log::debug;
    use test_log::test;

    use super::*;
    use crate::model::users::User;

    #[test(tokio::test)]
    async fn list_get_group_works() {
        let client = Client::default();

        let list = client.groups().list_groups().await.unwrap();
        debug!("list: {:?}", list);
        let id = list.first().unwrap().id.unwrap();

        let get = client.groups().get_group(id).await.unwrap();
        debug!("get: {:?}", get);
        assert_eq!(id, get.id.unwrap());
    }

    #[test(tokio::test)]
    async fn create_delete_restore_group_works() {
        let client = Client::default();

        let name = "fake-group-1".to_string();

        let create = client
            .groups()
            .create_group(CreateGroupRequest {
                name: name.clone(),
                group_type: Some("apartment".to_string()),
                simplify_by_default: Some(true),
                users: None,
            })
            .await
            .unwrap();
        debug!("create: {:?}", create);
        assert_eq!(create.name.unwrap(), name);

        let id = create.id.unwrap();

        let delete = client.groups().delete_group(id).await.unwrap();
        debug!("delete: {:?}", delete);
        assert!(delete.success);

        let restore = client.groups().restore_group(id).await.unwrap();
        debug!("restore: {:?}", restore);
        assert!(restore.success);

        let delete = client.groups().delete_group(id).await.unwrap();
        debug!("delete: {:?}", delete);
        assert!(delete.success);
    }

    #[test(tokio::test)]
    async fn add_remove_user_from_group_works() {
        let client = Client::default();

        let list = client.groups().list_groups().await.unwrap();
        debug!("list: {:#?}", list);

        // Find a group that is not "Non-group expenses"
        let mut group: Option<Group> = None;
        for g in list {
            if g.id.unwrap() != 0 {
                group = Some(g);
            }
        }
        debug!("group: {:#?}", group);
        let group_id = group.clone().unwrap().id.unwrap();

        // Find a user that is not ourself
        let own_user_id = client.users().get_current_user().await.unwrap().id.unwrap();
        let mut user: Option<User> = None;
        for u in group.unwrap().members.unwrap() {
            if u.id.unwrap() != own_user_id {
                user = Some(u);
            }
        }
        debug!("user: {:#?}", user);
        let user_id = user.unwrap().id.unwrap();

        let removed = client
            .groups()
            .remove_user_from_group(RemoveUserFromGroupRequest { group_id, user_id })
            .await
            .unwrap();
        debug!("removed: {:?}", removed);
        assert!(removed.success);

        let added = client
            .groups()
            .add_user_to_group(AddUserToGroupRequest {
                group_id,
                user_id: Some(user_id),
                ..AddUserToGroupRequest::default()
            })
            .await
            .unwrap();
        debug!("added: {:?}", added);
        assert!(added.success);
    }
}
