use actix_web::HttpResponse;

use crate::appv2::features::article::entities::Article;
pub use crate::appv2::features::article::presenters::SingleArticleResponse;
use crate::appv2::features::profile::entities::Profile;
use crate::appv2::features::tag::entities::Tag;

use super::entities::FavoriteInfo;

#[derive(Clone)]
pub struct FavoritePresenter {}
impl FavoritePresenter {
    pub fn new() -> Self {
        Self {}
    }

    pub fn complete(
        &self,
        (article, profile, favorite_info, tags_list): (Article, Profile, FavoriteInfo, Vec<Tag>),
    ) -> HttpResponse {
        let res_model = SingleArticleResponse::from((article, profile, favorite_info, tags_list));
        HttpResponse::Ok().json(res_model)
    }
}
