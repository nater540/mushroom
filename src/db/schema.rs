// @generated automatically by Diesel CLI.

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
    notes (id) {
        id -> Integer,
        mushroom_id -> Integer,
        title -> Text,
        body -> Text,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    progress (id) {
        id -> Integer,
        mushroom_id -> Integer,
        total -> Float,
        created_at -> Nullable<Timestamp>,
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

diesel::table! {
    strains (id) {
        id -> Integer,
        name -> Text,
        slug -> Text,
        description -> Nullable<Text>,
        species_id -> Integer,
    }
}

diesel::joinable!(mushrooms -> species (species_id));
diesel::joinable!(mushrooms -> strains (strain_id));
diesel::joinable!(notes -> mushrooms (mushroom_id));
diesel::joinable!(progress -> mushrooms (mushroom_id));

diesel::allow_tables_to_appear_in_same_query!(
    mushrooms,
    notes,
    progress,
    species,
    strains,
);
