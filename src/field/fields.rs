use crate::field::Field;
use sea_query::{DynIden, SimpleExpr};

// region:    --- Fields
#[derive(Debug, Clone)]
pub struct Fields(Vec<Field>);

// Constructor
impl Fields {
	pub fn new(fields: Vec<Field>) -> Self {
		Fields(fields)
	}
}

// Api
impl Fields {
	pub fn push(&mut self, field: Field) {
		self.0.push(field);
	}

	pub fn into_vec(self) -> Vec<Field> {
		self.0
	}

	/// returns a tuble: (Vec_of_column_idens, Vec_of_value_exprs)
	pub fn unzip(self) -> (Vec<DynIden>, Vec<SimpleExpr>) {
		self.0.into_iter().map(|f| (f.iden, f.value)).unzip()
	}

	/// returns Iterator of (column_iden, value_expr)
	pub fn zip(self) -> impl Iterator<Item = (DynIden, SimpleExpr)> {
		self.0.into_iter().map(|f| (f.iden, f.value))
	}
}

impl IntoIterator for Fields {
	type Item = Field;
	type IntoIter = std::vec::IntoIter<Self::Item>;

	fn into_iter(self) -> Self::IntoIter {
		self.0.into_iter()
	}
}

// endregion: --- Fields