// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListThemeAliases`](crate::operation::list_theme_aliases::builders::ListThemeAliasesFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`aws_account_id(impl ::std::convert::Into<String>)`](crate::operation::list_theme_aliases::builders::ListThemeAliasesFluentBuilder::aws_account_id) / [`set_aws_account_id(Option<String>)`](crate::operation::list_theme_aliases::builders::ListThemeAliasesFluentBuilder::set_aws_account_id): <p>The ID of the Amazon Web Services account that contains the theme aliases that you're listing.</p>
    ///   - [`theme_id(impl ::std::convert::Into<String>)`](crate::operation::list_theme_aliases::builders::ListThemeAliasesFluentBuilder::theme_id) / [`set_theme_id(Option<String>)`](crate::operation::list_theme_aliases::builders::ListThemeAliasesFluentBuilder::set_theme_id): <p>The ID for the theme.</p>
    ///   - [`next_token(impl ::std::convert::Into<String>)`](crate::operation::list_theme_aliases::builders::ListThemeAliasesFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_theme_aliases::builders::ListThemeAliasesFluentBuilder::set_next_token): <p>The token for the next set of results, or null if there are no more results.</p>
    ///   - [`max_results(i32)`](crate::operation::list_theme_aliases::builders::ListThemeAliasesFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::list_theme_aliases::builders::ListThemeAliasesFluentBuilder::set_max_results): <p>The maximum number of results to be returned per request.</p>
    /// - On success, responds with [`ListThemeAliasesOutput`](crate::operation::list_theme_aliases::ListThemeAliasesOutput) with field(s):
    ///   - [`theme_alias_list(Option<Vec<ThemeAlias>>)`](crate::operation::list_theme_aliases::ListThemeAliasesOutput::theme_alias_list): <p>A structure containing the list of the theme's aliases.</p>
    ///   - [`status(i32)`](crate::operation::list_theme_aliases::ListThemeAliasesOutput::status): <p>The HTTP status of the request.</p>
    ///   - [`request_id(Option<String>)`](crate::operation::list_theme_aliases::ListThemeAliasesOutput::request_id): <p>The Amazon Web Services request ID for this operation.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::list_theme_aliases::ListThemeAliasesOutput::next_token): <p>The token for the next set of results, or null if there are no more results.</p>
    /// - On failure, responds with [`SdkError<ListThemeAliasesError>`](crate::operation::list_theme_aliases::ListThemeAliasesError)
    pub fn list_theme_aliases(&self) -> crate::operation::list_theme_aliases::builders::ListThemeAliasesFluentBuilder {
        crate::operation::list_theme_aliases::builders::ListThemeAliasesFluentBuilder::new(self.handle.clone())
    }
}
