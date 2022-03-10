use diesel;
use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use super::schema::users;
use super::schema::users::dsl::users as all_users;
use super::schema::outbound_statistics::dsl::outbound_statistics as all_outbound_statistics;

// this is to get users from the database
#[derive(Serialize, Deserialize, Queryable)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub first_name: String,
}

// decode request data
#[derive(Deserialize)]
pub struct UserData {
    pub username: String
}

// this is to insert users to database
#[derive(Serialize, Deserialize, Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub username: String,
    pub password: String,
    pub first_name: String,
}


impl User {
    pub fn get_all_users(conn: &MysqlConnection) -> Vec<User> {
        all_users
            .order(users::id.desc())
            .load::<User>(conn)
            .expect("error!")
    }

    pub fn insert_user(user: NewUser, conn: &MysqlConnection) -> bool {
        diesel::insert_into(users::table)
            .values(&user)
            .execute(conn)
            .is_ok()
    }


    pub fn get_user_by_username(user: UserData, conn: &MysqlConnection) -> Vec<User> {
        all_users
            .filter(users::username.eq(user.username))
            .load::<User>(conn)
            .expect("error!")
    }


    pub fn update_by_username(user: String, first_name: String, conn: &MysqlConnection) -> usize {
        let updated_row = diesel::update(all_users.filter(users::username.eq(user)))
            .set(users::first_name.eq(first_name))
            .execute(conn)
            .unwrap();
        format!("{}", updated_row).parse().unwrap()
    }

    pub fn update_all(user: User, conn: &MysqlConnection) -> Vec<User> {
        diesel::update(all_users.filter(users::id.eq(user.id)))
            .set((
                users::username.eq(user.username),
                users::password.eq(user.password),
                users::first_name.eq(user.first_name)
            ))
            .execute(conn)
            .expect("update error!");

        all_users
            .filter(users::id.eq(user.id))
            .load::<User>(conn)
            .expect("find error!")
    }

    pub fn delete_by_name(user: String, conn: &MysqlConnection) -> bool {
        diesel::delete(all_users.filter(users::username.eq(user)))
            .execute(conn)
            .is_ok()
    }
}


/******************************************/




#[derive(Serialize, Deserialize, Queryable)]
pub struct OutboundStatistics {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub first_name: String,
}

// decode request data
#[derive(Deserialize)]
pub struct  StatisticsData {
    pub username: String
}

// this is to insert users to database
#[derive(Serialize, Deserialize, Insertable)]
#[table_name = "users"]
pub struct NewOutboundStatistics {
    pub username: String,
    pub password: String,
    pub first_name: String,
}


impl User {
    pub fn get_all_users(conn: &MysqlConnection) -> Vec<User> {
        all_users
            .order(users::id.desc())
            .load::<User>(conn)
            .expect("error!")
    }

    pub fn insert_user(user: NewUser, conn: &MysqlConnection) -> bool {
        diesel::insert_into(users::table)
            .values(&user)
            .execute(conn)
            .is_ok()
    }


    pub fn get_user_by_username(user: UserData, conn: &MysqlConnection) -> Vec<User> {
        all_users
            .filter(users::username.eq(user.username))
            .load::<User>(conn)
            .expect("error!")
    }


    pub fn update_by_username(user: String, first_name: String, conn: &MysqlConnection) -> usize {
        let updated_row = diesel::update(all_users.filter(users::username.eq(user)))
            .set(users::first_name.eq(first_name))
            .execute(conn)
            .unwrap();
        format!("{}", updated_row).parse().unwrap()
    }

    pub fn update_all(user: User, conn: &MysqlConnection) -> Vec<User> {
        diesel::update(all_users.filter(users::id.eq(user.id)))
            .set((
                users::username.eq(user.username),
                users::password.eq(user.password),
                users::first_name.eq(user.first_name)
            ))
            .execute(conn)
            .expect("update error!");

        all_users
            .filter(users::id.eq(user.id))
            .load::<User>(conn)
            .expect("find error!")
    }

    pub fn delete_by_name(user: String, conn: &MysqlConnection) -> bool {
        diesel::delete(all_users.filter(users::username.eq(user)))
            .execute(conn)
            .is_ok()
    }
}