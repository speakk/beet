// use crate::prelude::*;
use beet::prelude::*;



pub fn get(state: DefaultAppState) -> RsxRoot {
	let val = 333;
	rsx! {
		<div>{state.app_name} hello   {val}</div>
		// <PageLayout title=state.app_name.into()>
		// 	<metasdjdsk
		// 		slot="head"
		// 		name="description"
		// 		content="A simple page layout component"
		// 	/>
		// 	{1 + 33293893}
		// 	parj is. ok fut pkjjkklrty m,
		// </PageLayout>
	}
}




//
//
