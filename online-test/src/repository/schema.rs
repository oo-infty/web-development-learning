// @generated automatically by Diesel CLI.

diesel::table! {
    questions (id) {
        id -> Integer,
        kind -> Integer,
        content -> Text,
        option0 -> Nullable<Text>,
        option1 -> Nullable<Text>,
        option2 -> Nullable<Text>,
        option3 -> Nullable<Text>,
        answer -> Text,
    }
}

diesel::table! {
    scores (id) {
        id -> Integer,
        name -> Text,
        score -> Float,
        end_time -> Text,
        duration -> Integer,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    questions,
    scores,
);
