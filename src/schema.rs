table! {
    todo_lists (id) {
        id -> Int4,
        title -> Varchar,
    }
}

table! {
    todos (id) {
        id -> Int4,
        title -> Varchar,
        list_id -> Int4,
        checked -> Bool,
    }
}

allow_tables_to_appear_in_same_query!(
    todo_lists,
    todos,
);
