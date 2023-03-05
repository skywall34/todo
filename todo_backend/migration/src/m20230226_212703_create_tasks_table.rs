use sea_orm_migration::prelude::*;

use crate::m20230305_164028_create_users_table::Users;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20230226_212703_create_tasks_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                sea_query::Table::create()
                    .table(Tasks::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Tasks::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Tasks::Creator).string().not_null())
                    .col(ColumnDef::new(Tasks::Title).string().not_null())
                    .col(ColumnDef::new(Tasks::Body).string().not_null())
                    .col(ColumnDef::new(Tasks::Completed).boolean().not_null())
                    .col(ColumnDef::new(Tasks::UserId).integer().default(Value::Int(None)))
                    .foreign_key(
                        ForeignKey::create()
                            .name("user_id")
                            .from(Tasks::Table, Tasks::UserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade)
    
                    )
                    .to_owned()
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                sea_query::Table::drop()
                    .table(Tasks::Table)
                    .to_owned()
            )
            .await
    }
}

#[derive(Iden)]
pub enum Tasks {
    Table,
    Id,
    Creator,
    Title,
    Body,
    Completed,
    UserId
}