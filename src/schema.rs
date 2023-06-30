// @generated automatically by Diesel CLI.

diesel::table! {
    items (id) {
        id -> Int8,
        event -> Text,
        c_time -> Timestamp,
        m_time -> Timestamp,
    }
}
