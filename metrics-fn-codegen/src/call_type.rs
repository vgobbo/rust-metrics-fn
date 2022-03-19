use std::fmt::{Display, Formatter};

use proc_macro2::{Ident, Span};
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
	pub fn ident(&self, span: Span) -> Option<Ident> {
		match self {
			CallType::None => None,
			CallType::OwnedSelf => Some(Ident::new("self", span)),
			CallType::ReferenceSelf => Some(Ident::new("&self", span)),
			CallType::OwnedMutableSelf => Some(Ident::new("mut self", span)),
			CallType::ReferenceMutableSelf => Some(Ident::new("&mut self", span)),
		}
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