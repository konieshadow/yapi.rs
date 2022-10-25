use sea_orm_migration::{prelude::*, sea_orm::{Schema, EntityTrait}};
#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let builder = manager.get_database_backend();
        let schema = Schema::new(builder);

        create_table_by_entity(manager, &schema, yapi_entity::models::User).await;
        create_table_by_entity(manager, &schema, yapi_entity::models::Group).await;
        create_table_by_entity(manager, &schema, yapi_entity::models::GroupMember).await;
        create_table_by_entity(manager, &schema, yapi_entity::models::Project).await;
        create_table_by_entity(manager, &schema, yapi_entity::models::ProjectEnv).await;
        create_table_by_entity(manager, &schema, yapi_entity::models::ProjectMember).await;
        create_table_by_entity(manager, &schema, yapi_entity::models::Interface).await;
        create_table_by_entity(manager, &schema, yapi_entity::models::InterFaceCat).await;

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