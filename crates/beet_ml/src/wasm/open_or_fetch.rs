use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::Cache;
use web_sys::Request;
use web_sys::RequestInit;
use web_sys::Response;

/// Attempt to retreive a model from the cache, if it doesn't exist, fetch it from the network
pub async fn open_or_fetch(url: &str) -> Result<js_sys::Uint8Array, JsValue> {
	let cache_name = "bert-candle-cache";

	let cache_promise = web_sys::window().unwrap().caches()?.open(cache_name);
	let cache = JsFuture::from(cache_promise).await?.dyn_into::<Cache>()?;

	let request = Request::new_with_str(url)?;

	let cached_response_promise = cache.match_all_with_request(&request);
	if let Ok(cached_response) = JsFuture::from(cached_response_promise)
		.await?
		.dyn_into::<Response>()
	{
		let data = JsFuture::from(cached_response.array_buffer()?).await?;
		return Ok(js_sys::Uint8Array::new(&data));
	}

	let mut opts = RequestInit::new();
	opts.cache(web_sys::RequestCache::ForceCache);

	let res_promise = web_sys::window()
		.unwrap()
		.fetch_with_request_and_init(&request, &opts);
	let res = JsFuture::from(res_promise).await?.dyn_into::<Response>()?;

	JsFuture::from(cache.put_with_request(&request, &res.clone()?)).await?;

	let buffer = JsFuture::from(res.array_buffer()?).await?;
	Ok(js_sys::Uint8Array::new(&buffer))
}
