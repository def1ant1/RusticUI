//! Portal orchestration utilities shared across framework adapters.
//!
//! These helpers provide deterministic anchor and container markup that server
//! rendering backends can emit before the client framework hydrates.  Each
//! [`PortalMount`] produces a hidden anchor element colocated with the trigger
//! alongside a detached container appended to the end of the markup. When the
//! client framework boots it can look for these `data-portal-*` attributes and
//! attach the floating surface (menus, selects, tooltips) to the document body
//! without guessing how the server structured the DOM.
//!
//! Centralising the HTML generation keeps automation resilient—portal IDs are
//! derived from the automation identifiers that Material components already
//! expose which makes it trivial for QA suites to target the rendered popovers
//! regardless of the framework powering the page.

use mui_utils::attributes_to_html;

/// Distinguishes the type of surface being rendered.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PortalLayer {
    /// Floating surfaces such as menus, selects and tooltips.
    Popover,
}

impl PortalLayer {
    /// Returns the string identifier written into `data-portal-layer`.
    pub fn as_str(self) -> &'static str {
        match self {
            PortalLayer::Popover => "popover",
        }
    }
}

/// Lightweight description of the portal markup generated for a surface.
#[derive(Clone, Debug)]
pub struct PortalFragment {
    container_id: String,
    markup: String,
}

impl PortalFragment {
    /// Identifier attached to the detached container.
    pub fn container_id(&self) -> &str {
        &self.container_id
    }

    /// Raw HTML snippet representing the detached container.
    pub fn html(&self) -> &str {
        &self.markup
    }

    /// Consumes the fragment and returns the container markup.
    pub fn into_html(self) -> String {
        self.markup
    }
}

/// Descriptor that knows how to render both the anchor placeholder and detached
/// container for a portal surface.
#[derive(Clone, Debug)]
pub struct PortalMount {
    base_id: String,
    layer: PortalLayer,
}

impl PortalMount {
    /// Create a new portal mount for the provided `base_id` and layer.
    pub fn new(base_id: impl Into<String>, layer: PortalLayer) -> Self {
        Self {
            base_id: base_id.into(),
            layer,
        }
    }

    /// Convenience constructor for popover style surfaces.
    pub fn popover(base_id: impl Into<String>) -> Self {
        Self::new(base_id, PortalLayer::Popover)
    }

    /// Identifier applied to the hidden anchor element colocated with the
    /// trigger.
    pub fn anchor_id(&self) -> String {
        format!("{}-anchor", self.base_id)
    }

    /// Identifier applied to the detached container rendered after the host
    /// markup.
    pub fn container_id(&self) -> String {
        format!("{}-portal", self.base_id)
    }

    /// Returns the layer descriptor for downstream telemetry/tests.
    pub fn layer(&self) -> PortalLayer {
        self.layer
    }

    /// HTML snippet representing the hidden anchor element that sits next to
    /// the trigger. Frameworks attach positioning logic to this node at runtime.
    pub fn anchor_html(&self) -> String {
        let attrs = self.anchor_attributes();
        format!("<span {}></span>", attributes_to_html(&attrs))
    }

    /// Render the detached container wrapping the provided popover markup.
    pub fn wrap(&self, inner_html: impl AsRef<str>) -> PortalFragment {
        let attrs = self.container_attributes();
        let markup = format!(
            "<div {}>{}</div>",
            attributes_to_html(&attrs),
            inner_html.as_ref()
        );
        PortalFragment {
            container_id: self.container_id(),
            markup,
        }
    }

    /// Attribute list for the anchor placeholder.
    pub fn anchor_attributes(&self) -> Vec<(String, String)> {
        vec![
            ("id".into(), self.anchor_id()),
            ("data-portal-anchor".into(), self.base_id.clone()),
            ("data-portal-layer".into(), self.layer.as_str().to_string()),
            ("aria-hidden".into(), "true".into()),
        ]
    }

    /// Attribute list for the detached container.
    pub fn container_attributes(&self) -> Vec<(String, String)> {
        vec![
            ("id".into(), self.container_id()),
            ("data-portal-root".into(), self.base_id.clone()),
            ("data-portal-layer".into(), self.layer.as_str().to_string()),
            ("data-portal-anchor".into(), self.anchor_id()),
            ("role".into(), "presentation".into()),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn anchor_attributes_include_layer_metadata() {
        let mount = PortalMount::popover("orders");
        let attrs = mount.anchor_attributes();
        assert!(attrs
            .iter()
            .any(|(k, v)| k == "data-portal-layer" && v == "popover"));
        assert!(attrs
            .iter()
            .any(|(k, v)| k == "data-portal-anchor" && v == "orders"));
    }

    #[test]
    fn wrap_includes_container_metadata() {
        let mount = PortalMount::popover("orders");
        let fragment = mount.wrap("<ul></ul>");
        assert!(fragment.html().contains("data-portal-root=\"orders\""));
        assert!(fragment.html().contains("role=\"presentation\""));
        assert_eq!(fragment.container_id(), "orders-portal");
    }
}
