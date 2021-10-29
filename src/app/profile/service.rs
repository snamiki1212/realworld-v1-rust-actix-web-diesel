use super::model::Profile;
use crate::app::user::model::User;
use diesel::pg::PgConnection;

pub struct FetchProfile {
    pub me: User,
    pub username: String,
}
pub fn fetch(conn: &PgConnection, params: &FetchProfile) -> Profile {
    let FetchProfile { me, username } = params;
    let followee = User::find_by_username(&conn, username).expect("couldn't find user.");
    let is_following = me.is_following(&conn, &followee.id);
    let profile = Profile {
        username: me.username.to_owned(),
        bio: me.bio.to_owned(),
        image: me.image.to_owned(),
        following: is_following,
    };
    profile
}
