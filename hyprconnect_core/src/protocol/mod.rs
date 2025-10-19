pub mod device;

use anyhow::Error;

use std::{
    collections::HashMap,
    time::{SystemTime, UNIX_EPOCH},
};

use serde::{Deserialize, Serialize};

use crate::{
    device::{load_device_name, load_or_generate_device_id},
    plugins::{
        battery::{Battery, BatteryRequest},
        clipboard::{Clipboard, ClipboardConnect},
        connectivity_report::{ConnectivityReport, ConnectivityReportRequest},
        contacts::{
            ContactsRequestAllUuidTimestamps, ContactsRequestVcardsByUuid,
            ContactsResponseUidsTimestamps, ContactsResponseVcards,
        },
        find_my_phone::FindMyPhoneRequest,
        lock::{Lock, LockRequest},
        mousepad::{MousepadEcho, MousepadKeyboardState, MousepadRequest},
        mpris::{Mpris, MprisRequest},
        notification::{Notification, NotificationAction, NotificationReply, NotificationRequest},
        ping::Ping,
        presenter::Presenter,
        run_command::{RunCommand, RunCommandRequest},
        sftp::{Sftp, SftpRequest},
        share::{ShareRequest, ShareRequestUpdate},
        volume::{SystemVolume, SystemVolumeRequest},
        SmsAttachmentFile, SmsMessages, SmsRequest, SmsRequestAttachment, SmsRequestConversation,
        SmsRequestConversations, Telephony, TelephonyRequestMute,
    },
};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Packet {
    pub id: u128,
    #[serde(rename = "type")]
    pub packet_type: String,
    pub body: PacketBody,
    #[serde(rename = "payloadSize")]
    pub payload_size: Option<usize>,
    #[serde(default)]
    pub payload_transfer_info: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PacketBody {
    Identity(Identity),
    Pair(Pair),
    Battery(Battery),
    BatteryRequest(BatteryRequest),
    Clipboard(Clipboard),
    ClipboardConnect(ClipboardConnect),
    ConectivityReport(ConnectivityReport),
    ConectivityReportRequest(ConnectivityReportRequest),
    ContactsRequestAllUuidTimestamps(ContactsRequestAllUuidTimestamps),
    ContactsRequestVcardsByUuid(ContactsRequestVcardsByUuid),
    ContactsResponseVcards(ContactsResponseVcards),
    ContactsResponseUidsTimestamps(ContactsResponseUidsTimestamps),
    FindMyPhoneRequest(FindMyPhoneRequest),
    Lock(Lock),
    LockRequest(LockRequest),
    MousepadEcho(MousepadEcho),
    MousepadKeyboardState(MousepadKeyboardState),
    MousepadRequest(MousepadRequest),
    Mpris(Mpris),
    MprisRequest(MprisRequest),
    Notification(Notification),
    NotificationAction(NotificationAction),
    NotificationReply(NotificationReply),
    NotificationRequest(NotificationRequest),
    Ping(Ping),
    Presenter(Presenter),
    RunCommand(RunCommand),
    RunCommandRequest(RunCommandRequest),
    Sftp(Sftp),
    SftpRequest(SftpRequest),
    ShareRequest(ShareRequest),
    ShareRequestUpdate(ShareRequestUpdate),
    SmsAttachmentFile(SmsAttachmentFile),
    SmsMessages(SmsMessages),
    SmsRequest(SmsRequest),
    SmsRequestAttachment(SmsRequestAttachment),
    SmsRequestConversation(SmsRequestConversation),
    SmsRequestConversations(SmsRequestConversations),
    SystemVolume(SystemVolume),
    SystemVolumeRequest(SystemVolumeRequest),
    Telephony(Telephony),
    TelephonyRequestMute(TelephonyRequestMute),
}

impl Packet {
    pub fn generate_packet(
        body: PacketBody,
        payload_transfer_info: Option<serde_json::Value>,
    ) -> Result<Self, Error> {
        let packet_type = match body {
            PacketBody::Identity(_) => "kdeconnect.identity",
            PacketBody::Pair(_) => "kdeconnect.pair",
            PacketBody::Battery(_) => "kdeconnect.battery",
            PacketBody::BatteryRequest(_) => "kdeconnect.battery.request",
            PacketBody::Clipboard(_) => "kdeconnect.clipboard",
            PacketBody::ClipboardConnect(_) => "kdeconnect.clipboard.connect",
            PacketBody::ConectivityReport(_) => "kdeconnect.conectivity_report",
            PacketBody::ConectivityReportRequest(_) => "kdeconnect.conectivity_report.request",
            PacketBody::ContactsRequestAllUuidTimestamps(_) => {
                "kdeconnect.contacts.request_all_uids_timestamps"
            }
            PacketBody::ContactsRequestVcardsByUuid(_) => {
                "kdeconnect.contacts.request_vcards_by_uid"
            }
            PacketBody::ContactsResponseVcards(_) => "kdeconnect.contacts.response_all_uids_vcards",
            PacketBody::ContactsResponseUidsTimestamps(_) => {
                "kdeconnect.contacts.response_all_uids_timestamps"
            }
            PacketBody::FindMyPhoneRequest(_) => "kdeconnect.findmyphone.request",
            PacketBody::Lock(_) => "kdeconnect.lock",
            PacketBody::LockRequest(_) => "kdeconnect.lock.request",
            PacketBody::MousepadEcho(_) => "kdeconnect.mousepad.echo",
            PacketBody::MousepadKeyboardState(_) => "kdeconnect.mousepad.keyboardstate",
            PacketBody::MousepadRequest(_) => "kdeconnect.mousepad.request",
            PacketBody::Mpris(_) => "kdeconnect.mpris",
            PacketBody::MprisRequest(_) => "kdeconnect.mpris.request",
            PacketBody::Notification(_) => "kdeconnect.notification",
            PacketBody::NotificationAction(_) => "kdeconnect.notification.action",
            PacketBody::NotificationReply(_) => "kdeconnect.notification.reply",
            PacketBody::NotificationRequest(_) => "kdeconnect.notification.request",
            PacketBody::Ping(_) => "kdeconnect.ping",
            PacketBody::Presenter(_) => "kdeconnect.presenter",
            PacketBody::RunCommand(_) => "kdeconnect.runcommand",
            PacketBody::RunCommandRequest(_) => "kdeconnect.runcommand.request",
            PacketBody::Sftp(_) => "kdeconnect.sftp",
            PacketBody::SftpRequest(_) => "kdeconnect.sftp.request",
            PacketBody::ShareRequest(_) => "kdeconnect.share.request",
            PacketBody::ShareRequestUpdate(_) => "kdeconnect.share.request.update",
            PacketBody::SmsAttachmentFile(_) => "kdeconnect.sms.attachment_file",
            PacketBody::SmsMessages(_) => "kdeconnect.sms.messages",
            PacketBody::SmsRequest(_) => "kdeconnect.sms.request",
            PacketBody::SmsRequestAttachment(_) => "kdeconnect.sms.request_attachment",
            PacketBody::SmsRequestConversation(_) => "kdeconnect.sms.request_conversation",
            PacketBody::SmsRequestConversations(_) => "kdeconnect.sms.request_conversations",
            PacketBody::SystemVolume(_) => "kdeconnect.systemvolume",
            PacketBody::SystemVolumeRequest(_) => "kdeconnect.systemvolume.request",
            PacketBody::Telephony(_) => "kdeconnect.telephony",
            PacketBody::TelephonyRequestMute(_) => "kdeconnect.telephony.request_mute",
        }
        .to_string();

        let start = SystemTime::now();
        let current_time = start
            .duration_since(UNIX_EPOCH)
            .expect("time should go forward");

        let payload_size = serde_json::to_vec(&body).map(|v| v.len()).unwrap_or(0);

        Ok(Self {
            id: current_time.as_millis(),
            packet_type,
            body,
            payload_size: Some(payload_size),
            payload_transfer_info,
        })
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct RawPacket {
    pub id: u128,
    #[serde(rename = "type")]
    pub packet_type: String,
    pub body: serde_json::Value, // body genérico
    #[serde(rename = "payloadSize")]
    pub payload_size: Option<usize>,
    #[serde(default)]
    pub payload_transfer_info: Option<serde_json::Value>,
}

impl<'de> Deserialize<'de> for Packet {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let raw = RawPacket::deserialize(deserializer)?;

        // Ahora decidimos qué tipo concreto usar según packet_type
        let body = match raw.packet_type.as_str() {
            "kdeconnect.identity" => PacketBody::Identity(
                serde_json::from_value(raw.body).map_err(serde::de::Error::custom)?,
            ),
            "kdeconnect.pair" => PacketBody::Pair(
                serde_json::from_value(raw.body).map_err(serde::de::Error::custom)?,
            ),

            "kdeconnect.battery" => PacketBody::Battery(
                serde_json::from_value(raw.body).map_err(serde::de::Error::custom)?,
            ),
            "kdeconnect.battery.request" => PacketBody::BatteryRequest(
                serde_json::from_value(raw.body).map_err(serde::de::Error::custom)?,
            ),
            "kdeconnect.clipboard" => PacketBody::Clipboard(
                serde_json::from_value(raw.body).map_err(serde::de::Error::custom)?,
            ),
            "kdeconnect.clipboard.connect" => PacketBody::ClipboardConnect(
                serde_json::from_value(raw.body).map_err(serde::de::Error::custom)?,
            ),
            "kdeconnect.conectivity_report" => PacketBody::ConectivityReport(
                serde_json::from_value(raw.body).map_err(serde::de::Error::custom)?,
            ),
            "kdeconnect.conectivity_report.request" => PacketBody::ConectivityReportRequest(
                serde_json::from_value(raw.body).map_err(serde::de::Error::custom)?,
            ),
            "kdeconnect.contacts.request_all_uids_timestamps" => {
                PacketBody::ContactsRequestAllUuidTimestamps(
                    serde_json::from_value(raw.body).map_err(serde::de::Error::custom)?,
                )
            }
            "kdeconnect.contacts.request_vcards_by_uid" => PacketBody::ContactsRequestVcardsByUuid(
                serde_json::from_value(raw.body).map_err(serde::de::Error::custom)?,
            ),
            "kdeconnect.contacts.response_all_uids_vcards" => PacketBody::ContactsResponseVcards(
                serde_json::from_value(raw.body).map_err(serde::de::Error::custom)?,
            ),
            "kdeconnect.contacts.response_all_uids_timestamps" => {
                PacketBody::ContactsResponseUidsTimestamps(
                    serde_json::from_value(raw.body).map_err(serde::de::Error::custom)?,
                )
            }
            "kdeconnect.findmyphone.request" => PacketBody::FindMyPhoneRequest(
                serde_json::from_value(raw.body).map_err(serde::de::Error::custom)?,
            ),
            "kdeconnect.lock" => PacketBody::Lock(
                serde_json::from_value(raw.body).map_err(serde::de::Error::custom)?,
            ),
            "kdeconnect.lock.request" => PacketBody::LockRequest(
                serde_json::from_value(raw.body).map_err(serde::de::Error::custom)?,
            ),
            "kdeconnect.mousepad.echo" => PacketBody::MousepadEcho(
                serde_json::from_value(raw.body).map_err(serde::de::Error::custom)?,
            ),
            "kdeconnect.mousepad.keyboardstate" => PacketBody::MousepadKeyboardState(
                serde_json::from_value(raw.body).map_err(serde::de::Error::custom)?,
            ),
            "kdeconnect.mousepad.request" => PacketBody::MousepadRequest(
                serde_json::from_value(raw.body).map_err(serde::de::Error::custom)?,
            ),
            "kdeconnect.mpris" => PacketBody::Mpris(
                serde_json::from_value(raw.body).map_err(serde::de::Error::custom)?,
            ),
            "kdeconnect.mpris.request" => PacketBody::MprisRequest(
                serde_json::from_value(raw.body).map_err(serde::de::Error::custom)?,
            ),
            "kdeconnect.notification" => PacketBody::Notification(
                serde_json::from_value(raw.body).map_err(serde::de::Error::custom)?,
            ),
            "kdeconnect.notification.action" => PacketBody::NotificationAction(
                serde_json::from_value(raw.body).map_err(serde::de::Error::custom)?,
            ),
            "kdeconnect.notification.reply" => PacketBody::NotificationReply(
                serde_json::from_value(raw.body).map_err(serde::de::Error::custom)?,
            ),
            "kdeconnect.notification.request" => PacketBody::NotificationRequest(
                serde_json::from_value(raw.body).map_err(serde::de::Error::custom)?,
            ),
            "kdeconnect.notification.request" => PacketBody::NotificationRequest(
                serde_json::from_value(raw.body).map_err(serde::de::Error::custom)?,
            ),
            "kdeconnect.ping" => PacketBody::Ping(
                serde_json::from_value(raw.body).map_err(serde::de::Error::custom)?,
            ),
            "kdeconnect.presenter" => PacketBody::Presenter(
                serde_json::from_value(raw.body).map_err(serde::de::Error::custom)?,
            ),
            "kdeconnect.runcommand" => PacketBody::RunCommand(
                serde_json::from_value(raw.body).map_err(serde::de::Error::custom)?,
            ),
            "kdeconnect.runcommand.request" => PacketBody::RunCommandRequest(
                serde_json::from_value(raw.body).map_err(serde::de::Error::custom)?,
            ),
            "kdeconnect.sftp" => PacketBody::Sftp(
                serde_json::from_value(raw.body).map_err(serde::de::Error::custom)?,
            ),
            "kdeconnect.sftp.request" => PacketBody::SftpRequest(
                serde_json::from_value(raw.body).map_err(serde::de::Error::custom)?,
            ),
            "kdeconnect.share.request" => PacketBody::ShareRequest(
                serde_json::from_value(raw.body).map_err(serde::de::Error::custom)?,
            ),
            "kdeconnect.share.request.update" => PacketBody::ShareRequestUpdate(
                serde_json::from_value(raw.body).map_err(serde::de::Error::custom)?,
            ),
            "kdeconnect.sms.attachment_file" => PacketBody::SmsAttachmentFile(
                serde_json::from_value(raw.body).map_err(serde::de::Error::custom)?,
            ),
            "kdeconnect.sms.messages" => PacketBody::SmsMessages(
                serde_json::from_value(raw.body).map_err(serde::de::Error::custom)?,
            ),
            "kdeconnect.sms.request" => PacketBody::SmsRequest(
                serde_json::from_value(raw.body).map_err(serde::de::Error::custom)?,
            ),
            "kdeconnect.sms.request_attachment" => PacketBody::SmsRequestAttachment(
                serde_json::from_value(raw.body).map_err(serde::de::Error::custom)?,
            ),
            "kdeconnect.sms.request_conversation" => PacketBody::SmsRequestConversation(
                serde_json::from_value(raw.body).map_err(serde::de::Error::custom)?,
            ),
            "kdeconnect.sms.request_conversations" => PacketBody::SmsRequestConversations(
                serde_json::from_value(raw.body).map_err(serde::de::Error::custom)?,
            ),
            "kdeconnect.systemvolume" => PacketBody::SystemVolume(
                serde_json::from_value(raw.body).map_err(serde::de::Error::custom)?,
            ),
            "kdeconnect.systemvolume.request" => PacketBody::SystemVolumeRequest(
                serde_json::from_value(raw.body).map_err(serde::de::Error::custom)?,
            ),
            "kdeconnect.telephony" => PacketBody::Telephony(
                serde_json::from_value(raw.body).map_err(serde::de::Error::custom)?,
            ),
            "kdeconnect.telephony.request_mute" => PacketBody::TelephonyRequestMute(
                serde_json::from_value(raw.body).map_err(serde::de::Error::custom)?,
            ),
            other => {
                return Err(serde::de::Error::custom(format!(
                    "Tipo de paquete desconocido: {}",
                    other
                )));
            }
        };

        Ok(Packet {
            id: raw.id,
            packet_type: raw.packet_type,
            body,
            payload_size: raw.payload_size,
            payload_transfer_info: raw.payload_transfer_info,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Identity {
    #[serde(rename = "deviceId")]
    pub device_id: String,

    #[serde(rename = "deviceName")]
    pub device_name: String,

    /// "phone" o "desktop"
    #[serde(rename = "deviceType")]
    pub device_type: String,

    #[serde(rename = "incomingCapabilities")]
    pub incoming_capabilities: Vec<String>,
    #[serde(rename = "outgoingCapabilities")]
    pub outgoing_capabilities: Vec<String>,

    ///Solo válido 7
    #[serde(rename = "protocolVersion")]
    pub protocol_version: u8,

    /// Campos extra según el transporte: tcpPort, btAddress, etc.
    #[serde(flatten)]
    pub extras: HashMap<String, serde_json::Value>,
}

impl Identity {
    pub fn new(extras: HashMap<String, serde_json::Value>) -> Result<Self, Error> {
        Ok(Self {
            device_id: load_or_generate_device_id(),
            device_name: load_device_name(),
            device_type: "desktop".to_string(),
            incoming_capabilities: vec!["kdeconnect.ping".to_string()],
            outgoing_capabilities: vec!["kdeconnect.ping".to_string()],
            protocol_version: 8,
            extras,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pair {
    pub pair: bool,
}
