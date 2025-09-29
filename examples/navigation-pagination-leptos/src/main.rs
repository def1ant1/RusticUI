#[cfg(all(feature = "csr", not(feature = "ssr")))]
fn main() {
    navigation_pagination_leptos::hydrate();
}

#[cfg(all(feature = "ssr", not(feature = "csr")))]
#[tokio::main]
async fn main() {
    println!("{}", navigation_pagination_leptos::render_document());
}

#[cfg(all(feature = "csr", feature = "ssr"))]
#[tokio::main]
async fn main() {
    println!("{}", navigation_pagination_leptos::render_document());
}

#[cfg(all(not(feature = "csr"), not(feature = "ssr")))]
fn main() {}
