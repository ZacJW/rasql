pub mod type_gen;
pub mod client_gen;

use std::collections::HashMap;

use client_gen::AsyncClientCodeGenerator;
use convert_case::Casing;
use type_gen::TypeGenerator;

pub struct TableStruct {
    pub name: syn::Ident,
    pub fields: Vec<TableStructField>,
    pub db_alias: Option<String>,
}

pub struct TableStructField {
    pub name: syn::Ident,
    pub r#type: syn::Type,
    pub db_alias: Option<String>,
}

pub struct GeneratedTableStruct(pub proc_macro2::TokenStream);

pub struct TableStructImpls {
    pub base_table_impl: proc_macro2::TokenStream,
    pub table_with_pk_impl: Option<proc_macro2::TokenStream>,
}

fn sql_ident_to_type_name(ident: &sqlparser::ast::Ident) -> syn::Ident {
    let mut ident = ident.value.to_case(convert_case::Case::Pascal);
    if ident.chars().next().unwrap().is_ascii_digit() {
        ident.insert(0, '_');
    }
    syn::Ident::new(&ident, proc_macro2::Span::call_site())
}

fn sql_ident_to_field_name(ident: &sqlparser::ast::Ident) -> syn::Ident {
    let mut ident = ident.value.to_case(convert_case::Case::Snake);
    if ident.chars().next().unwrap().is_ascii_digit() {
        ident.insert(0, '_');
    }
    syn::Ident::new(&ident, proc_macro2::Span::call_site())
}

#[inline]
fn sql_ident_to_module_name(ident: &sqlparser::ast::Ident) -> syn::Ident {
    sql_ident_to_field_name(ident)
}

fn generate_table_struct_and_impls<
    Traits: rasql_traits::DbTraits,
    TypeGen: TypeGenerator<Traits>,
    Client: rasql_traits::r#async::Client<Traits = Traits>,
    ClientGen: AsyncClientCodeGenerator<Client>,
>(
    table: &crate::sql::Table,
    module_config: Option<&ModuleCodeGenConfig>,
) -> (GeneratedTableStruct, TableStructImpls) {
    let name = sql_ident_to_type_name(table.name.0.last().unwrap());
    let default_struct_config = StructCodeGenConfig {
        field_configs: HashMap::new(),
        deny_extra_fields: false,
    };
    let struct_config = module_config
        .and_then(|config| config.struct_configs.get(&name))
        .unwrap_or(&default_struct_config);

    let fields = table
        .columns
        .iter()
        .map(|column| {
            let default_field_config = StructFieldCodeGenConfig {
                rename: None,
                override_type: None,
                attrs: vec![],
                id_promote_mode: IdPromoteMode::None,
            };

            let field_config = struct_config
                .field_configs
                .get(&column.name)
                .unwrap_or(&default_field_config);

            let (name, db_alias) = match &field_config.rename {
                Some(rename) => (rename.clone(), Some(column.name.value.clone())),
                None => {
                    let name = sql_ident_to_field_name(&column.name);
                    if name.to_string() == column.name.value {
                        (name, None)
                    } else {
                        (name, Some(column.name.value.clone()))
                    }
                }
            };

            let r#type = match (&field_config.override_type, field_config.id_promote_mode) {
                (Some(r#type), _) => r#type.clone(),
                (None, IdPromoteMode::None) => TypeGen::sql_datatype_to_rust_type(&column.data_type).unwrap(),
                (None, IdPromoteMode::TrustedId) => todo!(),
                (None, IdPromoteMode::Id) => todo!(),
            };

            TableStructField {
                name,
                r#type,
                db_alias,
            }
        })
        .collect();

    let table_struct = TableStruct {
        name,
        fields,
        db_alias: todo!(),
    };
    (
        GeneratedTableStruct(TypeGen::generate_table_struct(&table_struct)),
        TableStructImpls {
            base_table_impl: todo!(),
            table_with_pk_impl: todo!(),
        },
    )
}

pub struct CodeGenConfig {
    pub module_configs: HashMap<syn::Ident, ModuleCodeGenConfig>,
}

pub struct ModuleCodeGenConfig {
    pub use_statements: Vec<syn::ItemUse>,
    pub struct_configs: HashMap<syn::Ident, StructCodeGenConfig>,
}

pub struct StructCodeGenConfig {
    pub field_configs: HashMap<sqlparser::ast::Ident, StructFieldCodeGenConfig>,
    pub deny_extra_fields: bool,
}

#[derive(Default)]
pub struct StructFieldCodeGenConfig {
    rename: Option<syn::Ident>,
    override_type: Option<syn::Type>,
    attrs: Vec<syn::Attribute>,
    id_promote_mode: IdPromoteMode,
}

#[derive(Clone, Copy, Default)]
pub enum IdPromoteMode {
    #[default]
    None,
    TrustedId,
    Id,
}
