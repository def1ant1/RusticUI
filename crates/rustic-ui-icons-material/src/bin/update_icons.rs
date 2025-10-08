// update_icons.rs - managed maintenance utility for refreshing the upstream
// Material Design SVG archive. The binary now leans heavily on shared library
// code so unit tests can exercise the exact same execution path that CI relies
// on.

use anyhow::Result;
use clap::Parser;
use rustic_ui_icons_material::icon_update::{
    run_update, HttpFetcher, UpdateOptions, UpdateOutcome, UpdateReuseReason, DEFAULT_ZIP_URL,
};

/// Command line interface for the icon updater.
///
/// The flags are intentionally automation-friendly: everything is surfaced via
/// stdout in machine-readable key/value pairs so CI systems can parse the
/// output without brittle scraping.
#[derive(Parser, Debug)]
#[command(author, version, about = "Download and refresh Material icon assets", long_about = None)]
struct Cli {
    /// Override the upstream archive location. Useful for mirroring or testing
    /// against pre-release icon drops.
    #[arg(long, value_name = "URL", default_value = DEFAULT_ZIP_URL)]
    source_url: String,
    /// Ignore cached metadata and force a full refresh of icon assets.
    #[arg(long)]
    force_refresh: bool,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let mut options = UpdateOptions::default();
    options.source_url = cli.source_url.clone();
    options.force_refresh = cli.force_refresh;

    if cli.force_refresh {
        println!("ICON_UPDATE_MODE=force-refresh");
    }

    println!(
        "ICON_UPDATE_REQUEST url={} cache={}",
        options.source_url,
        options.cache_dir.display()
    );

    let fetcher = HttpFetcher::default();
    match run_update(&fetcher, &options)? {
        UpdateOutcome::Reused {
            reason: UpdateReuseReason::HttpNotModified,
        } => {
            println!("ICON_UPDATE_STATUS=reused-http-not-modified");
            println!(
                "ICON_UPDATE_MESSAGE=Upstream archive reported 304 Not Modified; local assets left untouched"
            );
        }
        UpdateOutcome::Reused {
            reason: UpdateReuseReason::ChecksumMatch,
        } => {
            println!("ICON_UPDATE_STATUS=reused-checksum");
            println!(
                "ICON_UPDATE_MESSAGE=Downloaded archive matches existing assets; skipping rewrite"
            );
        }
        UpdateOutcome::Updated { installed } => {
            println!("ICON_UPDATE_STATUS=updated");
            println!(
                "ICON_UPDATE_MESSAGE=Refreshed {installed} icons in {}",
                options.icon_dir.display()
            );
        }
    }

    Ok(())
}
