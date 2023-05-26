
#[cfg(test)]
mod tests {

    use actix_web::{test::{self, TestRequest}, App};
    use actix_web::middleware::Logger;
    use serde_json::json;
    use crate::modules::users::model::User;
    use crate::tests::tests::{before_db_test, init};
    use crate::users::routes::init_routes;

    #[actix_rt::test]
    async fn test_user() {
        init();
        before_db_test();

        let mut app = test::init_service(
            App::new()
                .wrap(Logger::default())
                .configure(init_routes)
        ).await;

        let request_body = json!({
            "name": "user1",
            "password": "test",
        });

        let resp = TestRequest::post().uri("/users").set_json(&request_body).send_request(&mut app).await;
        assert!(resp.status().is_success(), "Failed to create user");
        let user: User = test::read_body_json(resp).await;

        // let bytes = to_bytes(resp.into_body()).await.unwrap();
        // let body_string = std::str::from_utf8(&bytes).unwrap();

        let resp = TestRequest::get().uri(&format!("/users/{}", user.user_id)).send_request(&mut app).await;
        assert!(resp.status().is_success(), "Failed to find user");

        let user: User = test::read_body_json(resp).await;
        assert_eq!(user.name, "user1", "Found wrong user");
        assert_eq!(user.password, "test", "Found wrong user");

        let request_body = json!({
            "name": "user2",
            "password": "new",
        });

        // dbg!("found user : {:?}", user);
        let resp = TestRequest::put().uri(&format!("/users/{}", user.user_id)).set_json(&request_body).send_request(&mut app).await;
        assert!(resp.status().is_success(), "Failed to update user");

        let user: User = test::read_body_json(resp).await;
        assert_eq!("user2", user.name, "Failed to change name for user");
        assert_eq!("new", user.password, "Failed to change password for user");

        let resp = TestRequest::delete().uri(&format!("/users/{}", user.user_id)).send_request(&mut app).await;
        assert!(resp.status().is_success(), "Failed to delete user");

        let resp = TestRequest::get().uri(&format!("/users/{}", user.user_id)).send_request(&mut app).await;
        assert!(resp.status().is_client_error(), "It should not be possible to find the user after deletion");
    }

}