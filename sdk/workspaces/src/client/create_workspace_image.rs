// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`CreateWorkspaceImage`](crate::operation::create_workspace_image::builders::CreateWorkspaceImageFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`name(impl ::std::convert::Into<String>)`](crate::operation::create_workspace_image::builders::CreateWorkspaceImageFluentBuilder::name) / [`set_name(Option<String>)`](crate::operation::create_workspace_image::builders::CreateWorkspaceImageFluentBuilder::set_name): <p>The name of the new WorkSpace image.</p>
    ///   - [`description(impl ::std::convert::Into<String>)`](crate::operation::create_workspace_image::builders::CreateWorkspaceImageFluentBuilder::description) / [`set_description(Option<String>)`](crate::operation::create_workspace_image::builders::CreateWorkspaceImageFluentBuilder::set_description): <p>The description of the new WorkSpace image.</p>
    ///   - [`workspace_id(impl ::std::convert::Into<String>)`](crate::operation::create_workspace_image::builders::CreateWorkspaceImageFluentBuilder::workspace_id) / [`set_workspace_id(Option<String>)`](crate::operation::create_workspace_image::builders::CreateWorkspaceImageFluentBuilder::set_workspace_id): <p>The identifier of the source WorkSpace</p>
    ///   - [`tags(Vec<Tag>)`](crate::operation::create_workspace_image::builders::CreateWorkspaceImageFluentBuilder::tags) / [`set_tags(Option<Vec<Tag>>)`](crate::operation::create_workspace_image::builders::CreateWorkspaceImageFluentBuilder::set_tags): <p>The tags that you want to add to the new WorkSpace image. To add tags when you're creating the image, you must create an IAM policy that grants your IAM user permission to use <code>workspaces:CreateTags</code>.</p>
    /// - On success, responds with [`CreateWorkspaceImageOutput`](crate::operation::create_workspace_image::CreateWorkspaceImageOutput) with field(s):
    ///   - [`image_id(Option<String>)`](crate::operation::create_workspace_image::CreateWorkspaceImageOutput::image_id): <p>The identifier of the new WorkSpace image.</p>
    ///   - [`name(Option<String>)`](crate::operation::create_workspace_image::CreateWorkspaceImageOutput::name): <p>The name of the image.</p>
    ///   - [`description(Option<String>)`](crate::operation::create_workspace_image::CreateWorkspaceImageOutput::description): <p>The description of the image.</p>
    ///   - [`operating_system(Option<OperatingSystem>)`](crate::operation::create_workspace_image::CreateWorkspaceImageOutput::operating_system): <p>The operating system that the image is running.</p>
    ///   - [`state(Option<WorkspaceImageState>)`](crate::operation::create_workspace_image::CreateWorkspaceImageOutput::state): <p>The availability status of the image.</p>
    ///   - [`required_tenancy(Option<WorkspaceImageRequiredTenancy>)`](crate::operation::create_workspace_image::CreateWorkspaceImageOutput::required_tenancy): <p>Specifies whether the image is running on dedicated hardware. When Bring Your Own License (BYOL) is enabled, this value is set to DEDICATED. For more information, see <a href="https://docs.aws.amazon.com/workspaces/latest/adminguide/byol-windows-images.htm"> Bring Your Own Windows Desktop Images.</a>.</p>
    ///   - [`created(Option<DateTime>)`](crate::operation::create_workspace_image::CreateWorkspaceImageOutput::created): <p>The date when the image was created.</p>
    ///   - [`owner_account_id(Option<String>)`](crate::operation::create_workspace_image::CreateWorkspaceImageOutput::owner_account_id): <p>The identifier of the Amazon Web Services account that owns the image.</p>
    /// - On failure, responds with [`SdkError<CreateWorkspaceImageError>`](crate::operation::create_workspace_image::CreateWorkspaceImageError)
    pub fn create_workspace_image(&self) -> crate::operation::create_workspace_image::builders::CreateWorkspaceImageFluentBuilder {
        crate::operation::create_workspace_image::builders::CreateWorkspaceImageFluentBuilder::new(self.handle.clone())
    }
}
