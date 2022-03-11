use crate::{
    client::Client,
    model::groups::{
        Group, GroupAddUserRequest, GroupAddUserResponse, GroupCreateRequest, GroupDeleteResponse,
        GroupRemoveUserRequest, GroupRemoveUserResponse, GroupRestoreResponse, GroupUser,
        GroupWrapper, GroupsWrapper,
    },
};

/// A Group represents a collection of users who share expenses together. For
/// example, some users use a Group to aggregate expenses related to an
/// apartment. Others use it to represent a trip. Expenses assigned to a group
/// are split among the users of that group. Importantly, two users in a Group
/// can also have expenses with one another outside of the Group.
///
/// [Splitwise API docs](https://dev.splitwise.com/#tag/groups)
#[derive(Debug)]
pub struct GroupsSvc<'c> {
    client: &'c Client,
}

impl<'c> GroupsSvc<'c> {
    /// Creates an instance of `GroupsSvc`.
    pub fn new(client: &'c Client) -> Self {
        Self { client }
    }

    /// List the current user's groups.
    ///
    /// **Note:** Expenses that are not associated with a group are listed in a
    /// group with ID 0.
    ///
    /// [Splitwise API docs](https://dev.splitwise.com/#tag/groups/paths/~1get_groups/get)
    pub async fn list_groups(&self) -> Result<Vec<Group>, anyhow::Error> {
        let url = self.client.base_url.join("get_groups")?;
        let response: GroupsWrapper = self.client.get(url).await?;
        Ok(response.groups)
    }

    /// Get information about a group.
    ///
    /// [Splitwise API docs](https://dev.splitwise.com/#tag/groups/paths/~1get_group~1{id}/get)
    pub async fn get_group(&self, id: i64) -> Result<Group, anyhow::Error> {
        let url = self.client.base_url.join(&format!("get_group/{}", id))?;
        let response: GroupWrapper = self.client.get(url).await?;
        Ok(response.group)
    }

    /// Creates a new group. Adds the current user to the group by default.
    ///
    /// **Note:** The user's email or ID must be provided.
    ///
    /// [Splitwise API docs](https://dev.splitwise.com/#tag/groups/paths/~1create_group/post)
    pub async fn create_group(&self, request: GroupCreateRequest) -> Result<Group, anyhow::Error> {
        let url = self.client.base_url.join("create_group")?;
        let response: GroupWrapper = self.client.post(url, &request).await?;
        Ok(response.group)
    }

    /// Delete an existing group. Destroys all associated records (expenses,
    /// etc).
    ///
    /// [Splitwise API docs](https://dev.splitwise.com/#tag/groups/paths/~1delete_group~1{id}/post)
    pub async fn delete_group(&self, id: i64) -> Result<GroupDeleteResponse, anyhow::Error> {
        let url = self.client.base_url.join(&format!("delete_group/{}", id))?;
        let response: GroupDeleteResponse = self.client.post_no_body(url).await?;
        Ok(response)
    }

    /// Restores a deleted group.
    ///
    /// **Note:** You must check the success value of the response.
    ///
    /// [Splitwise API docs](https://dev.splitwise.com/#tag/groups/paths/~1undelete_group~1{id}/post)
    pub async fn restore_group(&self, id: i64) -> Result<GroupRestoreResponse, anyhow::Error> {
        let url = self
            .client
            .base_url
            .join(&format!("undelete_group/{}", id))?;
        let response: GroupRestoreResponse = self.client.post_no_body(url).await?;
        Ok(response)
    }

    /// Add a user to a group.
    ///
    /// **Note:** You must check the success value of the response.
    ///
    /// [Splitwise API docs](https://dev.splitwise.com/#tag/groups/paths/~1add_user_to_group/post)
    pub async fn add_user_to_group(
        &self,
        group_id: i64,
        user: GroupUser,
    ) -> Result<GroupAddUserResponse, anyhow::Error> {
        let url = self.client.base_url.join("add_user_to_group")?;
        let request = GroupAddUserRequest {
            group_id,
            user_id: user.user_id,
            first_name: user.first_name,
            last_name: user.last_name,
            email: user.email,
        };
        let response: GroupAddUserResponse = self.client.post(url, &request).await?;
        Ok(response)
    }

    /// Remove a user from a group. Does not succeed if the user has a non-zero
    /// balance.
    ///
    /// **Note:** You must check the success value of the response.
    ///
    /// [Splitwise API docs](https://dev.splitwise.com/#tag/groups/paths/~1remove_user_from_group/post)
    pub async fn remove_user_from_group(
        &self,
        group_id: i64,
        user_id: i64,
    ) -> Result<GroupRemoveUserResponse, anyhow::Error> {
        let url = self.client.base_url.join("remove_user_from_group")?;
        let request = GroupRemoveUserRequest { user_id, group_id };
        let response: GroupRemoveUserResponse = self.client.post(url, &request).await?;
        Ok(response)
    }
}

#[cfg(test)]
mod integration_tests {
    use log::debug;
    use test_log::test;

    use super::*;

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
            .create_group(GroupCreateRequest {
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
}
