table! {
    clients (id) {
        update_timestamp -> Timestamp,
        id -> Unsigned<Bigint>,
        subdomain -> Varchar,
        remote_ip -> Varchar,
        sequence_number -> Bigint,
    }
}

table! {
    updates (id) {
        update_timestamp -> Timestamp,
        id -> Unsigned<Bigint>,
        client_id -> Unsigned<Bigint>,
        request_ip -> Varchar,
    }
}

joinable!(updates -> clients (client_id));

allow_tables_to_appear_in_same_query!(
    clients,
    updates,
);
