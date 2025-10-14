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

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Packet {
    pub id: u128,
    #[serde(rename = "type")]
    pub packet_type: String,
    pub body: PacketBody,
    #[serde(rename = "payloadSize")]
    pub payload_size: f64,
    #[serde(default)]
    pub payload_transfer_info: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
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
            PacketBody::ClipboardConnect(_) => "kdeconnect.clipboard",
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

        let payload_size = serde_json::to_vec(&body)
            .map(|v| v.len() as f64)
            .unwrap_or(0.0);

        Ok(Self {
            id: current_time.as_millis(),
            packet_type,
            body,
            payload_size,
            payload_transfer_info,
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
    #[serde(rename = "device_type")]
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
            incoming_capabilities: vec![],
            outgoing_capabilities: vec![],
            protocol_version: 7,
            extras,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pair {
    pub pair: bool,
}
