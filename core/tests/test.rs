use std::env::var;

use github_backlinks_core::{Context, Fetcher, Resolver, SearchResult};
use ureq::Agent;

struct UreqFetcherError(ureq::Error);

impl From<UreqFetcherError> for github_backlinks_core::Error {
	fn from(value: UreqFetcherError) -> Self { Self::RequestError }
}

struct UreqFetcher {
	token: Option<String>,
	agent: Agent,
}

impl UreqFetcher {
	fn new() -> Self {
		Self {
			token: None,
			agent: Agent::new(),
		}
	}
}

impl Fetcher for UreqFetcher {
	type Error = UreqFetcherError;

	fn set_context(&mut self, context: Context) {
		self.token.replace(context.github_token);
	}

	fn fetch_search_result(
		&mut self,
		repository_url: &str,
	) -> Result<SearchResult, Self::Error> {
		let a = self
			.agent
			.get("https://api.github.com/search/code")
			.set(
				"Authorization",
				&format!(
					"Bearer {}",
					self.token
						.as_ref()
						.map(|a| a.as_str())
						.unwrap()
				),
			)
			.query("q", repository_url)
			.call()
			.map_err(UreqFetcherError)?;

		Ok(a.into_json().unwrap())
	}
}

#[test]
fn a() {
	let url = "https://github.com/abiriadev/abiriadev";
	let mut fetcher = UreqFetcher::new();

	fetcher.set_context(Context {
		github_token: var("GH_TOKEN").unwrap(),
	});

	let res = Resolver::new(url.to_owned(), fetcher).fetch_backlinks();

	assert_eq!(res, Ok(vec![]));
}
