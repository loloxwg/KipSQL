use super::*;
use crate::types::{DataType, DataValue};
use sqlparser::ast::{Expr, Value};

#[derive(Debug, PartialEq, Clone)]
pub struct BoundExpr {
    pub kind: BoundExprKind,
    /// The return type of the expression.
    /// `None` means NULL.
    pub return_type: Option<DataType>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum BoundExprKind {
    Constant(DataValue),
}

impl BoundExpr {
    pub fn constant(value: DataValue) -> Self {
        BoundExpr {
            return_type: value.data_type(),
            kind: BoundExprKind::Constant(value),
        }
    }
}

impl Binder {
    /// Bind an expression.
    pub fn bind_expr(&mut self, expr: &Expr) -> Result<BoundExpr, BindError> {
        match expr {
            Expr::Value(v) => Ok(BoundExpr::constant(v.into())),
            _ => todo!("bind expression: {:?}", expr),
        }
    }
}
