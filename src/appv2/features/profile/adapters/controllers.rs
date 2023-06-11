use super::super::domains::profile_repository::ProfileRepository;
use super::super::usecases::show_profile_usecase::ProfileUsecase;
use super::presenters::{ProfilePresenter, ProfileResponse};
use crate::appv2::drivers::middlewares::{auth, state::AppState};
use crate::appv2::features::user::domains::user_repository::UserRepository;
use crate::utils::api::ApiResponse;
use actix_web::{web, HttpRequest, HttpResponse};

type UsernameSlug = String;

pub async fn show(
    state: web::Data<AppState>,
    req: HttpRequest,
    path: web::Path<UsernameSlug>,
) -> ApiResponse {
    let repositories = {
        let profileRepository = ProfileRepository::new(state.pool.clone()); // TODO: move to DI container.
        let userRepository = UserRepository::new(state.pool.clone()); // TODO: move to DI container.
        (profileRepository, userRepository)
    };
    let presenter = ProfilePresenter::new(); // TODO: move to DI container.
    let usecase = ProfileUsecase::new(repositories, presenter); // TODO: move to DI container.

    let profile = {
        let current_user = auth::get_current_user(&req)?;
        let username = path.into_inner();
        usecase.show(&current_user, &username)?
    };
    Ok(HttpResponse::Ok().json(profile))
}

pub async fn follow(
    state: web::Data<AppState>,
    req: HttpRequest,
    path: web::Path<UsernameSlug>,
) -> ApiResponse {
    let repositories = {
        let profileRepository = ProfileRepository::new(state.pool.clone()); // TODO: move to DI container.
        let userRepository = UserRepository::new(state.pool.clone()); // TODO: move to DI container.
        (profileRepository, userRepository)
    };
    let presenter = ProfilePresenter::new(); // TODO: move to DI container.
    let usecase = ProfileUsecase::new(repositories, presenter); // TODO: move to DI container.

    let profile = {
        let current_user = auth::get_current_user(&req)?;
        let username = path.into_inner();
        usecase.follow(&current_user, &username)?
    };
    Ok(HttpResponse::Ok().json(profile))
}

pub async fn unfollow(
    state: web::Data<AppState>,
    req: HttpRequest,
    path: web::Path<UsernameSlug>,
) -> ApiResponse {
    let conn = &mut state.get_conn()?;
    let current_user = auth::get_current_user(&req)?;
    let username = path.into_inner();
    let profile = current_user.unfollow(conn, &username)?;
    let res = ProfileResponse::from(profile);
    Ok(HttpResponse::Ok().json(res))
}
