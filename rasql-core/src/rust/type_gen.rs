use crate::rust::TableStructField;

use super::TableStruct;

pub trait TypeGenerator<Traits: rasql_traits::DbTraits> {
    fn sql_datatype_to_rust_type(datatype: &sqlparser::ast::DataType) -> syn::Type;

    fn generate_table_struct(table_struct: &TableStruct) -> proc_macro2::TokenStream;
}



#[cfg(feature = "tokio-postgres")]
pub struct TokioPostgresGenerator;

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
    
    fn generate_table_struct(table_struct: &TableStruct) -> proc_macro2::TokenStream {
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
