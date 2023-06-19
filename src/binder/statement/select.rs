use crate::binder::expression::BoundExpr;
use crate::binder::{BindError, Binder};
use crate::parser::Value;
use crate::types::DataValue;
use sqlparser::ast::{Expr, Query, SelectItem, SetExpr};

/// A bound `SELECT` statement.
#[derive(Debug, PartialEq, Clone)]
pub struct BoundSelect {
    pub values: Vec<BoundExpr>,
}

impl Binder {
    pub fn bind_select(&mut self, query: &Query) -> Result<BoundSelect, BindError> {
        match *query.body.clone() {
            SetExpr::Select(select) => {
                let mut values = vec![];
                for item in &select.projection {
                    match item {
                        SelectItem::UnnamedExpr(expr) => values.push(self.bind_expr(expr)?),
                        _ => todo!("not supported statement: {:#?}", query),
                    }
                }
                Ok(BoundSelect { values })
            }
            _ => todo!("not supported statement: {:#?}", query),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::binder::BoundStatement;
    use crate::catalog::RootCatalog;
    use crate::parser;
    use crate::parser::SQLParser;
    use std::sync::Arc;
    #[test]
    fn test_bind_select() {
        let sql = "SELECT 1, 2, 3";
        let stmt = parser::RSParser::parse_sql(sql).unwrap();
        let catalog = Arc::new(RootCatalog::new());
        let mut binder = Binder::new(catalog.clone());
        let stmt = binder.bind(&stmt[0]).unwrap();
        println!("{:#?}", stmt);

        assert_eq!(
            stmt,
            BoundStatement::Select(BoundSelect {
                values: vec![
                    BoundExpr::constant(DataValue::Int32(1)),
                    BoundExpr::constant(DataValue::Int32(2)),
                    BoundExpr::constant(DataValue::Int32(3)),
                ],
            })
        );
    }
}
