use regex::Regex;
use std::env;
use std::fs;
use std::io::{self};
use std::process;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <path-to-db-model-file>", args[0]);
        process::exit(1);
    }

    let file_path = &args[1];
    let source = fs::read_to_string(file_path)?;

    // Find the struct definition
    let struct_regex = Regex::new(r"(?ms)struct\s+([A-Za-z_][A-Za-z0-9_]*)\s*\{([^}]*)\}").unwrap();
    let captures = struct_regex
        .captures(&source)
        .expect("Could not find struct definition");

    let struct_name = captures.get(1).unwrap().as_str();
    let struct_fields = captures.get(2).unwrap().as_str();

    // Extract field declarations
    let field_regex = Regex::new(
        r"(?m)^\s*(?:#\[[^\]]*\]\s*)*(?:pub\s+)?([a-zA-Z_][a-zA-Z0-9_]*)\s*:\s*([^,]+),?",
    )
    .unwrap();

    let mut fields = Vec::new();

    for cap in field_regex.captures_iter(struct_fields) {
        let field_name = cap.get(1).unwrap().as_str();
        let field_type = cap.get(2).unwrap().as_str().trim();

        // Skip id, created_at, and updated_at which are common fields
        if field_name == "id" || field_name == "created_at" || field_name == "updated_at" {
            continue;
        }

        fields.push((field_name, field_type));
    }

    // Generate the GraphQL model struct
    let gql_name = format!("{}GQL", struct_name);

    println!("#[derive(async_graphql::SimpleObject, Clone, Debug)]");
    println!("pub struct {} {{", gql_name);
    println!("    pub id: Option<String>,");

    for (name, type_str) in &fields {
        println!("    pub {}: {},", name, type_str);
    }

    println!("    pub created_at: Option<String>,");
    println!("    pub updated_at: Option<String>,");
    println!("}}");

    // Generate the From implementation
    println!();
    println!("impl From<{}> for {} {{", struct_name, gql_name);
    println!("    fn from(value: {}) -> Self {{", struct_name);
    println!("        {} {{", gql_name);
    println!("            id: value.id.as_ref().map(|oid| oid.to_hex()),");

    for (name, type_str) in &fields {
        // Handle different field types
        if type_str.contains("String") {
            println!("            {}: value.{}.clone(),", name, name);
        } else if type_str.starts_with("Option<") && type_str.contains("String") {
            println!("            {}: value.{}.clone(),", name, name);
        } else {
            // For primitive types and enums
            println!("            {}: value.{},", name, name);
        }
    }

    println!("            created_at: value.created_at.map(|dt| dt.to_string()),");
    println!("            updated_at: value.updated_at.map(|dt| dt.to_string()),");
    println!("        }}");
    println!("    }}");
    println!("}}");

    Ok(())
}
