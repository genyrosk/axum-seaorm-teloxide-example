use ::entity::{message, message::Entity as Message};
use sea_orm::*;

pub struct Mutation;

impl Mutation {
    pub async fn create_message(
        db: &DbConn,
        data: message::CreateMessage,
    ) -> Result<message::Model, DbErr> {
        message::Entity::insert(data.into_active_model())
            .exec_with_returning(db)
            .await
    }

    pub async fn update_message_by_id(
        db: &DbConn,
        id: i32,
        data: message::UpdateMessage,
    ) -> Result<message::Model, DbErr> {
        let _message = Message::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find message.".to_owned()))?;

        let data = message::ActiveModel {
            id: Unchanged(id),
            text: Set(data.text),
            ..Default::default()
        };
        data.update(db).await
    }

    pub async fn delete_message(db: &DbConn, id: i32) -> Result<DeleteResult, DbErr> {
        let message: message::ActiveModel = Message::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find message.".to_owned()))
            .map(Into::into)?;

        message.delete(db).await
    }

    pub async fn delete_all_messages(db: &DbConn) -> Result<DeleteResult, DbErr> {
        Message::delete_many().exec(db).await
    }
}
