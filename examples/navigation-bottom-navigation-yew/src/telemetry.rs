use rustic_ui_headless::bottom_navigation::BottomNavigationAnalyticsEvent;
use serde::Serialize;
use time::{macros::datetime, OffsetDateTime};

/// Lightweight telemetry record serialised to newline-delimited JSON for portability.
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct TelemetryRecord {
    pub channel: String,
    pub item_tag: Option<String>,
    pub index: usize,
    #[serde(with = "time::serde::rfc3339")]
    pub occurred_at: OffsetDateTime,
}

impl TelemetryRecord {
    /// Convert a headless analytics payload into the transport-friendly record format.
    pub fn from_bottom_nav(
        event: BottomNavigationAnalyticsEvent,
        occurred_at: OffsetDateTime,
    ) -> Self {
        Self {
            channel: event.channel,
            item_tag: event.item_tag,
            index: event.index,
            occurred_at,
        }
    }

    /// Serialise the record into a JSON line so monitoring stacks can ingest it directly.
    pub fn to_json_line(&self) -> String {
        serde_json::to_string(self)
            .unwrap_or_else(|_| "{\"error\":\"serialization_failed\"}".to_string())
    }
}

/// Simple telemetry buffer used by the example to accumulate analytics events.
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

    /// Render the log as newline-delimited JSON, mirroring the SSR harness output.
    pub fn as_json_lines(&self) -> String {
        self.records
            .iter()
            .map(TelemetryRecord::to_json_line)
            .collect::<Vec<_>>()
            .join("\n")
    }
}

/// Render a stable telemetry sample for SSR documentation and tests.
pub fn sample_log() -> TelemetryLog {
    let mut log = TelemetryLog::default();
    log.push(TelemetryRecord {
        channel: "navigation.bottom".into(),
        item_tag: Some("destination.overview".into()),
        index: 0,
        occurred_at: datetime!(2024-01-01 12:00:00 UTC),
    });
    log
}
