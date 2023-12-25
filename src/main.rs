use model::{group::Group, user::User};
use persist::repository::{Repository, RepositoryImpl};

#[tokio::main]
async fn main() {
    let conn = persist::connect(true).await;
    let repo = RepositoryImpl::new(conn);

    let user = User::new("maru".into());
    let group = Group::new(vec![user.id.clone()]);

    // user
    let user_item = match user.try_into() {
        Ok(item) => item,
        Err(e) => panic!("failed to convert user to item: {}", e),
    };
    match repo.put_item("records".to_string(), user_item).await {
        Ok(_) => {}
        Err(e) => panic!("failed to put item: {}", e),
    }

    // group
    let group_item = match group.try_into() {
        Ok(item) => item,
        Err(e) => panic!("failed to convert group to item: {}", e),
    };
    match repo.put_item("records".to_string(), group_item).await {
        Ok(_) => {}
        Err(e) => panic!("failed to put item: {}", e),
    };
}
