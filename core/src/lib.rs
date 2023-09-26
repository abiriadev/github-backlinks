use serde::Deserialize;

#[derive(Deserialize)]
pub struct SearchResult {
	pub total_count: u32,
	pub incomplete_results: bool,
	pub items: SearchResultItem,
}

#[derive(Deserialize)]
pub struct SearchResultItem {
	pub repository: SearchResultItemRepository,
	pub html_url: String,
	pub score: u32,
}

#[derive(Deserialize)]
pub struct SearchResultItemRepository {
	pub full_name: String,
}
