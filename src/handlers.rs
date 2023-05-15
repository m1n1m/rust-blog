use actix_web::{Error, HttpResponse, Responder, web};
use diesel::RunQueryDsl;
use web::Data;
use crate::models::User;
use crate::Pool;
use crate::schema::users::dsl::users;
use slog::{info, Logger};

pub async fn get_users(db: Data<Pool>, log: Data<Logger>) -> Result<HttpResponse, Error> {

    info!(log,
        "get_users"
    );

    // let result = get_all_users(db);
    //
    // match result {
    //     Err(e) => println!("{:?}", e),
    //     Ok(data) => {
    //         eprintln!("{:?}", data);
    //         // for user in data {
    //         //     eprintln!("{:?}", user);
    //         // }
    //     }
    // }

    // dbg!(result).expect("TODO: panic message");
    // Ok(HttpResponse::Ok().finish())

    let usersList = web::block(move || get_all_users(db))
        .await?
        .map(|user| HttpResponse::Ok().json(user))
        .map_err(actix_web::error::ErrorInternalServerError);

    usersList

    // Ok(HttpResponse::Ok().json(usersList))

    // Ok(web::block(move || get_all_users(db))
    //     .await
    //     .map(|user| HttpResponse::Ok().json(user))
    //        .map_err(actix_web::error::ErrorInternalServerError)?
    //     // .map_err(|_| HttpResponse::InternalServerError())?)
}

fn get_all_users(pool: Data<Pool>) -> Result<Vec<User>, diesel::result::Error> {
    let mut conn = pool.get().unwrap();
    let items = users.load::<User>(&mut conn)?;
    Ok(items)
}

pub async fn get_users_test() -> impl Responder {
    format!("hello from get users")
}

pub async fn get_user_by_id() -> impl Responder {
    format!("hello from get users by id")
}

pub async fn add_user() -> impl Responder {
    format!("hello from add user")
}

pub async fn delete_user() -> impl Responder {
    format!("hello from delete user")
}