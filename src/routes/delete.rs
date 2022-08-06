use actix_web::{web, Error, HttpRequest, HttpResponse};
use actix_web::http::header;

use crate::prelude::*;

pub async fn delete<T: ActixAdminAppDataTrait, E: ActixAdminViewModelTrait>(
    _req: HttpRequest,
    data: web::Data<T>,
    _text: String,
    id: web::Path<i32>
) -> Result<HttpResponse, Error> {
    let db = &data.get_db();

    let _result = E::delete_entity(db, id.into_inner()).await;

    Ok(HttpResponse::Ok()
        .finish())
}

pub async fn delete_many<T: ActixAdminAppDataTrait, E: ActixAdminViewModelTrait>(
    _req: HttpRequest,
    data: web::Data<T>,
    text: String,
) -> Result<HttpResponse, Error> {
    let db = &data.get_db();
    let entity_name = E::get_entity_name();
    let entity_ids: Vec<i32> = text
        .split("&")
        .filter(|id| !id.is_empty())
        .map(|id_str| id_str.replace("ids=", "").parse::<i32>().unwrap()
    ).collect();
    
    // TODO: implement delete_many
    for id in entity_ids {
        let _result = E::delete_entity(db, id).await;
    }
    
    Ok(HttpResponse::SeeOther()
    .append_header((
        header::LOCATION,
        format!("/admin/{}/list?render_partial=true", entity_name),
    ))
    .finish())
}