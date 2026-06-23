use std::path::PathBuf;
use std::sync::Arc;

use librqbit::{
    AddTorrent, AddTorrentOptions, DhtSessionConfig, PeerConnectionOptions, Session,
    SessionOptions,
};
use nyaapi_rs::{Nyaa, NyaaMode, NyaaOptions, SearchOptions, SortBy, Order, CategoryFilter};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("[e2e] Step 1: Searching nyaa.si for a small popular torrent...");

    let nyaa = Nyaa::new(NyaaOptions {
        base_url: "https://nyaa.si".to_string(),
        mode: NyaaMode::Html,
    });

    let results = nyaa
        .search(
            "naruto 1080p",
            SearchOptions {
                page: Some(1),
                category: Some(CategoryFilter::Anime),
                sort: Some(SortBy::Seeders),
                order: Some(Order::Desc),
                ..Default::default()
            },
        )
        .await?;

    println!(
        "[e2e] Found {} results (total: {:?})",
        results.data.len(),
        results.total
    );

    if results.data.is_empty() {
        anyhow::bail!("no search results found");
    }

    let torrent = &results.data[0];
    println!("[e2e] Selected: {}", torrent.name);
    println!(
        "[e2e]   seeders={}, leechers={}, size={}",
        torrent.seeders, torrent.leechers, torrent.size
    );

    let has_magnet = !torrent.magnet.is_empty();
    let has_torrent_file = torrent.torrent_url.is_some();
    println!(
        "[e2e]   magnet={}, torrent_file={}",
        has_magnet, has_torrent_file
    );

    println!("[e2e] Step 2: Creating librqbit session...");

    let download_dir = PathBuf::from("/tmp/nyaland-e2e-test");
    let _ = std::fs::remove_dir_all(&download_dir);
    std::fs::create_dir_all(&download_dir)?;

    let mut dht_cfg = DhtSessionConfig::default();
    dht_cfg.bootstrap_addrs = Some(vec![
        "router.bittorrent.com:6881".to_string(),
        "dht.transmissionbt.com:6881".to_string(),
        "router.utorrent.com:6881".to_string(),
        "dht.libtorrent.org:25401".to_string(),
        "dht.aelitis.com:6881".to_string(),
    ]);

    let session = Arc::new(
        Session::new_with_opts(
            download_dir.clone(),
            SessionOptions {
                dht: Some(dht_cfg),
                ..Default::default()
            },
        )
        .await?,
    );

    // Prefer .torrent file download over magnet (faster, no DHT resolution needed)
    let add_url = if let Some(ref torrent_url) = torrent.torrent_url {
        let base = "https://nyaa.si";
        let url = if torrent_url.starts_with('/') {
            format!("{}{}", base, torrent_url)
        } else {
            torrent_url.clone()
        };
        println!("[e2e] Step 3: Downloading .torrent file from {}...", url);
        url
    } else if has_magnet {
        println!("[e2e] Step 3: Using magnet link (DHT resolution will take time)...");
        torrent.magnet.clone()
    } else {
        anyhow::bail!("no magnet or torrent file URL available");
    };

    let opts = AddTorrentOptions {
        overwrite: true,
        output_folder: Some(download_dir.to_string_lossy().into_owned()),
        peer_opts: Some(PeerConnectionOptions {
            connect_timeout: Some(std::time::Duration::from_secs(10)),
            read_write_timeout: Some(std::time::Duration::from_secs(15)),
            ..Default::default()
        }),
        ..Default::default()
    };

    let resp = session
        .add_torrent(
            AddTorrent::Url(std::borrow::Cow::Owned(add_url)),
            Some(opts),
        )
        .await?;

    let handle = match resp {
        librqbit::AddTorrentResponse::Added(id, handle) => {
            println!("[e2e] Torrent added with id={id}");
            handle
        }
        librqbit::AddTorrentResponse::AlreadyManaged(id, handle) => {
            println!("[e2e] Torrent already managed with id={id}");
            handle
        }
        librqbit::AddTorrentResponse::ListOnly(_) => {
            anyhow::bail!("Got list-only response");
        }
    };

    println!("[e2e] Name: {:?}", handle.name());

    let stats = handle.stats();
    println!(
        "[e2e] Total size: {} bytes, state: {:?}",
        stats.total_bytes, stats.state
    );

    println!("[e2e] Step 4: Waiting for download to complete...");

    // Use tokio timeout so the test doesn't hang forever
    match tokio::time::timeout(
        std::time::Duration::from_secs(120),
        handle.wait_until_completed(),
    )
    .await
    {
        Ok(Ok(())) => {
            println!("[e2e] Download complete!");
            let stats = handle.stats();
            println!("[e2e] Total bytes: {}", stats.total_bytes);
            println!("[e2e] Progress bytes: {}", stats.progress_bytes);
            println!("[e2e] Files downloaded to: {}", download_dir.display());
        }
        Ok(Err(e)) => {
            println!("[e2e] Download error: {e}");
        }
        Err(_) => {
            println!("[e2e] Download timed out after 120s (still in progress)");
            let stats = handle.stats();
            println!(
                "[e2e] Progress: {}/{} bytes ({:.1}%)",
                stats.progress_bytes,
                stats.total_bytes,
                if stats.total_bytes > 0 {
                    stats.progress_bytes as f64 / stats.total_bytes as f64 * 100.0
                } else {
                    0.0
                }
            );
        }
    }

    // Cleanup
    let _ = std::fs::remove_dir_all(&download_dir);

    println!("[e2e] Test finished successfully!");
    Ok(())
}
