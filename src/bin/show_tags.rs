extern crate conduit;
extern crate diesel;

use conduit::establish_connection;

use self::diesel::prelude::*;
use conduit::models::*;

fn main() {
    use conduit::schema::tags::dsl::*;

    let connection = establish_connection();
    // let results = tags.load::<Tag>(&connection).expect("Error loading tags");
    let list = tags
        // .filter(name.eq("react"))
        .limit(5)
        .load::<Tag>(&connection)
        .expect("Error loading tags");

    println!("Displaying {} list", list.len());
    for item in list {
        println!("{}", item.name);
        println!("----------\n");
    }
}
