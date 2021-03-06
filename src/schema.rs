table! {
    base_actors (id) {
        id -> Int4,
        display_name -> Varchar,
        profile_url -> Varchar,
        inbox_url -> Varchar,
        outbox_url -> Varchar,
        local_user -> Nullable<Int4>,
        original_json -> Jsonb,
    }
}

table! {
    base_posts (id) {
        id -> Int4,
        name -> Nullable<Varchar>,
        media_type -> Nullable<Varchar>,
        posted_by -> Nullable<Int4>,
        icon -> Nullable<Int4>,
        original_json -> Jsonb,
    }
}

table! {
    comments (id) {
        id -> Int4,
        conversation -> Int4,
        parent -> Int4,
        post -> Int4,
    }
}

table! {
    emails (id) {
        id -> Int4,
        email -> Nullable<Varchar>,
        user_id -> Int4,
    }
}

table! {
    files (id) {
        id -> Int4,
        file_path -> Varchar,
    }
}

table! {
    followers (id) {
        id -> Int4,
        follower -> Int4,
        follows -> Int4,
    }
}

table! {
    images (id) {
        id -> Int4,
        width -> Int4,
        height -> Int4,
        file_id -> Int4,
    }
}

table! {
    links (id) {
        id -> Int4,
        href -> Varchar,
        href_lang -> Varchar,
        height -> Nullable<Int4>,
        width -> Nullable<Int4>,
        preview -> Nullable<Text>,
        base_post -> Int4,
    }
}

table! {
    media_posts (id) {
        id -> Int4,
        file_id -> Int4,
        post_id -> Int4,
    }
}

table! {
    personas (id) {
        id -> Int4,
        default_visibility -> Varchar,
        is_searchable -> Bool,
        avatar -> Nullable<Int4>,
        shortname -> Varchar,
        base_actor -> Int4,
    }
}

table! {
    posts (id) {
        id -> Int4,
        content -> Text,
        source -> Nullable<Text>,
        base_post -> Int4,
    }
}

table! {
    reactions (id) {
        id -> Int4,
        reaction_type -> Varchar,
        comment_id -> Int4,
    }
}

table! {
    users (id) {
        id -> Int4,
        password -> Varchar,
        created_at -> Timestamptz,
        primary_email -> Int4,
    }
}

joinable!(base_actors -> users (local_user));
joinable!(base_posts -> base_actors (posted_by));
joinable!(base_posts -> images (icon));
joinable!(images -> files (file_id));
joinable!(links -> base_posts (base_post));
joinable!(media_posts -> files (file_id));
joinable!(media_posts -> posts (post_id));
joinable!(personas -> base_actors (base_actor));
joinable!(personas -> images (avatar));
joinable!(posts -> base_posts (base_post));
joinable!(reactions -> comments (comment_id));

allow_tables_to_appear_in_same_query!(
    base_actors,
    base_posts,
    comments,
    emails,
    files,
    followers,
    images,
    links,
    media_posts,
    personas,
    posts,
    reactions,
    users,
);
