use super::super::entities::profile::Profile;
use crate::app::user::model::User;
use diesel::pg::PgConnection;

pub struct ConverUserToProfile<'a> {
    pub user: &'a User,
    pub current_user: &'a Option<User>,
}

pub fn conver_user_to_profile(conn: &mut PgConnection, params: &ConverUserToProfile) -> Profile {
    let following = match params.current_user.as_ref() {
        Some(current_user) => current_user.is_following(conn, &params.user.id),
        None => false,
    };

    Profile {
        username: params.user.username.to_owned(),
        bio: params.user.bio.to_owned(),
        image: params.user.image.to_owned(),
        following,
    }
}
