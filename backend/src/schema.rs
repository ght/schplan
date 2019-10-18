table! {
    users (id) {
        id -> Nullable<Integer>,
        token -> Nullable<Text>,
        ip -> Nullable<Text>,
        username -> Text,
        password -> Text,
    }
}
