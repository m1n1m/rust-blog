CREATE TABLE users
(
    user_id  SERIAL PRIMARY KEY,
    name     VARCHAR NOT NULL,
    login    VARCHAR NOT NULL,
    password VARCHAR NOT NULL
);

CREATE TABLE blog_post
(
    blog_post_id SERIAL PRIMARY KEY,
    user_id      INT     NOT NULL references users,
    title        VARCHAR NOT NULL,
    body         TEXT    NOT NULL
);

CREATE TABLE post_comment
(
    post_comment_id SERIAL PRIMARY KEY,
    title           VARCHAR NOT NULL,
    body            TEXT    NOT NULL,
    blog_post_id    INT     NOT NULL references blog_post
);
