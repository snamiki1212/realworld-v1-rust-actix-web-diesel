use crate::app::drivers::middlewares::{auth, state::AppState};
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
    state
        .di_container
        .profile_usecase
        .fetch_profile_by_name(&current_user, &username)
}

pub async fn follow(
    state: web::Data<AppState>,
    req: HttpRequest,
    path: web::Path<UsernameSlug>,
) -> ApiResponse {
    let current_user = auth::get_current_user(&req)?;
    let target_username = path.into_inner();
    state
        .di_container
        .profile_usecase
        .follow_user(&current_user, &target_username)
}

pub async fn unfollow(
    state: web::Data<AppState>,
    req: HttpRequest,
    path: web::Path<UsernameSlug>,
) -> ApiResponse {
    let current_user = auth::get_current_user(&req)?;
    let target_username = path.into_inner();
    state
        .di_container
        .profile_usecase
        .unfollow_user(&current_user, &target_username)
}
