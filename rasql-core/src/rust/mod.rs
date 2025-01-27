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

fn sql_datatype_to_rust_type(datatype: &sqlparser::ast::DataType) -> syn::Type {
    match datatype {
        sqlparser::ast::DataType::Character(..)
        | sqlparser::ast::DataType::Char(..)
        | sqlparser::ast::DataType::CharacterVarying(..)
        | sqlparser::ast::DataType::CharVarying(..)
        | sqlparser::ast::DataType::Varchar(..)
        | sqlparser::ast::DataType::Nvarchar(..)
        | sqlparser::ast::DataType::Text
        | sqlparser::ast::DataType::TinyText
        | sqlparser::ast::DataType::MediumText
        | sqlparser::ast::DataType::LongText
        | sqlparser::ast::DataType::String(_)
        | sqlparser::ast::DataType::FixedString(_) => syn::Type::Verbatim(quote::quote! {String}),
        sqlparser::ast::DataType::Uuid => todo!(),
        sqlparser::ast::DataType::CharacterLargeObject(_) => todo!(),
        sqlparser::ast::DataType::CharLargeObject(_) => todo!(),
        sqlparser::ast::DataType::Clob(_) => todo!(),
        sqlparser::ast::DataType::Binary(_) => todo!(),
        sqlparser::ast::DataType::Varbinary(_) => todo!(),
        sqlparser::ast::DataType::Blob(_) => todo!(),
        sqlparser::ast::DataType::TinyBlob => todo!(),
        sqlparser::ast::DataType::MediumBlob => todo!(),
        sqlparser::ast::DataType::LongBlob => todo!(),
        sqlparser::ast::DataType::Bytes(_) => todo!(),
        sqlparser::ast::DataType::Numeric(exact_number_info) => todo!(),
        sqlparser::ast::DataType::Decimal(exact_number_info) => todo!(),
        sqlparser::ast::DataType::BigNumeric(exact_number_info) => todo!(),
        sqlparser::ast::DataType::BigDecimal(exact_number_info) => todo!(),
        sqlparser::ast::DataType::Dec(exact_number_info) => todo!(),
        sqlparser::ast::DataType::Float(_) => todo!(),
        sqlparser::ast::DataType::TinyInt(_) => todo!(),
        sqlparser::ast::DataType::UnsignedTinyInt(_) => todo!(),
        sqlparser::ast::DataType::Int2(_) => todo!(),
        sqlparser::ast::DataType::UnsignedInt2(_) => todo!(),
        sqlparser::ast::DataType::SmallInt(_) => todo!(),
        sqlparser::ast::DataType::UnsignedSmallInt(_) => todo!(),
        sqlparser::ast::DataType::MediumInt(_) => todo!(),
        sqlparser::ast::DataType::UnsignedMediumInt(_) => todo!(),
        sqlparser::ast::DataType::Int(_) => todo!(),
        sqlparser::ast::DataType::Int16 => todo!(),
        sqlparser::ast::DataType::Int128 => todo!(),
        sqlparser::ast::DataType::Int256 => todo!(),
        sqlparser::ast::DataType::Int32
        | sqlparser::ast::DataType::Int4(_)
        | sqlparser::ast::DataType::Integer(_) => syn::Type::Verbatim(quote::quote! {i32}),
        sqlparser::ast::DataType::UnsignedInt(_) => todo!(),
        sqlparser::ast::DataType::UnsignedInt4(_) => todo!(),
        sqlparser::ast::DataType::UnsignedInteger(_) => todo!(),
        sqlparser::ast::DataType::UInt8 => todo!(),
        sqlparser::ast::DataType::UInt16 => todo!(),
        sqlparser::ast::DataType::UInt32 => todo!(),
        sqlparser::ast::DataType::UInt64 => todo!(),
        sqlparser::ast::DataType::UInt128 => todo!(),
        sqlparser::ast::DataType::UInt256 => todo!(),
        sqlparser::ast::DataType::Int8(_)
        | sqlparser::ast::DataType::Int64
        | sqlparser::ast::DataType::BigInt(_) => syn::Type::Verbatim(quote::quote! {i64}),
        sqlparser::ast::DataType::UnsignedBigInt(_) => todo!(),
        sqlparser::ast::DataType::UnsignedInt8(_) => todo!(),
        sqlparser::ast::DataType::Float4
        | sqlparser::ast::DataType::Real
        | sqlparser::ast::DataType::Float32 => syn::Type::Verbatim(quote::quote! {f32}),
        sqlparser::ast::DataType::Float64
        | sqlparser::ast::DataType::Float8
        | sqlparser::ast::DataType::Double(..)
        | sqlparser::ast::DataType::DoublePrecision => syn::Type::Verbatim(quote::quote! {f64}),
        sqlparser::ast::DataType::Bool => todo!(),
        sqlparser::ast::DataType::Boolean => todo!(),
        sqlparser::ast::DataType::Date => todo!(),
        sqlparser::ast::DataType::Date32 => todo!(),
        sqlparser::ast::DataType::Time(_, timezone_info) => todo!(),
        sqlparser::ast::DataType::Datetime(_) => todo!(),
        sqlparser::ast::DataType::Datetime64(_, _) => todo!(),
        sqlparser::ast::DataType::Timestamp(_, timezone_info) => todo!(),
        sqlparser::ast::DataType::Interval => todo!(),
        sqlparser::ast::DataType::JSON => todo!(),
        sqlparser::ast::DataType::JSONB => todo!(),
        sqlparser::ast::DataType::Regclass => todo!(),
        sqlparser::ast::DataType::Bytea => todo!(),
        sqlparser::ast::DataType::Bit(_) => todo!(),
        sqlparser::ast::DataType::BitVarying(_) => todo!(),
        sqlparser::ast::DataType::Custom(object_name, vec) => todo!(),
        sqlparser::ast::DataType::Array(array_elem_type_def) => match array_elem_type_def {
            sqlparser::ast::ArrayElemTypeDef::None => unimplemented!(),
            sqlparser::ast::ArrayElemTypeDef::AngleBracket(data_type)
            | sqlparser::ast::ArrayElemTypeDef::SquareBracket(data_type, _)
            | sqlparser::ast::ArrayElemTypeDef::Parenthesis(data_type) => todo!(),
        },
        sqlparser::ast::DataType::Map(data_type, data_type1) => todo!(),
        sqlparser::ast::DataType::Tuple(vec) => todo!(),
        sqlparser::ast::DataType::Nested(vec) => todo!(),
        sqlparser::ast::DataType::Enum(vec, _) => todo!(),
        sqlparser::ast::DataType::Set(vec) => todo!(),
        sqlparser::ast::DataType::Struct(vec, struct_bracket_kind) => todo!(),
        sqlparser::ast::DataType::Union(vec) => todo!(),
        sqlparser::ast::DataType::Nullable(data_type) => todo!(),
        sqlparser::ast::DataType::LowCardinality(data_type) => todo!(),
        sqlparser::ast::DataType::Unspecified => todo!(),
        sqlparser::ast::DataType::Trigger => todo!(),
        sqlparser::ast::DataType::AnyType => todo!(),
    }
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
                (None, IdPromoteMode::None) => column.data_type,
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
