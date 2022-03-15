use std::any::Any;

use proc_macro::TokenStream;
use proc_macro2::{Delimiter, Group, Ident, TokenTree};
use quote::{quote_spanned, ToTokens};

mod fn_signature;

use fn_signature::*;

#[proc_macro_attribute]
pub fn dummy(_attr: TokenStream, item: TokenStream) -> TokenStream {
	item
}

#[proc_macro_attribute]
pub fn measure(attr: TokenStream, item: TokenStream) -> TokenStream {
	let attr = proc_macro2::TokenStream::from(attr);
	let item = proc_macro2::TokenStream::from(item);

	let span = proc_macro2::Span::call_site();

	let signature = FnSignature::from(item.clone()).rename("wrapped".to_owned());
	println!("{:#?}", signature);
	println!("{:#?}", signature.argument_names());
	println!(
		"{:#?}",
		signature.call(&["a".to_string(), "b".to_string(), "c".to_string()])
	);

	let wrapper = wrap_method(item);

	let output = quote_spanned! { span =>
		#wrapper
	};

	output.into()
}

fn wrap_method(stream: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
	let mut tokens = Vec::new();
	let mut iter = stream.clone().into_iter();
	while let Some(token) = iter.next() {
		tokens.push(token);
	}

	proc_macro2::TokenStream::from_iter(tokens.into_iter())
}

fn get_signature(stream: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
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

fn get_fn_body(stream: proc_macro2::TokenStream) -> TokenTree {
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
