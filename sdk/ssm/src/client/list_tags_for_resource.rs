// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListTagsForResource`](crate::operation::list_tags_for_resource::builders::ListTagsForResourceFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`resource_type(ResourceTypeForTagging)`](crate::operation::list_tags_for_resource::builders::ListTagsForResourceFluentBuilder::resource_type) / [`set_resource_type(Option<ResourceTypeForTagging>)`](crate::operation::list_tags_for_resource::builders::ListTagsForResourceFluentBuilder::set_resource_type): <p>Returns a list of tags for a specific resource type.</p>
    ///   - [`resource_id(impl ::std::convert::Into<String>)`](crate::operation::list_tags_for_resource::builders::ListTagsForResourceFluentBuilder::resource_id) / [`set_resource_id(Option<String>)`](crate::operation::list_tags_for_resource::builders::ListTagsForResourceFluentBuilder::set_resource_id): <p>The resource ID for which you want to see a list of tags.</p>
    /// - On success, responds with [`ListTagsForResourceOutput`](crate::operation::list_tags_for_resource::ListTagsForResourceOutput) with field(s):
    ///   - [`tag_list(Option<Vec<Tag>>)`](crate::operation::list_tags_for_resource::ListTagsForResourceOutput::tag_list): <p>A list of tags.</p>
    /// - On failure, responds with [`SdkError<ListTagsForResourceError>`](crate::operation::list_tags_for_resource::ListTagsForResourceError)
    pub fn list_tags_for_resource(&self) -> crate::operation::list_tags_for_resource::builders::ListTagsForResourceFluentBuilder {
        crate::operation::list_tags_for_resource::builders::ListTagsForResourceFluentBuilder::new(self.handle.clone())
    }
}
