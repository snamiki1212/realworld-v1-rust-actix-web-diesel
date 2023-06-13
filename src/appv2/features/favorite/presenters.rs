use super::entities::FavoriteInfo;
use crate::appv2::features::article::entities::Article;
pub use crate::appv2::features::article::presenters::SingleArticleResponse;
use crate::appv2::features::profile::entities::Profile;
use crate::appv2::features::tag::entities::Tag;
use actix_web::HttpResponse;

pub trait FavoritePresenter: Send + Sync + 'static {
    fn complete(&self, item: (Article, Profile, FavoriteInfo, Vec<Tag>)) -> HttpResponse;
}

#[derive(Clone)]
pub struct FavoritePresenterImpl {}
impl FavoritePresenterImpl {
    pub fn new() -> Self {
        Self {}
    }
}
impl FavoritePresenter for FavoritePresenterImpl {
    fn complete(
        &self,
        (article, profile, favorite_info, tags_list): (Article, Profile, FavoriteInfo, Vec<Tag>),
    ) -> HttpResponse {
        let res_model = SingleArticleResponse::from((article, profile, favorite_info, tags_list));
        HttpResponse::Ok().json(res_model)
    }
}
