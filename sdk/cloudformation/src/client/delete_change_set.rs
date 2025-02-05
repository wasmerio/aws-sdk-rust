// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteChangeSet`](crate::operation::delete_change_set::builders::DeleteChangeSetFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`change_set_name(impl ::std::convert::Into<String>)`](crate::operation::delete_change_set::builders::DeleteChangeSetFluentBuilder::change_set_name) / [`set_change_set_name(Option<String>)`](crate::operation::delete_change_set::builders::DeleteChangeSetFluentBuilder::set_change_set_name): <p>The name or Amazon Resource Name (ARN) of the change set that you want to delete.</p>
    ///   - [`stack_name(impl ::std::convert::Into<String>)`](crate::operation::delete_change_set::builders::DeleteChangeSetFluentBuilder::stack_name) / [`set_stack_name(Option<String>)`](crate::operation::delete_change_set::builders::DeleteChangeSetFluentBuilder::set_stack_name): <p>If you specified the name of a change set to delete, specify the stack name or Amazon Resource Name (ARN) that's associated with it.</p>
    /// - On success, responds with [`DeleteChangeSetOutput`](crate::operation::delete_change_set::DeleteChangeSetOutput)
    /// - On failure, responds with [`SdkError<DeleteChangeSetError>`](crate::operation::delete_change_set::DeleteChangeSetError)
    pub fn delete_change_set(&self) -> crate::operation::delete_change_set::builders::DeleteChangeSetFluentBuilder {
        crate::operation::delete_change_set::builders::DeleteChangeSetFluentBuilder::new(self.handle.clone())
    }
}
