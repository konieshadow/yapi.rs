use sea_orm_migration::{prelude::*, sea_orm::{Schema, EntityTrait}, async_trait::async_trait};
#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let builder = manager.get_database_backend();
        let schema = Schema::new(builder);

        create_table_by_entity(manager, &schema, yapi_entity::user_entity::Entity).await;
        create_table_by_entity(manager, &schema, yapi_entity::group_entity::Entity).await;
        create_table_by_entity(manager, &schema, yapi_entity::project_entity::Entity).await;
        create_table_by_entity(manager, &schema, yapi_entity::interface_cat_entity::Entity).await;
        create_table_by_entity(manager, &schema, yapi_entity::interface_entity::Entity).await;
        create_table_by_entity(manager, &schema, yapi_entity::group_member_entity::Entity).await;
        create_table_by_entity(manager, &schema, yapi_entity::project_member_entity::Entity).await;
        create_table_by_entity(manager, &schema, yapi_entity::project_env_entity::Entity).await;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        drop_table_by_entity(manager, yapi_entity::user_entity::Entity).await;
        drop_table_by_entity(manager, yapi_entity::group_entity::Entity).await;
        drop_table_by_entity(manager, yapi_entity::project_entity::Entity).await;
        drop_table_by_entity(manager, yapi_entity::interface_cat_entity::Entity).await;
        drop_table_by_entity(manager, yapi_entity::interface_entity::Entity).await;
        drop_table_by_entity(manager, yapi_entity::group_member_entity::Entity).await;
        drop_table_by_entity(manager, yapi_entity::project_member_entity::Entity).await;
        drop_table_by_entity(manager, yapi_entity::project_env_entity::Entity).await;

        Ok(())
    } 
}

async fn create_table_by_entity<E>(manager: &SchemaManager<'_>, schema: &Schema, entity: E)
where E: EntityTrait
{
    manager
        .create_table(
            schema.create_table_from_entity(entity)
        )
        .await.unwrap();

    for statament in schema.create_index_from_entity(entity) {
        manager
            .create_index(statament)
            .await.unwrap();
    }
    
}

async fn drop_table_by_entity<E>(manager: &SchemaManager<'_>, entity: E)
where E: EntityTrait
{
    manager
        .drop_table(Table::drop().table(entity.into_table_ref()).clone())
        .await.unwrap();
}