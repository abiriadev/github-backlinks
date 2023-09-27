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

#[derive(Debug, PartialEq, Eq)]
pub struct Backlink {
	pub repository: String,
	pub reference_count: u32,
}

pub struct Context {
	pub github_token: String,
}

pub trait Fetcher {
	type Error;

	fn set_context(&mut self, context: Context);

	fn fetch_search_result(
		&mut self,
		repository_url: &str,
	) -> Result<SearchResult, Self::Error>;
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
	RequestError,
}

pub struct Resolver<T> {
	url: String,
	fetcher: T,
}

impl<T> Resolver<T>
where
	T: Fetcher,
	<T as Fetcher>::Error: Into<Error>,
{
	pub fn new(url: String, fetcher: T) -> Self { Self { url, fetcher } }

	pub fn fetch_backlinks(&mut self) -> Result<Vec<Backlink>, Error> {
		let list = self
			.fetcher
			.fetch_search_result(&self.url)
			.map_err(|e| e.into())?;

		let items = list.items;
		let mut sb = HashMap::new();

		for item in items {
			*sb.entry(item.repository.full_name)
				.or_default() += 1;
		}

		Ok(sb
			.into_iter()
			.map(|(k, v)| Backlink {
				repository: k,
				reference_count: v,
			})
			.collect())
	}
}
