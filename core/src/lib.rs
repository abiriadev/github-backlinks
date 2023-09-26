use std::collections::HashMap;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct SearchResult {
	pub total_count: u32,
	pub incomplete_results: bool,
	pub items: Vec<SearchResultItem>,
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

pub struct Backlink {
	pub repository: String,
	pub reference_count: u32,
}

pub trait Fetcher {
	fn fetch_search_result(&mut self, repository_url: &str) -> SearchResult;
}

pub struct Resolver<T> {
	url: String,
	fetcher: T,
}

impl<T: Fetcher> Resolver<T> {
	pub fn new(url: String, fetcher: T) -> Self { Self { url, fetcher } }

	pub fn fetch_backlinks(&mut self) -> Vec<Backlink> {
		let list = self
			.fetcher
			.fetch_search_result(&self.url);

		let items = list.items;
		let mut sb = HashMap::new();

		for item in items {
			*sb.entry(item.repository.full_name)
				.or_default() += 1;
		}

		sb.into_iter()
			.map(|(k, v)| Backlink {
				repository: k,
				reference_count: v,
			})
			.collect()
	}
}
