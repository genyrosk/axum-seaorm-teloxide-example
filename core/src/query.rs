use ::entity::{message, message::Entity as Message};
use sea_orm::*;
// use sea_orm::{error::DbErr, DbConn, EntityTrait, PaginatorTrait, QueryOrder};

pub struct Query;

impl Query {
    pub async fn find_message_by_id(db: &DbConn, id: i32) -> Result<Option<message::Model>, DbErr> {
        Message::find_by_id(id).one(db).await
    }

    /// If ok, returns (message models, num pages).
    pub async fn find_messages_in_page(
        db: &DbConn,
        page: u64,
        messages_per_page: u64,
    ) -> Result<(Vec<message::Model>, u64), DbErr> {
        // Setup paginator
        let paginator = Message::find()
            .order_by_asc(message::Column::Id)
            .paginate(db, messages_per_page);
        let num_pages = paginator.num_pages().await?;

        // Fetch paginated messages
        paginator.fetch_page(page - 1).await.map(|p| (p, num_pages))
    }
}
