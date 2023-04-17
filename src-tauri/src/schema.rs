// @generated automatically by Diesel CLI.

diesel::table! {
    cars_for_sale (id) {
        id -> Integer,
        brand -> Varchar,
        model -> Varchar,
        year -> Integer,
        img -> Nullable<Varchar>,
        color -> Nullable<Varchar>,
        mileage -> Nullable<Integer>,
        category -> Nullable<Varchar>,
        price -> Float8,
    }
}

diesel::table! {
    cars_for_service (id) {
        id -> Integer,
        customer_id -> Integer,
        car_details -> Varchar,
        mechanic -> Varchar,
    }
}

diesel::table! {
    customer (id) {
        id -> Integer,
        first_name -> Varchar,
        last_name -> Varchar,
        birth_date -> Date,
        email -> Nullable<Varchar>,
        phone -> Nullable<Varchar>,
    }
}

diesel::table! {
    invoice (id) {
        id -> Integer,
        customer_id -> Integer,
        salesperson_id -> Integer,
        car_id -> Integer,
        amount -> Integer,
    }
}

diesel::table! {
    salesperson (id) {
        id -> Integer,
        first_name -> Nullable<Varchar>,
        last_name -> Nullable<Varchar>,
        commission -> Float8,
    }
}

diesel::joinable!(cars_for_service -> customer (customer_id));
diesel::joinable!(invoice -> cars_for_sale (car_id));
diesel::joinable!(invoice -> customer (customer_id));
diesel::joinable!(invoice -> salesperson (salesperson_id));

diesel::allow_tables_to_appear_in_same_query!(
    cars_for_sale,
    cars_for_service,
    customer,
    invoice,
    salesperson,
);
