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
        todo!()
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
