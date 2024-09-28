// @generated automatically by Diesel CLI.

diesel::table! {
    posts (id_posts) {
        id_posts -> Int4,
        title -> Nullable<Varchar>,
        body -> Nullable<Text>,
        published -> Bool,
    }
}

diesel::table! {
    users (id_users) {
        id_users -> Int4,
        fisrt_name -> Nullable<Varchar>,
        lastname -> Nullable<Varchar>,
        email -> Nullable<Varchar>,
        status -> Bool,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    posts,
    users,
);
