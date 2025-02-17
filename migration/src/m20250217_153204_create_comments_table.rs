use crate::m20220101_000001_create_table::User;
use crate::m20250217_153113_create_posts_table::Post;
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Comment::Table)
                    .if_not_exists()
                    .col(pk_auto(Comment::Id))
                    .col(ColumnDef::new(Comment::UserId).integer().not_null())
                    .col(ColumnDef::new(Comment::Text).string().not_null())
                    .col(ColumnDef::new(Comment::PostId).integer().not_null())
                    .col(ColumnDef::new(Comment::CreatedAt).date_time().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-comments-user-id")
                            .from(Comment::Table, Comment::UserId)
                            .to(User::Table, User::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-comments-post-id")
                            .from(Comment::Table, Comment::PostId)
                            .to(Post::Table, Post::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Comment::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Comment {
    Table,
    Id,
    UserId,
    PostId,
    CreatedAt,
    Text,
}
