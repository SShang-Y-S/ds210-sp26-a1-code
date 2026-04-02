use std::collections::HashMap;
use crate::dataset::{ColumnType, Dataset, Value, Row};
use crate::query::{Aggregation, Condition, Query};

pub fn row_matchs(dataset: &Dataset, row: &Row, filter: &Condition) -> bool{ //made this to find all the matches
    match filter{
        Condition::Equal(column_name, value) =>{
            let index = dataset.column_index(column_name);
            row.get_value(index) == value
        }
        Condition::Not(condition) =>{
            !row_matchs(dataset, row, condition)
        }
        Condition::And(left, right ) => {
            row_matchs(dataset, row, left) && row_matchs(dataset, row, right)
        }
        Condition::Or(left, right) =>{
            row_matchs(dataset, row, left) || row_matchs(dataset, row, right)
        }
    }
}

pub fn filter_dataset(dataset: &Dataset, filter: &Condition) -> Dataset {
    let mut filtered_dataset = Dataset::new(dataset.columns().clone());
    for row in dataset.iter(){
        if row_matchs(dataset, row, filter){
            filtered_dataset.add_row(row.clone());
        }
    }

     return filtered_dataset;
}

pub fn group_by_dataset(dataset: Dataset, group_by_column: &String) -> HashMap<Value, Dataset> {
    let mut subset = HashMap::new();
    for row in dataset.iter(){
        let column_index = dataset.column_index(group_by_column);
        let cell = row.get_value(column_index).clone();

        let tmp = subset.get_mut(&cell);
        match tmp{
            None => {
                let mut dataset = Dataset::new(dataset.columns().clone());
                dataset.add_row(row.clone());
                subset.insert(cell, dataset);
            }
            Some(new_dataset) => {
                new_dataset.add_row(row.clone());
            }
        }
        //subset.insert(cell, dataset);
    }
    return subset;
}

pub fn aggregate_dataset(dataset: HashMap<Value, Dataset>, aggregation: &Aggregation) -> HashMap<Value, Value> {
    // we borrow aggregation (&) because we need to read it, we dont need to own or modify it
    let mut result = HashMap::new();

    for(group_value, group_dataset)in dataset{
        //the function takes dataset without the &, meaning it owns the hashmap
        let aggregated_value = match aggregation {
            Aggregation::Count(_) => {
                Value::Integer(group_dataset.len() as i32)

            }
            Aggregation::Sum(column_name) => {
                let index = group_dataset.column_index(column_name);
                // the scope here is the index variable 
                let mut sum = 0;
                for row in group_dataset.iter(){
                    if let Value::Integer(n) = row.get_value(index){
                        sum += n;
                    }
                }
                Value::Integer(sum)
            }
            Aggregation::Average(column_name) => {
                let index = group_dataset.column_index(column_name);
                let mut sum = 0;
                let count = group_dataset.len() as i32;
                for row in group_dataset.iter(){
                    if let Value::Integer(n) = row.get_value(index){
                        sum += n;
                    }
                }
                Value::Integer(sum/count)
            }
        };
        result.insert(group_value, aggregated_value);
    }
    return result;
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