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
    #[serde(rename = "renoteId")]
    pub(crate) renote_id: Option<String>,
    pub(crate) text: String,
    pub(crate) visibilty: Option<Visibilty>,
    #[serde(rename = "localOnly")]
    pub(crate) local_only: Option<bool>,
    pub(crate) cw: Option<bool>,
    pub(crate) comment: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) enum Visibilty {
    Public,
    Home,
    Followers,
}
