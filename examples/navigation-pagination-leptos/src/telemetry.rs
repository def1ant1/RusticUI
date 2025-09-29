use rustic_ui_headless::pagination::PaginationAnalyticsEvent;
use serde::Serialize;
use time::{macros::datetime, OffsetDateTime};

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct TelemetryRecord {
    pub channel: String,
    pub page_tag: Option<String>,
    pub page_index: usize,
    #[serde(with = "time::serde::rfc3339")]
    pub occurred_at: OffsetDateTime,
}

impl TelemetryRecord {
    pub fn from_pagination(event: PaginationAnalyticsEvent, occurred_at: OffsetDateTime) -> Self {
        Self {
            channel: event.channel,
            page_tag: event.page_tag,
            page_index: event.page_index,
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
        channel: "navigation.pagination".into(),
        page_tag: Some("page.0".into()),
        page_index: 0,
        occurred_at: datetime!(2024-01-01 12:00:00 UTC),
    });
    log
}
