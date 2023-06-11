use super::presenters::TagPresenter;
use super::repositories::TagRepository;
use crate::error::AppError;
use actix_web::HttpResponse;

#[derive(Clone)]
pub struct TagUsecase {
    tag_repository: TagRepository,
    tag_presenter: TagPresenter,
}

impl TagUsecase {
    pub fn new(tag_repository: TagRepository, tag_presenter: TagPresenter) -> Self {
        Self {
            tag_repository,
            tag_presenter,
        }
    }

    pub fn list(&self) -> Result<HttpResponse, AppError> {
        let list = self.tag_repository.list()?;
        let res = self.tag_presenter.from_list(list);
        Ok(res)
    }
}
