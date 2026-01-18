//! # Function Metrics Codegen
//!
//! **ATTENTION: Don't use this directly!**
//!
//! This project is divided in a _build time_ module (this module), and a _runtime_ module (`metrics-fn`).
//!
//! The runtime module includes this automatically, so you should pretty much ignore this.
//!
//! See [metrics-fn](https://crates.io/crates/metrics-fn/) for more details.

use proc_macro::TokenStream;
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::{ToTokens, quote_spanned};
use syn::parse_macro_input;

mod call_type;
mod function;
mod return_type_classification;

use function::*;

use crate::return_type_classification::ReturnTypeClassification;

#[proc_macro_attribute]
pub fn dummy(_attr: TokenStream, item: TokenStream) -> TokenStream {
	item
}

#[proc_macro_attribute]
pub fn measure(attrs: TokenStream, item: TokenStream) -> TokenStream {
	let span = proc_macro2::Span::call_site();

	if !attrs.is_empty() {
		return syn::Error::new(span, "#[measure] does not take arguments.")
			.to_compile_error()
			.into();
	}

	let original_fn = parse_macro_input!(item as Function);
	let wrapped_fn =
		original_fn.rename(format!("{}__{}", original_fn.function.sig.ident.clone().to_string(), "wrapped").as_str());

	let wrapped_attrs_tokens = original_fn.attributes_tokens();
	let wrapped_call_tokens = wrapped_fn.call(span);
	let wrapped_call_fn_name = original_fn.function.sig.ident.clone().to_string();
	let wrapped_sig_tokens = wrapped_fn.function.sig.into_token_stream();
	let wrapped_body_tokens = original_fn.function.block.clone().into_token_stream();
	let wrapper_sig_tokens = original_fn.signature_full();
	let record_call_tokens = build_record_call(span, original_fn, wrapped_call_fn_name);

	let output = quote_spanned! { span =>
		#wrapped_attrs_tokens
		#wrapper_sig_tokens {

			let start__ = std::time::Instant::now();
			let output__ = #wrapped_call_tokens;
			let end__ = std::time::Instant::now();

			let module_name = module_path!();
			#record_call_tokens

			return output__;
		}

		#[allow(non_snake_case)]
		#wrapped_sig_tokens
		#wrapped_body_tokens
	};

	output.into()
}

fn build_record_call(span: Span, original_fn: Function, wrapped_call_fn_name: String) -> TokenStream2 {
	match original_fn.return_type() {
		ReturnTypeClassification::Result => {
			quote_spanned! { span =>
				let result__: core::result::Result<(), ()> = if output__.is_ok() {
					Ok(())
				} else {
					Err(())
				};
				metrics_fn::record(module_name, #wrapped_call_fn_name, result__, end__.duration_since(start__).as_secs_f64());
			}
		},
		_ => {
			quote_spanned! { span =>
				metrics_fn::record(module_name, #wrapped_call_fn_name, Ok(()), end__.duration_since(start__).as_secs_f64());
			}
		},
	}
}
