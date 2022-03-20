use std::fmt::{Display, Formatter};

use proc_macro2::{Delimiter, Group, Ident, Punct, Spacing, Span, TokenStream, TokenTree};
use quote::ToTokens;
use syn::parse::{Parse, ParseStream};
use syn::{FnArg, ItemFn, Pat};

use crate::call_type::CallType;

pub struct Function {
	pub function: ItemFn,
	pub call_type: CallType,
}

impl Function {
	pub fn new(item: ItemFn) -> Function {
		Function {
			call_type: CallType::from(&item.sig),
			function: item,
		}
	}

	pub fn rename(&self, new_name: &str) -> Function {
		let mut renamed = self.function.clone();
		renamed.sig.ident = Ident::new(new_name, renamed.sig.ident.span());

		Function::new(renamed)
	}

	pub fn argument_names(&self) -> Vec<String> {
		self.function
			.sig
			.inputs
			.iter()
			.filter_map(|arg| if let FnArg::Typed(arg) = arg { Some(arg) } else { None })
			.filter_map(|pat_type| {
				if let Pat::Ident(pat_ident) = &*pat_type.pat {
					Some(pat_ident.ident.to_string())
				} else {
					None
				}
			})
			.collect()
	}

	pub fn call(&self, span: Span) -> TokenStream {
		let mut tokens = Vec::new();
		tokens.push(TokenTree::Ident(self.function.sig.ident.clone()));
		tokens.push(self.call_arguments(span));

		if self.function.sig.asyncness.is_some() {
			tokens.push(TokenTree::Punct(Punct::new('.', Spacing::Alone)));
			tokens.push(TokenTree::Ident(Ident::new("await", span)));
		}

		TokenStream::from_iter(tokens.into_iter())
	}

	fn call_arguments(&self, span: Span) -> TokenTree {
		let names = self.argument_names();

		let mut tokens = Vec::new();

		if let Some(ident) = self.call_type.ident(span) {
			tokens.push(TokenTree::Ident(ident));
			tokens.push(TokenTree::Punct(Punct::new(',', Spacing::Alone)));
		}

		let ident_tokens: Vec<_> = names
			.into_iter()
			.map(|arg| TokenTree::Ident(Ident::new(arg.as_str(), span)))
			.collect();
		ident_tokens.into_iter().for_each(|ident| {
			tokens.push(ident);
			tokens.push(TokenTree::Punct(Punct::new(',', Spacing::Alone)));
		});

		if tokens.len() > 0 {
			tokens.remove(tokens.len() - 1);
		}

		TokenTree::Group(Group::new(
			Delimiter::Parenthesis,
			TokenStream::from_iter(tokens.into_iter()),
		))
	}
}

impl Parse for Function {
	fn parse(input: ParseStream) -> syn::Result<Self> {
		Ok(Function::new(input.parse()?))
	}
}

impl Display for Function {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		self.function.attrs.iter().for_each(|attr| {
			let _ = writeln!(f, "{}", attr.into_token_stream());
		});
		writeln!(f, "{}", self.function.sig.ident)?;
		writeln!(f, "{}", self.call_type)
	}
}
