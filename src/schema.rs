table! {
    outbound_statistics (id) {
        id -> Integer,
        sap_no -> Integer,
        out_type -> Varchar,
        out_category -> Varchar,
        deliver_year -> Integer,
        mark_month -> Nullable<Integer>,
        out_month -> Nullable<Integer>,
        out_date -> Nullable<Datetime>,
        province -> Nullable<Varchar>,
        city -> Nullable<Varchar>,
        county -> Nullable<Varchar>,
        area -> Nullable<Varchar>,
        area_level -> Nullable<Varchar>,
        bus_owner -> Nullable<Varchar>,
        customer_name -> Nullable<Varchar>,
        product_name -> Nullable<Varchar>,
        package_form -> Nullable<Varchar>,
        specifications -> Nullable<Varchar>,
        out_num -> Nullable<Integer>,
        single_price -> Nullable<Integer>,
        sale_num -> Nullable<Integer>,
        out_no -> Nullable<Integer>,
        deliver_no -> Nullable<Integer>,
        comment -> Nullable<Varchar>,
        batch_no -> Nullable<Varchar>,
        dead_line -> Nullable<Datetime>,
        return_label -> Nullable<Varchar>,
        invoice_pass -> Nullable<Varchar>,
        // invoice_type -> Nullable<Varchar>,
        // invoice_count -> Nullable<Integer>,
        // promoter_company_name -> Nullable<archar>,
        // foregin_money -> Nullable<Integer>,
        // crm_client_code -> Nullable<Varchar>,
        // pass_one -> Nullable<Varchar>,
        // pass_two -> Nullable<Varchar>,
        // pass_unit -> Nullable<Varchar>,
        // out_num_discount -> Nullable<Integer>,
    }
}

table! {
    users (id) {
        id -> Integer,
        username -> Varchar,
        password -> Varchar,
        first_name -> Varchar,
    }
}

allow_tables_to_appear_in_same_query!(
    outbound_statistics,
    users,
);
