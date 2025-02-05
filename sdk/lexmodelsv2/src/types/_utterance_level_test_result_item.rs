// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Contains information about multiple utterances in the results of a test set execution.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct UtteranceLevelTestResultItem {
    /// <p>The record number of the result.</p>
    pub record_number: ::std::option::Option<i64>,
    /// <p>The unique identifier for the conversation associated with the result.</p>
    pub conversation_id: ::std::option::Option<::std::string::String>,
    /// <p>Contains information about the turn associated with the result.</p>
    pub turn_result: ::std::option::Option<crate::types::TestSetTurnResult>,
}
impl UtteranceLevelTestResultItem {
    /// <p>The record number of the result.</p>
    pub fn record_number(&self) -> ::std::option::Option<i64> {
        self.record_number
    }
    /// <p>The unique identifier for the conversation associated with the result.</p>
    pub fn conversation_id(&self) -> ::std::option::Option<&str> {
        self.conversation_id.as_deref()
    }
    /// <p>Contains information about the turn associated with the result.</p>
    pub fn turn_result(&self) -> ::std::option::Option<&crate::types::TestSetTurnResult> {
        self.turn_result.as_ref()
    }
}
impl UtteranceLevelTestResultItem {
    /// Creates a new builder-style object to manufacture [`UtteranceLevelTestResultItem`](crate::types::UtteranceLevelTestResultItem).
    pub fn builder() -> crate::types::builders::UtteranceLevelTestResultItemBuilder {
        crate::types::builders::UtteranceLevelTestResultItemBuilder::default()
    }
}

/// A builder for [`UtteranceLevelTestResultItem`](crate::types::UtteranceLevelTestResultItem).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct UtteranceLevelTestResultItemBuilder {
    pub(crate) record_number: ::std::option::Option<i64>,
    pub(crate) conversation_id: ::std::option::Option<::std::string::String>,
    pub(crate) turn_result: ::std::option::Option<crate::types::TestSetTurnResult>,
}
impl UtteranceLevelTestResultItemBuilder {
    /// <p>The record number of the result.</p>
    pub fn record_number(mut self, input: i64) -> Self {
        self.record_number = ::std::option::Option::Some(input);
        self
    }
    /// <p>The record number of the result.</p>
    pub fn set_record_number(mut self, input: ::std::option::Option<i64>) -> Self {
        self.record_number = input;
        self
    }
    /// <p>The record number of the result.</p>
    pub fn get_record_number(&self) -> &::std::option::Option<i64> {
        &self.record_number
    }
    /// <p>The unique identifier for the conversation associated with the result.</p>
    pub fn conversation_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.conversation_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The unique identifier for the conversation associated with the result.</p>
    pub fn set_conversation_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.conversation_id = input;
        self
    }
    /// <p>The unique identifier for the conversation associated with the result.</p>
    pub fn get_conversation_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.conversation_id
    }
    /// <p>Contains information about the turn associated with the result.</p>
    pub fn turn_result(mut self, input: crate::types::TestSetTurnResult) -> Self {
        self.turn_result = ::std::option::Option::Some(input);
        self
    }
    /// <p>Contains information about the turn associated with the result.</p>
    pub fn set_turn_result(mut self, input: ::std::option::Option<crate::types::TestSetTurnResult>) -> Self {
        self.turn_result = input;
        self
    }
    /// <p>Contains information about the turn associated with the result.</p>
    pub fn get_turn_result(&self) -> &::std::option::Option<crate::types::TestSetTurnResult> {
        &self.turn_result
    }
    /// Consumes the builder and constructs a [`UtteranceLevelTestResultItem`](crate::types::UtteranceLevelTestResultItem).
    pub fn build(self) -> crate::types::UtteranceLevelTestResultItem {
        crate::types::UtteranceLevelTestResultItem {
            record_number: self.record_number,
            conversation_id: self.conversation_id,
            turn_result: self.turn_result,
        }
    }
}
