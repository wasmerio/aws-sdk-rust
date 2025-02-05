// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`CreateNodeFromTemplateJob`](crate::operation::create_node_from_template_job::builders::CreateNodeFromTemplateJobFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`template_type(TemplateType)`](crate::operation::create_node_from_template_job::builders::CreateNodeFromTemplateJobFluentBuilder::template_type) / [`set_template_type(Option<TemplateType>)`](crate::operation::create_node_from_template_job::builders::CreateNodeFromTemplateJobFluentBuilder::set_template_type): <p>The type of node.</p>
    ///   - [`output_package_name(impl ::std::convert::Into<String>)`](crate::operation::create_node_from_template_job::builders::CreateNodeFromTemplateJobFluentBuilder::output_package_name) / [`set_output_package_name(Option<String>)`](crate::operation::create_node_from_template_job::builders::CreateNodeFromTemplateJobFluentBuilder::set_output_package_name): <p>An output package name for the node.</p>
    ///   - [`output_package_version(impl ::std::convert::Into<String>)`](crate::operation::create_node_from_template_job::builders::CreateNodeFromTemplateJobFluentBuilder::output_package_version) / [`set_output_package_version(Option<String>)`](crate::operation::create_node_from_template_job::builders::CreateNodeFromTemplateJobFluentBuilder::set_output_package_version): <p>An output package version for the node.</p>
    ///   - [`node_name(impl ::std::convert::Into<String>)`](crate::operation::create_node_from_template_job::builders::CreateNodeFromTemplateJobFluentBuilder::node_name) / [`set_node_name(Option<String>)`](crate::operation::create_node_from_template_job::builders::CreateNodeFromTemplateJobFluentBuilder::set_node_name): <p>A name for the node.</p>
    ///   - [`node_description(impl ::std::convert::Into<String>)`](crate::operation::create_node_from_template_job::builders::CreateNodeFromTemplateJobFluentBuilder::node_description) / [`set_node_description(Option<String>)`](crate::operation::create_node_from_template_job::builders::CreateNodeFromTemplateJobFluentBuilder::set_node_description): <p>A description for the node.</p>
    ///   - [`template_parameters(HashMap<String, String>)`](crate::operation::create_node_from_template_job::builders::CreateNodeFromTemplateJobFluentBuilder::template_parameters) / [`set_template_parameters(Option<HashMap<String, String>>)`](crate::operation::create_node_from_template_job::builders::CreateNodeFromTemplateJobFluentBuilder::set_template_parameters): <p>Template parameters for the node.</p>
    ///   - [`job_tags(Vec<JobResourceTags>)`](crate::operation::create_node_from_template_job::builders::CreateNodeFromTemplateJobFluentBuilder::job_tags) / [`set_job_tags(Option<Vec<JobResourceTags>>)`](crate::operation::create_node_from_template_job::builders::CreateNodeFromTemplateJobFluentBuilder::set_job_tags): <p>Tags for the job.</p>
    /// - On success, responds with [`CreateNodeFromTemplateJobOutput`](crate::operation::create_node_from_template_job::CreateNodeFromTemplateJobOutput) with field(s):
    ///   - [`job_id(Option<String>)`](crate::operation::create_node_from_template_job::CreateNodeFromTemplateJobOutput::job_id): <p>The job's ID.</p>
    /// - On failure, responds with [`SdkError<CreateNodeFromTemplateJobError>`](crate::operation::create_node_from_template_job::CreateNodeFromTemplateJobError)
    pub fn create_node_from_template_job(&self) -> crate::operation::create_node_from_template_job::builders::CreateNodeFromTemplateJobFluentBuilder {
        crate::operation::create_node_from_template_job::builders::CreateNodeFromTemplateJobFluentBuilder::new(self.handle.clone())
    }
}
