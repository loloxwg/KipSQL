use super::*;
use crate::parser::Expr;
use crate::types::{DataType, DataValue};

mod column_ref;

pub use self::column_ref::*;

/// A bound expression.
#[derive(Debug, PartialEq, Clone)]
pub enum BoundExpr {
    Constant(DataValue),
    ColumnRef(BoundColumnRef),
}

impl BoundExpr {
    /// Get return type of the expression.
    ///
    /// Returns `None` if the type can not be decided.
    pub fn return_type(&self) -> Option<DataType> {
        match self {
            Self::Constant(v) => v.datatype(),
            Self::ColumnRef(c) => Some(c.return_type.clone()),
        }
    }
}

impl Binder {
    /// Bind an expression.
    pub fn bind_expr(&mut self, expr: &Expr) -> Result<BoundExpr, BindError> {
        match expr {
            Expr::Value(v) => Ok(BoundExpr::Constant(v.into())),
            Expr::Identifier(ident) => self.bind_column_ref(std::slice::from_ref(ident)),
            Expr::CompoundIdentifier(idents) => self.bind_column_ref(idents),
            _ => todo!("bind expression: {:?}", expr),
        }
    }
}
