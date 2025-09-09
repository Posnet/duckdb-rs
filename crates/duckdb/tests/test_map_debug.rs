use duckdb::{Connection, params};
use std::collections::HashMap;

#[test]
fn test_map_empty() -> Result<(), duckdb::Error> {
    println!("Starting test_map_empty");
    let conn = Connection::open_in_memory()?;
    
    conn.execute(
        "CREATE TABLE test_map (
            id INTEGER,
            attributes MAP(VARCHAR, VARCHAR)
        )",
        []
    )?;
    println!("Table created");
    
    {
        let mut appender = conn.appender("test_map")?;
        println!("Appender created");
        
        // Test empty map
        let empty_map: HashMap<String, String> = HashMap::new();
        println!("About to append empty map");
        appender.append_row(params![1, &empty_map])?;
        println!("Empty map appended");
        
        appender.flush()?;
        println!("Flushed");
    }
    
    Ok(())
}

#[test]
fn test_map_multiple() -> Result<(), duckdb::Error> {
    println!("Starting test_map_multiple");
    let conn = Connection::open_in_memory()?;
    
    conn.execute(
        "CREATE TABLE test_map (
            id INTEGER,
            attributes MAP(VARCHAR, VARCHAR)
        )",
        []
    )?;
    println!("Table created");
    
    {
        let mut appender = conn.appender("test_map")?;
        println!("Appender created");
        
        // Multiple rows
        for i in 1..4 {
            let mut map = HashMap::new();
            map.insert(format!("key{}", i), format!("value{}", i));
            println!("About to append map {}: {:?}", i, map);
            appender.append_row(params![i, &map])?;
            println!("Map {} appended", i);
        }
        
        appender.flush()?;
        println!("Flushed");
    }
    
    Ok(())
}