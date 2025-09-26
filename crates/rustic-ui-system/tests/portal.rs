use mui_system::portal::{PortalLayer, PortalMount};

#[test]
fn popover_anchor_markup_is_deterministic() {
    let mount = PortalMount::popover("orders-popover");
    let html = mount.anchor_html();
    assert!(html.contains("data-portal-layer=\"popover\""));
    assert!(html.contains("data-portal-anchor=\"orders-popover\""));
    assert!(html.contains("orders-popover-anchor"));
}

#[test]
fn popover_container_wraps_inner_markup() {
    let mount = PortalMount::new("orders-popover", PortalLayer::Popover);
    let fragment = mount.wrap("<ul><li>First</li></ul>");
    let html = fragment.into_html();
    assert!(html.starts_with("<div"));
    assert!(html.contains("data-portal-root=\"orders-popover\""));
    assert!(html.contains("orders-popover-anchor"));
    assert!(html.ends_with("</div>"));
}
