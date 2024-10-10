// @generated automatically by Diesel CLI.

#![allow(non_snake_case)]
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

// diesel::joinable!(WorkingCars -> CarToRepair (car_to_repair));
// diesel::joinable!(WorkingCars -> Garage (assigned_garage));

// diesel::allow_tables_to_appear_in_same_query!(users);
