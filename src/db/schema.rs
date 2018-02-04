table! {
    credentials (username) {
        username -> Text,
        password -> Text,
        salt -> Text,
        admin -> Bool,
    }
}
