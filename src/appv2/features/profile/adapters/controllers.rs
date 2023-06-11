use crate::appv2::drivers::middlewares::{auth, state::AppState};
use crate::utils::api::ApiResponse;
use actix_web::{web, HttpRequest};

type UsernameSlug = String;

pub async fn show(
    state: web::Data<AppState>,
    req: HttpRequest,
    path: web::Path<UsernameSlug>,
) -> ApiResponse {
    let current_user = auth::get_current_user(&req)?;
    let username = path.into_inner();
    let res = state
        .di_container
        .profile_usecase
        .show(&current_user, &username)?;
    Ok(res)
}

pub async fn follow(
    state: web::Data<AppState>,
    req: HttpRequest,
    path: web::Path<UsernameSlug>,
) -> ApiResponse {
    let current_user = auth::get_current_user(&req)?;
    let target_username = path.into_inner();
    let res = state
        .di_container
        .profile_usecase
        .follow(&current_user, &target_username)?;
    Ok(res)
}

pub async fn unfollow(
    state: web::Data<AppState>,
    req: HttpRequest,
    path: web::Path<UsernameSlug>,
) -> ApiResponse {
    let current_user = auth::get_current_user(&req)?;
    let target_username = path.into_inner();
    let res = state
        .di_container
        .profile_usecase
        .unfollow(&current_user, &target_username)?;
    Ok(res)
}
