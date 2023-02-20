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
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) enum Visibility {
    Public,
    Home,
    Followers,
    Specified,
}
