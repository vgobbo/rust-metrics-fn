use std::any::Any;

use proc_macro::TokenStream;
use proc_macro2::{Delimiter, Group, Ident, Punct, Spacing, TokenTree};
use quote::{quote_spanned, ToTokens};

#[derive(Debug)]
pub struct FnSignature {
	pub is_pub: bool,
	pub is_async: bool,
	pub name: String,
	pub arguments: Group,
}

pub enum Wants {
	VariableName,
	Colon,
	Comma,
}

impl FnSignature {
	pub fn call(&self, args: &[String]) -> proc_macro2::TokenStream {
		let span = proc_macro2::Span::call_site();

		let mut tokens = Vec::new();

		tokens.push(TokenTree::Ident(Ident::new(self.name.as_str(), span)));
		tokens.push(self.call_arguments(args));
		tokens.push(TokenTree::Punct(Punct::new(';', Spacing::Alone)));

		proc_macro2::TokenStream::from_iter(tokens.into_iter())
	}

	fn call_arguments(&self, args: &[String]) -> TokenTree {
		let span = proc_macro2::Span::call_site();

		let mut tokens = Vec::new();

		if args.len() > 0 {
			let ident_tokens: Vec<_> = args
				.into_iter()
				.map(|arg| TokenTree::Ident(Ident::new(arg.as_str(), span)))
				.collect();
			ident_tokens.into_iter().for_each(|ident| {
				tokens.push(ident);
				tokens.push(TokenTree::Punct(Punct::new(',', Spacing::Alone)));
			});
			tokens.remove(tokens.len() - 1);
		}

		TokenTree::Group(Group::new(
			Delimiter::Parenthesis,
			proc_macro2::TokenStream::from_iter(tokens.into_iter()),
		))
	}

	pub fn argument_names(&self) -> Vec<String> {
		let mut names = Vec::new();
		let mut wants = Wants::VariableName;
		let mut iter = self.arguments.stream().into_iter();
		while let Some(token) = iter.next() {
			match wants {
				Wants::VariableName => {
					if let Some(name) = Self::identifier(&token) {
						names.push(name);
						wants = Wants::Colon;
					}
				},
				Wants::Colon => {
					if Self::is_colon(&token) {
						wants = Wants::Comma;
					}
				},
				Wants::Comma => {
					if Self::is_comma(&token) {
						wants = Wants::VariableName;
					}
				},
			}
			println!("{:#?}", token);
		}

		names
	}

	fn keyword(token: &TokenTree) -> Option<String> {
		if let TokenTree::Ident(ident) = token {
			// we are only interested in the `mut` keyword.
			if ident == "mut" {
				return Some("mut".to_owned());
			}
		}
		return None;
	}

	fn identifier(token: &TokenTree) -> Option<String> {
		if let Some(_) = Self::keyword(token) {
			None
		} else if let TokenTree::Ident(ident) = token {
			Some(ident.to_string())
		} else {
			None
		}
	}

	fn is_colon(token: &TokenTree) -> bool {
		if let TokenTree::Punct(punct) = token {
			punct.as_char() == ':'
		} else {
			false
		}
	}

	fn is_comma(token: &TokenTree) -> bool {
		if let TokenTree::Punct(punct) = token {
			punct.as_char() == ','
		} else {
			false
		}
	}
}

impl Default for FnSignature {
	fn default() -> Self {
		FnSignature {
			is_pub: false,
			is_async: false,
			name: "".to_owned(),
			arguments: Group::new(Delimiter::Parenthesis, proc_macro2::TokenStream::new()),
		}
	}
}

impl From<proc_macro2::TokenStream> for FnSignature {
	fn from(stream: proc_macro2::TokenStream) -> Self {
		let mut signature = FnSignature::default();

		let mut iter = stream.into_iter();
		while let Some(token) = iter.next() {
			match token {
				TokenTree::Ident(ident) => {
					if ident == "pub" {
						signature.is_pub = true;
					} else if ident == "async" {
						signature.is_async = true;
					} else {
						signature.name = ident.to_string();
					}
				},
				TokenTree::Group(group) => {
					if group.delimiter() == Delimiter::Parenthesis {
						signature.arguments = group;
						// no more groups should be available.
						return signature;
					}
				},
				_ => (),
			}
		}

		panic!("Parameter list not found.");
	}
}