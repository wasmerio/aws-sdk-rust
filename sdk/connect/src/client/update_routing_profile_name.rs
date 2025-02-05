// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`UpdateRoutingProfileName`](crate::operation::update_routing_profile_name::builders::UpdateRoutingProfileNameFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`instance_id(impl ::std::convert::Into<String>)`](crate::operation::update_routing_profile_name::builders::UpdateRoutingProfileNameFluentBuilder::instance_id) / [`set_instance_id(Option<String>)`](crate::operation::update_routing_profile_name::builders::UpdateRoutingProfileNameFluentBuilder::set_instance_id): <p>The identifier of the Amazon Connect instance. You can <a href="https://docs.aws.amazon.com/connect/latest/adminguide/find-instance-arn.html">find the instance ID</a> in the Amazon Resource Name (ARN) of the instance.</p>
    ///   - [`routing_profile_id(impl ::std::convert::Into<String>)`](crate::operation::update_routing_profile_name::builders::UpdateRoutingProfileNameFluentBuilder::routing_profile_id) / [`set_routing_profile_id(Option<String>)`](crate::operation::update_routing_profile_name::builders::UpdateRoutingProfileNameFluentBuilder::set_routing_profile_id): <p>The identifier of the routing profile.</p>
    ///   - [`name(impl ::std::convert::Into<String>)`](crate::operation::update_routing_profile_name::builders::UpdateRoutingProfileNameFluentBuilder::name) / [`set_name(Option<String>)`](crate::operation::update_routing_profile_name::builders::UpdateRoutingProfileNameFluentBuilder::set_name): <p>The name of the routing profile. Must not be more than 127 characters.</p>
    ///   - [`description(impl ::std::convert::Into<String>)`](crate::operation::update_routing_profile_name::builders::UpdateRoutingProfileNameFluentBuilder::description) / [`set_description(Option<String>)`](crate::operation::update_routing_profile_name::builders::UpdateRoutingProfileNameFluentBuilder::set_description): <p>The description of the routing profile. Must not be more than 250 characters.</p>
    /// - On success, responds with [`UpdateRoutingProfileNameOutput`](crate::operation::update_routing_profile_name::UpdateRoutingProfileNameOutput)
    /// - On failure, responds with [`SdkError<UpdateRoutingProfileNameError>`](crate::operation::update_routing_profile_name::UpdateRoutingProfileNameError)
    pub fn update_routing_profile_name(&self) -> crate::operation::update_routing_profile_name::builders::UpdateRoutingProfileNameFluentBuilder {
        crate::operation::update_routing_profile_name::builders::UpdateRoutingProfileNameFluentBuilder::new(self.handle.clone())
    }
}
