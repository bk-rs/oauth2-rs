use oauth2_client::re_exports::{Deserialize_enum_str, Scope, Serialize_enum_str};

pub const TOKEN_URL: &str = "https://graph.facebook.com/v12.0/device/login_status";
pub const AUTHORIZATION_URL: &str = "https://accounts.google.com/o/oauth2/v2/auth";
pub const DEVICE_AUTHORIZATION_URL: &str = "https://graph.facebook.com/v12.0/device/login";

pub mod authorization_code_grant;
pub mod device_authorization_grant;

pub use device_authorization_grant::FacebookProviderForDevices;

#[derive(Deserialize_enum_str, Serialize_enum_str, Debug, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum FacebookScope {
    // Ref https://github.com/bk-rs/facebook-rs/blob/master/facebook-permission/src/lib.rs
    AdsManagement,
    AdsRead,
    AttributionRead,
    BusinessManagement,
    CatalogManagement,
    Email,
    GroupsAccessMemberInfo,
    InstagramBasic,
    InstagramContentPublish,
    InstagramManageComments,
    InstagramManageInsights,
    LeadsRetrieval,
    PagesEvents,
    PagesManageAds,
    PagesManageCta,
    PagesManageInstantArticles,
    PagesManageEngagement,
    PagesManageMetadata,
    PagesManagePosts,
    PagesMessaging,
    PagesReadEngagement,
    PagesReadUserContent,
    PagesShowList,
    PagesUserGender,
    PagesUserLocale,
    PagesUserTimezone,
    PublicProfile,
    PublishToGroups,
    PublishVideo,
    ReadInsights,
    UserAgeRange,
    UserBirthday,
    UserFriends,
    UserGender,
    UserHometown,
    UserLikes,
    UserLink,
    UserLocation,
    UserMessengerContact,
    UserPhotos,
    UserPosts,
    UserVideos,
    //
    //
    //
    #[serde(other)]
    Other(String),
}
impl Scope for FacebookScope {}
