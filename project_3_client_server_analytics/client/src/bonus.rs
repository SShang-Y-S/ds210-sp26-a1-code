extern crate tarpc;

use std::time::Instant;
use std::io::BufRead;

use analytics_lib::query::Query;
use analytics_lib::query::Condition;
use analytics_lib::dataset::Value;
use analytics_lib::query::Aggregation;
use regex::Regex;
use client::{start_client, solution};

fn parse_value(text: &str) -> Value{
    let cleaned_val = text.trim().trim_matches('"');
    if let Ok(num) = cleaned_val.parse::<i32>(){
        return Value::Integer(num);
    }
    else {
        return Value::String(cleaned_val.to_string());
    }//this would return the inputed reference of string to either Integer Value or String Value
}

fn parse_condition(text: &str) -> Condition{
    let condition = text.trim();
    if condition.contains(" AND "){
        let parts:Vec<&str> = condition.split(" AND ").collect();
        return Condition::And(
            Box::new(parse_condition(parts[0])), 
            Box::new(parse_condition(parts[1])),
        );
    }
    else if condition.contains(" OR "){
        let parts:Vec<&str> = condition.split(" OR ").collect();
        return Condition::Or(
            Box::new(parse_condition(parts[0])),
            Box::new(parse_condition(parts[1])),
        );
    }
    let re = Regex::new(r#"^\s*([A-Za-z0-9_]+)
    \s*==\s*("?[^"]+"?|\d+)\s*$"#).unwrap();
    let cap = re.captures(condition).unwrap();
    let column = cap.get(1).unwrap().as_str().to_string();
    let val_text = cap.get(2).unwrap().as_str();

    return Condition::Equal(column, parse_value(val_text));
}

// Your solution goes here.
fn parse_query_from_string(input: String) -> Query {
    let re = Regex::new(
        r#"^FILTER\s+(.+)\s+
        GROUP BY\s+([A-Za-z0-9_]+)\s+
        (COUNT/SUM/AVERAGE)\s+([A-Za-z0-9_]+)\s*$"#)
        .unwrap(); //Used ChatGPT to generate this line because I was having trouble processing all the conditions

    let cap = re.captures(&input).unwrap();

    let condition_text = cap.get(1).unwrap().as_str();
    let group_by = cap.get(2).unwrap().as_str().to_string();
    let agg_type = cap.get(3).unwrap().as_str();
    let agg_col =  cap.get(4).unwrap().as_str().to_string();


    let condition = parse_condition(condition_text);

    let aggregation; 
    if agg_type == "COUNT"{
        aggregation = Aggregation::Count(agg_col);
    } 
    else if agg_type == "SUM"{
        aggregation = Aggregation::Sum(agg_col);
    }
    else{
        aggregation = Aggregation::Average(agg_col);
    }

    return Query::new(condition, group_by, aggregation);
}

// Each defined rpc generates an async fn that serves the RPC
#[tokio::main]
async fn main() {
    // Establish connection to server.
    let rpc_client = start_client().await;

    // Get a handle to the standard input stream
    let stdin = std::io::stdin();

    // Lock the handle to gain access to BufRead methods like lines()
    println!("Enter your query:");
    for line_result in stdin.lock().lines() {
        // Handle potential errors when reading a line
        match line_result {
            Ok(query) => {
                if query == "exit" {
                    break;
                }

                // parse query.
                let query = parse_query_from_string(query);

                // Carry out query.
                let time = Instant::now();
                let dataset = solution::run_fast_rpc(&rpc_client, query).await;
                let duration = time.elapsed();

                // Print results.
                println!("{}", dataset);
                println!("Query took {:?} to executed", duration);
                println!("Enter your next query (or enter exit to stop):");
            },
            Err(error) => {
                eprintln!("Error reading line: {}", error);
                break;
            }
        }
    }
}