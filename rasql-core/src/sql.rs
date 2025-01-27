use std::collections::HashMap;

use sqlparser::ast::{ColumnDef, DataType, Ident, ObjectName, SchemaName, TableConstraint};

pub fn parse_sql_schema(
    sql_statements: impl IntoIterator<
        Item = impl TryInto<sqlparser::ast::Statement, Error = impl std::fmt::Debug>,
    >,
) {
    let mut schemas = HashMap::new();
    for statement in sql_statements {
        let statement: sqlparser::ast::Statement = statement.try_into().unwrap();
        match statement {
            sqlparser::ast::Statement::CreateSchema { schema_name, .. } => {
                schemas
                    .entry(schema_name.clone())
                    .or_insert_with(|| Schema {
                        name: schema_name,
                        tables: Default::default(),
                        types: Default::default(),
                    });
            }
            sqlparser::ast::Statement::CreateTable(sqlparser::ast::CreateTable {
                name,
                columns,
                constraints,
                ..
            }) => {
                let schema = schema_for_object(&mut schemas, &name);
                schema.types.insert(
                    name.clone(),
                    Type::Composite {
                        name: name.clone(),
                        fields: columns
                            .iter()
                            .map(|column| Field {
                                name: column.name.clone(),
                                r#type: column.data_type.clone(),
                            })
                            .collect(),
                    },
                );
                schema.tables.insert(
                    name.clone(),
                    Table {
                        name,
                        columns,
                        constraints,
                    },
                );
            }
            sqlparser::ast::Statement::AlterTable {
                name, operations, ..
            } => {
                let schema = schema_for_object(&mut schemas, &name);
                let Some(table) = schema.tables.get_mut(&name) else {
                    continue;
                };
                for op in operations {
                    match op {
                        sqlparser::ast::AlterTableOperation::AddConstraint(table_constraint) => {
                            table.constraints.push(table_constraint);
                        }
                        _ => (),
                    }
                }
            }
            sqlparser::ast::Statement::CreateType {
                name,
                representation:
                    sqlparser::ast::UserDefinedTypeRepresentation::Composite { attributes },
            } => {
                let schema = schema_for_object(&mut schemas, &name);
                schema.types.insert(
                    name.clone(),
                    Type::Composite {
                        name,
                        fields: attributes
                            .into_iter()
                            .map(|attr| Field {
                                name: attr.name,
                                r#type: attr.data_type,
                            })
                            .collect(),
                    },
                );
            }
            sqlparser::ast::Statement::CreateType {
                name,
                representation: sqlparser::ast::UserDefinedTypeRepresentation::Enum { labels },
            } => {
                let schema = schema_for_object(&mut schemas, &name);
                schema.types.insert(
                    name.clone(),
                    Type::Enum {
                        name,
                        variants: labels,
                    },
                );
            }
            _ => (),
        }
    }
}

fn schema_for_object<'a>(
    schemas: &'a mut HashMap<SchemaName, Schema>,
    object_name: &ObjectName,
) -> &'a mut Schema {
    let schema = match object_name.0.as_slice() {
        [_table_name] => schemas
            .entry(SchemaName::Simple(ObjectName(vec![Ident::new("public")])))
            .or_insert_with(|| Schema {
                name: SchemaName::Simple(ObjectName(vec![Ident::new("public")])),
                tables: Default::default(),
                types: Default::default(),
            }),
        [schema_name, _table_name] => schemas
            .entry(SchemaName::Simple(ObjectName(vec![schema_name.clone()])))
            .or_insert_with(|| Schema {
                name: SchemaName::Simple(ObjectName(vec![schema_name.clone()])),
                tables: Default::default(),
                types: Default::default(),
            }),
        [catalog_name, schema_name, _table_name] => schemas
            .entry(SchemaName::Simple(ObjectName(vec![
                catalog_name.clone(),
                schema_name.clone(),
            ])))
            .or_insert_with(|| Schema {
                name: SchemaName::Simple(ObjectName(vec![
                    catalog_name.clone(),
                    schema_name.clone(),
                ])),
                tables: Default::default(),
                types: Default::default(),
            }),
        _ => unreachable!(),
    };
    schema
}

pub struct Schema {
    pub name: SchemaName,
    pub tables: HashMap<ObjectName, Table>,
    pub types: HashMap<ObjectName, Type>,
}

pub struct Table {
    pub name: ObjectName,
    pub columns: Vec<ColumnDef>,
    pub constraints: Vec<TableConstraint>,
}

pub enum Type {
    Composite {
        name: ObjectName,
        fields: Vec<Field>,
    },
    Enum {
        name: ObjectName,
        variants: Vec<Ident>,
    },
}

pub struct Field {
    pub name: Ident,
    pub r#type: DataType,
}
