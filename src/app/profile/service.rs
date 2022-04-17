use super::model::Profile;
use crate::app::user::model::User;
use crate::error::AppError;
use diesel::pg::PgConnection;

pub struct FetchProfileByName {
    pub current_user: User,
    pub username: String,
}

pub fn fetch_by_name(
    conn: &PgConnection,
    FetchProfileByName {
        current_user,
        username,
    }: &FetchProfileByName,
) -> Result<Profile, AppError> {
    let profile = {
        let followee = User::find_by_username(conn, username)?;
        current_user.fetch_profile(conn, &followee.id)?
    };
    Ok(profile)
}

pub struct ConverUserToProfile<'a> {
    pub user: &'a User,
    pub current_user: &'a Option<User>,
}

pub fn conver_user_to_profile(conn: &PgConnection, params: &ConverUserToProfile) -> Profile {
    let following = match params.current_user.as_ref() {
        Some(me) => me.is_following(conn, &params.user.id),
        None => false,
    };

    Profile {
        username: params.user.username.to_owned(),
        bio: params.user.bio.to_owned(),
        image: params.user.image.to_owned(),
        following,
    }
}
