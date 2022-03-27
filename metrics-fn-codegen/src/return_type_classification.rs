use syn::{ReturnType, Type};

#[derive(Debug, Copy, Clone)]
pub enum ReturnTypeClassification {
	None,
	Result,
	Other,
}

impl From<&ReturnType> for ReturnTypeClassification {
	fn from(return_type: &ReturnType) -> Self {
		let mut class = ReturnTypeClassification::None;
		if let ReturnType::Type(_, boxed_type) = &return_type {
			class = ReturnTypeClassification::Other;

			let a_type = &**boxed_type;
			if let Type::Path(type_path) = &*a_type {
				let path = &type_path.path;
				if let Some(segment) = path.segments.last() {
					if segment.ident.to_string() == "Result" {
						class = ReturnTypeClassification::Result
					}
				}
			}
		}

		class
	}
}
