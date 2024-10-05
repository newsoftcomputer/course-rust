// @generated automatically by Diesel CLI.

diesel::table! {
    users (id_users) {
        id_users -> Uuid,
        #[max_length = 100]
        first_name -> Nullable<Bpchar>,
        #[max_length = 100]
        last_name -> Nullable<Bpchar>,
        #[max_length = 100]
        email -> Nullable<Bpchar>,
        status -> Bool,
    }
}
