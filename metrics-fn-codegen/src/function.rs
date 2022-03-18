use std::iter::Peekable;

use proc_macro2::token_stream::IntoIter;
use proc_macro2::{Delimiter, Group, Ident, Punct, Spacing, TokenStream as TokenStream2, TokenTree};
use quote::{ToTokens, TokenStreamExt};

#[derive(Debug)]
pub struct Function {
	pub is_pub: bool,
	pub is_async: bool,
	pub name: String,
	pub attributes: Vec<TokenTree>,
	pub arguments: Group,
	pub body: Group,
}

pub enum Wants {
	VariableName,
	Colon,
	Comma,
}

impl Function {
	pub fn rename(&self, name: String) -> Function {
		Function {
			is_pub: self.is_pub,
			is_async: self.is_async,
			name,
			attributes: self.attributes.clone(),
			arguments: self.arguments.clone(),
			body: self.body.clone(),
		}
	}

	pub fn call(&self, args: &[String]) -> TokenStream2 {
		self.call_build(self.is_async, args)
	}

	fn call_build(&self, do_await: bool, args: &[String]) -> TokenStream2 {
		let span = proc_macro2::Span::call_site();

		let mut tokens = Vec::new();

		tokens.push(TokenTree::Ident(Ident::new(self.name.as_str(), span)));
		tokens.push(self.call_arguments(args));

		if do_await {
			if !self.is_async {
				panic!("Not async.");
			}
			tokens.push(TokenTree::Ident(Ident::new("await", span)));
		}

		tokens.push(TokenTree::Punct(Punct::new(';', Spacing::Alone)));

		TokenStream2::from_iter(tokens.into_iter())
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
			TokenStream2::from_iter(tokens.into_iter()),
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

	fn parse_attributes(iter: &mut Peekable<IntoIter>) -> Vec<TokenTree> {
		let mut tokens = Vec::new();

		loop {
			if let Some(token) = iter.peek() {
				if let TokenTree::Ident(ident) = &token {
					if ident == "pub" || ident == "async" || ident == "fn" {
						break;
					}
				}
			}

			tokens.push(iter.next().unwrap());
		}

		tokens
	}
}

impl Default for Function {
	fn default() -> Self {
		Function {
			is_pub: false,
			is_async: false,
			name: "".to_owned(),
			attributes: Vec::new(),
			arguments: Group::new(Delimiter::Parenthesis, TokenStream2::new()),
			body: Group::new(Delimiter::Brace, TokenStream2::new()),
		}
	}
}

impl From<TokenStream2> for Function {
	fn from(stream: TokenStream2) -> Self {
		let mut function = Function::default();

		let mut iter = stream.into_iter().peekable();
		function.attributes = Function::parse_attributes(&mut iter);

		while let Some(token) = iter.next() {
			match token {
				TokenTree::Ident(ident) => {
					if ident == "pub" {
						function.is_pub = true;
					} else if ident == "async" {
						function.is_async = true;
					} else {
						function.name = ident.to_string();
					}
				},
				TokenTree::Group(group) => {
					if group.delimiter() == Delimiter::Parenthesis {
						function.arguments = group;
					} else if group.delimiter() == Delimiter::Brace {
						function.body = group;
						// no more groups should be available.
						return function;
					}
				},
				_ => (),
			}
		}

		panic!("Parameter list not found.");
	}
}

impl ToTokens for Function {
	fn to_tokens(&self, tokens: &mut TokenStream2) {
		let span = proc_macro2::Span::call_site();
		if self.is_pub {
			tokens.append(TokenTree::from(Ident::new("pub", span)));
		}
		if self.is_async {
			tokens.append(TokenTree::from(Ident::new("async", span)));
		}
		tokens.append(TokenTree::from(Ident::new("fn", span)));
		tokens.append(TokenTree::from(Ident::new(self.name.as_str(), span)));
		self.arguments.to_tokens(tokens);
	}
}