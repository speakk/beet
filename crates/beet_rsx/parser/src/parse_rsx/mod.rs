mod rstml_rust_to_hash;
mod rstml_to_rsx_template;
mod rusty_tracker_builder;
pub use rusty_tracker_builder::*;
pub mod tokens_to_rstml;
pub use self::rstml_rust_to_hash::*;
#[allow(unused_imports)]
pub use self::rstml_to_rsx_template::*;
pub use self::tokens_to_rstml::*;
pub mod rsx_file_visitor;
#[allow(unused_imports)]
pub use self::rsx_file_visitor::*;
pub mod rstml_to_rsx;
#[allow(unused_imports)]
pub use self::rstml_to_rsx::*;
use proc_macro2::TokenStream;
use syn::visit_mut::VisitMut;
use syn::Expr;
use syn::File;


#[derive(Debug, Clone)]
pub struct RsxIdents {
	/// the identifier that contains the effect registration functions,
	/// ie `SignalsRsx`, it will be called like `#register_ident::register_block(#block)`
	pub effect: syn::Path,
	pub event: syn::Path,
	pub mac: syn::Ident,
}

impl Default for RsxIdents {
	fn default() -> Self {
		Self {
			effect: syn::parse_quote!(beet::rsx::signals_rsx::SignalsRsx),
			event: syn::parse_quote!(beet::prelude::EventRegistry),
			mac: syn::parse_quote!(rsx),
		}
	}
}


#[derive(Debug, Clone)]
pub struct ParseRsx {
	pub include_errors: bool,
	pub idents: RsxIdents,
}

impl Default for ParseRsx {
	fn default() -> Self {
		Self {
			include_errors: true,
			idents: Default::default(),
		}
	}
}

impl ParseRsx {
	/// header to add to the top of each rust file
	pub const SHEBANG: &'static str = "// 🥁 AUTOGENERATED BY BEET 🥁\n// 🥁 AUTOGENERATED BY BEET 🥁\n// 🥁 AUTOGENERATED BY BEET 🥁";

	// entrypoint for file (preprosessor) parsing
	pub fn parse_file(
		&mut self,
		file: &str,
	) -> syn::Result<(File, RsxFileVisitorOut)> {
		// errors in preprocessed files are not included, rstml
		// gets confused
		self.include_errors = false;
		let mut file = syn::parse_file(file)?;
		let mut visitor = RsxFileVisitor::new(self);
		visitor.visit_file_mut(&mut file);
		// Validate space
		file.shebang = Some(Self::SHEBANG.to_string());
		Ok((file, visitor.into()))
	}


	/// entrypoint for inline (macro) parsing.
	/// Called when visiting an rsx macro.
	/// Mutated in place for efficient file parsing
	pub fn parse_rsx(&mut self, _tokens: &mut TokenStream) -> RstmlToRsx {
		todo!("use rstml_to_rsx");
	}

	/// Check if a path matches the macro, by default only the last segment is checked
	pub fn path_matches(&self, path: &syn::Path) -> bool {
		path.segments
			.last()
			.map_or(false, |seg| seg.ident == self.idents.mac)
	}
}

pub fn macro_or_err(expr: &Expr) -> syn::Result<&syn::Macro> {
	if let Expr::Macro(mac) = expr {
		Ok(&mac.mac)
	} else {
		Err(syn::Error::new_spanned(expr, "expected macro"))
	}
}
