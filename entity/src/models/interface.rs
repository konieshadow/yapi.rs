use sea_orm::{entity::prelude::*, ConnectionTrait, FromQueryResult, sea_query::{Query, Alias, Expr}, QuerySelect, QueryOrder, ItemsAndPagesNumber};
use serde::{Serialize, Deserialize};
use yapi_common::types::{ReqBodyForm, ReqQuery, ReqHeader, InterfaceDetail, ReqParam, InterfaceInfo, PageList, InterfaceList};
use yapi_common::traits::Paginator;
use yapi_macros::AutoTimestampModel;

use crate::traits::AutoTimestamp;


#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "String(Some(1))")]
pub enum InterfaceStatus {
    #[sea_orm(string_value = "undone")]
    Undone,

    #[sea_orm(string_value = "done")]
    Done,
}

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "String(Some(1))")]
pub enum ReqBodyType {
    #[sea_orm(string_value = "form")]
    Form,

    #[sea_orm(string_value = "json")]
    Json,

    #[sea_orm(string_value = "text")]
    Text,

    #[sea_orm(string_value = "file")]
    File,

    #[sea_orm(string_value = "raw")]
    Raw,
}

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, Serialize, Deserialize, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "String(Some(1))")]
pub enum ResBodyType {
    #[sea_orm(string_value = "json")]
    Json,

    #[sea_orm(string_value = "text")]
    Text,

    #[sea_orm(string_value = "xml")]
    Xml,

    #[sea_orm(string_value = "raw")]
    Raw,

    #[sea_orm(string_value = "json-schema")]
    JsonSchema,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, FromJsonQueryResult)]
pub struct ReqQuerys(pub Vec<ReqQuery>);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, FromJsonQueryResult)]
pub struct ReqHeaders(pub Vec<ReqHeader>);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, FromJsonQueryResult)]
pub struct ReqParams(pub Vec<ReqParam>);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, FromJsonQueryResult)]
pub struct ReqBodyForms(pub Vec<ReqBodyForm>);

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel, AutoTimestampModel)]
#[sea_orm(table_name = "interface")]
pub struct Model {
    #[sea_orm(primary_key, auto_increament = true)]
    pub id: u32,

    pub uid: u32,

    #[sea_orm(indexed)]
    pub project_id: u32,

    pub cat_id: u32,
    pub index: u32,
    pub title: String,
    pub method: String,
    pub path: String,

    #[sea_orm(default_value = "undone")]
    pub status: InterfaceStatus,

    #[sea_orm(default_value = "")]
    pub desc: String,
    #[sea_orm(default_value = "")]
    pub markdown: String,

    #[sea_orm(default_value = "[]")]
    pub req_params: ReqParams,

    #[sea_orm(default_value = "[]")]
    pub req_query: ReqQuerys,

    #[sea_orm(default_value = "[]")]
    pub req_headers: ReqHeaders,

    #[sea_orm(default_value = "form")]
    pub req_body_type: ReqBodyType,

    #[sea_orm(default_value = false)]
    pub req_body_is_json_schema: bool,

    #[sea_orm(default_value = "[]")]
    pub req_body_form: ReqBodyForms,

    #[sea_orm(default_value = "")]
    pub req_body_other: String,

    #[sea_orm(default_value = "json")]
    pub res_body_type: ResBodyType,

    #[sea_orm(default_value = true)]
    pub res_body_is_json_schema: bool,

    #[sea_orm(default_value = "")]
    pub res_body: String,

    #[sea_orm(default_value = true)]
    pub api_opened: bool,

    pub add_time: u32,
    pub up_time: u32,
}

#[derive(Debug, Clone, Copy, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

impl Model {
    pub fn to_interface_detail(self) -> InterfaceDetail {
        InterfaceDetail {
            id: self.id,
            uid: self.uid,
            cat_id: self.cat_id,
            project_id: self.project_id,
            title: self.title,
            method: self.method,
            path: self.path,
            status: self.status.into_value(),
            api_opened: self.api_opened,
            desc: self.desc,
            markdown: self.markdown,
            req_params: self.req_params.0,
            req_header: self.req_headers.0,
            req_query: self.req_query.0,
            req_body_type: self.req_body_type.into_value(),
            req_body_is_json_schema: self.req_body_is_json_schema,
            req_body_form: self.req_body_form.0,
            req_body_other: self.req_body_other,
            res_body_type: self.res_body_type.into_value(),
            res_body_is_json_schema: self.res_body_is_json_schema,
            res_body: self.res_body,
            add_time: self.add_time,
            up_time: self.up_time,
        }
    }
}

#[derive(Debug, Clone, FromQueryResult)]
pub struct InterfaceBaseInfo {
    pub id: u32,
    pub uid: u32,
    pub project_id: u32,
    pub cat_id: u32,
}

