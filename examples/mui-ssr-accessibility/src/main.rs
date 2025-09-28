use mui_ssr_accessibility::render_document;

#[tokio::main]
async fn main() {
    // Compose the full SSR document using the shared AppShell and automation
    // builders.  The resulting HTML is byte-for-byte compatible with the CSR
    // hydration pass so QA suites can diff the markup with confidence.
    let rendered = render_document().await;
    println!("{rendered}");
}
