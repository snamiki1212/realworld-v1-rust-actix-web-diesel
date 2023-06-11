use actix_web::HttpResponse;

pub use crate::app::article::response::SingleArticleResponse;
use crate::{
    app::{article::model::Article, tag::model::Tag},
    appv2::features::profile::entities::Profile,
};

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
