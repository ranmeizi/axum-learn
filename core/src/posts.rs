use common::entity::post;
use sea_orm::{DatabaseConnection,Set,ActiveModelTrait};
use anyhow::{self, Ok};

pub async fn create_first_post(db: &DatabaseConnection, title: String)->anyhow::Result<post::Model> {
    let post = post::ActiveModel {
        title: Set(title),
        text:Set("my first post".to_owned()),
        ..Default::default() // all other attributes are `NotSet`
    };

    let post: post::Model = post.insert(db).await?;

    Ok(post)
}
