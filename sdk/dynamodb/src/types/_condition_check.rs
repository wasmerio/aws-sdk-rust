// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Represents a request to perform a check that an item exists or to check the condition of specific attributes of the item.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ConditionCheck {
    /// <p>The primary key of the item to be checked. Each element consists of an attribute name and a value for that attribute.</p>
    pub key: ::std::option::Option<::std::collections::HashMap<::std::string::String, crate::types::AttributeValue>>,
    /// <p>Name of the table for the check item request.</p>
    pub table_name: ::std::option::Option<::std::string::String>,
    /// <p>A condition that must be satisfied in order for a conditional update to succeed. For more information, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.ConditionExpressions.html">Condition expressions</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
    pub condition_expression: ::std::option::Option<::std::string::String>,
    /// <p>One or more substitution tokens for attribute names in an expression. For more information, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.ExpressionAttributeNames.html">Expression attribute names</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
    pub expression_attribute_names: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>,
    /// <p>One or more values that can be substituted in an expression. For more information, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.ConditionExpressions.html">Condition expressions</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
    pub expression_attribute_values: ::std::option::Option<::std::collections::HashMap<::std::string::String, crate::types::AttributeValue>>,
    /// <p>Use <code>ReturnValuesOnConditionCheckFailure</code> to get the item attributes if the <code>ConditionCheck</code> condition fails. For <code>ReturnValuesOnConditionCheckFailure</code>, the valid values are: NONE and ALL_OLD.</p>
    pub return_values_on_condition_check_failure: ::std::option::Option<crate::types::ReturnValuesOnConditionCheckFailure>,
}
impl ConditionCheck {
    /// <p>The primary key of the item to be checked. Each element consists of an attribute name and a value for that attribute.</p>
    pub fn key(&self) -> ::std::option::Option<&::std::collections::HashMap<::std::string::String, crate::types::AttributeValue>> {
        self.key.as_ref()
    }
    /// <p>Name of the table for the check item request.</p>
    pub fn table_name(&self) -> ::std::option::Option<&str> {
        self.table_name.as_deref()
    }
    /// <p>A condition that must be satisfied in order for a conditional update to succeed. For more information, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.ConditionExpressions.html">Condition expressions</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
    pub fn condition_expression(&self) -> ::std::option::Option<&str> {
        self.condition_expression.as_deref()
    }
    /// <p>One or more substitution tokens for attribute names in an expression. For more information, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.ExpressionAttributeNames.html">Expression attribute names</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
    pub fn expression_attribute_names(&self) -> ::std::option::Option<&::std::collections::HashMap<::std::string::String, ::std::string::String>> {
        self.expression_attribute_names.as_ref()
    }
    /// <p>One or more values that can be substituted in an expression. For more information, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.ConditionExpressions.html">Condition expressions</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
    pub fn expression_attribute_values(
        &self,
    ) -> ::std::option::Option<&::std::collections::HashMap<::std::string::String, crate::types::AttributeValue>> {
        self.expression_attribute_values.as_ref()
    }
    /// <p>Use <code>ReturnValuesOnConditionCheckFailure</code> to get the item attributes if the <code>ConditionCheck</code> condition fails. For <code>ReturnValuesOnConditionCheckFailure</code>, the valid values are: NONE and ALL_OLD.</p>
    pub fn return_values_on_condition_check_failure(&self) -> ::std::option::Option<&crate::types::ReturnValuesOnConditionCheckFailure> {
        self.return_values_on_condition_check_failure.as_ref()
    }
}
impl ConditionCheck {
    /// Creates a new builder-style object to manufacture [`ConditionCheck`](crate::types::ConditionCheck).
    pub fn builder() -> crate::types::builders::ConditionCheckBuilder {
        crate::types::builders::ConditionCheckBuilder::default()
    }
}

