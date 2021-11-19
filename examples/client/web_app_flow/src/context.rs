use std::{collections::HashMap, error};

use http_api_isahc_client::IsahcClient;
use oauth2_amazon::{
    AmazonExtensionsBuilder, AmazonProviderWithWebServices, AmazonScope, AmazonTokenUrlRegion,
};
use oauth2_apple::AppleProviderWithAppleJs;
use oauth2_bitbucket::{
    BitbucketExtensionsBuilder, BitbucketProviderWithWebApplication, BitbucketScope,
};
use oauth2_digitalocean::{
    DigitaloceanExtensionsBuilder, DigitaloceanProviderWithWebApplication, DigitaloceanScope,
};
use oauth2_dropbox::{DropboxExtensionsBuilder, DropboxProviderWithWebApplication, DropboxScope};
use oauth2_facebook::{FacebookExtensionsBuilder, FacebookProviderForWebApp, FacebookScope};
use oauth2_github::{GithubExtensionsBuilder, GithubProviderWithWebApplication, GithubScope};
use oauth2_gitlab::{
    GitlabExtensionsBuilder, GitlabProviderForEndUsers, GitlabScope, BASE_URL_GITLAB_COM,
};
use oauth2_google::{
    GoogleExtensionsBuilder, GoogleProviderForWebServerApps,
    GoogleProviderForWebServerAppsAccessType, GoogleScope,
};
use oauth2_instagram::{
    InstagramExtensionsBuilder, InstagramProviderForBasicDisplayApi, InstagramScope,
};
use oauth2_linkedin::{
    LinkedinExtensionsBuilder, LinkedinProviderWithWebApplication, LinkedinScope,
};
use oauth2_mastodon::{
    MastodonExtensionsBuilder, MastodonProviderForEndUsers, MastodonScope, BASE_URL_MASTODON_SOCIAL,
};
use oauth2_microsoft::{
    MicrosoftExtensionsBuilder, MicrosoftProviderForWebApps, MicrosoftScope, TENANT_COMMON,
};
use oauth2_okta::{OktaProviderForWebApplication, OktaScope};
use oauth2_signin::{oauth2_client::DefaultExtensionsBuilder, web_app::SigninFlow};
use oauth2_twitch::{TwitchExtensionsBuilder, TwitchProviderForWebServerApps, TwitchScope};
use oauth2_yahoo::{YahooExtensionsBuilder, YahooProviderForWebApps, YahooScope};

use crate::config::Config;

pub struct Context {
    pub config: Config,
    pub signin_flow_map: HashMap<&'static str, SigninFlow<IsahcClient>>,
}

