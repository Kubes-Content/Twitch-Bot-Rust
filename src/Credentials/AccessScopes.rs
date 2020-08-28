pub mod Read{
    pub mod Analytics{
        pub const Extensions:&str = "analytics:read:extensions";
        pub const Games:&str = "analytics:read:games";
    }

    pub const Bits:&str = "bits:read";
    pub const Broadcast:&str = "user:read:broadcast";
    pub const EmailAddress:&str = "user:read:email";
    pub const Subscriptions:&str = "channel:read:subscriptions";
    pub const Chat:&str = "chat:read";
    pub const Whispers:&str = "whispers:read";
}

pub mod Edit{
    pub const Broadcast:&str = "user:edit:broadcast";
    pub const Clips:&str = "clips:edit";
    pub const User:&str = "user:edit";
    pub const Chat:&str = "chat:edit";
    pub const Whispers:&str = "whispers:edit";
}

pub const Moderate:&str = "channel:moderate";

pub fn get_all_scopes() -> String{
    return format!("{0}+{1}+{2}+{3}",
                    get_all_analytics_scopes(),
                    get_all_read_scopes(),
                    get_all_write_scopes(),
                    Moderate);
}

pub fn get_all_analytics_scopes() -> String {
    return format!("{0}+{1}",Read::Analytics::Extensions, Read::Analytics::Games);
}

pub fn get_all_read_scopes() -> String {
    return format!("{0}+{1}+{2}+{3}+{4}+{5}", Read::Bits, Read::Broadcast, Read::EmailAddress, Read::Subscriptions, Read::Chat, Read::Whispers);
}

pub fn get_all_write_scopes() -> String {
    return format!("{0}+{1}+{2}+{3}+{4}", Edit::Broadcast, Edit::Clips, Edit::User, Edit::Chat, Edit::Whispers);
}