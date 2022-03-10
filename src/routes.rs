use super::db::Conn as DbConn;
use rocket_contrib::json::Json;
use super::models::{User, NewUser};
use serde_json::Value;
use crate::models::UserData;



#[get("/all")]
pub fn get_all(conn: DbConn) -> Json<Value> {
    Json(json!({
        "status": 200,
        "result": User::get_all_users(&conn),
    }))
}

#[get("/delete/<username>")]
pub fn delete_user(conn: DbConn, username: String) -> Json<Value> {
    let status = User::delete_by_name(username, &conn);

    Json(json!({
        "status": 200,
        "result": status,
    }))
}


#[get("/updateName/<username>/<first_name>")]
pub fn update_first_name(conn: DbConn, username: String, first_name: String) -> Json<Value> {
    let code = User::update_by_username(username, first_name, &conn);
    let message;
    if code as i32 == 1 {
        message = String::from("更新成功!")
    } else {
        message =  String::from("更新失败!")
    }
    Json(json!({
        "status": 200,
        "code":code,
        "message": message,
    }))
}

#[post("/updateAll",format="application/json",data="<update_user>")]
pub fn updateall(conn: DbConn, update_user: Json<User>)->Json<Value>{
    Json(json!({
        "status": User::update_all(update_user.into_inner(), &conn),
        "result": "ok",
    }))
}

#[post("/newUser", format = "application/json", data = "<new_user>")]
pub fn new_user(conn: DbConn, new_user: Json<NewUser>) -> Json<Value> {
    Json(json!({
        "status": User::insert_user(new_user.into_inner(), &conn),
        "result": User::get_all_users(&conn).first(),
    }))
}

#[post("/getUser", format = "application/json", data = "<user_data>")]
pub fn find_user(conn: DbConn, user_data: Json<UserData>) -> Json<Value> {
    Json(json!({
        "status": 200,
        "result": User::get_user_by_username(user_data.into_inner(), &conn),
    }))
}