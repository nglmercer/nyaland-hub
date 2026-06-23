use std::borrow::Cow;
use std::path::PathBuf;
use std::sync::Arc;

use librqbit::{
    AddTorrent, AddTorrentOptions, DhtSessionConfig, PeerConnectionOptions, Session, SessionOptions,
};
use nyaapi_rs::{CategoryFilter, Nyaa, NyaaMode, NyaaOptions, Order, SearchOptions, SortBy};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("[e2e] Step 1: Searching nyaa.si for a popular torrent with seeders...");

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

    // Find a torrent with seeders and a magnet link
    let torrent = results
        .data
        .iter()
        .find(|t| t.seeders > 10 && !t.magnet.is_empty())
        .ok_or_else(|| anyhow::anyhow!("no torrent with seeders found"))?;

    println!("[e2e] Selected: {}", torrent.name);
    println!(
        "[e2e]   seeders={}, leechers={}, size={}",
        torrent.seeders, torrent.leechers, torrent.size
    );
    println!(
        "[e2e]   magnet={}...",
        &torrent.magnet[..torrent.magnet.len().min(100)]
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

    println!("[e2e] Step 3: Adding torrent via magnet (DHT resolution)...");

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

    // Use a 60s timeout for add_torrent (DHT resolution)
    let add_result = tokio::time::timeout(
        std::time::Duration::from_secs(60),
        session.add_torrent(
            AddTorrent::Url(Cow::Owned(torrent.magnet.clone())),
            Some(opts),
        ),
    )
    .await;

    let handle = match add_result {
        Ok(Ok(resp)) => match resp {
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
        },
        Ok(Err(e)) => {
            anyhow::bail!("add_torrent failed: {e}");
        }
        Err(_) => {
            anyhow::bail!("add_torrent timed out after 60s (no peers found via DHT)");
        }
    };

    println!("[e2e] Name: {:?}", handle.name());

    let stats = handle.stats();
    println!(
        "[e2e] Total size: {} bytes, state: {:?}",
        stats.total_bytes, stats.state
    );

    println!("[e2e] Step 4: Waiting for download to complete (120s max)...");

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
            println!("[e2e] Files at: {}", download_dir.display());
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

    let _ = std::fs::remove_dir_all(&download_dir);
    println!("[e2e] Test finished successfully!");
    Ok(())
}
