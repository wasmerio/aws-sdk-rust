// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`PutResourceSet`](crate::operation::put_resource_set::builders::PutResourceSetFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`resource_set(ResourceSet)`](crate::operation::put_resource_set::builders::PutResourceSetFluentBuilder::resource_set) / [`set_resource_set(Option<ResourceSet>)`](crate::operation::put_resource_set::builders::PutResourceSetFluentBuilder::set_resource_set): <p>Details about the resource set to be created or updated.&gt;</p>
    ///   - [`tag_list(Vec<Tag>)`](crate::operation::put_resource_set::builders::PutResourceSetFluentBuilder::tag_list) / [`set_tag_list(Option<Vec<Tag>>)`](crate::operation::put_resource_set::builders::PutResourceSetFluentBuilder::set_tag_list): <p>Retrieves the tags associated with the specified resource set. Tags are key:value pairs that you can use to categorize and manage your resources, for purposes like billing. For example, you might set the tag key to "customer" and the value to the customer name or ID. You can specify one or more tags to add to each Amazon Web Services resource, up to 50 tags for a resource.</p>
    /// - On success, responds with [`PutResourceSetOutput`](crate::operation::put_resource_set::PutResourceSetOutput) with field(s):
    ///   - [`resource_set(Option<ResourceSet>)`](crate::operation::put_resource_set::PutResourceSetOutput::resource_set): <p>Details about the resource set.</p>
    ///   - [`resource_set_arn(Option<String>)`](crate::operation::put_resource_set::PutResourceSetOutput::resource_set_arn): <p>The Amazon Resource Name (ARN) of the resource set.</p>
    /// - On failure, responds with [`SdkError<PutResourceSetError>`](crate::operation::put_resource_set::PutResourceSetError)
    pub fn put_resource_set(&self) -> crate::operation::put_resource_set::builders::PutResourceSetFluentBuilder {
        crate::operation::put_resource_set::builders::PutResourceSetFluentBuilder::new(self.handle.clone())
    }
}
