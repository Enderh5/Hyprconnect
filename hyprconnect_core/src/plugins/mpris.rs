use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Mpris {
    #[serde(rename = "playerList")]
    pub player_list: Option<Vec<String>>,
    pub player: Option<String>,

    #[serde(rename = "canPause")]
    pub can_pause: Option<bool>,
    #[serde(rename = "canPlay")]
    pub can_play: Option<bool>,
    #[serde(rename = "canGoNext")]
    pub can_go_next: Option<bool>,
    #[serde(rename = "canGoPrevious")]
    pub can_go_previous: Option<bool>,
    #[serde(rename = "canSeek")]
    pub can_seek: Option<bool>,
    #[serde(rename = "loopStatus")]
    pub loop_status: Option<LoopType>,
    pub shuffle: Option<bool>,
    pub pos: Option<f64>,
    #[serde(rename = "albumArtUrl")]
    pub album_art_url: Option<String>,

    #[deprecated(
        since = "1.0.0",
        note = "⚠️ Deprecated: An inclusive string of the format ‘Artist - Title’."
    )]
    #[serde(rename = "nowPlaying")]
    pub now_playing: Option<String>,

    pub artist: Option<String>,
    pub title: Option<String>,
    pub album: Option<String>,

    //Duranción del archivo en ms
    pub length: Option<f64>,

    //Volumen en porcentaje 0-100
    pub volume: Option<f64>,

    #[serde(rename = "supportAlbumArtPayload")]
    pub support_album_art_payload: Option<bool>,
    #[serde(rename = "transferringAlbumArt")]
    pub transferring_album_art: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MprisRequest {
    pub player: Option<String>,
    #[serde(rename = "requestNowPlaying")]
    pub request_now_playing: Option<bool>,
    #[serde(rename = "requestPlayerList")]
    pub request_player_list: Option<bool>,
    #[serde(rename = "requestVolume")]
    pub request_volume: Option<bool>,
    #[serde(rename = "Seek")]
    pub seek: Option<f64>,
    #[serde(rename = "setLoopStatus")]
    pub set_loop_status: Option<LoopType>,
    #[serde(rename = "setPosition")]
    pub set_position: Option<f64>,
    #[serde(rename = "setShuffle")]
    pub set_shuffle: Option<bool>,
    #[serde(rename = "setVolume")]
    pub set_volume: Option<bool>,
    pub action: MprisAction,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum LoopType {
    None,
    TrackList,
    PlayList,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum MprisAction {
    Pause,
    Play,
    PlayPause,
    Stop,
    Next,
    Previous,
}
