use proc_macro::TokenStream;
use proc_macro2::{Delimiter, TokenStream as TokenStream2, TokenTree};
use quote::quote_spanned;

mod fn_signature;

use fn_signature::*;

#[proc_macro_attribute]
pub fn dummy(_attr: TokenStream, item: TokenStream) -> TokenStream {
	item
}

#[proc_macro_attribute]
pub fn measure(_attr: TokenStream, item: TokenStream) -> TokenStream {
	let item = TokenStream2::from(item);

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
	let output = wrap(wrapper_attributes, wrapper_signature, wrapped_signature, wrapped_body);

	println!("{}", output.to_string());

	output.into()
}

fn wrap(
	wrapper_attributes: TokenStream2,
	wrapper_signature: FnSignature,
	wrapped_signature: FnSignature,
	wrapped_body: TokenTree,
) -> TokenStream2 {
	let span = proc_macro2::Span::call_site();

	let wrapped_call = wrapped_signature.call(wrapped_signature.argument_names().as_slice());

	let output = quote_spanned! { span =>
		#wrapper_attributes
		#wrapper_signature {
			#wrapped_signature #wrapped_body

			let start__ = std::time::Instant::now();
			let output__ = #wrapped_call;
			let end__ = std::time::Instant::now();

			log::info!("Time {:?}", end__.duration_since(start__));

			return output__;
		}
	};

	output
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
