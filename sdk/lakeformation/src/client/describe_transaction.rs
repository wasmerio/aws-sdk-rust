// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DescribeTransaction`](crate::operation::describe_transaction::builders::DescribeTransactionFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`transaction_id(impl ::std::convert::Into<String>)`](crate::operation::describe_transaction::builders::DescribeTransactionFluentBuilder::transaction_id) / [`set_transaction_id(Option<String>)`](crate::operation::describe_transaction::builders::DescribeTransactionFluentBuilder::set_transaction_id): <p>The transaction for which to return status.</p>
    /// - On success, responds with [`DescribeTransactionOutput`](crate::operation::describe_transaction::DescribeTransactionOutput) with field(s):
    ///   - [`transaction_description(Option<TransactionDescription>)`](crate::operation::describe_transaction::DescribeTransactionOutput::transaction_description): <p>Returns a <code>TransactionDescription</code> object containing information about the transaction.</p>
    /// - On failure, responds with [`SdkError<DescribeTransactionError>`](crate::operation::describe_transaction::DescribeTransactionError)
    pub fn describe_transaction(&self) -> crate::operation::describe_transaction::builders::DescribeTransactionFluentBuilder {
        crate::operation::describe_transaction::builders::DescribeTransactionFluentBuilder::new(self.handle.clone())
    }
}
