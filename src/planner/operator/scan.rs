use crate::catalog::{TableCatalog, TableName};
use crate::expression::ScalarExpression;
use crate::planner::LogicalPlan;
use crate::storage::Bounds;
use crate::types::index::IndexInfo;
use crate::types::ColumnId;
use itertools::Itertools;

use super::Operator;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct ScanOperator {
    pub table_name: TableName,
    pub primary_key: ColumnId,
    pub columns: Vec<ScalarExpression>,
    // Support push down limit.
    pub limit: Bounds,

    // Support push down predicate.
    // If pre_where is simple predicate, for example:  a > 1 then can calculate directly when read data.
    pub index_infos: Vec<IndexInfo>,
}
impl ScanOperator {
    pub fn build(table_name: TableName, table_catalog: &TableCatalog) -> LogicalPlan {
        let mut primary_key_option = None;
        // Fill all Columns in TableCatalog by default
        let columns = table_catalog
            .all_columns()
            .into_iter()
            .map(|column| {
                if column.desc.is_primary {
                    primary_key_option = column.id();
                }

                ScalarExpression::ColumnRef(column)
            })
            .collect_vec();
        let index_infos = table_catalog
            .indexes
            .iter()
            .map(|meta| IndexInfo {
                meta: meta.clone(),
                binaries: None,
            })
            .collect_vec();

        LogicalPlan {
            operator: Operator::Scan(ScanOperator {
                index_infos,
                table_name,
                primary_key: primary_key_option.unwrap(),
                columns,

                limit: (None, None),
            }),
            childrens: vec![],
            physical_option: None,
        }
    }
}
