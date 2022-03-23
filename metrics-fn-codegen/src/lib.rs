use proc_macro::TokenStream;
use quote::{quote_spanned, ToTokens};
use syn::{parse_macro_input, AttributeArgs};

mod call_type;
mod function;

use function::*;

#[proc_macro_attribute]
pub fn dummy(_attr: TokenStream, item: TokenStream) -> TokenStream {
	item
}

#[proc_macro_attribute]
pub fn measure(attrs: TokenStream, item: TokenStream) -> TokenStream {
	let span = proc_macro2::Span::call_site();

	let attrs = parse_macro_input!(attrs as AttributeArgs);

	if attrs.len() > 0 {
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

	let output = quote_spanned! { span =>
		#wrapped_attrs_tokens
		#wrapper_sig_tokens {

			let start__ = std::time::Instant::now();
			let output__ = #wrapped_call_tokens;
			let end__ = std::time::Instant::now();

			let module_name = module_path!();
			metrics_fn::record::<String>(module_name, #wrapped_call_fn_name, Ok(()), end__.duration_since(start__).as_secs_f64());

			return output__;
		}

		#[allow(non_snake_case)]
		#wrapped_sig_tokens
		#wrapped_body_tokens
	};

	output.into()
}
