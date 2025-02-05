// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteProject`](crate::operation::delete_project::builders::DeleteProjectFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`project(impl ::std::convert::Into<String>)`](crate::operation::delete_project::builders::DeleteProjectFluentBuilder::project) / [`set_project(Option<String>)`](crate::operation::delete_project::builders::DeleteProjectFluentBuilder::set_project): <p>The name or ARN of the project to delete.</p>
    /// - On success, responds with [`DeleteProjectOutput`](crate::operation::delete_project::DeleteProjectOutput)
    /// - On failure, responds with [`SdkError<DeleteProjectError>`](crate::operation::delete_project::DeleteProjectError)
    pub fn delete_project(&self) -> crate::operation::delete_project::builders::DeleteProjectFluentBuilder {
        crate::operation::delete_project::builders::DeleteProjectFluentBuilder::new(self.handle.clone())
    }
}
