CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE users
(
    user_id  UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name     VARCHAR NOT NULL,
    login    VARCHAR NOT NULL,
    password VARCHAR NOT NULL
);

CREATE TABLE blog_post
(
    blog_post_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id      UUID    NOT NULL references users(user_id),
    title        VARCHAR NOT NULL,
    body         TEXT    NOT NULL
);

CREATE TABLE post_comment
(
    post_comment_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    title           VARCHAR NOT NULL,
    body            TEXT    NOT NULL,
    blog_post_id    UUID    NOT NULL references blog_post(blog_post_id)
);
