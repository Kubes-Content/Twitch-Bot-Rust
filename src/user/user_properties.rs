use crate::debug::fail_safely;


primitiveWrapper!(UserId, u32, "(User ID: {})");

primitiveWrapper!(UserLogin, String, "(User Login: {})");

primitiveWrapper!(UserDisplayName, String, "(User Display Name: {})");

#[derive(Clone,PartialEq)]
pub enum UserTypeEnum {
    Basic,
    Admin,
    GlobalMod,
    Staff
}
impl ToString for UserTypeEnum {
    fn to_string(&self) -> String {
        match self {
            UserTypeEnum::Basic => { "Basic" },
            UserTypeEnum::Admin => { "Admin" },
            UserTypeEnum::GlobalMod => { "Global Mod" },
            UserTypeEnum::Staff => { "twitch Staff" },
        }.to_string()
    }
}
primitiveWrapper!(UserType, UserTypeEnum, "(User Type: {})");

impl UserType {
    pub fn new_from_string(string_value:String) -> UserType {
        match string_value.as_str() {
            "" => { UserType::new(UserTypeEnum::Basic) }
            "admin" => { UserType::new(UserTypeEnum::Admin) }
            "global_mod" => { UserType::new(UserTypeEnum::GlobalMod) }
            "staff" => { UserType::new(UserTypeEnum::Staff) }
            _ => { fail_safely(stringify!(format!("INCORRECT ENUM NAME '{}'", string_value))); UserType::new(UserTypeEnum::Basic) }
        }
    }
}

#[derive(Clone, PartialEq)]
pub enum UserBroadcasterTypeEnum {
    Basic,
    Affiliate,
    Partner
}
impl ToString for UserBroadcasterTypeEnum {
    fn to_string(&self) -> String {
        match self {
            UserBroadcasterTypeEnum::Basic => { "Basic" },
            UserBroadcasterTypeEnum::Affiliate => { "Affiliate" },
            UserBroadcasterTypeEnum::Partner => { "Partner" },
        }.to_string()
    }
}

primitiveWrapper!(UserBroadcasterType, UserBroadcasterTypeEnum, "(User Broadcaster Type: {})");

impl UserBroadcasterType {
    pub fn new_from_string(string_value:String) -> UserBroadcasterType  {
        match string_value.as_str() {
            "" => { UserBroadcasterType::new(UserBroadcasterTypeEnum::Basic) }
            "affiliate" => { UserBroadcasterType::new(UserBroadcasterTypeEnum::Affiliate) }
            "partner" => { UserBroadcasterType::new(UserBroadcasterTypeEnum::Partner) }
            _ => { fail_safely(stringify!(format!("INCORRECT ENUM VALUE RECEIVED value='{}'",string_value))); UserBroadcasterType::new(UserBroadcasterTypeEnum::Basic) }

        }
    }
}

primitiveWrapper!(UserDescription, String, "(User Description: {})");

primitiveWrapper!(UserProfileImageUrlFormat, String, "(User Profile Image URL Format: {})");

primitiveWrapper!(UserOfflineImageUrlFormat, String, "(User Profile Image URL Format: {})");

primitiveWrapper!(UserViewCount, u32, "(User View Count: {})");

primitiveWrapper!(UserEmail, String, "(User Email: {})");
