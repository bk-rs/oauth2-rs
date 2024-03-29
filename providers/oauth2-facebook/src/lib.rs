use oauth2_client::re_exports::{Deserialize_enum_str, Scope, Serialize_enum_str};

pub const TOKEN_URL: &str = "https://graph.facebook.com/v15.0/oauth/access_token";
pub const AUTHORIZATION_URL: &str = "https://www.facebook.com/v15.0/dialog/oauth";
pub const DEVICE_TOKEN_URL: &str = "https://graph.facebook.com/v15.0/device/login_status";
pub const DEVICE_AUTHORIZATION_URL: &str = "https://graph.facebook.com/v15.0/device/login";

pub mod authorization_code_grant;
pub mod device_authorization_grant;

pub use authorization_code_grant::FacebookProviderForWebApp;
pub use device_authorization_grant::FacebookProviderForDevices;

pub mod extensions;
pub use extensions::FacebookExtensionsBuilder;

#[derive(Deserialize_enum_str, Serialize_enum_str, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum FacebookScope {
    // Ref https://github.com/bk-rs/facebook-rs/blob/master/facebook-permission/src/lib.rs
    AdsManagement,
    AdsRead,
    AttributionRead,
    BusinessManagement,
    CatalogManagement,
    Email,
    GamingUserLocale,
    GroupsAccessMemberInfo,
    InstagramBasic,
    InstagramContentPublish,
    InstagramManageComments,
    InstagramManageInsights,
    InstagramShoppingTagProducts,
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
    PrivateComputationAccess,
    PublicProfile,
    PublishToGroups,
    PublishVideo,
    ReadInsights,
    ResearchApis,
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
    WhatsappBusinessManagement,
    WhatsappBusinessMessaging,
    //
    //
    //
    #[serde(other)]
    Other(String),
}
impl Scope for FacebookScope {}
