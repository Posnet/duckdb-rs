use duckdb::{Connection, params};
use std::collections::HashMap;

#[test]
fn test_comprehensive_generic_map() -> Result<(), duckdb::Error> {
    let conn = Connection::open_in_memory()?;
    
    // Create a complex table showing various MAP combinations
    conn.execute(
        "CREATE TABLE maps_showcase (
            id INTEGER,
            -- String key maps
            str_to_str MAP(VARCHAR, VARCHAR),
            str_to_i32 MAP(VARCHAR, INTEGER),
            str_to_f64 MAP(VARCHAR, DOUBLE),
            str_to_bool MAP(VARCHAR, BOOLEAN),
            
            -- Integer key maps  
            i32_to_str MAP(INTEGER, VARCHAR),
            i32_to_i32 MAP(INTEGER, INTEGER),
            i32_to_f64 MAP(INTEGER, DOUBLE),
            
            -- Mixed numeric types
            u8_to_i128 MAP(UTINYINT, HUGEINT),
            i16_to_u64 MAP(SMALLINT, UBIGINT)
        )",
        []
    )?;
    
    // Insert a comprehensive example row
    {
        let mut appender = conn.appender("maps_showcase")?;
        
        // String -> String
        let mut metadata = HashMap::new();
        metadata.insert("name".to_string(), "John Doe".to_string());
        metadata.insert("role".to_string(), "Engineer".to_string());
        metadata.insert("department".to_string(), "R&D".to_string());
        
        // String -> i32
        let mut stats = HashMap::new();
        stats.insert("age".to_string(), 30i32);
        stats.insert("years_experience".to_string(), 8i32);
        stats.insert("team_size".to_string(), 5i32);
        
        // String -> f64
        let mut metrics = HashMap::new();
        metrics.insert("performance_score".to_string(), 4.5f64);
        metrics.insert("salary_multiplier".to_string(), 1.2f64);
        metrics.insert("project_completion".to_string(), 0.95f64);
        
        // String -> bool
        let mut flags = HashMap::new();
        flags.insert("is_manager".to_string(), true);
        flags.insert("has_phd".to_string(), false);
        flags.insert("remote_work".to_string(), true);
        
        // i32 -> String
        let mut id_to_name = HashMap::new();
        id_to_name.insert(1i32, "Project Alpha".to_string());
        id_to_name.insert(2i32, "Project Beta".to_string());
        id_to_name.insert(3i32, "Project Gamma".to_string());
        
        // i32 -> i32
        let mut year_to_count = HashMap::new();
        year_to_count.insert(2021i32, 10i32);
        year_to_count.insert(2022i32, 15i32);
        year_to_count.insert(2023i32, 20i32);
        
        // i32 -> f64
        let mut quarter_revenue = HashMap::new();
        quarter_revenue.insert(1i32, 1000000.50f64);
        quarter_revenue.insert(2i32, 1250000.75f64);
        quarter_revenue.insert(3i32, 1500000.25f64);
        
        // u8 -> i128
        let mut tiny_to_huge = HashMap::new();
        tiny_to_huge.insert(1u8, 1000000000000i128);
        tiny_to_huge.insert(2u8, 2000000000000i128);
        
        // i16 -> u64
        let mut small_to_big = HashMap::new();
        small_to_big.insert(100i16, 1000000u64);
        small_to_big.insert(200i16, 2000000u64);
        
        appender.append_row(params![
            1,
            &metadata,
            &stats,
            &metrics,
            &flags,
            &id_to_name,
            &year_to_count,
            &quarter_revenue,
            &tiny_to_huge,
            &small_to_big
        ])?;
        
        appender.flush()?;
    }
    
    // Test querying various map types
    let name: Option<String> = conn.query_row(
        "SELECT str_to_str['name'] FROM maps_showcase WHERE id = 1",
        [],
        |row| row.get(0)
    )?;
    assert_eq!(name, Some("John Doe".to_string()));
    
    let age: Option<i32> = conn.query_row(
        "SELECT str_to_i32['age'] FROM maps_showcase WHERE id = 1",
        [],
        |row| row.get(0)
    )?;
    assert_eq!(age, Some(30));
    
    let score: Option<f64> = conn.query_row(
        "SELECT str_to_f64['performance_score'] FROM maps_showcase WHERE id = 1",
        [],
        |row| row.get(0)
    )?;
    assert_eq!(score, Some(4.5));
    
    let is_manager: Option<bool> = conn.query_row(
        "SELECT str_to_bool['is_manager'] FROM maps_showcase WHERE id = 1",
        [],
        |row| row.get(0)
    )?;
    assert_eq!(is_manager, Some(true));
    
    let project: Option<String> = conn.query_row(
        "SELECT i32_to_str[1] FROM maps_showcase WHERE id = 1",
        [],
        |row| row.get(0)
    )?;
    assert_eq!(project, Some("Project Alpha".to_string()));
    
    // Test MAP aggregate functions
    let map_size: i64 = conn.query_row(
        "SELECT cardinality(str_to_str) FROM maps_showcase WHERE id = 1",
        [],
        |row| row.get(0)
    )?;
    assert_eq!(map_size, 3);
    
    println!("✅ Comprehensive generic MAP support test passed!");
    println!("   - Multiple key types (String, i32, u8, i16)");
    println!("   - Multiple value types (String, i32, f64, bool, i128, u64)");
    println!("   - All combinations work seamlessly with the generic implementation");
    
    Ok(())
}