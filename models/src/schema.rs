// @generated automatically by Diesel CLI.

diesel::table! {
    blog_post (blog_post_id) {
        blog_post_id -> Uuid,
        user_id -> Uuid,
        title -> Varchar,
        body -> Text,
    }
}

diesel::table! {
    post_comment (post_comment_id) {
        post_comment_id -> Uuid,
        title -> Varchar,
        body -> Text,
        blog_post_id -> Uuid,
    }
}

diesel::table! {
    users (user_id) {
        user_id -> Uuid,
        name -> Varchar,
        login -> Varchar,
        password -> Varchar,
    }
}

diesel::joinable!(blog_post -> users (user_id));
diesel::joinable!(post_comment -> blog_post (blog_post_id));

diesel::allow_tables_to_appear_in_same_query!(
    blog_post,
    post_comment,
    users,
);
