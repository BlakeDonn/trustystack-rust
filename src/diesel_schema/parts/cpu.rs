diesel::table! {
    cpu (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        price -> Numeric,
        core_count -> Int4,
        #[max_length = 50]
        core_clock -> Varchar,
        #[max_length = 50]
        boost_clock -> Varchar,
        tdp -> Int4,
        #[max_length = 100]
        integrated_graphics -> Nullable<Varchar>,
        smt -> Bool,
    }
}
