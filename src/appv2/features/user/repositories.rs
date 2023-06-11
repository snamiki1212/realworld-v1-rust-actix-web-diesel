use crate::app::follow::model::{CreateFollow, DeleteFollow, Follow};
use crate::appv2::features::profile::entities::Profile;
use crate::appv2::features::user::entities::User;
use crate::error::AppError;
use crate::utils::db::DbPool;

type Token = String;

#[derive(Clone)]
pub struct UserRepository {
    pool: DbPool,
}

impl UserRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }

    pub fn signin(&self, email: &str, naive_password: &str) -> Result<(User, Token), AppError> {
        let conn = &mut self.pool.get()?;
        User::signin(conn, email, naive_password)
    }

    pub fn signup(
        &self,
        email: &str,
        username: &str,
        naive_password: &str,
    ) -> Result<(User, Token), AppError> {
        let conn = &mut self.pool.get()?;
        User::signup(conn, email, username, naive_password)
    }

    pub fn follow(&self, current_user: &User, target_username: &str) -> Result<Profile, AppError> {
        let conn = &mut self.pool.get()?;
        let t = User::by_username(target_username);

        let followee = {
            use diesel::prelude::*;
            t.first::<User>(conn)?
        };

        Follow::create(
            conn,
            &CreateFollow {
                follower_id: current_user.id,
                followee_id: followee.id,
            },
        )?;

        Ok(Profile {
            username: current_user.username.clone(),
            bio: current_user.bio.clone(),
            image: current_user.image.clone(),
            following: true,
        })
    }

    pub fn unfollow(
        &self,
        current_user: &User,
        target_username: &str,
    ) -> Result<Profile, AppError> {
        let conn = &mut self.pool.get()?;
        let t = User::by_username(target_username);
        let followee = {
            use diesel::prelude::*;
            t.first::<User>(conn)?
        };

        Follow::delete(
            conn,
            &DeleteFollow {
                followee_id: followee.id,
                follower_id: current_user.id,
            },
        )?;

        Ok(Profile {
            username: current_user.username.clone(),
            bio: current_user.bio.clone(),
            image: current_user.image.clone(),
            following: false,
        })
    }
}
