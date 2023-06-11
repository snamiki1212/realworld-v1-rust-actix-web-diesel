use super::entities::Profile;
use crate::app::user::model::User;
use diesel::pg::PgConnection;

#[deprecated(note = "use repository")]
pub struct ConverUserToProfile<'a> {
    pub user: &'a User,
    pub current_user: &'a Option<User>,
}

#[deprecated(note = "use repository")]
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
