use std::collections::HashMap;
use crate::dataset::{ColumnType, Dataset, Value, Row};
use crate::query::{Aggregation, Condition, Query};


// both
pub fn filter_dataset(dataset: &Dataset, filter: &Condition) -> Dataset {
    let mut output = Dataset::new(dataset.columns().clone());
    
    for row in dataset.iter() {
        if check_conditions(dataset, filter, row) {
            output.add_row(row.clone());
        }
    }

    return output;
}

// helper function that checks the equal condition and returns t/f, then checks the other conditions
fn check_conditions(dataset: &Dataset, filter: &Condition, row: &Row) -> bool {

    match filter {
        Condition::Equal(string, value) => {

            let col_index = dataset.column_index(string);
            
            if row.get_value(col_index) == value {
                return true;
            }

            return false;
        },
        Condition::Not(condition1) => {
            !check_conditions(dataset, condition1, row)
        },
        Condition::And(condition1, condition2) => {
            check_conditions(dataset, condition1, row) && check_conditions(dataset, condition2, row)
        },
        Condition::Or(condition1, condition2) => {
            check_conditions(dataset, condition1, row) || check_conditions(dataset, condition2, row)
        }
    }
}

// student 1
pub fn group_by_dataset(dataset: Dataset, group_by_column: &String) -> HashMap<Value, Dataset> {
    todo!("Implement this!");
}

// student 2
pub fn aggregate_dataset(dataset: HashMap<Value, Dataset>, aggregation: &Aggregation) -> HashMap<Value, Value> {
    todo!("Implement this!");
}

pub fn compute_query_on_dataset(dataset: &Dataset, query: &Query) -> Dataset {
    let filtered = filter_dataset(dataset, query.get_filter());
    let grouped = group_by_dataset(filtered, query.get_group_by());
    let aggregated = aggregate_dataset(grouped, query.get_aggregate());

    // Create the name of the columns.
    let group_by_column_name = query.get_group_by();
    let group_by_column_type = dataset.column_type(group_by_column_name);
    let columns = vec![
        (group_by_column_name.clone(), group_by_column_type.clone()),
        (query.get_aggregate().get_result_column_name(), ColumnType::Integer),
    ];

    // Create result dataset object and fill it with the results.
    let mut result = Dataset::new(columns);
    for (grouped_value, aggregation_value) in aggregated {
        result.add_row(Row::new(vec![grouped_value, aggregation_value]));
    }
    return result;
}