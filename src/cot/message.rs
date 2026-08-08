use arma_rs::{FromArma, FromArmaError};
use chrono::{Duration, SecondsFormat, Utc};
use uuid::Uuid;

pub struct MessagePayload {
    pub sender_callsign: String,
    pub chatroom: String,
    pub message_text: String,
    pub point_lat: f64,
    pub point_lon: f64,
    pub point_hae: f32,
    pub sender_uid: String,
}

impl FromArma for MessagePayload {
    fn from_arma(data: String) -> Result<Self, FromArmaError> {
        let (sender_callsign, chatroom, message_text, point_lat, point_lon, point_hae, sender_uid) =
            <(String, String, String, f64, f64, f32, String)>::from_arma(data)?;

        Ok(Self {
            sender_callsign,
            chatroom,
            message_text,
            point_lat,
            point_lon,
            point_hae,
            sender_uid,
        })
    }
}

pub struct MessageCot {
    pub sender_callsign: String,
    pub chatroom: String,
    pub message_text: String,
    pub point_lat: f64,
    pub point_lon: f64,
    pub point_hae: f32,
    pub sender_uid: String,
}

impl MessageCot {
    pub fn from_payload(p: MessagePayload) -> Self {
        Self {
            sender_callsign: p.sender_callsign,
            chatroom: p.chatroom,
            message_text: p.message_text,
            point_lat: p.point_lat,
            point_lon: p.point_lon,
            point_hae: p.point_hae,
            sender_uid: p.sender_uid,
        }
    }

    pub fn to_xml(&self) -> String {
        let created_time = Utc::now().to_rfc3339_opts(SecondsFormat::Millis, true);
        let stale_time =
            (Utc::now() + Duration::days(1)).to_rfc3339_opts(SecondsFormat::Millis, true);

        // MESSAGE ID (random UUID)
        let message_uuid = Uuid::new_v4().to_string();

        // FULL EVENT UID
        // format: GeoChat.{sender}.{chatroom}.{uuid}
        let event_uid = format!(
            "GeoChat.{}.{}.{}",
            self.sender_uid,
            self.chatroom.replace(" ", "_"),
            message_uuid,
        );

        let mut xml = String::new();

        xml.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\"?>");

        xml.push_str(
            format!(
                "<event version=\"2.0\" uid=\"{}\" type=\"b-t-f\" time=\"{}\" start=\"{}\" stale=\"{}\" how=\"h-g-i-g-o\" access=\"Undefined\">",
                event_uid, created_time, created_time, stale_time
            )
            .as_str(),
        );

        xml.push_str(
            format!(
                "<point lat=\"{}\" lon=\"{}\" hae=\"{}\" ce=\"10.3\" le=\"9999999.0\"/>",
                self.point_lat, self.point_lon, self.point_hae
            )
            .as_str(),
        );

        xml.push_str("<detail>");

        // ========== CHAT OBJECT ==========

        xml.push_str(
            format!(
                "<__chat parent=\"RootContactGroup\" groupOwner=\"false\" \
                 messageId=\"{}\" chatroom=\"{}\" id=\"{}\" senderCallsign=\"{}\">",
                message_uuid, self.chatroom, self.chatroom, self.sender_callsign,
            )
            .as_str(),
        );

        xml.push_str(
            format!(
                "<chatgrp uid0=\"{}\" uid1=\"{}\" id=\"{}\" />",
                self.sender_uid, self.chatroom, self.chatroom
            )
            .as_str(),
        );

        xml.push_str("</__chat>");

        // ========== LINK ELEMENT ==========
        xml.push_str(
            format!(
                "<link uid=\"{}\" type=\"a-f-G-U-C\" relation=\"p-p\" />",
                self.sender_uid
            )
            .as_str(),
        );

        // ========== SERVER DEST ==========
        // This is optional — you may remove or customize it
        xml.push_str(
            format!(
                "<__serverdestination destinations=\"0.0.0.0:0:tcp:{}\" />",
                self.sender_uid
            )
            .as_str(),
        );

        // ========== MESSAGE REMARKS ==========
        xml.push_str(
            format!(
                "<remarks source=\"ARMATAK.{}\" to=\"{}\" time=\"{}\">{}</remarks>",
                self.sender_uid, self.chatroom, created_time, self.message_text
            )
            .as_str(),
        );

        xml.push_str("</detail></event>");

        xml
    }
}

#[cfg(test)]
mod tests {
    use super::{MessageCot, MessagePayload};
    use arma_rs::FromArma;

    #[test]
    fn parses_chat_payload_and_serializes_geochat_event() {
        let payload = MessagePayload::from_arma(
            r#"["Falcon","Operations Room","Message received",-30.1,-51.2,15.0,"sender-1"]"#
                .to_string(),
        )
        .expect("chat payload should parse");

        assert_eq!(payload.sender_callsign, "Falcon");
        assert_eq!(payload.chatroom, "Operations Room");
        assert_eq!(payload.message_text, "Message received");
        assert_eq!(payload.sender_uid, "sender-1");

        let cot = MessageCot::from_payload(payload);
        let xml = cot.to_xml();

        assert!(xml.starts_with("<?xml version=\"1.0\" encoding=\"UTF-8\"?>"));
        assert!(xml.contains("type=\"b-t-f\""));
        assert!(xml.contains("uid=\"GeoChat.sender-1.Operations_Room."));
        assert!(xml.contains("lat=\"-30.1\" lon=\"-51.2\" hae=\"15\""));
        assert!(xml.contains("chatroom=\"Operations Room\""));
        assert!(xml.contains("senderCallsign=\"Falcon\""));
        assert!(xml.contains("<chatgrp uid0=\"sender-1\" uid1=\"Operations Room\" id=\"Operations Room\" />"));
        assert!(xml.contains("<link uid=\"sender-1\" type=\"a-f-G-U-C\" relation=\"p-p\" />"));
        assert!(xml.contains("<__serverdestination destinations=\"0.0.0.0:0:tcp:sender-1\" />"));
        assert!(xml.contains("<remarks source=\"ARMATAK.sender-1\" to=\"Operations Room\""));
        assert!(xml.contains(">Message received</remarks>"));
        assert!(xml.ends_with("</detail></event>"));
    }
}
