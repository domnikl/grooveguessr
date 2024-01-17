use crate::db_schema::users;
use diesel::{prelude::*, upsert::on_constraint};
use rand::seq::SliceRandom;

use crate::{models::user::User, DbPool};

use super::Error;

pub struct UserService<'a> {
    db_pool: &'a DbPool,
}

impl<'a> UserService<'a> {
    pub fn new(db_pool: &'a DbPool) -> Self {
        Self { db_pool }
    }

    pub fn register(&self, user: User) -> Result<User, Error> {
        let mut conn = self.db_pool.get().map_err(Error::DbConnection)?;

        diesel::insert_into(users::table)
            .values(&user)
            .on_conflict(on_constraint("users_pkey"))
            .do_update()
            .set(users::email.eq(user.email.clone()))
            .execute(&mut conn)
            .map_err(Error::Db)?;

        Ok(user)
    }

    pub fn save(&self, user: User) -> Result<User, Error> {
        let mut conn = self.db_pool.get().map_err(Error::DbConnection)?;

        diesel::update(users::table)
            .set(&user)
            .filter(users::id.eq(user.id.clone()))
            .execute(&mut conn)
            .map_err(Error::Db)?;

        Ok(user)
    }

    pub fn find(&self, user_id: &str) -> Result<User, Error> {
        let mut conn = self.db_pool.get()?;

        let user = users::table
            .filter(users::id.eq(user_id))
            .limit(1)
            .get_result::<User>(&mut conn)
            .map_err(Error::Db)?;

        Ok(user)
    }
}

pub fn generate_random_name() -> String {
    let adjectives = [
        "adorable",
        "beautiful",
        "clean",
        "drab",
        "elegant",
        "fancy",
        "glamorous",
        "handsome",
        "long",
        "magnificent",
        "old-fashioned",
        "plain",
        "quaint",
        "sparkling",
        "ugliest",
        "unsightly",
        "angry",
        "bewildered",
        "clumsy",
        "defeated",
        "embarrassed",
        "fierce",
        "grumpy",
        "helpless",
        "itchy",
        "jealous",
        "lazy",
        "mysterious",
        "nervous",
        "obnoxious",
        "panicky",
        "repulsive",
        "scary",
        "thoughtless",
        "uptight",
        "worried",
    ];

    let animal = vec![
        "alligator",
        "ant",
        "bear",
        "bee",
        "bird",
        "camel",
        "cat",
        "cheetah",
        "chicken",
        "chimpanzee",
        "cow",
        "crocodile",
        "deer",
        "dog",
        "dolphin",
        "duck",
        "eagle",
        "elephant",
        "fish",
        "fly",
        "fox",
        "frog",
        "giraffe",
        "goat",
        "goldfish",
        "hamster",
        "hippopotamus",
        "horse",
        "kangaroo",
        "kitten",
        "lion",
        "lobster",
        "monkey",
        "octopus",
        "owl",
        "panda",
        "pig",
        "puppy",
        "rabbit",
        "rat",
        "scorpion",
        "seal",
        "shark",
        "sheep",
        "snail",
        "snake",
        "spider",
        "squirrel",
        "tiger",
        "turtle",
        "wolf",
        "zebra",
    ];

    let adjective = adjectives.choose(&mut rand::thread_rng());
    let animal = animal.choose(&mut rand::thread_rng());

    // get random adjective and animal
    format!("{} {}", adjective.unwrap(), animal.unwrap())
}
