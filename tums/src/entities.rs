use serde::{Deserialize, Serialize};

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
    pub(crate) user: User,
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
pub(crate) struct User {
    pub(crate) id: String,
    pub(crate) username: String,
    pub(crate) is_cat: bool,
    pub(crate) is_bot: bool,
}

impl User {
    pub(crate) async fn me() -> anyhow::Result<Self> {
        use crate::{confs::CONFS, init::REQWEST_CLIENT, log};
        use serde_json::json;

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
}