impl Context {
    pub fn new(config: Config) -> Result<Self, Box<dyn error::Error>> {
        let clients_config = config.clients_config.to_owned();

        let mut signin_flow_map = HashMap::new();
        signin_flow_map.insert(
            "github",
            SigninFlow::new(
                IsahcClient::new()?,
                GithubProviderWithWebApplication::new(
                    clients_config.github.client_id.to_owned(),
                    clients_config.github.client_secret.to_owned(),
                    clients_config.github.redirect_uri.to_owned(),
                )?,
                vec![GithubScope::PublicRepo, GithubScope::UserEmail],
                GithubExtensionsBuilder,
            ),
        );
        signin_flow_map.insert(
            "google",
            SigninFlow::new(
                IsahcClient::new()?,
                GoogleProviderForWebServerApps::new(
                    clients_config.google.client_id.to_owned(),
                    clients_config.google.client_secret.to_owned(),
                    clients_config.google.redirect_uri.to_owned(),
                )?
                .configure(|mut x| {
                    x.access_type = Some(GoogleProviderForWebServerAppsAccessType::Offline);
                    x.include_granted_scopes = Some(true);
                }),
                vec![
                    GoogleScope::Email,
                    GoogleScope::Profile,
                    GoogleScope::Openid,
                ],
                GoogleExtensionsBuilder,
            ),
        );
        signin_flow_map.insert(
            "twitch",
            SigninFlow::new(
                IsahcClient::new()?,
                TwitchProviderForWebServerApps::new(
                    clients_config.twitch.client_id.to_owned(),
                    clients_config.twitch.client_secret.to_owned(),
                    clients_config.twitch.redirect_uri.to_owned(),
                )?,
                vec![TwitchScope::UserReadEmail],
                TwitchExtensionsBuilder,
            ),
        );
        signin_flow_map.insert(
            "mastodon-social",
            SigninFlow::new(
                IsahcClient::new()?,
                MastodonProviderForEndUsers::new(
                    BASE_URL_MASTODON_SOCIAL,
                    clients_config.mastodon_social.client_id.to_owned(),
                    clients_config.mastodon_social.client_secret.to_owned(),
                    clients_config.mastodon_social.redirect_uri.to_owned(),
                )?,
                vec![MastodonScope::Read, MastodonScope::Write],
                MastodonExtensionsBuilder,
            ),
        );
        signin_flow_map.insert(
            "apple",
            SigninFlow::new(
                IsahcClient::new()?,
                AppleProviderWithAppleJs::new(
                    clients_config.apple.client_id.to_owned(),
                    clients_config.apple.client_secret.to_owned(),
                    clients_config.apple.redirect_uri.to_owned(),
                )?,
                None,
                DefaultExtensionsBuilder,
            ),
        );
        signin_flow_map.insert(
            "instagram",
            SigninFlow::new(
                IsahcClient::new()?,
                InstagramProviderForBasicDisplayApi::new(
                    clients_config.instagram.client_id.to_owned(),
                    clients_config.instagram.client_secret.to_owned(),
                    clients_config.instagram.redirect_uri.to_owned(),
                )?,
                vec![InstagramScope::UserMedia, InstagramScope::UserProfile],
                InstagramExtensionsBuilder,
            ),
        );
        signin_flow_map.insert(
            "facebook",
            SigninFlow::new(
                IsahcClient::new()?,
                FacebookProviderForWebApp::new(
                    clients_config.facebook.client_id.to_owned(),
                    clients_config.facebook.client_secret.to_owned(),
                    clients_config.facebook.redirect_uri.to_owned(),
                )?,
                vec![FacebookScope::Email, FacebookScope::PublicProfile],
                FacebookExtensionsBuilder,
            ),
        );
        signin_flow_map.insert(
            "amazon",
            SigninFlow::new(
                IsahcClient::new()?,
                AmazonProviderWithWebServices::new(
                    clients_config.amazon.client_id.to_owned(),
                    clients_config.amazon.client_secret.to_owned(),
                    clients_config.amazon.redirect_uri.to_owned(),
                    AmazonTokenUrlRegion::NA,
                )?,
                vec![AmazonScope::Profile, AmazonScope::PostalCode],
                AmazonExtensionsBuilder,
            ),
        );
        signin_flow_map.insert(
            "gitlab",
            SigninFlow::new(
                IsahcClient::new()?,
                GitlabProviderForEndUsers::new(
                    BASE_URL_GITLAB_COM,
                    clients_config.gitlab.client_id.to_owned(),
                    clients_config.gitlab.client_secret.to_owned(),
                    clients_config.gitlab.redirect_uri.to_owned(),
                )?,
                vec![
                    GitlabScope::Openid,
                    GitlabScope::Profile,
                    GitlabScope::Email,
                    GitlabScope::ReadUser,
                ],
                GitlabExtensionsBuilder,
            ),
        );
        signin_flow_map.insert(
            "bitbucket",
            SigninFlow::new(
                IsahcClient::new()?,
                BitbucketProviderWithWebApplication::new(
                    clients_config.bitbucket.client_id.to_owned(),
                    clients_config.bitbucket.client_secret.to_owned(),
                    clients_config.bitbucket.redirect_uri.to_owned(),
                )?,
                vec![
                    BitbucketScope::Account,
                    BitbucketScope::Email,
                    BitbucketScope::Repository,
                ],
                BitbucketExtensionsBuilder,
            ),
        );
        signin_flow_map.insert(
            "digitalocean",
            SigninFlow::new(
                IsahcClient::new()?,
                DigitaloceanProviderWithWebApplication::new(
                    clients_config.digitalocean.client_id.to_owned(),
                    clients_config.digitalocean.client_secret.to_owned(),
                    clients_config.digitalocean.redirect_uri.to_owned(),
                )?,
                vec![DigitaloceanScope::Read, DigitaloceanScope::Write],
                DigitaloceanExtensionsBuilder,
            ),
        );
        signin_flow_map.insert(
            "dropbox",
            SigninFlow::new(
                IsahcClient::new()?,
                DropboxProviderWithWebApplication::new(
                    clients_config.dropbox.client_id.to_owned(),
                    clients_config.dropbox.client_secret.to_owned(),
                    clients_config.dropbox.redirect_uri.to_owned(),
                )?,
                vec![DropboxScope::AccountInfoRead, DropboxScope::SharingRead],
                DropboxExtensionsBuilder,
            ),
        );
        signin_flow_map.insert(
            "linkedin",
            SigninFlow::new(
                IsahcClient::new()?,
                LinkedinProviderWithWebApplication::new(
                    clients_config.linkedin.client_id.to_owned(),
                    clients_config.linkedin.client_secret.to_owned(),
                    clients_config.linkedin.redirect_uri.to_owned(),
                )?,
                vec![
                    LinkedinScope::ReadLiteprofile,
                    LinkedinScope::ReadEmailaddress,
                ],
                LinkedinExtensionsBuilder,
            ),
        );
        signin_flow_map.insert(
            "microsoft",
            SigninFlow::new(
                IsahcClient::new()?,
                MicrosoftProviderForWebApps::new(
                    TENANT_COMMON,
                    clients_config.microsoft.client_id.to_owned(),
                    clients_config.microsoft.client_secret.to_owned(),
                    clients_config.microsoft.redirect_uri,
                )?,
                vec![
                    MicrosoftScope::Openid,
                    MicrosoftScope::Email,
                    MicrosoftScope::Profile,
                ],
                MicrosoftExtensionsBuilder,
            ),
        );
        signin_flow_map.insert(
            "yahoo",
            SigninFlow::new(
                IsahcClient::new()?,
                YahooProviderForWebApps::new(
                    clients_config.yahoo.client_id.to_owned(),
                    clients_config.yahoo.client_secret.to_owned(),
                    clients_config.yahoo.redirect_uri,
                )?,
                vec![YahooScope::Openid, YahooScope::Email, YahooScope::Profile],
                YahooExtensionsBuilder,
            ),
        );
        signin_flow_map.insert(
            "okta",
            SigninFlow::new(
                IsahcClient::new()?,
                OktaProviderForWebApplication::new(
                    clients_config
                        .okta
                        .extra
                        .get("domain")
                        .cloned()
                        .unwrap()
                        .as_str()
                        .unwrap(),
                    None,
                    clients_config.okta.client_id.to_owned(),
                    clients_config.okta.client_secret.to_owned(),
                    clients_config.okta.redirect_uri,
                )?,
                vec![OktaScope::Openid, OktaScope::Email, OktaScope::Profile],
                DefaultExtensionsBuilder,
            ),
        );

        Ok(Self {
            config,
            signin_flow_map,
        })
    }
}
