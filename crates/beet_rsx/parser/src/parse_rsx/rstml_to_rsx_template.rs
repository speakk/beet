use crate::prelude::*;
use proc_macro2::Literal;
use proc_macro2::TokenStream;
use quote::quote;
use quote::ToTokens;
use rstml::atoms::OpenTag;
use rstml::node::CustomNode;
use rstml::node::Node;
use rstml::node::NodeAttribute;
use rstml::node::NodeComment;
use rstml::node::NodeElement;
use rstml::node::NodeText;
use syn::spanned::Spanned;

/// Convert rstml nodes to a ron file.
/// Rust block token streams will be hashed by [Span::start]
#[derive(Debug, Default)]
pub struct RstmlToRsxTemplate {
	rusty_tracker: RustyTrackerBuilder,
}


impl RstmlToRsxTemplate {
	/// for use with rsx_template! macro
	pub fn from_macro(&mut self, tokens: TokenStream) -> TokenStream {
		let str_tokens = self
			.map_tokens(tokens, "unknown")
			.to_string()
			.to_token_stream();
		quote! {
			{
				let mut root = RsxTemplateRoot::from_ron(#str_tokens).unwrap();
				root.location.file = std::file!().to_string();
				root
			}
		}
	}
	pub fn map_tokens(
		&mut self,
		tokens: TokenStream,
		file: &str,
	) -> TokenStream {
		let span = tokens.span();
		let (nodes, _rstml_errors) = tokens_to_rstml(tokens);
		let node = self.map_nodes(nodes);
		let line = Literal::usize_unsuffixed(span.start().line);
		let col = Literal::usize_unsuffixed(span.start().column);

		quote! {
			RsxTemplateRoot (
				node: #node,
				location: RsxLocation(
					file: #file,
					line: #line,
					col: #col
				)
			)
		}
	}

	/// wraps in fragment if multiple nodes
	pub fn map_nodes<C: CustomNode>(
		&mut self,
		nodes: Vec<Node<C>>,
	) -> TokenStream {
		let mut nodes = nodes
			.into_iter()
			.map(|node| self.map_node(node))
			.collect::<Vec<_>>();
		if nodes.len() == 1 {
			nodes.pop().unwrap().to_token_stream()
		} else {
			quote! {Fragment([#(#nodes),*])}
		}
	}

	/// returns an RsxTemplateNode
	pub fn map_node<C: CustomNode>(&mut self, node: Node<C>) -> TokenStream {
		match node {
			Node::Doctype(_) => quote! {Doctype},
			Node::Comment(NodeComment { value, .. }) => {
				quote! {Comment(#value)}
			}
			Node::Fragment(node_fragment) => {
				let children = node_fragment
					.children
					.into_iter()
					.map(|n| self.map_node(n));
				quote! {
					Fragment([#(#children),*])
				}
			}
			Node::Block(block) => {
				let tracker = self.rusty_tracker.next_tracker_ron(&block);
				quote! {RustBlock(#tracker)}
			}
			Node::Text(NodeText { value }) => {
				quote! {Text(#value)}
			}
			Node::RawText(raw) => {
				let val = raw.to_string_best();
				quote! {Text(#val)}
			}
			Node::Element(NodeElement {
				open_tag,
				children,
				close_tag,
			}) => {
				let tag = open_tag.name.to_string();
				let self_closing = close_tag.is_none();
				if tag.starts_with(|c: char| c.is_uppercase()) {
					self.map_component(tag, open_tag, children)
				} else {
					let attributes = open_tag
						.attributes
						.into_iter()
						.map(|a| self.map_attribute(a))
						.collect::<Vec<_>>();
					let children = self.map_nodes(children);
					quote! {
							Element (
								tag: #tag,
								self_closing: #self_closing,
								attributes: [#(#attributes),*],
								children: #children
							)
					}
				}
			}
			Node::Custom(_) => unimplemented!(),
		}
	}

	fn map_attribute(&mut self, attr: NodeAttribute) -> TokenStream {
		match attr {
			NodeAttribute::Block(block) => {
				let tracker = self.rusty_tracker.next_tracker_ron(&block);
				quote! {Block(#tracker)}
			}
			NodeAttribute::Attribute(attr) => {
				let key = attr.key.to_string();
				match attr.value() {
					None => {
						quote! {Key ( key: #key )}
					}
					Some(syn::Expr::Lit(expr_lit)) => {
						let value = lit_to_string(&expr_lit.lit);
						quote! {
								KeyValue (
								key: #key,
								value: #value
								)
						}
					}
					Some(value) => {
						let tracker =
							self.rusty_tracker.next_tracker_ron(&value);
						// we dont need to handle events for serialization,
						// thats an rstml_to_rsx concern so having the tracker is enough
						quote! {
							BlockValue (
								key: #key,
								tracker: #tracker
							)
						}
					}
				}
			}
		}
	}
	fn map_component<C: CustomNode>(
		&mut self,
		tag: String,
		open_tag: OpenTag,
		children: Vec<Node<C>>,
	) -> TokenStream {
		let tracker = self.rusty_tracker.next_tracker_ron(&open_tag);
		// components disregard all the context and rely on the tracker
		// we rely on the hydrated node to provide the attributes and children
		let slot_children = self.map_nodes(children);

		quote! {
			Component (
				tracker: #tracker,
				tag: #tag,
				slot_children: #slot_children,
			)
		}
	}
}
pub fn lit_to_string(lit: &syn::Lit) -> String {
	match lit {
		syn::Lit::Int(lit_int) => lit_int.base10_digits().to_string(),
		syn::Lit::Float(lit_float) => lit_float.base10_digits().to_string(),
		syn::Lit::Bool(lit_bool) => lit_bool.value.to_string(),
		syn::Lit::Str(lit_str) => lit_str.value(),
		syn::Lit::ByteStr(lit_byte_str) => {
			String::from_utf8_lossy(&lit_byte_str.value()).into_owned()
		}
		syn::Lit::Byte(lit_byte) => lit_byte.value().to_string(),
		syn::Lit::Char(lit_char) => lit_char.value().to_string(),
		syn::Lit::Verbatim(lit_verbatim) => lit_verbatim.to_string(),
		syn::Lit::CStr(_) => unimplemented!(),
		_ => unimplemented!(),
	}
}
