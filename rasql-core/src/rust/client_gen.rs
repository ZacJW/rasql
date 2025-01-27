
pub trait AsyncClientCodeGenerator<Client: rasql_traits::r#async::Client> {
    /// Create a token stream for usage of `client` to prepare `statement_str` for
    /// later execution, evaluating to a value of type
    /// `Result<Client::PreparedStatement, Client::PrepareError>`
    ///
    /// - `client` is an expr of type `&Client`
    /// - `statement_str` is an expr of type `&str`
    fn generate_prepare_statement(
        client: &syn::Expr,
        statement_str: &syn::Expr,
    ) -> proc_macro2::TokenStream;

    /// Create a token stream for usage of `client` to execute the `prepared_statement`
    /// with the provided `parameters`, evaluating to a value of type
    /// `Result<Client::Rows, Client::QueryError>`.
    ///
    /// - `client` is an expr of type `&Client`
    /// - `prepared_statement` is an expr of type `&Client::PreparedStatement`
    /// - exprs in `parameters` are references of types that can be assumed to be compatible with the client
    fn generate_query_many_with_statement(
        client: &syn::Expr,
        prepared_statement: &syn::Expr,
        parameters: &[&syn::Expr],
    ) -> proc_macro2::TokenStream;

    /// Create a token stream for usage of `client` to execute the `prepared_statement`
    /// with the provided `parameters`, evaluating to a value of type
    /// `Result<Option<Client::Row>, Client::QueryError>`.
    ///
    /// - `client` is an expr of type `&Client`
    /// - `prepared_statement` is an expr of type `&Client::PreparedStatement`
    /// - exprs in `parameters` are references of types that can be assumed to be compatible with the client
    fn generate_query_one_or_none_with_statement(
        client: &syn::Expr,
        prepared_statement: &syn::Expr,
        parameters: &[&syn::Expr],
    ) -> proc_macro2::TokenStream;

    /// Create a token stream for usage of `row` to read a column, evaluating to a value of type
    /// `Result<T, Client::RowReadColumnError>` where `T` is any type compatible with the database client.
    ///
    /// - `row` is an expr of type `&Client::Row`
    /// - `column_name` is an expr of type `&str`
    fn generate_row_read_column(
        row: &syn::Expr,
        column_name: &syn::Expr,
    ) -> proc_macro2::TokenStream;

    /// Create a token stream for usage of `client` to execute the `prepared_statement`
    /// with the provided `parameters`, evaluating to a value of type
    /// `Result<Client::InsertOutcome, Client::InsertError>`.
    ///
    /// - `client` is an expr of type `&Client`
    /// - `prepared_statement` is an expr of type `&Client::PreparedStatement`
    /// - exprs in `parameters` are references of types that can be assumed to be compatible with the client
    fn generate_insert_with_statement(
        client: &syn::Expr,
        prepared_statement: &syn::Expr,
        parameters: &[&syn::Expr],
    ) -> proc_macro2::TokenStream;

    /// Create a token stream for usage of `client` to execute the `prepared_statement`
    /// with the provided `parameters`, evaluating to a value of type
    /// `Result<Client::UpdateOutcome, Client::UpdateError>`.
    ///
    /// - `client` is an expr of type `&Client`
    /// - `prepared_statement` is an expr of type `&Client::PreparedStatement`
    /// - exprs in `parameters` are references of types that can be assumed to be compatible with the client
    fn generate_update_with_statement(
        client: &syn::Expr,
        prepared_statement: &syn::Expr,
        parameters: &[&syn::Expr],
    ) -> proc_macro2::TokenStream;

    /// Create a token stream for usage of `client` to execute the `prepared_statement`
    /// with the provided `parameters`, evaluating to a value of type
    /// `Result<Client::DeleteOutcome, Client::DeleteError>`.
    ///
    /// - `client` is an expr of type `&Client`
    /// - `prepared_statement` is an expr of type `&Client::PreparedStatement`
    /// - exprs in `parameters` are references of types that can be assumed to be compatible with the client
    fn generate_delete_with_statement(
        client: &syn::Expr,
        prepared_statement: &syn::Expr,
        parameters: &[&syn::Expr],
    ) -> proc_macro2::TokenStream;
}

#[cfg(feature = "tokio-postgres")]
impl AsyncClientCodeGenerator<tokio_postgres::Client> for super::type_gen::TokioPostgresGenerator {
    fn generate_prepare_statement(
        client: &syn::Expr,
        statement_str: &syn::Expr,
    ) -> proc_macro2::TokenStream {
        quote::quote!(#client.prepare(#statement_str).await)
    }

    fn generate_query_many_with_statement(
        client: &syn::Expr,
        prepared_statement: &syn::Expr,
        parameters: &[&syn::Expr],
    ) -> proc_macro2::TokenStream {
        quote::quote!(#client.query(#prepared_statement, &[#(#parameters,)*]).await)
    }

    fn generate_query_one_or_none_with_statement(
        client: &syn::Expr,
        prepared_statement: &syn::Expr,
        parameters: &[&syn::Expr],
    ) -> proc_macro2::TokenStream {
        quote::quote!(#client.query_opt(#prepared_statement, &[#(#parameters,)*]).await)
    }

    fn generate_row_read_column(
        row: &syn::Expr,
        column_name: &syn::Expr,
    ) -> proc_macro2::TokenStream {
        quote::quote!(#row.try_get(#column_name))
    }

    fn generate_insert_with_statement(
        client: &syn::Expr,
        prepared_statement: &syn::Expr,
        parameters: &[&syn::Expr],
    ) -> proc_macro2::TokenStream {
        Self::generate_execute(client, prepared_statement, parameters)
    }

    fn generate_update_with_statement(
        client: &syn::Expr,
        prepared_statement: &syn::Expr,
        parameters: &[&syn::Expr],
    ) -> proc_macro2::TokenStream {
        Self::generate_execute(client, prepared_statement, parameters)
    }

    fn generate_delete_with_statement(
        client: &syn::Expr,
        prepared_statement: &syn::Expr,
        parameters: &[&syn::Expr],
    ) -> proc_macro2::TokenStream {
        Self::generate_execute(client, prepared_statement, parameters)
    }
}
