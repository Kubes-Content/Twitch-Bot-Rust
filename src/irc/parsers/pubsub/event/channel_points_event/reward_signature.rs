use crate::user::user_properties::UserId;
use url::Url;


pub struct RewardSignature {
    pub id: String,
    pub channel_id: UserId,
    pub title: String,
    pub prompt: String,
    pub cost: u32,
    pub user_input_required: bool,
    pub sub_only: bool,
    pub image:Option<ImageUrlSignature>,
    pub default_image:ImageUrlSignature,
    pub background_color:String,
    pub enabled:bool,
    pub paused:bool,
    pub in_stock:bool,
    pub max_per_stream:(bool, u32),
    pub should_redemptions_skip_request_queue:bool
}



pub struct ImageUrlSignature {
    pub url_1x:Url,
    pub url_2x:Url,
    pub url_4x:Url
}