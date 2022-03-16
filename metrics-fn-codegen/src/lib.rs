use std::any::Any;

use proc_macro::TokenStream;
use proc_macro2::{Delimiter, Group, Ident, TokenStream as TokenStream2, TokenTree};
use quote::{quote_spanned, ToTokens, TokenStreamExt};

mod fn_signature;

use fn_signature::*;

#[proc_macro_attribute]
pub fn dummy(_attr: TokenStream, item: TokenStream) -> TokenStream {
	item
}

#[proc_macro_attribute]
pub fn measure(attr: TokenStream, item: TokenStream) -> TokenStream {
	let attr = TokenStream2::from(attr);
	let item = TokenStream2::from(item);

	let span = proc_macro2::Span::call_site();

	// parse the fn.
	let fn_attributes = get_fn_attributes(item.clone());
	let fn_signature = FnSignature::from(item.clone());
	let fn_body = get_fn_body(item.clone());

	// map the fn to wrapper and wrapped variables
	let wrapper_attributes = fn_attributes;
	let wrapper_signature = fn_signature;
	let wrapped_signature = wrapper_signature.rename("wrapped".to_owned());
	let wrapped_body = fn_body;

	// build the wrapped fn.
	let wrapper = wrap(wrapper_attributes, wrapper_signature, wrapped_signature, wrapped_body);

	let output = quote_spanned! { span =>
		#wrapper
	};

	output.into()
}

fn wrap(
	wrapper_attributes: TokenStream2,
	wrapper_signature: FnSignature,
	wrapped_signature: FnSignature,
	wrapped_body: TokenTree,
) -> TokenStream2 {
	todo!()
}

fn get_signature(stream: TokenStream2) -> TokenStream2 {
	let mut iter = stream.into_iter();
	while let Some(token) = iter.next() {
		if let TokenTree::Group(group) = token {
			if group.delimiter() == Delimiter::Parenthesis {
				return group.into_token_stream();
			}
		}
	}

	panic!("Parameter list not found.");
}

fn get_fn_attributes(stream: TokenStream2) -> TokenStream2 {
	let mut tokens = Vec::new();

	let mut iter = stream.into_iter();
	while let Some(token) = iter.next() {
		if let TokenTree::Ident(ident) = &token {
			if ident == "pub" || ident == "async" || ident == "fn" {
				break;
			}
		}
		tokens.push(token);
	}

	TokenStream2::from_iter(tokens.into_iter())
}

fn get_fn_body(stream: TokenStream2) -> TokenTree {
	let mut iter = stream.into_iter();

	// skip until the 'fn' keyword.
	while let Some(token) = iter.next() {
		if let TokenTree::Ident(ident) = token {
			if ident == "fn" {
				break;
			}
		}
	}

	// skip until a group with a brace delimiter.
	while let Some(token) = iter.next() {
		if let TokenTree::Group(group) = &token {
			if group.delimiter() == Delimiter::Brace {
				return token;
			}
		}
	}

	panic!("Function body not found.");
}
