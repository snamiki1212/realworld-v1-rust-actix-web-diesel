use super::entities::FavoriteInfo;
use crate::app::features::article::entities::Article;
pub use crate::app::features::article::presenters::SingleArticleResponse;
use crate::app::features::profile::entities::Profile;
use crate::app::features::tag::entities::Tag;
use actix_web::HttpResponse;

pub trait FavoritePresenter: Send + Sync + 'static {
    fn to_single_json(&self, item: (Article, Profile, FavoriteInfo, Vec<Tag>)) -> HttpResponse;
}

#[derive(Clone)]
pub struct FavoritePresenterImpl {}
impl FavoritePresenterImpl {
    pub fn new() -> Self {
        Self {}
    }
}
impl FavoritePresenter for FavoritePresenterImpl {
    fn to_single_json(
        &self,
        (article, profile, favorite_info, tags_list): (Article, Profile, FavoriteInfo, Vec<Tag>),
    ) -> HttpResponse {
        let res_model = SingleArticleResponse::from((article, profile, favorite_info, tags_list));
        HttpResponse::Ok().json(res_model)
    }
}
