use sea_orm_migration::{prelude::*, sea_orm::{Schema, EntityTrait}};
#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let builder = manager.get_database_backend();
        let schema = Schema::new(builder);

        create_table_by_entity(manager, &schema, yapi_entity::user::Entity).await;
        create_table_by_entity(manager, &schema, yapi_entity::group::Entity).await;
        create_table_by_entity(manager, &schema, yapi_entity::group_member::Entity).await;
        create_table_by_entity(manager, &schema, yapi_entity::project::Entity).await;
        create_table_by_entity(manager, &schema, yapi_entity::project_env::Entity).await;
        create_table_by_entity(manager, &schema, yapi_entity::project_member::Entity).await;
        create_table_by_entity(manager, &schema, yapi_entity::interface::Entity).await;
        create_table_by_entity(manager, &schema, yapi_entity::interface_cat::Entity).await;

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
