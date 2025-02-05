// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`BatchGetView`](crate::operation::batch_get_view::builders::BatchGetViewFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`view_arns(Vec<String>)`](crate::operation::batch_get_view::builders::BatchGetViewFluentBuilder::view_arns) / [`set_view_arns(Option<Vec<String>>)`](crate::operation::batch_get_view::builders::BatchGetViewFluentBuilder::set_view_arns): <p>A list of <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon resource names (ARNs)</a> that identify the views you want details for.</p>
    /// - On success, responds with [`BatchGetViewOutput`](crate::operation::batch_get_view::BatchGetViewOutput) with field(s):
    ///   - [`views(Option<Vec<View>>)`](crate::operation::batch_get_view::BatchGetViewOutput::views): <p>A structure with a list of objects with details for each of the specified views.</p>
    ///   - [`errors(Option<Vec<BatchGetViewError>>)`](crate::operation::batch_get_view::BatchGetViewOutput::errors): <p>If any of the specified ARNs result in an error, then this structure describes the error.</p>
    /// - On failure, responds with [`SdkError<BatchGetViewError>`](crate::operation::batch_get_view::BatchGetViewError)
    pub fn batch_get_view(&self) -> crate::operation::batch_get_view::builders::BatchGetViewFluentBuilder {
        crate::operation::batch_get_view::builders::BatchGetViewFluentBuilder::new(self.handle.clone())
    }
}
