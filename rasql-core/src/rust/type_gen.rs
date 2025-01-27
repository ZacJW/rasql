use thiserror::Error;

use crate::rust::TableStructField;

use super::TableStruct;

pub trait TypeGenerator<Traits: rasql_traits::DbTraits> {
    fn sql_datatype_to_rust_type(
        &self,
        datatype: &sqlparser::ast::DataType,
    ) -> Result<syn::Type, UnsupportedDataType>;

    fn generate_table_struct(&self, table_struct: &TableStruct) -> proc_macro2::TokenStream;
}

#[derive(Debug, Error)]
#[error("Type generator does not support the following SQL datatype: {0}")]
pub struct UnsupportedDataType(pub sqlparser::ast::DataType);

#[cfg(feature = "tokio-postgres")]
pub struct TokioPostgresGenerator {
    pub use_rust_decimal: UseRustDecimal,
    pub use_uuid: UseUuid,
}

#[cfg(feature = "tokio-postgres")]
pub enum UseRustDecimal {
    DontUse,
    Version1,
}

#[cfg(feature = "tokio-postgres")]
pub enum UseUuid {
    DontUse,
    Version0_8,
    Version1,
}

#[cfg(feature = "tokio-postgres")]
impl TokioPostgresGenerator {
    pub(super) fn generate_execute(
        client: &syn::Expr,
        prepared_statement: &syn::Expr,
        parameters: &[&syn::Expr],
    ) -> proc_macro2::TokenStream {
        quote::quote!(#client.execute(#prepared_statement, &[#(#parameters,)*]).await)
    }
}

#[cfg(feature = "tokio-postgres")]
impl TypeGenerator<rasql_traits::PostgresTypesTraits> for TokioPostgresGenerator {
    fn sql_datatype_to_rust_type(
        &self,
        datatype: &sqlparser::ast::DataType,
    ) -> Result<syn::Type, UnsupportedDataType> {
        Ok(match datatype {
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
            | sqlparser::ast::DataType::FixedString(_) => {
                syn::Type::Verbatim(quote::quote! {String})
            }
            sqlparser::ast::DataType::Uuid
                if matches!(self.use_uuid, UseUuid::Version0_8 | UseUuid::Version1) =>
            {
                syn::Type::Verbatim(quote::quote! {uuid::Uuid})
            }
            sqlparser::ast::DataType::Varbinary(_)
            | sqlparser::ast::DataType::Blob(_)
            | sqlparser::ast::DataType::TinyBlob
            | sqlparser::ast::DataType::MediumBlob
            | sqlparser::ast::DataType::LongBlob
            | sqlparser::ast::DataType::Bytes(_)
            | sqlparser::ast::DataType::Bytea
            | sqlparser::ast::DataType::Binary(_) => syn::Type::Verbatim(quote::quote! {Vec<u8>}),
            sqlparser::ast::DataType::Numeric(..)
            | sqlparser::ast::DataType::Decimal(..)
            | sqlparser::ast::DataType::Dec(..)
                if matches!(self.use_rust_decimal, UseRustDecimal::Version1) =>
            {
                syn::Type::Verbatim(quote::quote! {rust_decimal::Decimal})
            }
            sqlparser::ast::DataType::Int2(_) => syn::Type::Verbatim(quote::quote! {i16}),
            sqlparser::ast::DataType::UnsignedInt2(_) => syn::Type::Verbatim(quote::quote! {u16}),
            sqlparser::ast::DataType::Int16 => todo!(),
            sqlparser::ast::DataType::Int128 => todo!(),
            sqlparser::ast::DataType::Int256 => todo!(),
            sqlparser::ast::DataType::Int(_)
            | sqlparser::ast::DataType::Int32
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
            sqlparser::ast::DataType::Float(_)
            | sqlparser::ast::DataType::Float4
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
            sqlparser::ast::DataType::Bit(_) => todo!(),
            sqlparser::ast::DataType::BitVarying(_) => todo!(),
            sqlparser::ast::DataType::Custom(object_name, vec) => todo!(),
            sqlparser::ast::DataType::Array(array_elem_type_def) => match array_elem_type_def {
                sqlparser::ast::ArrayElemTypeDef::None => {
                    return Err(UnsupportedDataType(datatype.clone()))
                }
                sqlparser::ast::ArrayElemTypeDef::AngleBracket(data_type)
                | sqlparser::ast::ArrayElemTypeDef::SquareBracket(data_type, _)
                | sqlparser::ast::ArrayElemTypeDef::Parenthesis(data_type) => {
                    let inner_type = self.sql_datatype_to_rust_type(&datatype)?;
                    syn::Type::Verbatim(quote::quote! {Vec<#inner_type>})
                }
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
            sqlparser::ast::DataType::Trigger => todo!(),
            _ => return Err(UnsupportedDataType(datatype.clone())),
        })
    }

    fn generate_table_struct(&self, table_struct: &TableStruct) -> proc_macro2::TokenStream {
        let TableStruct {
            name,
            fields,
            db_alias,
        } = table_struct;
        let db_alias = db_alias
            .as_deref()
            .map(|db_alias| quote::quote!(#[postgres(name = #db_alias)]));

        let fields = fields.iter().map(
            |TableStructField {
                 name,
                 r#type,
                 db_alias,
             }| {
                let db_alias = db_alias
                    .as_deref()
                    .map(|db_alias| quote::quote!(#[postgres(name = #db_alias)]));
                quote::quote!(#db_alias #name : #r#type)
            },
        );
        quote::quote!(
            #[derive(ToSql, FromSql)]
            #db_alias
            struct #name {
                #(#fields,)*
            }
        )
    }
}
