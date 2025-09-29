use rustic_ui_headless::speed_dial::SpeedDialAnalyticsEvent;
use serde::Serialize;
use time::{macros::datetime, OffsetDateTime};

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct TelemetryRecord {
    pub channel: String,
    pub event: String,
    pub action_index: Option<usize>,
    pub action_tag: Option<String>,
    #[serde(with = "time::serde::rfc3339")]
    pub occurred_at: OffsetDateTime,
}

impl TelemetryRecord {
    pub fn from_speed_dial(event: SpeedDialAnalyticsEvent, occurred_at: OffsetDateTime) -> Self {
        let (event_name, action_index, action_tag) = match event.kind {
            rustic_ui_headless::speed_dial::SpeedDialAnalyticsKind::Opened => {
                ("opened".to_string(), None, None)
            }
            rustic_ui_headless::speed_dial::SpeedDialAnalyticsKind::Closed => {
                ("closed".to_string(), None, None)
            }
            rustic_ui_headless::speed_dial::SpeedDialAnalyticsKind::Action { index, tag } => {
                ("action".to_string(), Some(index), tag)
            }
        };
        Self {
            channel: event.channel,
            event: event_name,
            action_index,
            action_tag,
            occurred_at,
        }
    }

    pub fn to_json_line(&self) -> String {
        serde_json::to_string(self)
            .unwrap_or_else(|_| "{\"error\":\"serialization_failed\"}".into())
    }
}

#[derive(Debug, Clone, Default, Serialize, PartialEq, Eq)]
pub struct TelemetryLog {
    records: Vec<TelemetryRecord>,
}

impl TelemetryLog {
    pub fn push(&mut self, record: TelemetryRecord) {
        self.records.push(record);
    }

    pub fn iter(&self) -> impl Iterator<Item = &TelemetryRecord> {
        self.records.iter()
    }

    pub fn as_json_lines(&self) -> String {
        self.records
            .iter()
            .map(TelemetryRecord::to_json_line)
            .collect::<Vec<_>>()
            .join("\n")
    }
}

pub fn sample_log() -> TelemetryLog {
    let mut log = TelemetryLog::default();
    log.push(TelemetryRecord {
        channel: "navigation.speed_dial".into(),
        event: "action".into(),
        action_index: Some(0),
        action_tag: Some("action.create".into()),
        occurred_at: datetime!(2024-01-01 12:00:00 UTC),
    });
    log
}
