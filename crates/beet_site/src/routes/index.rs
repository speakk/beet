use crate::prelude::*;
use beet::prelude::*;



pub fn get(state: DefaultAppState) -> RsxRoot {
	let val = 4333;
	rsx! {
		<div>{state.app_name} hello nuymber  {val}</div>
		// <PageLayout title=state.app_name.into()>
		// 	<metasdjdsk
		// 		slot="head"
		// 		name="description"
		// 		content="A simple page layout component"
		// 	/>
		// 	{val + val}
		// 	hello world
		// </PageLayout>
	}
}




//
//
