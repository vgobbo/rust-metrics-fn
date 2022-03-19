use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{quote_spanned, ToTokens};
use syn::parse_macro_input;

mod call_type;
mod function;

use function::*;

#[proc_macro_attribute]
pub fn dummy(_attr: TokenStream, item: TokenStream) -> TokenStream {
	item
}

#[proc_macro_attribute]
pub fn measure(_attr: TokenStream, item: TokenStream) -> TokenStream {
	let span = proc_macro2::Span::call_site();

	let original_fn = parse_macro_input!(item as Function);
	let wrapped_fn = original_fn.rename("wrapped");

	let attr_tokens = TokenStream2::from_iter(
		original_fn
			.function
			.attrs
			.iter()
			.map(|attr| attr.into_token_stream().into_iter())
			.flatten(),
	);
	let wrapped_call = wrapped_fn.call(span);
	let wrapped_sig_tokens = wrapped_fn.function.sig.into_token_stream();
	let wrapped_body_tokens = original_fn.function.block.into_token_stream();
	let wrapper_sig_tokens = original_fn.function.sig.into_token_stream();

	let output = quote_spanned! { span =>
		#attr_tokens
		#wrapper_sig_tokens {
			#wrapped_sig_tokens
			#wrapped_body_tokens

			let start__ = std::time::Instant::now();
			let output__ = #wrapped_call;
			let end__ = std::time::Instant::now();

			println!("Time {:?}", end__.duration_since(start__));

			return output__;
		}
	};

	output.into()
}
