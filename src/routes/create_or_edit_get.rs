use actix_web::{error, web, Error, HttpRequest, HttpResponse};
use sea_orm::DatabaseConnection;
use tera::{Context};
use actix_session::{Session};
use crate::ActixAdminError;
use crate::ActixAdminNotification;
use crate::prelude::*;

use super::DEFAULT_ENTITIES_PER_PAGE;
use super::Params;
use super::{ add_auth_context, user_can_access_page, render_unauthorized};

pub async fn create_get<E: ActixAdminViewModelTrait>(
    session: Session,
    req: HttpRequest,
    data: web::Data<ActixAdmin>,
    db: web::Data<DatabaseConnection>,
    _body: web::Payload,
    _text: String,
) -> Result<HttpResponse, Error> {
    let db = db.get_ref();
    let model = ActixAdminModel::create_empty();
    
    create_or_edit_get::<E>(&session, req, &data, db, Ok(model)).await
}

pub async fn edit_get<E: ActixAdminViewModelTrait>(
    session: Session,
    req: HttpRequest,
    data: web::Data<ActixAdmin>,
    db: web::Data<DatabaseConnection>,
    _text: String,
    id: web::Path<i32>
) -> Result<HttpResponse, Error> {
    let db = db.get_ref();
    let model = E::get_entity(db, id.into_inner()).await;

    create_or_edit_get::<E>(&session, req, &data, db, model).await
}

async fn create_or_edit_get<E: ActixAdminViewModelTrait>(session: &Session, req: HttpRequest, data: &web::Data<ActixAdmin>, db: &sea_orm::DatabaseConnection, model_result: Result<ActixAdminModel, ActixAdminError>) -> Result<HttpResponse, Error>{
    let actix_admin = &data.get_ref();
    let mut ctx = Context::new();
    add_auth_context(&session, actix_admin, &mut ctx);
    let entity_names = &actix_admin.entity_names;
    ctx.insert("entity_names", entity_names);
    let entity_name = E::get_entity_name();
    let mut errors: Vec<crate::ActixAdminError> = Vec::new();

    let view_model = actix_admin.view_models.get(&entity_name).unwrap();

    if !user_can_access_page(&session, actix_admin, view_model) {
        return render_unauthorized(&ctx, &actix_admin);
    }

    let model;
    match model_result {
        Ok(res) => {
            model = res;
        },
        Err(e) => {
            #[cfg(feature="enable-tracing")]
            tracing::error!("{e}");
            errors.push(e);
            model = ActixAdminModel::create_empty();
        }
    }

    let mut http_response_code = match errors.is_empty() {
        true => HttpResponse::Ok(),
        false => HttpResponse::InternalServerError(),
    };    
    let notifications: Vec<ActixAdminNotification> = errors.into_iter()
        .map(|err| ActixAdminNotification::from(err))
        .collect();

    let params = web::Query::<Params>::from_query(req.query_string()).unwrap();

    let page = params.page.unwrap_or(1);
    let entities_per_page = params
        .entities_per_page
        .unwrap_or(DEFAULT_ENTITIES_PER_PAGE);
    let render_partial = req.headers().contains_key("HX-Target");
    let search = params.search.clone().unwrap_or(String::new());
    let sort_by = params.sort_by.clone().unwrap_or(view_model.primary_key.to_string());
    let sort_order = params.sort_order.as_ref().unwrap_or(&SortOrder::Asc);

    ctx.insert("view_model", &ActixAdminViewModelSerializable::from(view_model.clone()));
    ctx.insert("select_lists", &E::get_select_lists(db).await?);
    ctx.insert("base_path", &E::get_base_path(&entity_name));
    ctx.insert("model", &model);
    ctx.insert("notifications", &notifications);
    ctx.insert("entities_per_page", &entities_per_page);
    ctx.insert("render_partial", &render_partial);
    ctx.insert("search", &search);
    ctx.insert("sort_by", &sort_by);
    ctx.insert("sort_order", &sort_order);
    ctx.insert("page", &page);
    
    let body = actix_admin.tera
        .render("create_or_edit.html", &ctx)
        .map_err(|err| {
            #[cfg(feature="enable-tracing")]
            tracing::error!("{err}");
            error::ErrorInternalServerError(err)
        })?;
    Ok(http_response_code.content_type("text/html").body(body))
}
