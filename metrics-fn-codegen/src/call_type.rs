use std::fmt::{Display, Formatter};

use proc_macro2::{Ident, Punct, Spacing, Span, TokenTree};
use syn::FnArg::Receiver;
use syn::Signature;

pub enum CallType {
	None,
	OwnedSelf,
	ReferenceSelf,
	OwnedMutableSelf,
	ReferenceMutableSelf,
}

impl CallType {
	pub fn tokens(&self, span: Span) -> Vec<TokenTree> {
		match self {
			CallType::None => vec![],
			CallType::OwnedSelf => vec![Self::token_self(span)],
			CallType::ReferenceSelf => vec![Self::token_reference(), Self::token_self(span)],
			CallType::OwnedMutableSelf => vec![Self::token_mut(span), Self::token_self(span)],
			CallType::ReferenceMutableSelf => {
				vec![Self::token_reference(), Self::token_mut(span), Self::token_self(span)]
			},
		}
	}

	fn token_mut(span: Span) -> TokenTree {
		TokenTree::from(Ident::new("mut", span))
	}

	fn token_self(span: Span) -> TokenTree {
		TokenTree::from(Ident::new("self", span))
	}

	fn token_reference() -> TokenTree {
		TokenTree::from(Punct::new('&', Spacing::Alone))
	}
}

impl From<&Signature> for CallType {
	fn from(sig: &Signature) -> Self {
		let receiver_opt = sig
			.inputs
			.iter()
			.filter_map(|arg| {
				if let Receiver(receiver) = arg {
					Some(receiver)
				} else {
					None
				}
			})
			.next();

		if let Some(receiver) = receiver_opt {
			if receiver.mutability.is_some() {
				if receiver.reference.is_some() {
					CallType::ReferenceMutableSelf
				} else {
					CallType::OwnedMutableSelf
				}
			} else {
				if receiver.reference.is_some() {
					CallType::ReferenceSelf
				} else {
					CallType::OwnedSelf
				}
			}
		} else {
			CallType::None
		}
	}
}

impl Display for CallType {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			CallType::None => write!(f, "None"),
			CallType::OwnedSelf => write!(f, "OwnedSelf"),
			CallType::ReferenceSelf => write!(f, "ReferenceSelf"),
			CallType::OwnedMutableSelf => write!(f, "OwnedMutableSelf"),
			CallType::ReferenceMutableSelf => write!(f, "ReferenceMutableSelf"),
		}
	}
}
