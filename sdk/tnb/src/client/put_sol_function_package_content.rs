// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`PutSolFunctionPackageContent`](crate::operation::put_sol_function_package_content::builders::PutSolFunctionPackageContentFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`vnf_pkg_id(impl ::std::convert::Into<String>)`](crate::operation::put_sol_function_package_content::builders::PutSolFunctionPackageContentFluentBuilder::vnf_pkg_id) / [`set_vnf_pkg_id(Option<String>)`](crate::operation::put_sol_function_package_content::builders::PutSolFunctionPackageContentFluentBuilder::set_vnf_pkg_id): <p>Function package ID.</p>
    ///   - [`content_type(PackageContentType)`](crate::operation::put_sol_function_package_content::builders::PutSolFunctionPackageContentFluentBuilder::content_type) / [`set_content_type(Option<PackageContentType>)`](crate::operation::put_sol_function_package_content::builders::PutSolFunctionPackageContentFluentBuilder::set_content_type): <p>Function package content type.</p>
    ///   - [`file(Blob)`](crate::operation::put_sol_function_package_content::builders::PutSolFunctionPackageContentFluentBuilder::file) / [`set_file(Option<Blob>)`](crate::operation::put_sol_function_package_content::builders::PutSolFunctionPackageContentFluentBuilder::set_file): <p>Function package file.</p>
    /// - On success, responds with [`PutSolFunctionPackageContentOutput`](crate::operation::put_sol_function_package_content::PutSolFunctionPackageContentOutput) with field(s):
    ///   - [`id(Option<String>)`](crate::operation::put_sol_function_package_content::PutSolFunctionPackageContentOutput::id): <p>Function package ID.</p>
    ///   - [`vnfd_id(Option<String>)`](crate::operation::put_sol_function_package_content::PutSolFunctionPackageContentOutput::vnfd_id): <p>Function package descriptor ID.</p>
    ///   - [`vnf_product_name(Option<String>)`](crate::operation::put_sol_function_package_content::PutSolFunctionPackageContentOutput::vnf_product_name): <p>Function product name.</p>
    ///   - [`vnf_provider(Option<String>)`](crate::operation::put_sol_function_package_content::PutSolFunctionPackageContentOutput::vnf_provider): <p>Function provider.</p>
    ///   - [`vnfd_version(Option<String>)`](crate::operation::put_sol_function_package_content::PutSolFunctionPackageContentOutput::vnfd_version): <p>Function package descriptor version.</p>
    ///   - [`metadata(Option<PutSolFunctionPackageContentMetadata>)`](crate::operation::put_sol_function_package_content::PutSolFunctionPackageContentOutput::metadata): <p>Function package metadata.</p>
    /// - On failure, responds with [`SdkError<PutSolFunctionPackageContentError>`](crate::operation::put_sol_function_package_content::PutSolFunctionPackageContentError)
    pub fn put_sol_function_package_content(
        &self,
    ) -> crate::operation::put_sol_function_package_content::builders::PutSolFunctionPackageContentFluentBuilder {
        crate::operation::put_sol_function_package_content::builders::PutSolFunctionPackageContentFluentBuilder::new(self.handle.clone())
    }
}