/// A builder for [`ConditionCheck`](crate::types::ConditionCheck).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct ConditionCheckBuilder {
    pub(crate) key: ::std::option::Option<::std::collections::HashMap<::std::string::String, crate::types::AttributeValue>>,
    pub(crate) table_name: ::std::option::Option<::std::string::String>,
    pub(crate) condition_expression: ::std::option::Option<::std::string::String>,
    pub(crate) expression_attribute_names: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>,
    pub(crate) expression_attribute_values: ::std::option::Option<::std::collections::HashMap<::std::string::String, crate::types::AttributeValue>>,
    pub(crate) return_values_on_condition_check_failure: ::std::option::Option<crate::types::ReturnValuesOnConditionCheckFailure>,
}
impl ConditionCheckBuilder {
    /// Adds a key-value pair to `key`.
    ///
    /// To override the contents of this collection use [`set_key`](Self::set_key).
    ///
    /// <p>The primary key of the item to be checked. Each element consists of an attribute name and a value for that attribute.</p>
    pub fn key(mut self, k: impl ::std::convert::Into<::std::string::String>, v: crate::types::AttributeValue) -> Self {
        let mut hash_map = self.key.unwrap_or_default();
        hash_map.insert(k.into(), v);
        self.key = ::std::option::Option::Some(hash_map);
        self
    }
    /// <p>The primary key of the item to be checked. Each element consists of an attribute name and a value for that attribute.</p>
    pub fn set_key(mut self, input: ::std::option::Option<::std::collections::HashMap<::std::string::String, crate::types::AttributeValue>>) -> Self {
        self.key = input;
        self
    }
    /// <p>The primary key of the item to be checked. Each element consists of an attribute name and a value for that attribute.</p>
    pub fn get_key(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, crate::types::AttributeValue>> {
        &self.key
    }
    /// <p>Name of the table for the check item request.</p>
    pub fn table_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.table_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Name of the table for the check item request.</p>
    pub fn set_table_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.table_name = input;
        self
    }
    /// <p>Name of the table for the check item request.</p>
    pub fn get_table_name(&self) -> &::std::option::Option<::std::string::String> {
        &self.table_name
    }
    /// <p>A condition that must be satisfied in order for a conditional update to succeed. For more information, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.ConditionExpressions.html">Condition expressions</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
    pub fn condition_expression(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.condition_expression = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A condition that must be satisfied in order for a conditional update to succeed. For more information, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.ConditionExpressions.html">Condition expressions</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
    pub fn set_condition_expression(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.condition_expression = input;
        self
    }
    /// <p>A condition that must be satisfied in order for a conditional update to succeed. For more information, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.ConditionExpressions.html">Condition expressions</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
    pub fn get_condition_expression(&self) -> &::std::option::Option<::std::string::String> {
        &self.condition_expression
    }
    /// Adds a key-value pair to `expression_attribute_names`.
    ///
    /// To override the contents of this collection use [`set_expression_attribute_names`](Self::set_expression_attribute_names).
    ///
    /// <p>One or more substitution tokens for attribute names in an expression. For more information, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.ExpressionAttributeNames.html">Expression attribute names</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
    pub fn expression_attribute_names(
        mut self,
        k: impl ::std::convert::Into<::std::string::String>,
        v: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        let mut hash_map = self.expression_attribute_names.unwrap_or_default();
        hash_map.insert(k.into(), v.into());
        self.expression_attribute_names = ::std::option::Option::Some(hash_map);
        self
    }
    /// <p>One or more substitution tokens for attribute names in an expression. For more information, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.ExpressionAttributeNames.html">Expression attribute names</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
    pub fn set_expression_attribute_names(
        mut self,
        input: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>,
    ) -> Self {
        self.expression_attribute_names = input;
        self
    }
    /// <p>One or more substitution tokens for attribute names in an expression. For more information, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.ExpressionAttributeNames.html">Expression attribute names</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
    pub fn get_expression_attribute_names(
        &self,
    ) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>> {
        &self.expression_attribute_names
    }
    /// Adds a key-value pair to `expression_attribute_values`.
    ///
    /// To override the contents of this collection use [`set_expression_attribute_values`](Self::set_expression_attribute_values).
    ///
    /// <p>One or more values that can be substituted in an expression. For more information, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.ConditionExpressions.html">Condition expressions</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
    pub fn expression_attribute_values(mut self, k: impl ::std::convert::Into<::std::string::String>, v: crate::types::AttributeValue) -> Self {
        let mut hash_map = self.expression_attribute_values.unwrap_or_default();
        hash_map.insert(k.into(), v);
        self.expression_attribute_values = ::std::option::Option::Some(hash_map);
        self
    }
    /// <p>One or more values that can be substituted in an expression. For more information, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.ConditionExpressions.html">Condition expressions</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
    pub fn set_expression_attribute_values(
        mut self,
        input: ::std::option::Option<::std::collections::HashMap<::std::string::String, crate::types::AttributeValue>>,
    ) -> Self {
        self.expression_attribute_values = input;
        self
    }
    /// <p>One or more values that can be substituted in an expression. For more information, see <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Expressions.ConditionExpressions.html">Condition expressions</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
    pub fn get_expression_attribute_values(
        &self,
    ) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, crate::types::AttributeValue>> {
        &self.expression_attribute_values
    }
    /// <p>Use <code>ReturnValuesOnConditionCheckFailure</code> to get the item attributes if the <code>ConditionCheck</code> condition fails. For <code>ReturnValuesOnConditionCheckFailure</code>, the valid values are: NONE and ALL_OLD.</p>
    pub fn return_values_on_condition_check_failure(mut self, input: crate::types::ReturnValuesOnConditionCheckFailure) -> Self {
        self.return_values_on_condition_check_failure = ::std::option::Option::Some(input);
        self
    }
    /// <p>Use <code>ReturnValuesOnConditionCheckFailure</code> to get the item attributes if the <code>ConditionCheck</code> condition fails. For <code>ReturnValuesOnConditionCheckFailure</code>, the valid values are: NONE and ALL_OLD.</p>
    pub fn set_return_values_on_condition_check_failure(
        mut self,
        input: ::std::option::Option<crate::types::ReturnValuesOnConditionCheckFailure>,
    ) -> Self {
        self.return_values_on_condition_check_failure = input;
        self
    }
    /// <p>Use <code>ReturnValuesOnConditionCheckFailure</code> to get the item attributes if the <code>ConditionCheck</code> condition fails. For <code>ReturnValuesOnConditionCheckFailure</code>, the valid values are: NONE and ALL_OLD.</p>
    pub fn get_return_values_on_condition_check_failure(&self) -> &::std::option::Option<crate::types::ReturnValuesOnConditionCheckFailure> {
        &self.return_values_on_condition_check_failure
    }
    /// Consumes the builder and constructs a [`ConditionCheck`](crate::types::ConditionCheck).
    pub fn build(self) -> crate::types::ConditionCheck {
        crate::types::ConditionCheck {
            key: self.key,
            table_name: self.table_name,
            condition_expression: self.condition_expression,
            expression_attribute_names: self.expression_attribute_names,
            expression_attribute_values: self.expression_attribute_values,
            return_values_on_condition_check_failure: self.return_values_on_condition_check_failure,
        }
    }
}
