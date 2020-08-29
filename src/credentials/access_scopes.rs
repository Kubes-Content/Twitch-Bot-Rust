pub mod read {
    pub mod analytics {
        pub const EXTENSIONS:&str = "analytics:read:extensions";
        pub const GAMES:&str = "analytics:read:games";
    }

    pub const BITS:&str = "bits:read";
    pub const BROADCAST:&str = "user:read:broadcast";
    pub const EMAIL_ADDRESS:&str = "user:read:email";
    pub const SUBSCRIPTIONS:&str = "channel:read:subscriptions";
    pub const CHAT:&str = "chat:read";
    pub const WHISPERS:&str = "whispers:read";
}

pub mod edit {
    pub const BROADCAST:&str = "user:edit:broadcast";
    pub const CLIPS:&str = "clips:edit";
    pub const USER:&str = "user:edit";
    pub const CHAT:&str = "chat:edit";
    pub const WHISPERS:&str = "whispers:edit";
}

pub const MODERATE:&str = "channel:moderate";

pub fn get_all_scopes() -> String{
    return format!("{0}+{1}+{2}+{3}",
                    get_all_analytics_scopes(),
                    get_all_read_scopes(),
                    get_all_write_scopes(),
                   MODERATE);
}

pub fn get_all_analytics_scopes() -> String {
    return format!("{0}+{1}", read::analytics::EXTENSIONS, read::analytics::GAMES);
}

pub fn get_all_read_scopes() -> String {
    return format!("{0}+{1}+{2}+{3}+{4}+{5}", read::BITS, read::BROADCAST, read::EMAIL_ADDRESS, read::SUBSCRIPTIONS, read::CHAT, read::WHISPERS);
}

pub fn get_all_write_scopes() -> String {
    return format!("{0}+{1}+{2}+{3}+{4}", edit::BROADCAST, edit::CLIPS, edit::USER, edit::CHAT, edit::WHISPERS);
}