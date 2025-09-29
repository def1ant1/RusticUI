#[cfg(all(feature = "csr", not(feature = "ssr")))]
fn main() {
    navigation_speed_dial_dioxus::hydrate();
}

#[cfg(all(feature = "ssr", not(feature = "csr")))]
fn main() {
    println!("{}", navigation_speed_dial_dioxus::render_document());
}

#[cfg(all(feature = "csr", feature = "ssr"))]
fn main() {
    println!("{}", navigation_speed_dial_dioxus::render_document());
}

#[cfg(all(not(feature = "csr"), not(feature = "ssr")))]
fn main() {}
