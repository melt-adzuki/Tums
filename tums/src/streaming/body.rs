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
    pub(crate) channel_type: String,
    pub(crate) body: NoteBody,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct NoteBody {
    pub(crate) id: String,
    #[serde(rename = "renoteId")]
    pub(crate) renote_id: Option<String>,
    pub(crate) text: Option<String>,
    pub(crate) visibility: Visibility,
    #[serde(rename = "localOnly")]
    pub(crate) local_only: Option<bool>,
    pub(crate) cw: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) enum Visibility {
    #[serde(rename = "public")]
    Public,
    #[serde(rename = "home")]
    Home,
    #[serde(rename = "followers")]
    Followers,
    #[serde(rename = "specified")]
    Specified,
}