impl Entity {
    pub async fn find_max_interface_index<C>(db: &C, project_id: u32, cat_id: u32) -> Result<u32, DbErr>
    where C: ConnectionTrait
    {
        #[derive(FromQueryResult)]
        struct Result {
            index: Option<u32>,
        }

        let mut stmt = Query::select();
        stmt.expr_as(Expr::col(Column::Index).max(), Alias::new("index"))
            .from(Entity)
            .and_where(Column::ProjectId.eq(project_id))
            .and_where(Column::CatId.eq(cat_id));

        let builder = db.get_database_backend();
        let result = Result::find_by_statement(builder.build(&stmt))
            .one(db)
            .await?;

        Ok(result.and_then(|m| m.index).unwrap_or(0))
    }

    pub async fn find_interface_base_info<C>(db: &C, interface_id: u32) -> Result<Option<InterfaceBaseInfo>, DbErr>
    where C: ConnectionTrait
    {
        Entity::find_by_id(interface_id)
            .select_only()
            .column(Column::Id)
            .column(Column::Uid)
            .column(Column::ProjectId)
            .column(Column::CatId)
            .into_model::<InterfaceBaseInfo>()
            .one(db)
            .await
    }

    pub async fn find_interface_info_by_project<C>(db: &C, project_id: u32) -> Result<Vec<InterfaceInfo>, DbErr>
    where C: ConnectionTrait
    {
        Entity::find()
            .select_only()
            .column(Column::Id)
            .column(Column::Uid)
            .column(Column::ProjectId)
            .column(Column::CatId)
            .column(Column::Title)
            .column(Column::Method)
            .column(Column::Path)
            .column(Column::Status)
            .column(Column::ApiOpened)
            .column(Column::AddTime)
            .column(Column::UpTime)
            .filter(Column::ProjectId.eq(project_id))
            .order_by_asc(Column::Index)
            .into_model::<InterfaceInfo>()
            .all(db)
            .await
    }

    pub async fn find_interface_info_page_by_cat<C>(db: &C, project_id: u32, query: InterfaceList) -> Result<PageList<InterfaceInfo>, DbErr>
    where C: ConnectionTrait
    {
        let ItemsAndPagesNumber {
            number_of_items: count,
            number_of_pages: total,
        } = Entity::find()
            .filter(
                Column::ProjectId.eq(project_id)
                    .and(Column::CatId.eq(query.id))
            )
            .paginate(db, query.page_size())
            .num_items_and_pages()
            .await?;

        let list = Entity::find()
            .select_only()
            .column(Column::Id)
            .column(Column::Uid)
            .column(Column::ProjectId)
            .column(Column::CatId)
            .column(Column::Title)
            .column(Column::Method)
            .column(Column::Path)
            .column(Column::Status)
            .column(Column::ApiOpened)
            .column(Column::AddTime)
            .column(Column::UpTime)
            .filter(
                Column::ProjectId.eq(project_id)
                    .and(Column::CatId.eq(query.id))
            )
            .order_by_asc(Column::Index)
            .into_model::<InterfaceInfo>()
            .paginate(db, query.page_size())
            .fetch_page(query.page())
            .await?;

        Ok(PageList::new(count, total, list))
    }

    pub async fn find_interface_info_page_by_project<C>(db: &C, query: InterfaceList) -> Result<PageList<InterfaceInfo>, DbErr>
    where C: ConnectionTrait
    {
        let ItemsAndPagesNumber {
            number_of_items: count,
            number_of_pages: total,
        } = Entity::find()
            .filter(
                Column::ProjectId.eq(query.id)
            )
            .paginate(db, query.page_size())
            .num_items_and_pages()
            .await?;

        let list = Entity::find()
            .select_only()
            .column(Column::Id)
            .column(Column::Uid)
            .column(Column::ProjectId)
            .column(Column::CatId)
            .column(Column::Title)
            .column(Column::Method)
            .column(Column::Path)
            .column(Column::Status)
            .column(Column::ApiOpened)
            .column(Column::AddTime)
            .column(Column::UpTime)
            .filter(
                Column::ProjectId.eq(query.id)
            )
            .order_by_asc(Column::Id)
            .into_model::<InterfaceInfo>()
            .paginate(db, query.page_size())
            .fetch_page(query.page())
            .await?;

        Ok(PageList::new(count, total, list))
    }

    pub async fn find_cat_ids_by_interface_ids<C>(db: &C, interface_ids: &[u32]) -> Result<Vec<u32>, DbErr>
    where C: ConnectionTrait
    {
        #[derive(FromQueryResult)]
        struct Result {
            cat_id: u32,
        }

        let mut stmt = Query::select();
        stmt.column(Column::CatId)
            .distinct()
            .from(Entity)
            .and_where(Column::Id.is_in(interface_ids.to_owned()));

        let builder = db.get_database_backend();
        let list = Result::find_by_statement(builder.build(&stmt))
            .all(db)
            .await?
            .iter()
            .map(|m| m.cat_id)
            .collect();

        Ok(list)
    }
}