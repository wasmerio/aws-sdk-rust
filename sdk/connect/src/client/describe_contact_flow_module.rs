// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DescribeContactFlowModule`](crate::operation::describe_contact_flow_module::builders::DescribeContactFlowModuleFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`instance_id(impl ::std::convert::Into<String>)`](crate::operation::describe_contact_flow_module::builders::DescribeContactFlowModuleFluentBuilder::instance_id) / [`set_instance_id(Option<String>)`](crate::operation::describe_contact_flow_module::builders::DescribeContactFlowModuleFluentBuilder::set_instance_id): <p>The identifier of the Amazon Connect instance. You can <a href="https://docs.aws.amazon.com/connect/latest/adminguide/find-instance-arn.html">find the instance ID</a> in the Amazon Resource Name (ARN) of the instance.</p>
    ///   - [`contact_flow_module_id(impl ::std::convert::Into<String>)`](crate::operation::describe_contact_flow_module::builders::DescribeContactFlowModuleFluentBuilder::contact_flow_module_id) / [`set_contact_flow_module_id(Option<String>)`](crate::operation::describe_contact_flow_module::builders::DescribeContactFlowModuleFluentBuilder::set_contact_flow_module_id): <p>The identifier of the flow module.</p>
    /// - On success, responds with [`DescribeContactFlowModuleOutput`](crate::operation::describe_contact_flow_module::DescribeContactFlowModuleOutput) with field(s):
    ///   - [`contact_flow_module(Option<ContactFlowModule>)`](crate::operation::describe_contact_flow_module::DescribeContactFlowModuleOutput::contact_flow_module): <p>Information about the flow module.</p>
    /// - On failure, responds with [`SdkError<DescribeContactFlowModuleError>`](crate::operation::describe_contact_flow_module::DescribeContactFlowModuleError)
    pub fn describe_contact_flow_module(&self) -> crate::operation::describe_contact_flow_module::builders::DescribeContactFlowModuleFluentBuilder {
        crate::operation::describe_contact_flow_module::builders::DescribeContactFlowModuleFluentBuilder::new(self.handle.clone())
    }
}
