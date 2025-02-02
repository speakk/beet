use beet_router::prelude::*;


#[tokio::main]
async fn main() {
	let mut router = DefaultFileRouter::default();
	// usually its directly in src but test_site is a subdirectory
	router.src_dir = "src/test_site".into();
	beet_router::test_site::test_site_router::collect_file_routes(&mut router);
	router.routes_to_html_files().await.unwrap();
}
