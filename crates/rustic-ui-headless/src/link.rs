#![deny(missing_docs)]
//! Headless helpers for instrumentation friendly links.
//!
//! The state keeps analytics identifiers, accessibility metadata, and keyboard
//! semantics consistent across frameworks.  Renderers simply merge the returned
//! attribute pairs into their markup, guaranteeing that SSR, hydration, and
//! automation tooling observe the same contracts regardless of runtime.

use crate::{aria, interaction::ControlKey};

/// Analytics payload emitted when the link is activated.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LinkAnalyticsEvent {
    /// Logical analytics channel surfaced on the rendered element.
    pub channel: String,
    /// Optional analytics tag supplied by the adapter for granular reporting.
    pub link_tag: Option<String>,
}

/// Describes the outcome of handling a keyboard event.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct LinkKeyboardOutcome {
    /// Whether the event should trigger navigation.
    pub activate: bool,
    /// Analytics payload associated with the activation.
    pub analytics: Option<LinkAnalyticsEvent>,
}

/// Headless state backing instrumentation aware links.
#[derive(Debug, Clone, Default)]
pub struct LinkState {
    disabled: bool,
    activate_on_space: bool,
    analytics_channel: Option<String>,
    analytics_tag: Option<String>,
}

impl LinkState {
    /// Construct a new link controller.
    pub fn new(activate_on_space: bool) -> Self {
        Self {
            disabled: false,
            activate_on_space,
            analytics_channel: None,
            analytics_tag: None,
        }
    }

    /// Returns whether the link is currently disabled.
    pub fn is_disabled(&self) -> bool {
        self.disabled
    }

    /// Mark the link as disabled which removes it from the tab order.
    pub fn set_disabled(&mut self, disabled: bool) {
        self.disabled = disabled;
    }

    /// Configure the analytics channel surfaced on the rendered element.
    pub fn set_analytics_channel(&mut self, channel: Option<impl Into<String>>) {
        self.analytics_channel = channel.map(Into::into);
    }

    /// Configure the analytics tag used when emitting activation events.
    pub fn set_analytics_tag(&mut self, tag: Option<impl Into<String>>) {
        self.analytics_tag = tag.map(Into::into);
    }

    /// Returns the attribute builder describing the anchor element.
    pub fn attributes(&self) -> LinkAttributes<'_> {
        LinkAttributes {
            state: self,
            href: None,
            id: None,
            rel: None,
            target: None,
        }
    }

    /// Handle keyboard events returning whether the link should activate.
    pub fn on_key(&self, key: ControlKey) -> LinkKeyboardOutcome {
        if self.disabled {
            return LinkKeyboardOutcome::default();
        }
        match key {
            ControlKey::Enter => LinkKeyboardOutcome {
                activate: true,
                analytics: self.analytics_payload(),
            },
            ControlKey::Space if self.activate_on_space => LinkKeyboardOutcome {
                activate: true,
                analytics: self.analytics_payload(),
            },
            _ => LinkKeyboardOutcome::default(),
        }
    }

    /// Build the analytics payload when the link activates.
    pub fn analytics_payload(&self) -> Option<LinkAnalyticsEvent> {
        self.analytics_channel
            .as_ref()
            .map(|channel| LinkAnalyticsEvent {
                channel: channel.clone(),
                link_tag: self.analytics_tag.clone(),
            })
    }
}

/// Builder describing the anchor attributes.
#[derive(Debug, Clone)]
pub struct LinkAttributes<'a> {
    state: &'a LinkState,
    href: Option<&'a str>,
    id: Option<&'a str>,
    rel: Option<&'a str>,
    target: Option<&'a str>,
}

impl<'a> LinkAttributes<'a> {
    /// Assign the `href` destination.
    pub fn href(mut self, value: &'a str) -> Self {
        self.href = Some(value);
        self
    }

    /// Assign the `id` attribute.
    pub fn id(mut self, value: &'a str) -> Self {
        self.id = Some(value);
        self
    }

    /// Assign the `rel` attribute.
    pub fn rel(mut self, value: &'a str) -> Self {
        self.rel = Some(value);
        self
    }

    /// Assign the `target` attribute.
    pub fn target(mut self, value: &'a str) -> Self {
        self.target = Some(value);
        self
    }

    /// Collect the configured attributes into reusable pairs.
    pub fn as_pairs(&self) -> Vec<(&'static str, String)> {
        let mut pairs = Vec::with_capacity(8);
        pairs.push(("role", aria::role_link().to_string()));
        pairs.push((
            "tabindex",
            if self.state.disabled { "-1" } else { "0" }.to_string(),
        ));
        if let Some(href) = self.href {
            pairs.push(("href", href.to_string()));
        }
        if let Some(id) = self.id {
            pairs.push(("id", id.to_string()));
        }
        if let Some(rel) = self.rel {
            pairs.push(("rel", rel.to_string()));
        }
        if let Some(target) = self.target {
            pairs.push(("target", target.to_string()));
        }
        if self.state.disabled {
            pairs.push(("aria-disabled", "true".into()));
            pairs.push(("data-disabled", "true".into()));
        }
        if let Some(channel) = self.state.analytics_channel.as_ref() {
            pairs.push(("data-rustic-analytics-channel", channel.clone()));
        }
        if let Some(tag) = self.state.analytics_tag.as_ref() {
            pairs.push(("data-rustic-analytics-id", tag.clone()));
        }
        pairs
    }
}
