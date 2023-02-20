use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{confs::CONFS, init::REQWEST_CLIENT, log};

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct StreamingBody {
    #[serde(rename = "type")]
    pub(crate) streaming_type: String,
    pub(crate) body: ChannelBody,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct ChannelBody {
    pub(crate) id: String,
    #[serde(rename = "type")]
    pub(crate) channel_type: ChannelType,
    pub(crate) body: NoteBody,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) enum ChannelType {
    Note,
    Mention,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct NoteBody {
    pub(crate) id: String,
    pub(crate) renote_id: Option<String>,
    pub(crate) text: Option<String>,
    pub(crate) visibility: Visibility,
    pub(crate) local_only: Option<bool>,
    pub(crate) cw: Option<String>,
    pub(crate) user: NotedUser,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) enum Visibility {
    Public,
    Home,
    Followers,
    Specified,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct NotedUser {
    pub(crate) id: String,
    pub(crate) is_cat: bool,
    pub(crate) is_bot: bool,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct User {
    pub(crate) id: String,
    pub(crate) is_cat: bool,
    pub(crate) is_bot: bool,
    pub(crate) is_moderator: Option<bool>,
    pub(crate) is_admin: Option<bool>,
    pub(crate) roles: Option<Vec<Role>>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Role {
    pub(crate) name: String,
    pub(crate) is_moderator: bool,
    pub(crate) is_administrator: bool,
}

impl User {
    pub(crate) async fn me() -> Result<Self> {
        log!("BOOT" -> "Getting ready for my account...".cyan());

        let me: User = REQWEST_CLIENT
            .post(format!("https://{}/api/i", CONFS.mk_endpnt))
            .json(&json!({
                "i": CONFS.mk_token,
            }))
            .send()
            .await?
            .json()
            .await?;

        Ok(me)
    }

    pub(crate) async fn from(id: &str) -> Result<Self> {
        log!(
            "INFO" | "{} {}{}",
            "Fetching the account of".cyan(),
            id.green().bold(),
            "...".cyan()
        );

        let user: User = REQWEST_CLIENT
            .post(format!("https://{}/api/users/show", CONFS.mk_endpnt))
            .json(&json!({
                "i": CONFS.mk_token,
                "userId": id,
            }))
            .send()
            .await?
            .json()
            .await?;

        Ok(user)
    }

    pub(crate) async fn is_tums_mod(&self) -> bool {
        self.is_moderator.is_some_and(|b| b)
            || self.is_admin.is_some_and(|b| b)
            || self.roles.clone().is_some_and(|r| {
                r.iter()
                    .any(|r| r.is_moderator || r.is_administrator || r.name == *"Tums")
            })
    }
}
