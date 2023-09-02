// @generated automatically by Diesel CLI.

diesel::table! {

    submission (id) {
        id -> Int8,
        memory -> Nullable<Float4>,
        #[max_length = 10]
        memory_display -> Nullable<Varchar>,
        memory_percentile -> Nullable<Float4>,
        notes -> Nullable<Text>,
        runtime -> Nullable<Float4>,
        runtime_percentile -> Nullable<Float4>,
        status_code -> Nullable<Int4>,
        timestamp -> Nullable<Int4>,
        #[max_length = 100]
        code_hash -> Nullable<Varchar>,
        #[max_length = 100]
        username -> Nullable<Varchar>,
        #[max_length = 32]
        question_id -> Nullable<Varchar>,
    }
}
