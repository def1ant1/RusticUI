# Navigation Drawer with Yew

The navigation drawer blueprint demonstrates how to combine `rustic_ui_headless`
`DrawerState` with `rustic_ui_material`'s responsive layout helpers inside a Yew
application. It showcases manual vs controlled behaviour, anchor switching and
accessible markup ready for analytics instrumentation.

## Highlights

- **Routing friendly** – integrates `yew-router` to keep drawer selections in
sync with navigation state while dispatching route changes when menu items are
activated.
- **Responsive anchor** – `DrawerLayoutOptions` transitions from a modal drawer
on mobile to a top anchored sheet on large screens.
- **Accessible** – markup automatically exposes `role="dialog"`,
`aria-modal`, labelled headings and automation friendly data attributes.
- **Automated setup** – the bootstrap script generates a runnable project with
Trunk, exhaustive comments, and hooks for axe-core verification.

## Usage

```bash
./examples/navigation-drawer-yew/scripts/bootstrap.sh
cd target/navigation-drawer-yew-demo
trunk serve --open
```

## Key excerpt

```rust
let mut layout = DrawerLayoutOptions::default();
layout.anchor.lg = Some(DrawerAnchor::Top);
layout.anchor.xl = Some(DrawerAnchor::Top);

let theme = Theme::default();
let mut state = DrawerState::new(
    true,
    unsafe { std::mem::transmute(1u8) },
    DrawerVariant::Modal,
    DrawerAnchor::Top,
);
state.sync_open(true);

let props = DrawerProps {
    state: &state,
    surface: state.surface_attributes().id("app-drawer").labelled_by("drawer-title"),
    backdrop: state.backdrop_attributes(),
    body: "<header id=\"drawer-title\">Navigation</header><nav role=\"navigation\">...</nav>",
    layout: &layout,
    theme: &theme,
    viewport: Some(theme.breakpoints.xl),
    on_toggle_event: Some("drawer-toggle"),
};

let render = drawer::yew::render(props);
```

The generated project adds routing callbacks, keyboard shortcuts and automated
axe-core audits so teams can drop the scaffold into continuous delivery
pipelines.
