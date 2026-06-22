use nyaapi_rs::{Nyaa, NyaaMode, NyaaOptions, SearchOptions, CategoryFilter, TrustedFilter, SortBy, Order, SearchResult, TorrentDetail};

use crate::types::SearchParams;

pub struct NyaaClient {
    client: Nyaa,
}

impl NyaaClient {
    pub fn new(base_url: &str) -> Self {
        let client = Nyaa::new(NyaaOptions {
            base_url: base_url.to_string(),
            mode: NyaaMode::Html,
        });
        Self { client }
    }

    pub async fn search(&self, params: &SearchParams) -> Result<SearchResult, String> {
        let category = params.category.as_deref().map(parse_category).unwrap_or(CategoryFilter::All);
        let filter = params.filter.as_deref().map(parse_filter).unwrap_or(TrustedFilter::NoFilter);
        let sort = params.sort.as_deref().map(parse_sort).unwrap_or(SortBy::Date);
        let order = params.order.as_deref().map(parse_order).unwrap_or(Order::Desc);

        let options = SearchOptions {
            page: params.page,
            category: Some(category),
            filter: Some(filter),
            sort: Some(sort),
            order: Some(order),
        };

        self.client.search(&params.query, options).await.map_err(|e| e.to_string())
    }

    pub async fn view(&self, id: u64) -> Result<Option<TorrentDetail>, String> {
        self.client.view(id).await.map_err(|e| e.to_string())
    }
}

fn parse_category(s: &str) -> CategoryFilter {
    match s.to_lowercase().as_str() {
        "anime" => CategoryFilter::Anime,
        "audio" => CategoryFilter::Audio,
        "literature" => CategoryFilter::Literature,
        "live-action" | "liveaction" => CategoryFilter::LiveAction,
        "pictures" => CategoryFilter::Pictures,
        "software" => CategoryFilter::Software,
        "games" => CategoryFilter::Games,
        _ => CategoryFilter::All,
    }
}

fn parse_filter(s: &str) -> TrustedFilter {
    match s.to_lowercase().as_str() {
        "trusted" | "trustedonly" => TrustedFilter::TrustedOnly,
        "noremakes" => TrustedFilter::NoRemakes,
        _ => TrustedFilter::NoFilter,
    }
}

fn parse_sort(s: &str) -> SortBy {
    match s.to_lowercase().as_str() {
        "seeders" => SortBy::Seeders,
        "leechers" => SortBy::Leechers,
        "downloads" => SortBy::Downloads,
        "size" => SortBy::Size,
        "comments" => SortBy::Comments,
        _ => SortBy::Date,
    }
}

fn parse_order(s: &str) -> Order {
    match s.to_lowercase().as_str() {
        "asc" => Order::Asc,
        _ => Order::Desc,
    }
}
