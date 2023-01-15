// @generated automatically by Diesel CLI.

diesel::table! {
    submission (id) {
        id -> Int4,
        last_testcase -> Nullable<Varchar>,
        memory -> Nullable<Float4>,
        memory_display -> Nullable<Varchar>,
        memory_percentile -> Nullable<Float4>,
        notes -> Nullable<Text>,
        runtime -> Nullable<Float4>,
        runtime_percentile -> Nullable<Float4>,
        status_code -> Nullable<Int4>,
        timestamp -> Nullable<Int4>,
        code_hash -> Nullable<Varchar>,
    }
}
