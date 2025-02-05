// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`UpdateExport`](crate::operation::update_export::builders::UpdateExportFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`export_id(impl ::std::convert::Into<String>)`](crate::operation::update_export::builders::UpdateExportFluentBuilder::export_id) / [`set_export_id(Option<String>)`](crate::operation::update_export::builders::UpdateExportFluentBuilder::set_export_id): <p>The unique identifier Amazon Lex assigned to the export.</p>
    ///   - [`file_password(impl ::std::convert::Into<String>)`](crate::operation::update_export::builders::UpdateExportFluentBuilder::file_password) / [`set_file_password(Option<String>)`](crate::operation::update_export::builders::UpdateExportFluentBuilder::set_file_password): <p>The new password to use to encrypt the export zip archive.</p>
    /// - On success, responds with [`UpdateExportOutput`](crate::operation::update_export::UpdateExportOutput) with field(s):
    ///   - [`export_id(Option<String>)`](crate::operation::update_export::UpdateExportOutput::export_id): <p>The unique identifier Amazon Lex assigned to the export.</p>
    ///   - [`resource_specification(Option<ExportResourceSpecification>)`](crate::operation::update_export::UpdateExportOutput::resource_specification): <p>A description of the type of resource that was exported, either a bot or a bot locale.</p>
    ///   - [`file_format(Option<ImportExportFileFormat>)`](crate::operation::update_export::UpdateExportOutput::file_format): <p>The file format used for the files that define the resource. The <code>TSV</code> format is required to export a custom vocabulary only; otherwise use <code>LexJson</code> format.</p>
    ///   - [`export_status(Option<ExportStatus>)`](crate::operation::update_export::UpdateExportOutput::export_status): <p>The status of the export. When the status is <code>Completed</code> the export archive is available for download.</p>
    ///   - [`creation_date_time(Option<DateTime>)`](crate::operation::update_export::UpdateExportOutput::creation_date_time): <p>The date and time that the export was created.</p>
    ///   - [`last_updated_date_time(Option<DateTime>)`](crate::operation::update_export::UpdateExportOutput::last_updated_date_time): <p>The date and time that the export was last updated.</p>
    /// - On failure, responds with [`SdkError<UpdateExportError>`](crate::operation::update_export::UpdateExportError)
    pub fn update_export(&self) -> crate::operation::update_export::builders::UpdateExportFluentBuilder {
        crate::operation::update_export::builders::UpdateExportFluentBuilder::new(self.handle.clone())
    }
}
