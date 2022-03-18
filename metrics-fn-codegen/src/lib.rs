use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote_spanned;

mod function;

use function::*;

#[proc_macro_attribute]
pub fn dummy(_attr: TokenStream, item: TokenStream) -> TokenStream {
	item
}

#[proc_macro_attribute]
pub fn measure(_attr: TokenStream, item: TokenStream) -> TokenStream {
	let item = TokenStream2::from(item);

	// parse the fn.
	let function = Function::from(item.clone());

	// map the fn to wrapper and wrapped variables
	let wrapper_fn = function;
	let wrapped_fn = wrapper_fn.rename("wrapped".to_owned());

	// build the wrapped fn.
	let output = wrap(wrapper_fn, wrapped_fn);

	println!("{}", output.to_string());

	output.into()
}

fn wrap(wrapper_fn: Function, wrapped_fn: Function) -> TokenStream2 {
	let span = proc_macro2::Span::call_site();

	let wrapper_attributes = TokenStream2::from_iter(wrapper_fn.attributes.clone().into_iter());
	let wrapped_body = wrapped_fn.body.clone();
	let wrapped_call = wrapped_fn.call(wrapped_fn.argument_names().as_slice());

	let output = quote_spanned! { span =>
		#wrapper_attributes
		#wrapper_fn {
			#wrapped_fn #wrapped_body

			let start__ = std::time::Instant::now();
			let output__ = #wrapped_call;
			let end__ = std::time::Instant::now();

			println!("Time {:?}", end__.duration_since(start__));

			return output__;
		}
	};

	output
}
