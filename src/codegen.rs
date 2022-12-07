use convert_case::{Case, Casing};

use crate::ir_builder::ir;

pub fn generate(def: &ir::Def) -> String {
    let mut code = String::new();

    code.push_str(&format!(
        "{} = {}",
        gen_def_name(&def.name),
        gen_schema_class(&def.class)
    ));
    code.push(' ');
    code.push_str(&gen_block(&def.block, 1));

    code
}

fn indent(nesting: usize) -> String {
    const INDENT: &str = "  ";
    INDENT.repeat(nesting)
}

fn gen_block(block: &[ir::Stmt], nesting: usize) -> String {
    assert!(nesting > 0);

    let mut code = String::new();
    code.push_str("do\n");

    for stmt in block {
        code.push_str(&format!("{}{}\n", indent(nesting), gen_stmt(stmt)));
    }

    code.push_str(&format!("{}end\n", indent(nesting - 1)));

    code
}

fn gen_schema_class(schema_class: &ir::SchemaClass) -> String {
    match schema_class {
        ir::SchemaClass::Params => "Dry::Schema::Params".to_string(),
    }
}

fn gen_stmt(stmt: &ir::Stmt) -> String {
    match stmt {
        ir::Stmt::Required { name, r#macro } => {
            format!("required({name}).{}", gen_macro(r#macro))
        }
        ir::Stmt::Optional { name, r#macro } => {
            format!("optional({name}).{}", gen_macro(r#macro))
        }
    }
}

fn gen_macro(r#macro: &ir::Macro) -> String {
    match r#macro {
        ir::Macro::Value { ty } => match ty {
            ir::Type::Integer => "value(:integer)".to_string(),
        },
    }
}

fn gen_def_name(name: &str) -> String {
    name.to_case(Case::Pascal)
}