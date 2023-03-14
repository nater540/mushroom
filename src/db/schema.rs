// @generated automatically by Diesel CLI.

diesel::table! {
    mushroom_notes (id) {
        id -> Integer,
        mushroom_id -> Integer,
        title -> Text,
        body -> Text,
    }
}

diesel::table! {
    mushrooms (id) {
        id -> Integer,
        parent_id -> Nullable<Integer>,
        name -> Text,
        label -> Nullable<Text>,
        species_id -> Integer,
        strain_id -> Integer,
        petri_dish -> Bool,
        germination_date -> Nullable<Timestamp>,
    }
}

diesel::table! {
    species (id) {
        id -> Integer,
        name -> Text,
        slug -> Text,
        description -> Nullable<Text>,
        psillow_link -> Nullable<Text>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    mushroom_notes,
    mushrooms,
    species,
);
