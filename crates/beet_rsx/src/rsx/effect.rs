use crate::prelude::*;
use anyhow::Result;

pub type RegisterEffect = Box<dyn FnOnce(&RsxContext) -> Result<()>>;


pub struct Effect {
	/// the function for registering the effect with
	/// its reactive framework
	pub(super) register: RegisterEffect,
	/// the location of the effect in the rsx macro,
	/// this may or may not be populated depending
	/// on the settings of the parser
	pub tracker: Option<RustyTracker>,
}

impl Effect {
	pub fn new(
		register: RegisterEffect,
		tracker: Option<RustyTracker>,
	) -> Self {
		Self {
			register,
			tracker,
		}
	}

	/// call the FnOnce register func and replace it
	/// with an empty one.
	pub fn take(&mut self) -> Self {
		let register =
			std::mem::replace(&mut self.register, Box::new(|_| Ok(())));
		let tracker = self.tracker.take();
		Self {
			register,
			tracker,
		}
	}

	pub fn register(self, cx: &RsxContext) -> Result<()> { (self.register)(cx) }
}

impl std::fmt::Debug for Effect {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("Effect")
			.field("tracker", &self.tracker)
			.field("register", &std::any::type_name_of_val(&self.register))
			.finish()
	}
}
