primitive_wrapper!(UserId, u32, "(User ID: {})");

impl UserId {
    pub const LENGTH: usize = 8;
}

primitive_wrapper!(ChannelId, UserId, "(Channel: {})");
impl From<u32> for ChannelId {
    fn from(v: u32) -> Self {
        ChannelId {
            value: UserId { value: v },
        }
    }
}

primitive_wrapper!(UserLogin, String, "(User Login: {})");

primitive_wrapper!(UserDisplayName, String, "(User Display Name: {})");

#[derive(Clone, PartialEq, Hash)]
pub enum UserTypeEnum {
    Basic,
    Admin,
    GlobalMod,
    Staff,
}
impl ToString for UserTypeEnum {
    fn to_string(&self) -> String {
        match self {
            UserTypeEnum::Basic => "Basic",
            UserTypeEnum::Admin => "Admin",
            UserTypeEnum::GlobalMod => "Global Mod",
            UserTypeEnum::Staff => "twitch Staff",
        }
        .to_string()
    }
}
primitive_wrapper!(UserType, UserTypeEnum, "(User Type: {})");

impl UserType {
    pub fn new_from_string(string_value: String) -> UserType {
        match string_value.as_str() {
            "" => UserType::from(UserTypeEnum::Basic),
            "admin" => UserType::from(UserTypeEnum::Admin),
            "global_mod" => UserType::from(UserTypeEnum::GlobalMod),
            "staff" => UserType::from(UserTypeEnum::Staff),
            _ => {
                panic!("INCORRECT ENUM NAME '{}'", string_value)
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Hash)]
pub enum UserBroadcasterTypeEnum {
    Basic,
    Affiliate,
    Partner,
}
impl ToString for UserBroadcasterTypeEnum {
    fn to_string(&self) -> String {
        format!("{:?}", self)
    }
}

primitive_wrapper!(
    UserBroadcasterType,
    UserBroadcasterTypeEnum,
    "(User Broadcaster Type: {})"
);

impl UserBroadcasterType {
    pub fn new_from_string(string_value: String) -> UserBroadcasterType {
        match string_value.as_str() {
            "" => UserBroadcasterType::from(UserBroadcasterTypeEnum::Basic),
            "affiliate" => UserBroadcasterType::from(UserBroadcasterTypeEnum::Affiliate),
            "partner" => UserBroadcasterType::from(UserBroadcasterTypeEnum::Partner),
            _ => {
                panic!("INCORRECT ENUM VALUE RECEIVED value='{}'", string_value)
            }
        }
    }
}

primitive_wrapper!(UserDescription, String, "(User Description: {})");

primitive_wrapper!(
    UserProfileImageUrlFormat,
    String,
    "(User Profile Image URL Format: {})"
);

primitive_wrapper!(
    UserOfflineImageUrlFormat,
    String,
    "(User Profile Image URL Format: {})"
);

primitive_wrapper!(UserViewCount, u32, "(User View Count: {})");

primitive_wrapper!(UserEmail, String, "(User Email: {})");
