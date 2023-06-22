use crate::binder::expression::BoundExpr;
use crate::binder::table_ref::BoundTableRef;
use crate::binder::{BindError, Binder};
use crate::types::DataValue;
use sqlparser::ast::{Query, SelectItem, SetExpr};

/// A bound `SELECT` statement.
#[derive(Debug, PartialEq, Clone)]
pub struct BoundSelect {
    pub select_list: Vec<BoundExpr>,
    pub from_list: Vec<BoundTableRef>,
}

impl Binder {
    pub fn bind_select(&mut self, query: &Query) -> Result<BoundSelect, BindError> {
        let select = match &query.body {
            SetExpr::Select(select) => &**select,
            _ => todo!("not select"),
        };

        let mut from_list = vec![];
        assert!(select.from.len() <= 1, "multiple tables are not supported");
        for table_with_join in select.from.iter() {
            let table_ref = self.bind_table_with_joins(table_with_join)?;
            from_list.push(table_ref);
        }
        assert!(select.selection.is_none(), "WHERE clause is not supported");
        assert!(
            query.order_by.is_empty(),
            "ORDER BY clause is not supported"
        );
        assert!(query.limit.is_none(), "LIMIT clause is not supported");
        assert!(query.offset.is_none(), "OFFSET clause is not supported");
        assert!(
            select.group_by.is_empty(),
            "GROUP BY clause is not supported"
        );
        assert!(!select.distinct, "DISTINCT is not supported");
        // Bind the select list.
        let mut select_list = vec![];
        for item in select.projection.iter() {
            match item {
                SelectItem::UnnamedExpr(expr) => {
                    select_list.push(self.bind_expr(&expr)?);
                }
                SelectItem::ExprWithAlias { expr, .. } => {
                    select_list.push(self.bind_expr(&expr)?);
                }
                // SelectItem::Wildcard => {
                //     select_list.extend(self.bind_all_column_refs()?);
                // }
                _ => todo!("not supported select item: {:?}", item),
            }
        }
        Ok(BoundSelect {
            select_list,
            from_list,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::binder::BoundStatement;
    use crate::catalog::DatabaseCatalog;
    use crate::parser;
    use crate::parser::SQLParser;
    use std::sync::Arc;

    #[test]
    fn test_bind_select() {
        let sql = "SELECT 1, 2, 3";
        let stmt = parser::RSParser::parse_sql(sql).unwrap();
        let catalog = Arc::new(DatabaseCatalog::new());
        let mut binder = Binder::new(catalog.clone());
        let stmt = binder.bind(&stmt[0]).unwrap();
        println!("{:#?}", stmt);

        assert_eq!(
            stmt,
            BoundStatement::Select(BoundSelect {
                select_list: vec![
                    BoundExpr::Constant(DataValue::Int32(1)),
                    BoundExpr::Constant(DataValue::Int32(2)),
                    BoundExpr::Constant(DataValue::Int32(3)),
                ],
                from_list: vec![],
            })
        );
    }
}
