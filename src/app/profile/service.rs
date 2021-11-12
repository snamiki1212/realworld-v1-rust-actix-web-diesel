use super::model::Profile;
use crate::app::user::model::User;
use crate::error::AppError;
use diesel::pg::PgConnection;
use uuid::Uuid;

pub struct FetchProfileByName {
    pub me: User,
    pub username: String,
}
pub fn fetch_by_name(
    conn: &PgConnection,
    params: &FetchProfileByName,
) -> Result<Profile, AppError> {
    let FetchProfileByName { me, username } = params;
    let followee = User::find_by_username(&conn, username)?;
    let profile = fetch_profile_by_id(
        &conn,
        &FetchProfileById {
            user: me.to_owned(),
            id: followee.id,
        },
    )?;
    Ok(profile)
}

pub struct FetchProfileById {
    pub user: User,
    pub id: Uuid,
}
pub fn fetch_profile_by_id(
    conn: &PgConnection,
    params: &FetchProfileById,
) -> Result<Profile, AppError> {
    let FetchProfileById { user, id } = params;
    let is_following = user.is_following(&conn, id);
    let profile = Profile {
        username: user.username.to_owned(),
        bio: user.bio.to_owned(),
        image: user.image.to_owned(),
        following: is_following,
    };
    Ok(profile)
}

pub struct ConverUserToProfile<'a> {
    pub user: &'a User,
    pub me: &'a Option<User>,
}
pub fn conver_user_to_profile(conn: &PgConnection, params: &ConverUserToProfile) -> Profile {
    let following = match params.me.as_ref() {
        Some(me) => me.is_following(conn, &params.user.id),
        None => false,
    };
    let profile = Profile {
        username: params.user.username.to_owned(),
        bio: params.user.bio.to_owned(),
        image: params.user.image.to_owned(),
        following,
    };
    profile
}
