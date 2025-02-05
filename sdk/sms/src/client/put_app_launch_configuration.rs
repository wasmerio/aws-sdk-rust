// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`PutAppLaunchConfiguration`](crate::operation::put_app_launch_configuration::builders::PutAppLaunchConfigurationFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`app_id(impl ::std::convert::Into<String>)`](crate::operation::put_app_launch_configuration::builders::PutAppLaunchConfigurationFluentBuilder::app_id) / [`set_app_id(Option<String>)`](crate::operation::put_app_launch_configuration::builders::PutAppLaunchConfigurationFluentBuilder::set_app_id): <p>The ID of the application.</p>
    ///   - [`role_name(impl ::std::convert::Into<String>)`](crate::operation::put_app_launch_configuration::builders::PutAppLaunchConfigurationFluentBuilder::role_name) / [`set_role_name(Option<String>)`](crate::operation::put_app_launch_configuration::builders::PutAppLaunchConfigurationFluentBuilder::set_role_name): <p>The name of service role in the customer's account that CloudFormation uses to launch the application.</p>
    ///   - [`auto_launch(bool)`](crate::operation::put_app_launch_configuration::builders::PutAppLaunchConfigurationFluentBuilder::auto_launch) / [`set_auto_launch(Option<bool>)`](crate::operation::put_app_launch_configuration::builders::PutAppLaunchConfigurationFluentBuilder::set_auto_launch): <p>Indicates whether the application is configured to launch automatically after replication is complete.</p>
    ///   - [`server_group_launch_configurations(Vec<ServerGroupLaunchConfiguration>)`](crate::operation::put_app_launch_configuration::builders::PutAppLaunchConfigurationFluentBuilder::server_group_launch_configurations) / [`set_server_group_launch_configurations(Option<Vec<ServerGroupLaunchConfiguration>>)`](crate::operation::put_app_launch_configuration::builders::PutAppLaunchConfigurationFluentBuilder::set_server_group_launch_configurations): <p>Information about the launch configurations for server groups in the application.</p>
    /// - On success, responds with [`PutAppLaunchConfigurationOutput`](crate::operation::put_app_launch_configuration::PutAppLaunchConfigurationOutput)
    /// - On failure, responds with [`SdkError<PutAppLaunchConfigurationError>`](crate::operation::put_app_launch_configuration::PutAppLaunchConfigurationError)
    pub fn put_app_launch_configuration(&self) -> crate::operation::put_app_launch_configuration::builders::PutAppLaunchConfigurationFluentBuilder {
        crate::operation::put_app_launch_configuration::builders::PutAppLaunchConfigurationFluentBuilder::new(self.handle.clone())
    }
}
