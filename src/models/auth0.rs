use ::serde::{Deserialize, Serialize};

pub type Auth0Users = Vec<Auth0User>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Auth0User {
    #[serde(rename = "app_metadata")]
    pub app_metadata: AppMetadata,
    #[serde(rename = "created_at")]
    pub created_at: String,
    pub email: String,
    #[serde(rename = "email_verified")]
    pub email_verified: bool,
    #[serde(rename = "family_name")]
    pub family_name: String,
    #[serde(rename = "given_name")]
    pub given_name: String,
    pub identities: Vec<Identity>,
    #[serde(rename = "last_ip")]
    pub last_ip: String,
    #[serde(rename = "last_login")]
    pub last_login: String,
    #[serde(rename = "ldap_fetched_at")]
    pub ldap_fetched_at: String,
    #[serde(rename = "logins_count")]
    pub logins_count: i64,
    pub name: String,
    pub nickname: String,
    pub picture: String,
    #[serde(rename = "updated_at")]
    pub updated_at: String,
    #[serde(rename = "user_id")]
    pub user_id: String,
    #[serde(rename = "user_metadata")]
    pub user_metadata: UserMetadata,
    pub username: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppMetadata {
    pub stopforumspam: Stopforumspam,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Stopforumspam {
    pub email: Email,
    pub ip: Ip,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Email {
    #[serde(rename = "_result_for")]
    pub result_for: String,
    pub appears: i64,
    pub frequency: i64,
    pub value: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ip {
    #[serde(rename = "_result_for")]
    pub result_for: String,
    pub appears: i64,
    pub asn: i64,
    pub country: String,
    pub frequency: i64,
    pub value: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Identity {
    pub connection: String,
    pub is_social: bool,
    pub provider: String,
    #[serde(rename = "user_id")]
    pub user_id: String,
    #[serde(rename = "access_token")]
    pub access_token: Option<String>,
    #[serde(rename = "expires_in")]
    pub expires_in: Option<i64>,
    pub profile_data: Option<ProfileData>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProfileData {
    pub email: String,
    #[serde(rename = "email_verified")]
    pub email_verified: bool,
    #[serde(rename = "family_name")]
    pub family_name: String,
    #[serde(rename = "given_name")]
    pub given_name: String,
    pub locale: String,
    pub name: String,
    pub picture: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserMetadata {
    #[serde(rename = "family_name")]
    pub family_name: String,
    #[serde(rename = "first_verification_email_sent")]
    pub first_verification_email_sent: bool,
    #[serde(rename = "given_name")]
    pub given_name: String,
}
