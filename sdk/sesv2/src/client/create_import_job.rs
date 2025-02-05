// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`CreateImportJob`](crate::operation::create_import_job::builders::CreateImportJobFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`import_destination(ImportDestination)`](crate::operation::create_import_job::builders::CreateImportJobFluentBuilder::import_destination) / [`set_import_destination(Option<ImportDestination>)`](crate::operation::create_import_job::builders::CreateImportJobFluentBuilder::set_import_destination): <p>The destination for the import job.</p>
    ///   - [`import_data_source(ImportDataSource)`](crate::operation::create_import_job::builders::CreateImportJobFluentBuilder::import_data_source) / [`set_import_data_source(Option<ImportDataSource>)`](crate::operation::create_import_job::builders::CreateImportJobFluentBuilder::set_import_data_source): <p>The data source for the import job.</p>
    /// - On success, responds with [`CreateImportJobOutput`](crate::operation::create_import_job::CreateImportJobOutput) with field(s):
    ///   - [`job_id(Option<String>)`](crate::operation::create_import_job::CreateImportJobOutput::job_id): <p>A string that represents the import job ID.</p>
    /// - On failure, responds with [`SdkError<CreateImportJobError>`](crate::operation::create_import_job::CreateImportJobError)
    pub fn create_import_job(&self) -> crate::operation::create_import_job::builders::CreateImportJobFluentBuilder {
        crate::operation::create_import_job::builders::CreateImportJobFluentBuilder::new(self.handle.clone())
    }
}
