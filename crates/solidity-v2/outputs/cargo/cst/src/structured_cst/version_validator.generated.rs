// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::too_many_lines)]
#![allow(clippy::match_single_binding)]

use std::ops::Range;

use slang_solidity_v2_common::versions::LanguageVersion;

use super::nodes::*;

/// Describes a version constraint that was violated.
#[derive(Clone, Debug, PartialEq)]
pub enum VersionConstraint {
    From {
        from: LanguageVersion,
    },
    Till {
        till: LanguageVersion,
    },
    Range {
        from: LanguageVersion,
        till: LanguageVersion,
    },
}

/// An error produced when a CST node is not valid for the given language version.
#[derive(Clone, Debug, PartialEq)]
pub struct VersionValidationError {
    /// The text range of the invalid node.
    pub range: Range<usize>,
    /// The version constraint that was violated.
    pub specifier: VersionConstraint,
}

/// Validate that all nodes in the given `SourceUnit` are valid for the given language version.
pub fn validate_version(
    source_unit: &SourceUnit,
    version: LanguageVersion,
) -> Vec<VersionValidationError> {
    let mut errors = Vec::new();
    validate_source_unit(source_unit, version, &mut errors);
    errors
}

//
// Sequence validators
//

fn validate_additive_expression(
    node: &AdditiveExpression,
    version: LanguageVersion,
    errors: &mut Vec<VersionValidationError>,
) {
    let node = node.as_ref();

    validate_expression(&node.left_operand, version, errors);

    validate_expression(&node.right_operand, version, errors);
}

fn validate_and_expression(
    node: &AndExpression,
    version: LanguageVersion,
    errors: &mut Vec<VersionValidationError>,
) {
    let node = node.as_ref();

    validate_expression(&node.left_operand, version, errors);

    validate_expression(&node.right_operand, version, errors);
}

fn validate_array_expression(
    node: &ArrayExpression,
    version: LanguageVersion,
    errors: &mut Vec<VersionValidationError>,
) {
    let node = node.as_ref();

    validate_array_values(&node.items, version, errors);
}

fn validate_array_type_name(
    node: &ArrayTypeName,
    version: LanguageVersion,
    errors: &mut Vec<VersionValidationError>,
) {
    let node = node.as_ref();

    validate_type_name(&node.operand, version, errors);

    if let Some(ref child) = node.index {
        validate_expression(child, version, errors);
    }
}

fn validate_assembly_statement(
    node: &AssemblyStatement,
    version: LanguageVersion,
    errors: &mut Vec<VersionValidationError>,
) {
    let node = node.as_ref();

    if let Some(ref child) = node.label {}
    if let Some(ref child) = node.flags {
        if version < LanguageVersion::V0_8_13 {
            errors.push(VersionValidationError {
                range: first_range_of_yul_flags_declaration(child),
                specifier: VersionConstraint::From {
                    from: LanguageVersion::V0_8_13,
                },
            });
        } else {
            validate_yul_flags_declaration(child, version, errors);
        }
    }
}

fn validate_assignment_expression(
    node: &AssignmentExpression,
    version: LanguageVersion,
    errors: &mut Vec<VersionValidationError>,
) {
    let node = node.as_ref();

    validate_expression(&node.left_operand, version, errors);

    validate_expression(&node.right_operand, version, errors);
}

fn validate_bitwise_and_expression(
    node: &BitwiseAndExpression,
    version: LanguageVersion,
    errors: &mut Vec<VersionValidationError>,
) {
    let node = node.as_ref();

    validate_expression(&node.left_operand, version, errors);

    validate_expression(&node.right_operand, version, errors);
}

fn validate_bitwise_or_expression(
    node: &BitwiseOrExpression,
    version: LanguageVersion,
    errors: &mut Vec<VersionValidationError>,
) {
    let node = node.as_ref();

    validate_expression(&node.left_operand, version, errors);

    validate_expression(&node.right_operand, version, errors);
}

fn validate_bitwise_xor_expression(
    node: &BitwiseXorExpression,
    version: LanguageVersion,
    errors: &mut Vec<VersionValidationError>,
) {
    let node = node.as_ref();

    validate_expression(&node.left_operand, version, errors);

    validate_expression(&node.right_operand, version, errors);
}

fn validate_block(
    node: &Block,
    version: LanguageVersion,
    errors: &mut Vec<VersionValidationError>,
) {
    let node = node.as_ref();

    validate_statements(&node.statements, version, errors);
}

fn validate_call_options_expression(
    node: &CallOptionsExpression,
    version: LanguageVersion,
    errors: &mut Vec<VersionValidationError>,
) {
    let node = node.as_ref();

    validate_expression(&node.operand, version, errors);

    validate_call_options(&node.options, version, errors);
}

fn validate_catch_clause(
    node: &CatchClause,
    version: LanguageVersion,
    errors: &mut Vec<VersionValidationError>,
) {
    let node = node.as_ref();

    if let Some(ref child) = node.error {
        validate_catch_clause_error(child, version, errors);
    }

    validate_block(&node.body, version, errors);
}

fn validate_catch_clause_error(
    node: &CatchClauseError,
    version: LanguageVersion,
    errors: &mut Vec<VersionValidationError>,
) {
    let node = node.as_ref();

    if let Some(ref child) = node.name {}

    validate_parameters_declaration(&node.parameters, version, errors);
}

fn validate_conditional_expression(
    node: &ConditionalExpression,
    version: LanguageVersion,
    errors: &mut Vec<VersionValidationError>,
) {
    let node = node.as_ref();

    validate_expression(&node.operand, version, errors);

    validate_expression(&node.true_expression, version, errors);

    validate_expression(&node.false_expression, version, errors);
}

fn validate_constant_definition(
    node: &ConstantDefinition,
    version: LanguageVersion,
    errors: &mut Vec<VersionValidationError>,
) {
    let node = node.as_ref();

    validate_type_name(&node.type_name, version, errors);

    validate_expression(&node.value, version, errors);
}

fn validate_constructor_definition(
    node: &ConstructorDefinition,
    version: LanguageVersion,
    errors: &mut Vec<VersionValidationError>,
) {
    let node = node.as_ref();

    validate_parameters_declaration(&node.parameters, version, errors);

    validate_constructor_attributes(&node.attributes, version, errors);

    validate_block(&node.body, version, errors);
}

fn validate_contract_definition(
    node: &ContractDefinition,
    version: LanguageVersion,
    errors: &mut Vec<VersionValidationError>,
) {
    let node = node.as_ref();

    if let Some(ref child) = node.abstract_keyword {}

    validate_contract_specifiers(&node.specifiers, version, errors);

    validate_contract_members(&node.members, version, errors);
}

fn validate_do_while_statement(
    node: &DoWhileStatement,
    version: LanguageVersion,
    errors: &mut Vec<VersionValidationError>,
) {
    let node = node.as_ref();

    validate_statement(&node.body, version, errors);

    validate_expression(&node.condition, version, errors);
}

fn validate_else_branch(
    node: &ElseBranch,
    version: LanguageVersion,
    errors: &mut Vec<VersionValidationError>,
) {
    let node = node.as_ref();

    validate_statement(&node.body, version, errors);
}

fn validate_emit_statement(
    node: &EmitStatement,
    version: LanguageVersion,
    errors: &mut Vec<VersionValidationError>,
) {
    let node = node.as_ref();

    validate_arguments_declaration(&node.arguments, version, errors);
}

fn validate_equality_expression(
    node: &EqualityExpression,
    version: LanguageVersion,
    errors: &mut Vec<VersionValidationError>,
) {
    let node = node.as_ref();

    validate_expression(&node.left_operand, version, errors);

    validate_expression(&node.right_operand, version, errors);
}

fn validate_error_definition(
    node: &ErrorDefinition,
    version: LanguageVersion,
    errors: &mut Vec<VersionValidationError>,
) {
    let node = node.as_ref();

    if version < LanguageVersion::V0_8_4 {
        errors.push(VersionValidationError {
            range: first_range_seq_error_definition(node),
            specifier: VersionConstraint::From {
                from: LanguageVersion::V0_8_4,
            },
        });
        return;
    }

    validate_error_parameters_declaration(&node.members, version, errors);
}

fn validate_error_parameter(
    node: &ErrorParameter,
    version: LanguageVersion,
    errors: &mut Vec<VersionValidationError>,
) {
    let node = node.as_ref();

    if version < LanguageVersion::V0_8_4 {
        errors.push(VersionValidationError {
            range: first_range_seq_error_parameter(node),
            specifier: VersionConstraint::From {
                from: LanguageVersion::V0_8_4,
            },
        });
        return;
    }

    validate_type_name(&node.type_name, version, errors);
    if let Some(ref child) = node.name {}
}

fn validate_error_parameters_declaration(
    node: &ErrorParametersDeclaration,
    version: LanguageVersion,
    errors: &mut Vec<VersionValidationError>,
) {
    let node = node.as_ref();

    if version < LanguageVersion::V0_8_4 {
        errors.push(VersionValidationError {
            range: first_range_seq_error_parameters_declaration(node),
            specifier: VersionConstraint::From {
                from: LanguageVersion::V0_8_4,
            },
        });
        return;
    }

    validate_error_parameters(&node.parameters, version, errors);
}

fn validate_event_definition(
    node: &EventDefinition,
    version: LanguageVersion,
    errors: &mut Vec<VersionValidationError>,
) {
    let node = node.as_ref();

    validate_event_parameters_declaration(&node.parameters, version, errors);
    if let Some(ref child) = node.anonymous_keyword {}
}

fn validate_event_parameter(
    node: &EventParameter,
    version: LanguageVersion,
    errors: &mut Vec<VersionValidationError>,
) {
    let node = node.as_ref();

    validate_type_name(&node.type_name, version, errors);
    if let Some(ref child) = node.indexed_keyword {}
    if let Some(ref child) = node.name {}
}

fn validate_event_parameters_declaration(
    node: &EventParametersDeclaration,
    version: LanguageVersion,
    errors: &mut Vec<VersionValidationError>,
) {
    let node = node.as_ref();

    validate_event_parameters(&node.parameters, version, errors);
}

fn validate_exponentiation_expression(
    node: &ExponentiationExpression,
    version: LanguageVersion,
    errors: &mut Vec<VersionValidationError>,
) {
    let node = node.as_ref();

    validate_expression(&node.left_operand, version, errors);

    validate_expression(&node.right_operand, version, errors);
}

fn validate_expression_statement(
    node: &ExpressionStatement,
    version: LanguageVersion,
    errors: &mut Vec<VersionValidationError>,
) {
    let node = node.as_ref();

    validate_expression(&node.expression, version, errors);
}

fn validate_fallback_function_definition(
    node: &FallbackFunctionDefinition,
    version: LanguageVersion,
    errors: &mut Vec<VersionValidationError>,
) {
    let node = node.as_ref();

    validate_parameters_declaration(&node.parameters, version, errors);

    validate_fallback_function_attributes(&node.attributes, version, errors);
    if let Some(ref child) = node.returns {
        validate_returns_declaration(child, version, errors);
    }

    validate_function_body(&node.body, version, errors);
}

fn validate_for_statement(
    node: &ForStatement,
    version: LanguageVersion,
    errors: &mut Vec<VersionValidationError>,
) {
    let node = node.as_ref();

    validate_for_statement_initialization(&node.initialization, version, errors);

    validate_for_statement_condition(&node.condition, version, errors);
    if let Some(ref child) = node.iterator {
        validate_expression(child, version, errors);
    }

    validate_statement(&node.body, version, errors);
}

fn validate_function_call_expression(
    node: &FunctionCallExpression,
    version: LanguageVersion,
    errors: &mut Vec<VersionValidationError>,
) {
    let node = node.as_ref();

    validate_expression(&node.operand, version, errors);

    validate_arguments_declaration(&node.arguments, version, errors);
}

fn validate_function_definition(
    node: &FunctionDefinition,
    version: LanguageVersion,
    errors: &mut Vec<VersionValidationError>,
) {
    let node = node.as_ref();

    validate_parameters_declaration(&node.parameters, version, errors);

    validate_function_attributes(&node.attributes, version, errors);
    if let Some(ref child) = node.returns {
        validate_returns_declaration(child, version, errors);
    }

    validate_function_body(&node.body, version, errors);
}

fn validate_function_type(
    node: &FunctionType,
    version: LanguageVersion,
    errors: &mut Vec<VersionValidationError>,
) {
    let node = node.as_ref();

    validate_parameters_declaration(&node.parameters, version, errors);

    if let Some(ref child) = node.returns {
        validate_returns_declaration(child, version, errors);
    }
}

fn validate_if_statement(
    node: &IfStatement,
    version: LanguageVersion,
    errors: &mut Vec<VersionValidationError>,
) {
    let node = node.as_ref();

    validate_expression(&node.condition, version, errors);

    validate_statement(&node.body, version, errors);
    if let Some(ref child) = node.else_branch {
        validate_else_branch(child, version, errors);
    }
}

fn validate_index_access_end(
    node: &IndexAccessEnd,
    version: LanguageVersion,
    errors: &mut Vec<VersionValidationError>,
) {
    let node = node.as_ref();

    if let Some(ref child) = node.end {
        validate_expression(child, version, errors);
    }
}

fn validate_index_access_expression(
    node: &IndexAccessExpression,
    version: LanguageVersion,
    errors: &mut Vec<VersionValidationError>,
) {
    let node = node.as_ref();

    validate_expression(&node.operand, version, errors);

    if let Some(ref child) = node.start {
        validate_expression(child, version, errors);
    }
    if let Some(ref child) = node.end {
        validate_index_access_end(child, version, errors);
    }
}

fn validate_inequality_expression(
    node: &InequalityExpression,
    version: LanguageVersion,
    errors: &mut Vec<VersionValidationError>,
) {
    let node = node.as_ref();

    validate_expression(&node.left_operand, version, errors);

    validate_expression(&node.right_operand, version, errors);
}

fn validate_inheritance_specifier(
    node: &InheritanceSpecifier,
    version: LanguageVersion,
    errors: &mut Vec<VersionValidationError>,
) {
    let node = node.as_ref();

    validate_inheritance_types(&node.types, version, errors);
}

fn validate_inheritance_type(
    node: &InheritanceType,
    version: LanguageVersion,
    errors: &mut Vec<VersionValidationError>,
) {
    let node = node.as_ref();

    if let Some(ref child) = node.arguments {
        validate_arguments_declaration(child, version, errors);
    }
}

fn validate_interface_definition(
    node: &InterfaceDefinition,
    version: LanguageVersion,
    errors: &mut Vec<VersionValidationError>,
) {
    let node = node.as_ref();

    if let Some(ref child) = node.inheritance {
        validate_inheritance_specifier(child, version, errors);
    }

    validate_interface_members(&node.members, version, errors);
}

fn validate_library_definition(
    node: &LibraryDefinition,
    version: LanguageVersion,
    errors: &mut Vec<VersionValidationError>,
) {
    let node = node.as_ref();

    validate_library_members(&node.members, version, errors);
}

fn validate_mapping_key(
    node: &MappingKey,
    version: LanguageVersion,
    errors: &mut Vec<VersionValidationError>,
) {
    let node = node.as_ref();

    if let Some(ref child) = node.name {
        if version < LanguageVersion::V0_8_18 {
            errors.push(VersionValidationError {
                range: first_range_of_identifier(child),
                specifier: VersionConstraint::From {
                    from: LanguageVersion::V0_8_18,
                },
            });
        } else {
        }
    }
}

fn validate_mapping_type(
    node: &MappingType,
    version: LanguageVersion,
    errors: &mut Vec<VersionValidationError>,
) {
    let node = node.as_ref();

    validate_mapping_key(&node.key_type, version, errors);

    validate_mapping_value(&node.value_type, version, errors);
}

fn validate_mapping_value(
    node: &MappingValue,
    version: LanguageVersion,
    errors: &mut Vec<VersionValidationError>,
) {
    let node = node.as_ref();

    validate_type_name(&node.type_name, version, errors);
    if let Some(ref child) = node.name {
        if version < LanguageVersion::V0_8_18 {
            errors.push(VersionValidationError {
                range: first_range_of_identifier(child),
                specifier: VersionConstraint::From {
                    from: LanguageVersion::V0_8_18,
                },
            });
        } else {
        }
    }
}

fn validate_member_access_expression(
    node: &MemberAccessExpression,
    version: LanguageVersion,
    errors: &mut Vec<VersionValidationError>,
) {
    let node = node.as_ref();

    validate_expression(&node.operand, version, errors);
}

fn validate_modifier_definition(
    node: &ModifierDefinition,
    version: LanguageVersion,
    errors: &mut Vec<VersionValidationError>,
) {
    let node = node.as_ref();

    if let Some(ref child) = node.parameters {
        validate_parameters_declaration(child, version, errors);
    }

    validate_function_body(&node.body, version, errors);
}

fn validate_modifier_invocation(
    node: &ModifierInvocation,
    version: LanguageVersion,
    errors: &mut Vec<VersionValidationError>,
) {
    let node = node.as_ref();

    if let Some(ref child) = node.arguments {
        validate_arguments_declaration(child, version, errors);
    }
}

fn validate_multi_typed_declaration(
    node: &MultiTypedDeclaration,
    version: LanguageVersion,
    errors: &mut Vec<VersionValidationError>,
) {
    let node = node.as_ref();

    validate_multi_typed_declaration_elements(&node.elements, version, errors);

    validate_variable_declaration_value(&node.value, version, errors);
}

fn validate_multi_typed_declaration_element(
    node: &MultiTypedDeclarationElement,
    version: LanguageVersion,
    errors: &mut Vec<VersionValidationError>,
) {
    let node = node.as_ref();

    if let Some(ref child) = node.member {
        validate_variable_declaration(child, version, errors);
    }
}

fn validate_multiplicative_expression(
    node: &MultiplicativeExpression,
    version: LanguageVersion,
    errors: &mut Vec<VersionValidationError>,
) {
    let node = node.as_ref();

    validate_expression(&node.left_operand, version, errors);

    validate_expression(&node.right_operand, version, errors);
}

fn validate_named_argument(
    node: &NamedArgument,
    version: LanguageVersion,
    errors: &mut Vec<VersionValidationError>,
) {
    let node = node.as_ref();

    validate_expression(&node.value, version, errors);
}

fn validate_named_argument_group(
    node: &NamedArgumentGroup,
    version: LanguageVersion,
    errors: &mut Vec<VersionValidationError>,
) {
    let node = node.as_ref();

    validate_named_arguments(&node.arguments, version, errors);
}

fn validate_named_arguments_declaration(
    node: &NamedArgumentsDeclaration,
    version: LanguageVersion,
    errors: &mut Vec<VersionValidationError>,
) {
    let node = node.as_ref();

    validate_named_argument_group(&node.arguments, version, errors);
}

fn validate_new_expression(
    node: &NewExpression,
    version: LanguageVersion,
    errors: &mut Vec<VersionValidationError>,
) {
    let node = node.as_ref();

    validate_type_name(&node.type_name, version, errors);
}

fn validate_or_expression(
    node: &OrExpression,
    version: LanguageVersion,
    errors: &mut Vec<VersionValidationError>,
) {
    let node = node.as_ref();

    validate_expression(&node.left_operand, version, errors);

    validate_expression(&node.right_operand, version, errors);
}

fn validate_parameter(
    node: &Parameter,
    version: LanguageVersion,
    errors: &mut Vec<VersionValidationError>,
) {
    let node = node.as_ref();

    validate_type_name(&node.type_name, version, errors);
    if let Some(ref child) = node.storage_location {}
    if let Some(ref child) = node.name {}
}

fn validate_parameters_declaration(
    node: &ParametersDeclaration,
    version: LanguageVersion,
    errors: &mut Vec<VersionValidationError>,
) {
    let node = node.as_ref();

    validate_parameters(&node.parameters, version, errors);
}

fn validate_positional_arguments_declaration(
    node: &PositionalArgumentsDeclaration,
    version: LanguageVersion,
    errors: &mut Vec<VersionValidationError>,
) {
    let node = node.as_ref();

    validate_positional_arguments(&node.arguments, version, errors);
}

fn validate_postfix_expression(
    node: &PostfixExpression,
    version: LanguageVersion,
    errors: &mut Vec<VersionValidationError>,
) {
    let node = node.as_ref();

    validate_expression(&node.operand, version, errors);
}

fn validate_prefix_expression(
    node: &PrefixExpression,
    version: LanguageVersion,
    errors: &mut Vec<VersionValidationError>,
) {
    let node = node.as_ref();

    validate_expression(&node.operand, version, errors);
}

fn validate_receive_function_definition(
    node: &ReceiveFunctionDefinition,
    version: LanguageVersion,
    errors: &mut Vec<VersionValidationError>,
) {
    let node = node.as_ref();

    validate_parameters_declaration(&node.parameters, version, errors);

    validate_receive_function_attributes(&node.attributes, version, errors);

    validate_function_body(&node.body, version, errors);
}

fn validate_return_statement(
    node: &ReturnStatement,
    version: LanguageVersion,
    errors: &mut Vec<VersionValidationError>,
) {
    let node = node.as_ref();

    if let Some(ref child) = node.expression {
        validate_expression(child, version, errors);
    }
}

fn validate_returns_declaration(
    node: &ReturnsDeclaration,
    version: LanguageVersion,
    errors: &mut Vec<VersionValidationError>,
) {
    let node = node.as_ref();

    validate_parameters_declaration(&node.variables, version, errors);
}

fn validate_revert_statement(
    node: &RevertStatement,
    version: LanguageVersion,
    errors: &mut Vec<VersionValidationError>,
) {
    let node = node.as_ref();

    if version < LanguageVersion::V0_8_4 {
        errors.push(VersionValidationError {
            range: first_range_seq_revert_statement(node),
            specifier: VersionConstraint::From {
                from: LanguageVersion::V0_8_4,
            },
        });
        return;
    }

    validate_arguments_declaration(&node.arguments, version, errors);
}

fn validate_shift_expression(
    node: &ShiftExpression,
    version: LanguageVersion,
    errors: &mut Vec<VersionValidationError>,
) {
    let node = node.as_ref();

    validate_expression(&node.left_operand, version, errors);

    validate_expression(&node.right_operand, version, errors);
}

fn validate_single_typed_declaration(
    node: &SingleTypedDeclaration,
    version: LanguageVersion,
    errors: &mut Vec<VersionValidationError>,
) {
    let node = node.as_ref();

    validate_variable_declaration(&node.declaration, version, errors);
    if let Some(ref child) = node.value {
        validate_variable_declaration_value(child, version, errors);
    }
}

fn validate_source_unit(
    node: &SourceUnit,
    version: LanguageVersion,
    errors: &mut Vec<VersionValidationError>,
) {
    let node = node.as_ref();

    validate_source_unit_members(&node.members, version, errors);
}

fn validate_state_variable_definition(
    node: &StateVariableDefinition,
    version: LanguageVersion,
    errors: &mut Vec<VersionValidationError>,
) {
    let node = node.as_ref();

    validate_type_name(&node.type_name, version, errors);

    validate_state_variable_attributes(&node.attributes, version, errors);

    if let Some(ref child) = node.value {
        validate_state_variable_definition_value(child, version, errors);
    }
}

fn validate_state_variable_definition_value(
    node: &StateVariableDefinitionValue,
    version: LanguageVersion,
    errors: &mut Vec<VersionValidationError>,
) {
    let node = node.as_ref();

    validate_expression(&node.value, version, errors);
}

fn validate_storage_layout_specifier(
    node: &StorageLayoutSpecifier,
    version: LanguageVersion,
    errors: &mut Vec<VersionValidationError>,
) {
    let node = node.as_ref();

    if version < LanguageVersion::V0_8_29 {
        errors.push(VersionValidationError {
            range: first_range_seq_storage_layout_specifier(node),
            specifier: VersionConstraint::From {
                from: LanguageVersion::V0_8_29,
            },
        });
        return;
    }

    validate_expression(&node.expression, version, errors);
}

fn validate_struct_definition(
    node: &StructDefinition,
    version: LanguageVersion,
    errors: &mut Vec<VersionValidationError>,
) {
    let node = node.as_ref();

    validate_struct_members(&node.members, version, errors);
}

fn validate_struct_member(
    node: &StructMember,
    version: LanguageVersion,
    errors: &mut Vec<VersionValidationError>,
) {
    let node = node.as_ref();

    validate_type_name(&node.type_name, version, errors);
}

fn validate_try_statement(
    node: &TryStatement,
    version: LanguageVersion,
    errors: &mut Vec<VersionValidationError>,
) {
    let node = node.as_ref();

    validate_expression(&node.expression, version, errors);
    if let Some(ref child) = node.returns {
        validate_returns_declaration(child, version, errors);
    }

    validate_block(&node.body, version, errors);

    validate_catch_clauses(&node.catch_clauses, version, errors);
}

fn validate_tuple_expression(
    node: &TupleExpression,
    version: LanguageVersion,
    errors: &mut Vec<VersionValidationError>,
) {
    let node = node.as_ref();

    validate_tuple_values(&node.items, version, errors);
}

fn validate_tuple_value(
    node: &TupleValue,
    version: LanguageVersion,
    errors: &mut Vec<VersionValidationError>,
) {
    let node = node.as_ref();

    if let Some(ref child) = node.expression {
        validate_expression(child, version, errors);
    }
}

fn validate_type_expression(
    node: &TypeExpression,
    version: LanguageVersion,
    errors: &mut Vec<VersionValidationError>,
) {
    let node = node.as_ref();

    validate_type_name(&node.type_name, version, errors);
}

fn validate_unchecked_block(
    node: &UncheckedBlock,
    version: LanguageVersion,
    errors: &mut Vec<VersionValidationError>,
) {
    let node = node.as_ref();

    validate_block(&node.block, version, errors);
}

fn validate_user_defined_value_type_definition(
    node: &UserDefinedValueTypeDefinition,
    version: LanguageVersion,
    errors: &mut Vec<VersionValidationError>,
) {
    let node = node.as_ref();

    if version < LanguageVersion::V0_8_8 {
        errors.push(VersionValidationError {
            range: first_range_seq_user_defined_value_type_definition(node),
            specifier: VersionConstraint::From {
                from: LanguageVersion::V0_8_8,
            },
        });
        return;
    }
}

fn validate_using_alias(
    node: &UsingAlias,
    version: LanguageVersion,
    errors: &mut Vec<VersionValidationError>,
) {
    let node = node.as_ref();

    if version < LanguageVersion::V0_8_19 {
        errors.push(VersionValidationError {
            range: first_range_seq_using_alias(node),
            specifier: VersionConstraint::From {
                from: LanguageVersion::V0_8_19,
            },
        });
        return;
    }

    validate_using_operator(&node.operator, version, errors);
}

fn validate_using_deconstruction(
    node: &UsingDeconstruction,
    version: LanguageVersion,
    errors: &mut Vec<VersionValidationError>,
) {
    let node = node.as_ref();

    if version < LanguageVersion::V0_8_13 {
        errors.push(VersionValidationError {
            range: first_range_seq_using_deconstruction(node),
            specifier: VersionConstraint::From {
                from: LanguageVersion::V0_8_13,
            },
        });
        return;
    }

    validate_using_deconstruction_symbols(&node.symbols, version, errors);
}

fn validate_using_deconstruction_symbol(
    node: &UsingDeconstructionSymbol,
    version: LanguageVersion,
    errors: &mut Vec<VersionValidationError>,
) {
    let node = node.as_ref();

    if version < LanguageVersion::V0_8_13 {
        errors.push(VersionValidationError {
            range: first_range_seq_using_deconstruction_symbol(node),
            specifier: VersionConstraint::From {
                from: LanguageVersion::V0_8_13,
            },
        });
        return;
    }

    if let Some(ref child) = node.alias {
        if version < LanguageVersion::V0_8_19 {
            errors.push(VersionValidationError {
                range: first_range_of_using_alias(child),
                specifier: VersionConstraint::From {
                    from: LanguageVersion::V0_8_19,
                },
            });
        } else {
            validate_using_alias(child, version, errors);
        }
    }
}

fn validate_using_directive(
    node: &UsingDirective,
    version: LanguageVersion,
    errors: &mut Vec<VersionValidationError>,
) {
    let node = node.as_ref();

    validate_using_clause(&node.clause, version, errors);

    validate_using_target(&node.target, version, errors);
    if let Some(ref child) = node.global_keyword {
        if version < LanguageVersion::V0_8_13 {
            errors.push(VersionValidationError {
                range: first_range_of_global_keyword(child),
                specifier: VersionConstraint::From {
                    from: LanguageVersion::V0_8_13,
                },
            });
        } else {
        }
    }
}

fn validate_variable_declaration(
    node: &VariableDeclaration,
    version: LanguageVersion,
    errors: &mut Vec<VersionValidationError>,
) {
    let node = node.as_ref();

    validate_type_name(&node.type_name, version, errors);
    if let Some(ref child) = node.storage_location {}
}

fn validate_variable_declaration_statement(
    node: &VariableDeclarationStatement,
    version: LanguageVersion,
    errors: &mut Vec<VersionValidationError>,
) {
    let node = node.as_ref();

    validate_variable_declaration_target(&node.target, version, errors);
}

fn validate_variable_declaration_value(
    node: &VariableDeclarationValue,
    version: LanguageVersion,
    errors: &mut Vec<VersionValidationError>,
) {
    let node = node.as_ref();

    validate_expression(&node.expression, version, errors);
}

fn validate_while_statement(
    node: &WhileStatement,
    version: LanguageVersion,
    errors: &mut Vec<VersionValidationError>,
) {
    let node = node.as_ref();

    validate_expression(&node.condition, version, errors);

    validate_statement(&node.body, version, errors);
}

fn validate_yul_flags_declaration(
    node: &YulFlagsDeclaration,
    version: LanguageVersion,
    errors: &mut Vec<VersionValidationError>,
) {
    let node = node.as_ref();

    if version < LanguageVersion::V0_8_13 {
        errors.push(VersionValidationError {
            range: first_range_seq_yul_flags_declaration(node),
            specifier: VersionConstraint::From {
                from: LanguageVersion::V0_8_13,
            },
        });
        return;
    }

    validate_yul_flags(&node.flags, version, errors);
}

//
// Choice validators
//

fn validate_arguments_declaration(
    node: &ArgumentsDeclaration,
    version: LanguageVersion,
    errors: &mut Vec<VersionValidationError>,
) {
    match node {
        ArgumentsDeclaration::PositionalArgumentsDeclaration(child) => {
            validate_positional_arguments_declaration(child, version, errors);
        }
        ArgumentsDeclaration::NamedArgumentsDeclaration(child) => {
            validate_named_arguments_declaration(child, version, errors);
        }
    }
}

fn validate_constructor_attribute(
    node: &ConstructorAttribute,
    version: LanguageVersion,
    errors: &mut Vec<VersionValidationError>,
) {
    match node {
        ConstructorAttribute::ModifierInvocation(child) => {
            validate_modifier_invocation(child, version, errors);
        }
        ConstructorAttribute::InternalKeyword(child) => {}
        ConstructorAttribute::PayableKeyword(child) => {}
        ConstructorAttribute::PublicKeyword(child) => {}
    }
}

fn validate_contract_member(
    node: &ContractMember,
    version: LanguageVersion,
    errors: &mut Vec<VersionValidationError>,
) {
    match node {
        ContractMember::UsingDirective(child) => {
            validate_using_directive(child, version, errors);
        }
        ContractMember::FunctionDefinition(child) => {
            validate_function_definition(child, version, errors);
        }
        ContractMember::ConstructorDefinition(child) => {
            validate_constructor_definition(child, version, errors);
        }
        ContractMember::ReceiveFunctionDefinition(child) => {
            validate_receive_function_definition(child, version, errors);
        }
        ContractMember::FallbackFunctionDefinition(child) => {
            validate_fallback_function_definition(child, version, errors);
        }
        ContractMember::ModifierDefinition(child) => {
            validate_modifier_definition(child, version, errors);
        }
        ContractMember::StructDefinition(child) => {
            validate_struct_definition(child, version, errors);
        }
        ContractMember::EnumDefinition(child) => {}
        ContractMember::EventDefinition(child) => {
            validate_event_definition(child, version, errors);
        }
        ContractMember::ErrorDefinition(child) => {
            if version < LanguageVersion::V0_8_4 {
                errors.push(VersionValidationError {
                    range: first_range_of_error_definition(child),
                    specifier: VersionConstraint::From {
                        from: LanguageVersion::V0_8_4,
                    },
                });
                return;
            }
            validate_error_definition(child, version, errors);
        }
        ContractMember::UserDefinedValueTypeDefinition(child) => {
            if version < LanguageVersion::V0_8_8 {
                errors.push(VersionValidationError {
                    range: first_range_of_user_defined_value_type_definition(child),
                    specifier: VersionConstraint::From {
                        from: LanguageVersion::V0_8_8,
                    },
                });
                return;
            }
            validate_user_defined_value_type_definition(child, version, errors);
        }
        ContractMember::StateVariableDefinition(child) => {
            validate_state_variable_definition(child, version, errors);
        }
    }
}

fn validate_contract_specifier(
    node: &ContractSpecifier,
    version: LanguageVersion,
    errors: &mut Vec<VersionValidationError>,
) {
    match node {
        ContractSpecifier::InheritanceSpecifier(child) => {
            validate_inheritance_specifier(child, version, errors);
        }
        ContractSpecifier::StorageLayoutSpecifier(child) => {
            if version < LanguageVersion::V0_8_29 {
                errors.push(VersionValidationError {
                    range: first_range_of_storage_layout_specifier(child),
                    specifier: VersionConstraint::From {
                        from: LanguageVersion::V0_8_29,
                    },
                });
                return;
            }
            validate_storage_layout_specifier(child, version, errors);
        }
    }
}

fn validate_expression(
    node: &Expression,
    version: LanguageVersion,
    errors: &mut Vec<VersionValidationError>,
) {
    match node {
        Expression::AssignmentExpression(child) => {
            validate_assignment_expression(child, version, errors);
        }
        Expression::ConditionalExpression(child) => {
            validate_conditional_expression(child, version, errors);
        }
        Expression::OrExpression(child) => {
            validate_or_expression(child, version, errors);
        }
        Expression::AndExpression(child) => {
            validate_and_expression(child, version, errors);
        }
        Expression::EqualityExpression(child) => {
            validate_equality_expression(child, version, errors);
        }
        Expression::InequalityExpression(child) => {
            validate_inequality_expression(child, version, errors);
        }
        Expression::BitwiseOrExpression(child) => {
            validate_bitwise_or_expression(child, version, errors);
        }
        Expression::BitwiseXorExpression(child) => {
            validate_bitwise_xor_expression(child, version, errors);
        }
        Expression::BitwiseAndExpression(child) => {
            validate_bitwise_and_expression(child, version, errors);
        }
        Expression::ShiftExpression(child) => {
            validate_shift_expression(child, version, errors);
        }
        Expression::AdditiveExpression(child) => {
            validate_additive_expression(child, version, errors);
        }
        Expression::MultiplicativeExpression(child) => {
            validate_multiplicative_expression(child, version, errors);
        }
        Expression::ExponentiationExpression(child) => {
            validate_exponentiation_expression(child, version, errors);
        }
        Expression::PostfixExpression(child) => {
            validate_postfix_expression(child, version, errors);
        }
        Expression::PrefixExpression(child) => {
            validate_prefix_expression(child, version, errors);
        }
        Expression::FunctionCallExpression(child) => {
            validate_function_call_expression(child, version, errors);
        }
        Expression::CallOptionsExpression(child) => {
            validate_call_options_expression(child, version, errors);
        }
        Expression::MemberAccessExpression(child) => {
            validate_member_access_expression(child, version, errors);
        }
        Expression::IndexAccessExpression(child) => {
            validate_index_access_expression(child, version, errors);
        }
        Expression::NewExpression(child) => {
            validate_new_expression(child, version, errors);
        }
        Expression::TupleExpression(child) => {
            validate_tuple_expression(child, version, errors);
        }
        Expression::TypeExpression(child) => {
            validate_type_expression(child, version, errors);
        }
        Expression::ArrayExpression(child) => {
            validate_array_expression(child, version, errors);
        }
        Expression::HexNumberExpression(child) => {}
        Expression::DecimalNumberExpression(child) => {}
        Expression::StringExpression(child) => {}
        Expression::ElementaryType(child) => {}
        Expression::PayableKeyword(child) => {}
        Expression::ThisKeyword(child) => {}
        Expression::SuperKeyword(child) => {}
        Expression::TrueKeyword(child) => {}
        Expression::FalseKeyword(child) => {}
        Expression::Identifier(child) => {}
    }
}

fn validate_fallback_function_attribute(
    node: &FallbackFunctionAttribute,
    version: LanguageVersion,
    errors: &mut Vec<VersionValidationError>,
) {
    match node {
        FallbackFunctionAttribute::ModifierInvocation(child) => {
            validate_modifier_invocation(child, version, errors);
        }
        FallbackFunctionAttribute::OverrideSpecifier(child) => {}
        FallbackFunctionAttribute::ExternalKeyword(child) => {}
        FallbackFunctionAttribute::PayableKeyword(child) => {}
        FallbackFunctionAttribute::PureKeyword(child) => {}
        FallbackFunctionAttribute::ViewKeyword(child) => {}
        FallbackFunctionAttribute::VirtualKeyword(child) => {}
    }
}

fn validate_for_statement_condition(
    node: &ForStatementCondition,
    version: LanguageVersion,
    errors: &mut Vec<VersionValidationError>,
) {
    match node {
        ForStatementCondition::ExpressionStatement(child) => {
            validate_expression_statement(child, version, errors);
        }
        ForStatementCondition::Semicolon(child) => {}
    }
}

fn validate_for_statement_initialization(
    node: &ForStatementInitialization,
    version: LanguageVersion,
    errors: &mut Vec<VersionValidationError>,
) {
    match node {
        ForStatementInitialization::VariableDeclarationStatement(child) => {
            validate_variable_declaration_statement(child, version, errors);
        }
        ForStatementInitialization::ExpressionStatement(child) => {
            validate_expression_statement(child, version, errors);
        }
        ForStatementInitialization::Semicolon(child) => {}
    }
}

fn validate_function_attribute(
    node: &FunctionAttribute,
    version: LanguageVersion,
    errors: &mut Vec<VersionValidationError>,
) {
    match node {
        FunctionAttribute::ModifierInvocation(child) => {
            validate_modifier_invocation(child, version, errors);
        }
        FunctionAttribute::OverrideSpecifier(child) => {}
        FunctionAttribute::ExternalKeyword(child) => {}
        FunctionAttribute::InternalKeyword(child) => {}
        FunctionAttribute::PayableKeyword(child) => {}
        FunctionAttribute::PrivateKeyword(child) => {}
        FunctionAttribute::PublicKeyword(child) => {}
        FunctionAttribute::PureKeyword(child) => {}
        FunctionAttribute::ViewKeyword(child) => {}
        FunctionAttribute::VirtualKeyword(child) => {}
    }
}

fn validate_function_body(
    node: &FunctionBody,
    version: LanguageVersion,
    errors: &mut Vec<VersionValidationError>,
) {
    match node {
        FunctionBody::Block(child) => {
            validate_block(child, version, errors);
        }
        FunctionBody::Semicolon(child) => {}
    }
}

fn validate_receive_function_attribute(
    node: &ReceiveFunctionAttribute,
    version: LanguageVersion,
    errors: &mut Vec<VersionValidationError>,
) {
    match node {
        ReceiveFunctionAttribute::ModifierInvocation(child) => {
            validate_modifier_invocation(child, version, errors);
        }
        ReceiveFunctionAttribute::OverrideSpecifier(child) => {}
        ReceiveFunctionAttribute::ExternalKeyword(child) => {}
        ReceiveFunctionAttribute::PayableKeyword(child) => {}
        ReceiveFunctionAttribute::VirtualKeyword(child) => {}
    }
}

fn validate_source_unit_member(
    node: &SourceUnitMember,
    version: LanguageVersion,
    errors: &mut Vec<VersionValidationError>,
) {
    match node {
        SourceUnitMember::PragmaDirective(child) => {}
        SourceUnitMember::ImportDirective(child) => {}
        SourceUnitMember::ContractDefinition(child) => {
            validate_contract_definition(child, version, errors);
        }
        SourceUnitMember::InterfaceDefinition(child) => {
            validate_interface_definition(child, version, errors);
        }
        SourceUnitMember::LibraryDefinition(child) => {
            validate_library_definition(child, version, errors);
        }
        SourceUnitMember::StructDefinition(child) => {
            validate_struct_definition(child, version, errors);
        }
        SourceUnitMember::EnumDefinition(child) => {}
        SourceUnitMember::FunctionDefinition(child) => {
            validate_function_definition(child, version, errors);
        }
        SourceUnitMember::ErrorDefinition(child) => {
            if version < LanguageVersion::V0_8_4 {
                errors.push(VersionValidationError {
                    range: first_range_of_error_definition(child),
                    specifier: VersionConstraint::From {
                        from: LanguageVersion::V0_8_4,
                    },
                });
                return;
            }
            validate_error_definition(child, version, errors);
        }
        SourceUnitMember::UserDefinedValueTypeDefinition(child) => {
            if version < LanguageVersion::V0_8_8 {
                errors.push(VersionValidationError {
                    range: first_range_of_user_defined_value_type_definition(child),
                    specifier: VersionConstraint::From {
                        from: LanguageVersion::V0_8_8,
                    },
                });
                return;
            }
            validate_user_defined_value_type_definition(child, version, errors);
        }
        SourceUnitMember::UsingDirective(child) => {
            if version < LanguageVersion::V0_8_13 {
                errors.push(VersionValidationError {
                    range: first_range_of_using_directive(child),
                    specifier: VersionConstraint::From {
                        from: LanguageVersion::V0_8_13,
                    },
                });
                return;
            }
            validate_using_directive(child, version, errors);
        }
        SourceUnitMember::EventDefinition(child) => {
            if version < LanguageVersion::V0_8_22 {
                errors.push(VersionValidationError {
                    range: first_range_of_event_definition(child),
                    specifier: VersionConstraint::From {
                        from: LanguageVersion::V0_8_22,
                    },
                });
                return;
            }
            validate_event_definition(child, version, errors);
        }
        SourceUnitMember::ConstantDefinition(child) => {
            validate_constant_definition(child, version, errors);
        }
    }
}

fn validate_state_variable_attribute(
    node: &StateVariableAttribute,
    version: LanguageVersion,
    errors: &mut Vec<VersionValidationError>,
) {
    match node {
        StateVariableAttribute::OverrideSpecifier(child) => {}
        StateVariableAttribute::ConstantKeyword(child) => {}
        StateVariableAttribute::InternalKeyword(child) => {}
        StateVariableAttribute::PrivateKeyword(child) => {}
        StateVariableAttribute::PublicKeyword(child) => {}
        StateVariableAttribute::ImmutableKeyword(child) => {}
        StateVariableAttribute::TransientKeyword(child) => {
            if version < LanguageVersion::V0_8_27 {
                errors.push(VersionValidationError {
                    range: first_range_of_transient_keyword(child),
                    specifier: VersionConstraint::From {
                        from: LanguageVersion::V0_8_27,
                    },
                });
                return;
            }
        }
    }
}

fn validate_statement(
    node: &Statement,
    version: LanguageVersion,
    errors: &mut Vec<VersionValidationError>,
) {
    match node {
        Statement::IfStatement(child) => {
            validate_if_statement(child, version, errors);
        }
        Statement::ForStatement(child) => {
            validate_for_statement(child, version, errors);
        }
        Statement::WhileStatement(child) => {
            validate_while_statement(child, version, errors);
        }
        Statement::DoWhileStatement(child) => {
            validate_do_while_statement(child, version, errors);
        }
        Statement::ContinueStatement(child) => {}
        Statement::BreakStatement(child) => {}
        Statement::ReturnStatement(child) => {
            validate_return_statement(child, version, errors);
        }
        Statement::EmitStatement(child) => {
            validate_emit_statement(child, version, errors);
        }
        Statement::TryStatement(child) => {
            validate_try_statement(child, version, errors);
        }
        Statement::RevertStatement(child) => {
            if version < LanguageVersion::V0_8_4 {
                errors.push(VersionValidationError {
                    range: first_range_of_revert_statement(child),
                    specifier: VersionConstraint::From {
                        from: LanguageVersion::V0_8_4,
                    },
                });
                return;
            }
            validate_revert_statement(child, version, errors);
        }
        Statement::AssemblyStatement(child) => {
            validate_assembly_statement(child, version, errors);
        }
        Statement::Block(child) => {
            validate_block(child, version, errors);
        }
        Statement::UncheckedBlock(child) => {
            validate_unchecked_block(child, version, errors);
        }
        Statement::VariableDeclarationStatement(child) => {
            validate_variable_declaration_statement(child, version, errors);
        }
        Statement::ExpressionStatement(child) => {
            validate_expression_statement(child, version, errors);
        }
    }
}

fn validate_type_name(
    node: &TypeName,
    version: LanguageVersion,
    errors: &mut Vec<VersionValidationError>,
) {
    match node {
        TypeName::ArrayTypeName(child) => {
            validate_array_type_name(child, version, errors);
        }
        TypeName::FunctionType(child) => {
            validate_function_type(child, version, errors);
        }
        TypeName::MappingType(child) => {
            validate_mapping_type(child, version, errors);
        }
        TypeName::ElementaryType(child) => {}
        TypeName::IdentifierPath(child) => {}
    }
}

fn validate_using_clause(
    node: &UsingClause,
    version: LanguageVersion,
    errors: &mut Vec<VersionValidationError>,
) {
    match node {
        UsingClause::IdentifierPath(child) => {}
        UsingClause::UsingDeconstruction(child) => {
            if version < LanguageVersion::V0_8_13 {
                errors.push(VersionValidationError {
                    range: first_range_of_using_deconstruction(child),
                    specifier: VersionConstraint::From {
                        from: LanguageVersion::V0_8_13,
                    },
                });
                return;
            }
            validate_using_deconstruction(child, version, errors);
        }
    }
}

fn validate_using_operator(
    node: &UsingOperator,
    version: LanguageVersion,
    errors: &mut Vec<VersionValidationError>,
) {
    if version < LanguageVersion::V0_8_19 {
        errors.push(VersionValidationError {
            range: first_range_of_using_operator(node),
            specifier: VersionConstraint::From {
                from: LanguageVersion::V0_8_19,
            },
        });
        return;
    }

    match node {
        UsingOperator::Ampersand(child) => {}
        UsingOperator::Asterisk(child) => {}
        UsingOperator::BangEqual(child) => {}
        UsingOperator::Bar(child) => {}
        UsingOperator::Caret(child) => {}
        UsingOperator::EqualEqual(child) => {}
        UsingOperator::GreaterThan(child) => {}
        UsingOperator::GreaterThanEqual(child) => {}
        UsingOperator::LessThan(child) => {}
        UsingOperator::LessThanEqual(child) => {}
        UsingOperator::Minus(child) => {}
        UsingOperator::Percent(child) => {}
        UsingOperator::Plus(child) => {}
        UsingOperator::Slash(child) => {}
        UsingOperator::Tilde(child) => {}
    }
}

fn validate_using_target(
    node: &UsingTarget,
    version: LanguageVersion,
    errors: &mut Vec<VersionValidationError>,
) {
    match node {
        UsingTarget::TypeName(child) => {
            validate_type_name(child, version, errors);
        }
        UsingTarget::Asterisk(child) => {}
    }
}

fn validate_variable_declaration_target(
    node: &VariableDeclarationTarget,
    version: LanguageVersion,
    errors: &mut Vec<VersionValidationError>,
) {
    match node {
        VariableDeclarationTarget::SingleTypedDeclaration(child) => {
            validate_single_typed_declaration(child, version, errors);
        }
        VariableDeclarationTarget::MultiTypedDeclaration(child) => {
            validate_multi_typed_declaration(child, version, errors);
        }
    }
}

//
// Collection validators
//

fn validate_array_values(
    node: &ArrayValues,
    version: LanguageVersion,
    errors: &mut Vec<VersionValidationError>,
) {
    for child in &node.elements {
        validate_expression(child, version, errors);
    }
}

fn validate_call_options(
    node: &CallOptions,
    version: LanguageVersion,
    errors: &mut Vec<VersionValidationError>,
) {
    for child in &node.elements {
        validate_named_argument(child, version, errors);
    }
}

fn validate_catch_clauses(
    node: &CatchClauses,
    version: LanguageVersion,
    errors: &mut Vec<VersionValidationError>,
) {
    for child in &node.elements {
        validate_catch_clause(child, version, errors);
    }
}

fn validate_constructor_attributes(
    node: &ConstructorAttributes,
    version: LanguageVersion,
    errors: &mut Vec<VersionValidationError>,
) {
    for child in &node.elements {
        validate_constructor_attribute(child, version, errors);
    }
}

fn validate_contract_members(
    node: &ContractMembers,
    version: LanguageVersion,
    errors: &mut Vec<VersionValidationError>,
) {
    for child in &node.elements {
        validate_contract_member(child, version, errors);
    }
}

fn validate_contract_specifiers(
    node: &ContractSpecifiers,
    version: LanguageVersion,
    errors: &mut Vec<VersionValidationError>,
) {
    for child in &node.elements {
        validate_contract_specifier(child, version, errors);
    }
}

fn validate_error_parameters(
    node: &ErrorParameters,
    version: LanguageVersion,
    errors: &mut Vec<VersionValidationError>,
) {
    if version < LanguageVersion::V0_8_4 {
        errors.push(VersionValidationError {
            range: first_range_of_error_parameters(node),
            specifier: VersionConstraint::From {
                from: LanguageVersion::V0_8_4,
            },
        });
        return;
    }
    for child in &node.elements {
        validate_error_parameter(child, version, errors);
    }
}

fn validate_event_parameters(
    node: &EventParameters,
    version: LanguageVersion,
    errors: &mut Vec<VersionValidationError>,
) {
    for child in &node.elements {
        validate_event_parameter(child, version, errors);
    }
}

fn validate_fallback_function_attributes(
    node: &FallbackFunctionAttributes,
    version: LanguageVersion,
    errors: &mut Vec<VersionValidationError>,
) {
    for child in &node.elements {
        validate_fallback_function_attribute(child, version, errors);
    }
}

fn validate_function_attributes(
    node: &FunctionAttributes,
    version: LanguageVersion,
    errors: &mut Vec<VersionValidationError>,
) {
    for child in &node.elements {
        validate_function_attribute(child, version, errors);
    }
}

fn validate_inheritance_types(
    node: &InheritanceTypes,
    version: LanguageVersion,
    errors: &mut Vec<VersionValidationError>,
) {
    for child in &node.elements {
        validate_inheritance_type(child, version, errors);
    }
}

fn validate_interface_members(
    node: &InterfaceMembers,
    version: LanguageVersion,
    errors: &mut Vec<VersionValidationError>,
) {
    for child in &node.elements {
        validate_contract_member(child, version, errors);
    }
}

fn validate_library_members(
    node: &LibraryMembers,
    version: LanguageVersion,
    errors: &mut Vec<VersionValidationError>,
) {
    for child in &node.elements {
        validate_contract_member(child, version, errors);
    }
}

fn validate_multi_typed_declaration_elements(
    node: &MultiTypedDeclarationElements,
    version: LanguageVersion,
    errors: &mut Vec<VersionValidationError>,
) {
    for child in &node.elements {
        validate_multi_typed_declaration_element(child, version, errors);
    }
}

fn validate_named_arguments(
    node: &NamedArguments,
    version: LanguageVersion,
    errors: &mut Vec<VersionValidationError>,
) {
    for child in &node.elements {
        validate_named_argument(child, version, errors);
    }
}

fn validate_parameters(
    node: &Parameters,
    version: LanguageVersion,
    errors: &mut Vec<VersionValidationError>,
) {
    for child in &node.elements {
        validate_parameter(child, version, errors);
    }
}

fn validate_positional_arguments(
    node: &PositionalArguments,
    version: LanguageVersion,
    errors: &mut Vec<VersionValidationError>,
) {
    for child in &node.elements {
        validate_expression(child, version, errors);
    }
}

fn validate_receive_function_attributes(
    node: &ReceiveFunctionAttributes,
    version: LanguageVersion,
    errors: &mut Vec<VersionValidationError>,
) {
    for child in &node.elements {
        validate_receive_function_attribute(child, version, errors);
    }
}

fn validate_source_unit_members(
    node: &SourceUnitMembers,
    version: LanguageVersion,
    errors: &mut Vec<VersionValidationError>,
) {
    for child in &node.elements {
        validate_source_unit_member(child, version, errors);
    }
}

fn validate_state_variable_attributes(
    node: &StateVariableAttributes,
    version: LanguageVersion,
    errors: &mut Vec<VersionValidationError>,
) {
    for child in &node.elements {
        validate_state_variable_attribute(child, version, errors);
    }
}

fn validate_statements(
    node: &Statements,
    version: LanguageVersion,
    errors: &mut Vec<VersionValidationError>,
) {
    for child in &node.elements {
        validate_statement(child, version, errors);
    }
}

fn validate_struct_members(
    node: &StructMembers,
    version: LanguageVersion,
    errors: &mut Vec<VersionValidationError>,
) {
    for child in &node.elements {
        validate_struct_member(child, version, errors);
    }
}

fn validate_tuple_values(
    node: &TupleValues,
    version: LanguageVersion,
    errors: &mut Vec<VersionValidationError>,
) {
    for child in &node.elements {
        validate_tuple_value(child, version, errors);
    }
}

fn validate_using_deconstruction_symbols(
    node: &UsingDeconstructionSymbols,
    version: LanguageVersion,
    errors: &mut Vec<VersionValidationError>,
) {
    if version < LanguageVersion::V0_8_13 {
        errors.push(VersionValidationError {
            range: first_range_of_using_deconstruction_symbols(node),
            specifier: VersionConstraint::From {
                from: LanguageVersion::V0_8_13,
            },
        });
        return;
    }
    for child in &node.elements {
        validate_using_deconstruction_symbol(child, version, errors);
    }
}

fn validate_yul_flags(
    node: &YulFlags,
    version: LanguageVersion,
    errors: &mut Vec<VersionValidationError>,
) {
    if version < LanguageVersion::V0_8_13 {
        errors.push(VersionValidationError {
            range: first_range_of_yul_flags(node),
            specifier: VersionConstraint::From {
                from: LanguageVersion::V0_8_13,
            },
        });
        return;
    }
}

//
// Helper functions to get the first terminal range from a node
//

fn first_range_seq_abicoder_pragma(node: &AbicoderPragmaStruct) -> Range<usize> {
    first_range_of_abicoder_keyword(&node.abicoder_keyword)
}

fn first_range_of_abicoder_pragma(node: &AbicoderPragma) -> Range<usize> {
    first_range_seq_abicoder_pragma(node.as_ref())
}

fn first_range_seq_additive_expression(node: &AdditiveExpressionStruct) -> Range<usize> {
    first_range_of_expression(&node.left_operand)
}

fn first_range_of_additive_expression(node: &AdditiveExpression) -> Range<usize> {
    first_range_seq_additive_expression(node.as_ref())
}

fn first_range_seq_address_type(node: &AddressTypeStruct) -> Range<usize> {
    first_range_of_address_keyword(&node.address_keyword)
}

fn first_range_of_address_type(node: &AddressType) -> Range<usize> {
    first_range_seq_address_type(node.as_ref())
}

fn first_range_seq_and_expression(node: &AndExpressionStruct) -> Range<usize> {
    first_range_of_expression(&node.left_operand)
}

fn first_range_of_and_expression(node: &AndExpression) -> Range<usize> {
    first_range_seq_and_expression(node.as_ref())
}

fn first_range_seq_array_expression(node: &ArrayExpressionStruct) -> Range<usize> {
    first_range_of_open_bracket(&node.open_bracket)
}

fn first_range_of_array_expression(node: &ArrayExpression) -> Range<usize> {
    first_range_seq_array_expression(node.as_ref())
}

fn first_range_seq_array_type_name(node: &ArrayTypeNameStruct) -> Range<usize> {
    first_range_of_type_name(&node.operand)
}

fn first_range_of_array_type_name(node: &ArrayTypeName) -> Range<usize> {
    first_range_seq_array_type_name(node.as_ref())
}

fn first_range_seq_assembly_statement(node: &AssemblyStatementStruct) -> Range<usize> {
    first_range_of_assembly_keyword(&node.assembly_keyword)
}

fn first_range_of_assembly_statement(node: &AssemblyStatement) -> Range<usize> {
    first_range_seq_assembly_statement(node.as_ref())
}

fn first_range_seq_assignment_expression(node: &AssignmentExpressionStruct) -> Range<usize> {
    first_range_of_expression(&node.left_operand)
}

fn first_range_of_assignment_expression(node: &AssignmentExpression) -> Range<usize> {
    first_range_seq_assignment_expression(node.as_ref())
}

fn first_range_seq_bitwise_and_expression(node: &BitwiseAndExpressionStruct) -> Range<usize> {
    first_range_of_expression(&node.left_operand)
}

fn first_range_of_bitwise_and_expression(node: &BitwiseAndExpression) -> Range<usize> {
    first_range_seq_bitwise_and_expression(node.as_ref())
}

fn first_range_seq_bitwise_or_expression(node: &BitwiseOrExpressionStruct) -> Range<usize> {
    first_range_of_expression(&node.left_operand)
}

fn first_range_of_bitwise_or_expression(node: &BitwiseOrExpression) -> Range<usize> {
    first_range_seq_bitwise_or_expression(node.as_ref())
}

fn first_range_seq_bitwise_xor_expression(node: &BitwiseXorExpressionStruct) -> Range<usize> {
    first_range_of_expression(&node.left_operand)
}

fn first_range_of_bitwise_xor_expression(node: &BitwiseXorExpression) -> Range<usize> {
    first_range_seq_bitwise_xor_expression(node.as_ref())
}

fn first_range_seq_block(node: &BlockStruct) -> Range<usize> {
    first_range_of_open_brace(&node.open_brace)
}

fn first_range_of_block(node: &Block) -> Range<usize> {
    first_range_seq_block(node.as_ref())
}

fn first_range_seq_break_statement(node: &BreakStatementStruct) -> Range<usize> {
    first_range_of_break_keyword(&node.break_keyword)
}

fn first_range_of_break_statement(node: &BreakStatement) -> Range<usize> {
    first_range_seq_break_statement(node.as_ref())
}

fn first_range_seq_call_options_expression(node: &CallOptionsExpressionStruct) -> Range<usize> {
    first_range_of_expression(&node.operand)
}

fn first_range_of_call_options_expression(node: &CallOptionsExpression) -> Range<usize> {
    first_range_seq_call_options_expression(node.as_ref())
}

fn first_range_seq_catch_clause(node: &CatchClauseStruct) -> Range<usize> {
    first_range_of_catch_keyword(&node.catch_keyword)
}

fn first_range_of_catch_clause(node: &CatchClause) -> Range<usize> {
    first_range_seq_catch_clause(node.as_ref())
}

fn first_range_seq_catch_clause_error(node: &CatchClauseErrorStruct) -> Range<usize> {
    if let Some(ref child) = node.name {
        return first_range_of_identifier(child);
    }
    first_range_of_parameters_declaration(&node.parameters)
}

fn first_range_of_catch_clause_error(node: &CatchClauseError) -> Range<usize> {
    first_range_seq_catch_clause_error(node.as_ref())
}

fn first_range_seq_conditional_expression(node: &ConditionalExpressionStruct) -> Range<usize> {
    first_range_of_expression(&node.operand)
}

fn first_range_of_conditional_expression(node: &ConditionalExpression) -> Range<usize> {
    first_range_seq_conditional_expression(node.as_ref())
}

fn first_range_seq_constant_definition(node: &ConstantDefinitionStruct) -> Range<usize> {
    first_range_of_type_name(&node.type_name)
}

fn first_range_of_constant_definition(node: &ConstantDefinition) -> Range<usize> {
    first_range_seq_constant_definition(node.as_ref())
}

fn first_range_seq_constructor_definition(node: &ConstructorDefinitionStruct) -> Range<usize> {
    first_range_of_constructor_keyword(&node.constructor_keyword)
}

fn first_range_of_constructor_definition(node: &ConstructorDefinition) -> Range<usize> {
    first_range_seq_constructor_definition(node.as_ref())
}

fn first_range_seq_continue_statement(node: &ContinueStatementStruct) -> Range<usize> {
    first_range_of_continue_keyword(&node.continue_keyword)
}

fn first_range_of_continue_statement(node: &ContinueStatement) -> Range<usize> {
    first_range_seq_continue_statement(node.as_ref())
}

fn first_range_seq_contract_definition(node: &ContractDefinitionStruct) -> Range<usize> {
    if let Some(ref child) = node.abstract_keyword {
        return first_range_of_abstract_keyword(child);
    }
    first_range_of_contract_keyword(&node.contract_keyword)
}

fn first_range_of_contract_definition(node: &ContractDefinition) -> Range<usize> {
    first_range_seq_contract_definition(node.as_ref())
}

fn first_range_seq_decimal_number_expression(node: &DecimalNumberExpressionStruct) -> Range<usize> {
    first_range_of_decimal_literal(&node.literal)
}

fn first_range_of_decimal_number_expression(node: &DecimalNumberExpression) -> Range<usize> {
    first_range_seq_decimal_number_expression(node.as_ref())
}

fn first_range_seq_do_while_statement(node: &DoWhileStatementStruct) -> Range<usize> {
    first_range_of_do_keyword(&node.do_keyword)
}

fn first_range_of_do_while_statement(node: &DoWhileStatement) -> Range<usize> {
    first_range_seq_do_while_statement(node.as_ref())
}

fn first_range_seq_else_branch(node: &ElseBranchStruct) -> Range<usize> {
    first_range_of_else_keyword(&node.else_keyword)
}

fn first_range_of_else_branch(node: &ElseBranch) -> Range<usize> {
    first_range_seq_else_branch(node.as_ref())
}

fn first_range_seq_emit_statement(node: &EmitStatementStruct) -> Range<usize> {
    first_range_of_emit_keyword(&node.emit_keyword)
}

fn first_range_of_emit_statement(node: &EmitStatement) -> Range<usize> {
    first_range_seq_emit_statement(node.as_ref())
}

fn first_range_seq_enum_definition(node: &EnumDefinitionStruct) -> Range<usize> {
    first_range_of_enum_keyword(&node.enum_keyword)
}

fn first_range_of_enum_definition(node: &EnumDefinition) -> Range<usize> {
    first_range_seq_enum_definition(node.as_ref())
}

fn first_range_seq_equality_expression(node: &EqualityExpressionStruct) -> Range<usize> {
    first_range_of_expression(&node.left_operand)
}

fn first_range_of_equality_expression(node: &EqualityExpression) -> Range<usize> {
    first_range_seq_equality_expression(node.as_ref())
}

fn first_range_seq_error_definition(node: &ErrorDefinitionStruct) -> Range<usize> {
    first_range_of_error_keyword(&node.error_keyword)
}

fn first_range_of_error_definition(node: &ErrorDefinition) -> Range<usize> {
    first_range_seq_error_definition(node.as_ref())
}

fn first_range_seq_error_parameter(node: &ErrorParameterStruct) -> Range<usize> {
    first_range_of_type_name(&node.type_name)
}

fn first_range_of_error_parameter(node: &ErrorParameter) -> Range<usize> {
    first_range_seq_error_parameter(node.as_ref())
}

fn first_range_seq_error_parameters_declaration(
    node: &ErrorParametersDeclarationStruct,
) -> Range<usize> {
    first_range_of_open_paren(&node.open_paren)
}

fn first_range_of_error_parameters_declaration(node: &ErrorParametersDeclaration) -> Range<usize> {
    first_range_seq_error_parameters_declaration(node.as_ref())
}

fn first_range_seq_event_definition(node: &EventDefinitionStruct) -> Range<usize> {
    first_range_of_event_keyword(&node.event_keyword)
}

fn first_range_of_event_definition(node: &EventDefinition) -> Range<usize> {
    first_range_seq_event_definition(node.as_ref())
}

fn first_range_seq_event_parameter(node: &EventParameterStruct) -> Range<usize> {
    first_range_of_type_name(&node.type_name)
}

fn first_range_of_event_parameter(node: &EventParameter) -> Range<usize> {
    first_range_seq_event_parameter(node.as_ref())
}

fn first_range_seq_event_parameters_declaration(
    node: &EventParametersDeclarationStruct,
) -> Range<usize> {
    first_range_of_open_paren(&node.open_paren)
}

fn first_range_of_event_parameters_declaration(node: &EventParametersDeclaration) -> Range<usize> {
    first_range_seq_event_parameters_declaration(node.as_ref())
}

fn first_range_seq_experimental_pragma(node: &ExperimentalPragmaStruct) -> Range<usize> {
    first_range_of_experimental_keyword(&node.experimental_keyword)
}

fn first_range_of_experimental_pragma(node: &ExperimentalPragma) -> Range<usize> {
    first_range_seq_experimental_pragma(node.as_ref())
}

fn first_range_seq_exponentiation_expression(
    node: &ExponentiationExpressionStruct,
) -> Range<usize> {
    first_range_of_expression(&node.left_operand)
}

fn first_range_of_exponentiation_expression(node: &ExponentiationExpression) -> Range<usize> {
    first_range_seq_exponentiation_expression(node.as_ref())
}

fn first_range_seq_expression_statement(node: &ExpressionStatementStruct) -> Range<usize> {
    first_range_of_expression(&node.expression)
}

fn first_range_of_expression_statement(node: &ExpressionStatement) -> Range<usize> {
    first_range_seq_expression_statement(node.as_ref())
}

fn first_range_seq_fallback_function_definition(
    node: &FallbackFunctionDefinitionStruct,
) -> Range<usize> {
    first_range_of_fallback_keyword(&node.fallback_keyword)
}

fn first_range_of_fallback_function_definition(node: &FallbackFunctionDefinition) -> Range<usize> {
    first_range_seq_fallback_function_definition(node.as_ref())
}

fn first_range_seq_for_statement(node: &ForStatementStruct) -> Range<usize> {
    first_range_of_for_keyword(&node.for_keyword)
}

fn first_range_of_for_statement(node: &ForStatement) -> Range<usize> {
    first_range_seq_for_statement(node.as_ref())
}

fn first_range_seq_function_call_expression(node: &FunctionCallExpressionStruct) -> Range<usize> {
    first_range_of_expression(&node.operand)
}

fn first_range_of_function_call_expression(node: &FunctionCallExpression) -> Range<usize> {
    first_range_seq_function_call_expression(node.as_ref())
}

fn first_range_seq_function_definition(node: &FunctionDefinitionStruct) -> Range<usize> {
    first_range_of_function_keyword(&node.function_keyword)
}

fn first_range_of_function_definition(node: &FunctionDefinition) -> Range<usize> {
    first_range_seq_function_definition(node.as_ref())
}

fn first_range_seq_function_type(node: &FunctionTypeStruct) -> Range<usize> {
    first_range_of_function_keyword(&node.function_keyword)
}

fn first_range_of_function_type(node: &FunctionType) -> Range<usize> {
    first_range_seq_function_type(node.as_ref())
}

fn first_range_seq_hex_number_expression(node: &HexNumberExpressionStruct) -> Range<usize> {
    first_range_of_hex_literal(&node.literal)
}

fn first_range_of_hex_number_expression(node: &HexNumberExpression) -> Range<usize> {
    first_range_seq_hex_number_expression(node.as_ref())
}

fn first_range_seq_if_statement(node: &IfStatementStruct) -> Range<usize> {
    first_range_of_if_keyword(&node.if_keyword)
}

fn first_range_of_if_statement(node: &IfStatement) -> Range<usize> {
    first_range_seq_if_statement(node.as_ref())
}

fn first_range_seq_import_alias(node: &ImportAliasStruct) -> Range<usize> {
    first_range_of_as_keyword(&node.as_keyword)
}

fn first_range_of_import_alias(node: &ImportAlias) -> Range<usize> {
    first_range_seq_import_alias(node.as_ref())
}

fn first_range_seq_import_deconstruction(node: &ImportDeconstructionStruct) -> Range<usize> {
    first_range_of_open_brace(&node.open_brace)
}

fn first_range_of_import_deconstruction(node: &ImportDeconstruction) -> Range<usize> {
    first_range_seq_import_deconstruction(node.as_ref())
}

fn first_range_seq_import_deconstruction_symbol(
    node: &ImportDeconstructionSymbolStruct,
) -> Range<usize> {
    first_range_of_identifier(&node.name)
}

fn first_range_of_import_deconstruction_symbol(node: &ImportDeconstructionSymbol) -> Range<usize> {
    first_range_seq_import_deconstruction_symbol(node.as_ref())
}

fn first_range_seq_import_directive(node: &ImportDirectiveStruct) -> Range<usize> {
    first_range_of_import_keyword(&node.import_keyword)
}

fn first_range_of_import_directive(node: &ImportDirective) -> Range<usize> {
    first_range_seq_import_directive(node.as_ref())
}

fn first_range_seq_index_access_end(node: &IndexAccessEndStruct) -> Range<usize> {
    first_range_of_colon(&node.colon)
}

fn first_range_of_index_access_end(node: &IndexAccessEnd) -> Range<usize> {
    first_range_seq_index_access_end(node.as_ref())
}

fn first_range_seq_index_access_expression(node: &IndexAccessExpressionStruct) -> Range<usize> {
    first_range_of_expression(&node.operand)
}

fn first_range_of_index_access_expression(node: &IndexAccessExpression) -> Range<usize> {
    first_range_seq_index_access_expression(node.as_ref())
}

fn first_range_seq_inequality_expression(node: &InequalityExpressionStruct) -> Range<usize> {
    first_range_of_expression(&node.left_operand)
}

fn first_range_of_inequality_expression(node: &InequalityExpression) -> Range<usize> {
    first_range_seq_inequality_expression(node.as_ref())
}

fn first_range_seq_inheritance_specifier(node: &InheritanceSpecifierStruct) -> Range<usize> {
    first_range_of_is_keyword(&node.is_keyword)
}

fn first_range_of_inheritance_specifier(node: &InheritanceSpecifier) -> Range<usize> {
    first_range_seq_inheritance_specifier(node.as_ref())
}

fn first_range_seq_inheritance_type(node: &InheritanceTypeStruct) -> Range<usize> {
    first_range_of_identifier_path(&node.type_name)
}

fn first_range_of_inheritance_type(node: &InheritanceType) -> Range<usize> {
    first_range_seq_inheritance_type(node.as_ref())
}

fn first_range_seq_interface_definition(node: &InterfaceDefinitionStruct) -> Range<usize> {
    first_range_of_interface_keyword(&node.interface_keyword)
}

fn first_range_of_interface_definition(node: &InterfaceDefinition) -> Range<usize> {
    first_range_seq_interface_definition(node.as_ref())
}

fn first_range_seq_library_definition(node: &LibraryDefinitionStruct) -> Range<usize> {
    first_range_of_library_keyword(&node.library_keyword)
}

fn first_range_of_library_definition(node: &LibraryDefinition) -> Range<usize> {
    first_range_seq_library_definition(node.as_ref())
}

fn first_range_seq_mapping_key(node: &MappingKeyStruct) -> Range<usize> {
    first_range_of_mapping_key_type(&node.key_type)
}

fn first_range_of_mapping_key(node: &MappingKey) -> Range<usize> {
    first_range_seq_mapping_key(node.as_ref())
}

fn first_range_seq_mapping_type(node: &MappingTypeStruct) -> Range<usize> {
    first_range_of_mapping_keyword(&node.mapping_keyword)
}

fn first_range_of_mapping_type(node: &MappingType) -> Range<usize> {
    first_range_seq_mapping_type(node.as_ref())
}

fn first_range_seq_mapping_value(node: &MappingValueStruct) -> Range<usize> {
    first_range_of_type_name(&node.type_name)
}

fn first_range_of_mapping_value(node: &MappingValue) -> Range<usize> {
    first_range_seq_mapping_value(node.as_ref())
}

fn first_range_seq_member_access_expression(node: &MemberAccessExpressionStruct) -> Range<usize> {
    first_range_of_expression(&node.operand)
}

fn first_range_of_member_access_expression(node: &MemberAccessExpression) -> Range<usize> {
    first_range_seq_member_access_expression(node.as_ref())
}

fn first_range_seq_modifier_definition(node: &ModifierDefinitionStruct) -> Range<usize> {
    first_range_of_modifier_keyword(&node.modifier_keyword)
}

fn first_range_of_modifier_definition(node: &ModifierDefinition) -> Range<usize> {
    first_range_seq_modifier_definition(node.as_ref())
}

fn first_range_seq_modifier_invocation(node: &ModifierInvocationStruct) -> Range<usize> {
    first_range_of_identifier_path(&node.name)
}

fn first_range_of_modifier_invocation(node: &ModifierInvocation) -> Range<usize> {
    first_range_seq_modifier_invocation(node.as_ref())
}

fn first_range_seq_multi_typed_declaration(node: &MultiTypedDeclarationStruct) -> Range<usize> {
    first_range_of_open_paren(&node.open_paren)
}

fn first_range_of_multi_typed_declaration(node: &MultiTypedDeclaration) -> Range<usize> {
    first_range_seq_multi_typed_declaration(node.as_ref())
}

fn first_range_seq_multi_typed_declaration_element(
    node: &MultiTypedDeclarationElementStruct,
) -> Range<usize> {
    node.member
        .as_ref()
        .map_or(0..0, |child| first_range_of_variable_declaration(child))
}

fn first_range_of_multi_typed_declaration_element(
    node: &MultiTypedDeclarationElement,
) -> Range<usize> {
    first_range_seq_multi_typed_declaration_element(node.as_ref())
}

fn first_range_seq_multiplicative_expression(
    node: &MultiplicativeExpressionStruct,
) -> Range<usize> {
    first_range_of_expression(&node.left_operand)
}

fn first_range_of_multiplicative_expression(node: &MultiplicativeExpression) -> Range<usize> {
    first_range_seq_multiplicative_expression(node.as_ref())
}

fn first_range_seq_named_argument(node: &NamedArgumentStruct) -> Range<usize> {
    first_range_of_identifier(&node.name)
}

fn first_range_of_named_argument(node: &NamedArgument) -> Range<usize> {
    first_range_seq_named_argument(node.as_ref())
}

fn first_range_seq_named_argument_group(node: &NamedArgumentGroupStruct) -> Range<usize> {
    first_range_of_open_brace(&node.open_brace)
}

fn first_range_of_named_argument_group(node: &NamedArgumentGroup) -> Range<usize> {
    first_range_seq_named_argument_group(node.as_ref())
}

fn first_range_seq_named_arguments_declaration(
    node: &NamedArgumentsDeclarationStruct,
) -> Range<usize> {
    first_range_of_open_paren(&node.open_paren)
}

fn first_range_of_named_arguments_declaration(node: &NamedArgumentsDeclaration) -> Range<usize> {
    first_range_seq_named_arguments_declaration(node.as_ref())
}

fn first_range_seq_named_import(node: &NamedImportStruct) -> Range<usize> {
    first_range_of_asterisk(&node.asterisk)
}

fn first_range_of_named_import(node: &NamedImport) -> Range<usize> {
    first_range_seq_named_import(node.as_ref())
}

fn first_range_seq_new_expression(node: &NewExpressionStruct) -> Range<usize> {
    first_range_of_new_keyword(&node.new_keyword)
}

fn first_range_of_new_expression(node: &NewExpression) -> Range<usize> {
    first_range_seq_new_expression(node.as_ref())
}

fn first_range_seq_or_expression(node: &OrExpressionStruct) -> Range<usize> {
    first_range_of_expression(&node.left_operand)
}

fn first_range_of_or_expression(node: &OrExpression) -> Range<usize> {
    first_range_seq_or_expression(node.as_ref())
}

fn first_range_seq_override_paths_declaration(
    node: &OverridePathsDeclarationStruct,
) -> Range<usize> {
    first_range_of_open_paren(&node.open_paren)
}

fn first_range_of_override_paths_declaration(node: &OverridePathsDeclaration) -> Range<usize> {
    first_range_seq_override_paths_declaration(node.as_ref())
}

fn first_range_seq_override_specifier(node: &OverrideSpecifierStruct) -> Range<usize> {
    first_range_of_override_keyword(&node.override_keyword)
}

fn first_range_of_override_specifier(node: &OverrideSpecifier) -> Range<usize> {
    first_range_seq_override_specifier(node.as_ref())
}

fn first_range_seq_parameter(node: &ParameterStruct) -> Range<usize> {
    first_range_of_type_name(&node.type_name)
}

fn first_range_of_parameter(node: &Parameter) -> Range<usize> {
    first_range_seq_parameter(node.as_ref())
}

fn first_range_seq_parameters_declaration(node: &ParametersDeclarationStruct) -> Range<usize> {
    first_range_of_open_paren(&node.open_paren)
}

fn first_range_of_parameters_declaration(node: &ParametersDeclaration) -> Range<usize> {
    first_range_seq_parameters_declaration(node.as_ref())
}

fn first_range_seq_path_import(node: &PathImportStruct) -> Range<usize> {
    first_range_of_string_literal(&node.path)
}

fn first_range_of_path_import(node: &PathImport) -> Range<usize> {
    first_range_seq_path_import(node.as_ref())
}

fn first_range_seq_positional_arguments_declaration(
    node: &PositionalArgumentsDeclarationStruct,
) -> Range<usize> {
    first_range_of_open_paren(&node.open_paren)
}

fn first_range_of_positional_arguments_declaration(
    node: &PositionalArgumentsDeclaration,
) -> Range<usize> {
    first_range_seq_positional_arguments_declaration(node.as_ref())
}

fn first_range_seq_postfix_expression(node: &PostfixExpressionStruct) -> Range<usize> {
    first_range_of_expression(&node.operand)
}

fn first_range_of_postfix_expression(node: &PostfixExpression) -> Range<usize> {
    first_range_seq_postfix_expression(node.as_ref())
}

fn first_range_seq_pragma_directive(node: &PragmaDirectiveStruct) -> Range<usize> {
    first_range_of_pragma_keyword(&node.pragma_keyword)
}

fn first_range_of_pragma_directive(node: &PragmaDirective) -> Range<usize> {
    first_range_seq_pragma_directive(node.as_ref())
}

fn first_range_seq_prefix_expression(node: &PrefixExpressionStruct) -> Range<usize> {
    first_range_of_expression_prefix_expression_operator(
        &node.expression_prefix_expression_operator,
    )
}

fn first_range_of_prefix_expression(node: &PrefixExpression) -> Range<usize> {
    first_range_seq_prefix_expression(node.as_ref())
}

fn first_range_seq_receive_function_definition(
    node: &ReceiveFunctionDefinitionStruct,
) -> Range<usize> {
    first_range_of_receive_keyword(&node.receive_keyword)
}

fn first_range_of_receive_function_definition(node: &ReceiveFunctionDefinition) -> Range<usize> {
    first_range_seq_receive_function_definition(node.as_ref())
}

fn first_range_seq_return_statement(node: &ReturnStatementStruct) -> Range<usize> {
    first_range_of_return_keyword(&node.return_keyword)
}

fn first_range_of_return_statement(node: &ReturnStatement) -> Range<usize> {
    first_range_seq_return_statement(node.as_ref())
}

fn first_range_seq_returns_declaration(node: &ReturnsDeclarationStruct) -> Range<usize> {
    first_range_of_returns_keyword(&node.returns_keyword)
}

fn first_range_of_returns_declaration(node: &ReturnsDeclaration) -> Range<usize> {
    first_range_seq_returns_declaration(node.as_ref())
}

fn first_range_seq_revert_statement(node: &RevertStatementStruct) -> Range<usize> {
    first_range_of_revert_keyword(&node.revert_keyword)
}

fn first_range_of_revert_statement(node: &RevertStatement) -> Range<usize> {
    first_range_seq_revert_statement(node.as_ref())
}

fn first_range_seq_shift_expression(node: &ShiftExpressionStruct) -> Range<usize> {
    first_range_of_expression(&node.left_operand)
}

fn first_range_of_shift_expression(node: &ShiftExpression) -> Range<usize> {
    first_range_seq_shift_expression(node.as_ref())
}

fn first_range_seq_single_typed_declaration(node: &SingleTypedDeclarationStruct) -> Range<usize> {
    first_range_of_variable_declaration(&node.declaration)
}

fn first_range_of_single_typed_declaration(node: &SingleTypedDeclaration) -> Range<usize> {
    first_range_seq_single_typed_declaration(node.as_ref())
}

fn first_range_seq_source_unit(node: &SourceUnitStruct) -> Range<usize> {
    first_range_of_source_unit_members(&node.members)
}

fn first_range_of_source_unit(node: &SourceUnit) -> Range<usize> {
    first_range_seq_source_unit(node.as_ref())
}

fn first_range_seq_state_variable_definition(node: &StateVariableDefinitionStruct) -> Range<usize> {
    first_range_of_type_name(&node.type_name)
}

fn first_range_of_state_variable_definition(node: &StateVariableDefinition) -> Range<usize> {
    first_range_seq_state_variable_definition(node.as_ref())
}

fn first_range_seq_state_variable_definition_value(
    node: &StateVariableDefinitionValueStruct,
) -> Range<usize> {
    first_range_of_equal(&node.equal)
}

fn first_range_of_state_variable_definition_value(
    node: &StateVariableDefinitionValue,
) -> Range<usize> {
    first_range_seq_state_variable_definition_value(node.as_ref())
}

fn first_range_seq_storage_layout_specifier(node: &StorageLayoutSpecifierStruct) -> Range<usize> {
    first_range_of_layout_keyword(&node.layout_keyword)
}

fn first_range_of_storage_layout_specifier(node: &StorageLayoutSpecifier) -> Range<usize> {
    first_range_seq_storage_layout_specifier(node.as_ref())
}

fn first_range_seq_struct_definition(node: &StructDefinitionStruct) -> Range<usize> {
    first_range_of_struct_keyword(&node.struct_keyword)
}

fn first_range_of_struct_definition(node: &StructDefinition) -> Range<usize> {
    first_range_seq_struct_definition(node.as_ref())
}

fn first_range_seq_struct_member(node: &StructMemberStruct) -> Range<usize> {
    first_range_of_type_name(&node.type_name)
}

fn first_range_of_struct_member(node: &StructMember) -> Range<usize> {
    first_range_seq_struct_member(node.as_ref())
}

fn first_range_seq_try_statement(node: &TryStatementStruct) -> Range<usize> {
    first_range_of_try_keyword(&node.try_keyword)
}

fn first_range_of_try_statement(node: &TryStatement) -> Range<usize> {
    first_range_seq_try_statement(node.as_ref())
}

fn first_range_seq_tuple_expression(node: &TupleExpressionStruct) -> Range<usize> {
    first_range_of_open_paren(&node.open_paren)
}

fn first_range_of_tuple_expression(node: &TupleExpression) -> Range<usize> {
    first_range_seq_tuple_expression(node.as_ref())
}

fn first_range_seq_tuple_value(node: &TupleValueStruct) -> Range<usize> {
    node.expression
        .as_ref()
        .map_or(0..0, |child| first_range_of_expression(child))
}

fn first_range_of_tuple_value(node: &TupleValue) -> Range<usize> {
    first_range_seq_tuple_value(node.as_ref())
}

fn first_range_seq_type_expression(node: &TypeExpressionStruct) -> Range<usize> {
    first_range_of_type_keyword(&node.type_keyword)
}

fn first_range_of_type_expression(node: &TypeExpression) -> Range<usize> {
    first_range_seq_type_expression(node.as_ref())
}

fn first_range_seq_unchecked_block(node: &UncheckedBlockStruct) -> Range<usize> {
    first_range_of_unchecked_keyword(&node.unchecked_keyword)
}

fn first_range_of_unchecked_block(node: &UncheckedBlock) -> Range<usize> {
    first_range_seq_unchecked_block(node.as_ref())
}

fn first_range_seq_user_defined_value_type_definition(
    node: &UserDefinedValueTypeDefinitionStruct,
) -> Range<usize> {
    first_range_of_type_keyword(&node.type_keyword)
}

fn first_range_of_user_defined_value_type_definition(
    node: &UserDefinedValueTypeDefinition,
) -> Range<usize> {
    first_range_seq_user_defined_value_type_definition(node.as_ref())
}

fn first_range_seq_using_alias(node: &UsingAliasStruct) -> Range<usize> {
    first_range_of_as_keyword(&node.as_keyword)
}

fn first_range_of_using_alias(node: &UsingAlias) -> Range<usize> {
    first_range_seq_using_alias(node.as_ref())
}

fn first_range_seq_using_deconstruction(node: &UsingDeconstructionStruct) -> Range<usize> {
    first_range_of_open_brace(&node.open_brace)
}

fn first_range_of_using_deconstruction(node: &UsingDeconstruction) -> Range<usize> {
    first_range_seq_using_deconstruction(node.as_ref())
}

fn first_range_seq_using_deconstruction_symbol(
    node: &UsingDeconstructionSymbolStruct,
) -> Range<usize> {
    first_range_of_identifier_path(&node.name)
}

fn first_range_of_using_deconstruction_symbol(node: &UsingDeconstructionSymbol) -> Range<usize> {
    first_range_seq_using_deconstruction_symbol(node.as_ref())
}

fn first_range_seq_using_directive(node: &UsingDirectiveStruct) -> Range<usize> {
    first_range_of_using_keyword(&node.using_keyword)
}

fn first_range_of_using_directive(node: &UsingDirective) -> Range<usize> {
    first_range_seq_using_directive(node.as_ref())
}

fn first_range_seq_variable_declaration(node: &VariableDeclarationStruct) -> Range<usize> {
    first_range_of_type_name(&node.type_name)
}

fn first_range_of_variable_declaration(node: &VariableDeclaration) -> Range<usize> {
    first_range_seq_variable_declaration(node.as_ref())
}

fn first_range_seq_variable_declaration_statement(
    node: &VariableDeclarationStatementStruct,
) -> Range<usize> {
    first_range_of_variable_declaration_target(&node.target)
}

fn first_range_of_variable_declaration_statement(
    node: &VariableDeclarationStatement,
) -> Range<usize> {
    first_range_seq_variable_declaration_statement(node.as_ref())
}

fn first_range_seq_variable_declaration_value(
    node: &VariableDeclarationValueStruct,
) -> Range<usize> {
    first_range_of_equal(&node.equal)
}

fn first_range_of_variable_declaration_value(node: &VariableDeclarationValue) -> Range<usize> {
    first_range_seq_variable_declaration_value(node.as_ref())
}

fn first_range_seq_version_pragma(node: &VersionPragmaStruct) -> Range<usize> {
    first_range_of_solidity_keyword(&node.solidity_keyword)
}

fn first_range_of_version_pragma(node: &VersionPragma) -> Range<usize> {
    first_range_seq_version_pragma(node.as_ref())
}

fn first_range_seq_version_range(node: &VersionRangeStruct) -> Range<usize> {
    first_range_of_version_literal(&node.start)
}

fn first_range_of_version_range(node: &VersionRange) -> Range<usize> {
    first_range_seq_version_range(node.as_ref())
}

fn first_range_seq_version_term(node: &VersionTermStruct) -> Range<usize> {
    if let Some(ref child) = node.operator {
        return first_range_of_version_operator(child);
    }
    first_range_of_version_literal(&node.literal)
}

fn first_range_of_version_term(node: &VersionTerm) -> Range<usize> {
    first_range_seq_version_term(node.as_ref())
}

fn first_range_seq_while_statement(node: &WhileStatementStruct) -> Range<usize> {
    first_range_of_while_keyword(&node.while_keyword)
}

fn first_range_of_while_statement(node: &WhileStatement) -> Range<usize> {
    first_range_seq_while_statement(node.as_ref())
}

fn first_range_seq_yul_block(node: &YulBlockStruct) -> Range<usize> {
    first_range_of_yul_open_brace(&node.open_brace)
}

fn first_range_of_yul_block(node: &YulBlock) -> Range<usize> {
    first_range_seq_yul_block(node.as_ref())
}

fn first_range_seq_yul_break_statement(node: &YulBreakStatementStruct) -> Range<usize> {
    first_range_of_yul_break_keyword(&node.break_keyword)
}

fn first_range_of_yul_break_statement(node: &YulBreakStatement) -> Range<usize> {
    first_range_seq_yul_break_statement(node.as_ref())
}

fn first_range_seq_yul_continue_statement(node: &YulContinueStatementStruct) -> Range<usize> {
    first_range_of_yul_continue_keyword(&node.continue_keyword)
}

fn first_range_of_yul_continue_statement(node: &YulContinueStatement) -> Range<usize> {
    first_range_seq_yul_continue_statement(node.as_ref())
}

fn first_range_seq_yul_default_case(node: &YulDefaultCaseStruct) -> Range<usize> {
    first_range_of_yul_default_keyword(&node.default_keyword)
}

fn first_range_of_yul_default_case(node: &YulDefaultCase) -> Range<usize> {
    first_range_seq_yul_default_case(node.as_ref())
}

fn first_range_seq_yul_flags_declaration(node: &YulFlagsDeclarationStruct) -> Range<usize> {
    first_range_of_yul_open_paren(&node.open_paren)
}

fn first_range_of_yul_flags_declaration(node: &YulFlagsDeclaration) -> Range<usize> {
    first_range_seq_yul_flags_declaration(node.as_ref())
}

fn first_range_seq_yul_for_statement(node: &YulForStatementStruct) -> Range<usize> {
    first_range_of_yul_for_keyword(&node.for_keyword)
}

fn first_range_of_yul_for_statement(node: &YulForStatement) -> Range<usize> {
    first_range_seq_yul_for_statement(node.as_ref())
}

fn first_range_seq_yul_function_call_expression(
    node: &YulFunctionCallExpressionStruct,
) -> Range<usize> {
    first_range_of_yul_expression(&node.operand)
}

fn first_range_of_yul_function_call_expression(node: &YulFunctionCallExpression) -> Range<usize> {
    first_range_seq_yul_function_call_expression(node.as_ref())
}

fn first_range_seq_yul_function_definition(node: &YulFunctionDefinitionStruct) -> Range<usize> {
    first_range_of_yul_function_keyword(&node.function_keyword)
}

fn first_range_of_yul_function_definition(node: &YulFunctionDefinition) -> Range<usize> {
    first_range_seq_yul_function_definition(node.as_ref())
}

fn first_range_seq_yul_if_statement(node: &YulIfStatementStruct) -> Range<usize> {
    first_range_of_yul_if_keyword(&node.if_keyword)
}

fn first_range_of_yul_if_statement(node: &YulIfStatement) -> Range<usize> {
    first_range_seq_yul_if_statement(node.as_ref())
}

fn first_range_seq_yul_leave_statement(node: &YulLeaveStatementStruct) -> Range<usize> {
    first_range_of_yul_leave_keyword(&node.leave_keyword)
}

fn first_range_of_yul_leave_statement(node: &YulLeaveStatement) -> Range<usize> {
    first_range_seq_yul_leave_statement(node.as_ref())
}

fn first_range_seq_yul_parameters_declaration(
    node: &YulParametersDeclarationStruct,
) -> Range<usize> {
    first_range_of_yul_open_paren(&node.open_paren)
}

fn first_range_of_yul_parameters_declaration(node: &YulParametersDeclaration) -> Range<usize> {
    first_range_seq_yul_parameters_declaration(node.as_ref())
}

fn first_range_seq_yul_returns_declaration(node: &YulReturnsDeclarationStruct) -> Range<usize> {
    first_range_of_yul_minus_greater_than(&node.minus_greater_than)
}

fn first_range_of_yul_returns_declaration(node: &YulReturnsDeclaration) -> Range<usize> {
    first_range_seq_yul_returns_declaration(node.as_ref())
}

fn first_range_seq_yul_switch_statement(node: &YulSwitchStatementStruct) -> Range<usize> {
    first_range_of_yul_switch_keyword(&node.switch_keyword)
}

fn first_range_of_yul_switch_statement(node: &YulSwitchStatement) -> Range<usize> {
    first_range_seq_yul_switch_statement(node.as_ref())
}

fn first_range_seq_yul_value_case(node: &YulValueCaseStruct) -> Range<usize> {
    first_range_of_yul_case_keyword(&node.case_keyword)
}

fn first_range_of_yul_value_case(node: &YulValueCase) -> Range<usize> {
    first_range_seq_yul_value_case(node.as_ref())
}

fn first_range_seq_yul_variable_assignment_statement(
    node: &YulVariableAssignmentStatementStruct,
) -> Range<usize> {
    first_range_of_yul_paths(&node.variables)
}

fn first_range_of_yul_variable_assignment_statement(
    node: &YulVariableAssignmentStatement,
) -> Range<usize> {
    first_range_seq_yul_variable_assignment_statement(node.as_ref())
}

fn first_range_seq_yul_variable_declaration_statement(
    node: &YulVariableDeclarationStatementStruct,
) -> Range<usize> {
    first_range_of_yul_let_keyword(&node.let_keyword)
}

fn first_range_of_yul_variable_declaration_statement(
    node: &YulVariableDeclarationStatement,
) -> Range<usize> {
    first_range_seq_yul_variable_declaration_statement(node.as_ref())
}

fn first_range_seq_yul_variable_declaration_value(
    node: &YulVariableDeclarationValueStruct,
) -> Range<usize> {
    first_range_of_yul_colon_equal(&node.assignment)
}

fn first_range_of_yul_variable_declaration_value(
    node: &YulVariableDeclarationValue,
) -> Range<usize> {
    first_range_seq_yul_variable_declaration_value(node.as_ref())
}

fn first_range_of_abicoder_version(node: &AbicoderVersion) -> Range<usize> {
    match node {
        AbicoderVersion::AbicoderV1Keyword(child) => first_range_of_abicoder_v1_keyword(child),
        AbicoderVersion::AbicoderV2Keyword(child) => first_range_of_abicoder_v2_keyword(child),
    }
}

fn first_range_of_arguments_declaration(node: &ArgumentsDeclaration) -> Range<usize> {
    match node {
        ArgumentsDeclaration::PositionalArgumentsDeclaration(child) => {
            first_range_of_positional_arguments_declaration(child)
        }
        ArgumentsDeclaration::NamedArgumentsDeclaration(child) => {
            first_range_of_named_arguments_declaration(child)
        }
    }
}

fn first_range_of_constructor_attribute(node: &ConstructorAttribute) -> Range<usize> {
    match node {
        ConstructorAttribute::ModifierInvocation(child) => {
            first_range_of_modifier_invocation(child)
        }
        ConstructorAttribute::InternalKeyword(child) => first_range_of_internal_keyword(child),
        ConstructorAttribute::PayableKeyword(child) => first_range_of_payable_keyword(child),
        ConstructorAttribute::PublicKeyword(child) => first_range_of_public_keyword(child),
    }
}

fn first_range_of_contract_member(node: &ContractMember) -> Range<usize> {
    match node {
        ContractMember::UsingDirective(child) => first_range_of_using_directive(child),
        ContractMember::FunctionDefinition(child) => first_range_of_function_definition(child),
        ContractMember::ConstructorDefinition(child) => {
            first_range_of_constructor_definition(child)
        }
        ContractMember::ReceiveFunctionDefinition(child) => {
            first_range_of_receive_function_definition(child)
        }
        ContractMember::FallbackFunctionDefinition(child) => {
            first_range_of_fallback_function_definition(child)
        }
        ContractMember::ModifierDefinition(child) => first_range_of_modifier_definition(child),
        ContractMember::StructDefinition(child) => first_range_of_struct_definition(child),
        ContractMember::EnumDefinition(child) => first_range_of_enum_definition(child),
        ContractMember::EventDefinition(child) => first_range_of_event_definition(child),
        ContractMember::ErrorDefinition(child) => first_range_of_error_definition(child),
        ContractMember::UserDefinedValueTypeDefinition(child) => {
            first_range_of_user_defined_value_type_definition(child)
        }
        ContractMember::StateVariableDefinition(child) => {
            first_range_of_state_variable_definition(child)
        }
    }
}

fn first_range_of_contract_specifier(node: &ContractSpecifier) -> Range<usize> {
    match node {
        ContractSpecifier::InheritanceSpecifier(child) => {
            first_range_of_inheritance_specifier(child)
        }
        ContractSpecifier::StorageLayoutSpecifier(child) => {
            first_range_of_storage_layout_specifier(child)
        }
    }
}

fn first_range_of_elementary_type(node: &ElementaryType) -> Range<usize> {
    match node {
        ElementaryType::BoolKeyword(child) => first_range_of_bool_keyword(child),
        ElementaryType::StringKeyword(child) => first_range_of_string_keyword(child),
        ElementaryType::AddressType(child) => first_range_of_address_type(child),
        ElementaryType::BytesKeyword(child) => first_range_of_bytes_keyword(child),
        ElementaryType::IntKeyword(child) => first_range_of_int_keyword(child),
        ElementaryType::UintKeyword(child) => first_range_of_uint_keyword(child),
        ElementaryType::FixedKeyword(child) => first_range_of_fixed_keyword(child),
        ElementaryType::UfixedKeyword(child) => first_range_of_ufixed_keyword(child),
    }
}

fn first_range_of_experimental_feature(node: &ExperimentalFeature) -> Range<usize> {
    match node {
        ExperimentalFeature::ABIEncoderV2Keyword(child) => {
            first_range_of_abi_encoder_v2_keyword(child)
        }
        ExperimentalFeature::SMTCheckerKeyword(child) => first_range_of_smt_checker_keyword(child),
        ExperimentalFeature::PragmaStringLiteral(child) => {
            first_range_of_pragma_string_literal(child)
        }
    }
}

fn first_range_of_expression(node: &Expression) -> Range<usize> {
    match node {
        Expression::AssignmentExpression(child) => first_range_of_assignment_expression(child),
        Expression::ConditionalExpression(child) => first_range_of_conditional_expression(child),
        Expression::OrExpression(child) => first_range_of_or_expression(child),
        Expression::AndExpression(child) => first_range_of_and_expression(child),
        Expression::EqualityExpression(child) => first_range_of_equality_expression(child),
        Expression::InequalityExpression(child) => first_range_of_inequality_expression(child),
        Expression::BitwiseOrExpression(child) => first_range_of_bitwise_or_expression(child),
        Expression::BitwiseXorExpression(child) => first_range_of_bitwise_xor_expression(child),
        Expression::BitwiseAndExpression(child) => first_range_of_bitwise_and_expression(child),
        Expression::ShiftExpression(child) => first_range_of_shift_expression(child),
        Expression::AdditiveExpression(child) => first_range_of_additive_expression(child),
        Expression::MultiplicativeExpression(child) => {
            first_range_of_multiplicative_expression(child)
        }
        Expression::ExponentiationExpression(child) => {
            first_range_of_exponentiation_expression(child)
        }
        Expression::PostfixExpression(child) => first_range_of_postfix_expression(child),
        Expression::PrefixExpression(child) => first_range_of_prefix_expression(child),
        Expression::FunctionCallExpression(child) => first_range_of_function_call_expression(child),
        Expression::CallOptionsExpression(child) => first_range_of_call_options_expression(child),
        Expression::MemberAccessExpression(child) => first_range_of_member_access_expression(child),
        Expression::IndexAccessExpression(child) => first_range_of_index_access_expression(child),
        Expression::NewExpression(child) => first_range_of_new_expression(child),
        Expression::TupleExpression(child) => first_range_of_tuple_expression(child),
        Expression::TypeExpression(child) => first_range_of_type_expression(child),
        Expression::ArrayExpression(child) => first_range_of_array_expression(child),
        Expression::HexNumberExpression(child) => first_range_of_hex_number_expression(child),
        Expression::DecimalNumberExpression(child) => {
            first_range_of_decimal_number_expression(child)
        }
        Expression::StringExpression(child) => first_range_of_string_expression(child),
        Expression::ElementaryType(child) => first_range_of_elementary_type(child),
        Expression::PayableKeyword(child) => first_range_of_payable_keyword(child),
        Expression::ThisKeyword(child) => first_range_of_this_keyword(child),
        Expression::SuperKeyword(child) => first_range_of_super_keyword(child),
        Expression::TrueKeyword(child) => first_range_of_true_keyword(child),
        Expression::FalseKeyword(child) => first_range_of_false_keyword(child),
        Expression::Identifier(child) => first_range_of_identifier(child),
    }
}

fn first_range_of_expression_additive_expression_operator(
    node: &Expression_AdditiveExpression_Operator,
) -> Range<usize> {
    match node {
        Expression_AdditiveExpression_Operator::Minus(child) => first_range_of_minus(child),
        Expression_AdditiveExpression_Operator::Plus(child) => first_range_of_plus(child),
    }
}

fn first_range_of_expression_assignment_expression_operator(
    node: &Expression_AssignmentExpression_Operator,
) -> Range<usize> {
    match node {
        Expression_AssignmentExpression_Operator::AmpersandEqual(child) => {
            first_range_of_ampersand_equal(child)
        }
        Expression_AssignmentExpression_Operator::AsteriskEqual(child) => {
            first_range_of_asterisk_equal(child)
        }
        Expression_AssignmentExpression_Operator::BarEqual(child) => {
            first_range_of_bar_equal(child)
        }
        Expression_AssignmentExpression_Operator::CaretEqual(child) => {
            first_range_of_caret_equal(child)
        }
        Expression_AssignmentExpression_Operator::Equal(child) => first_range_of_equal(child),
        Expression_AssignmentExpression_Operator::GreaterThanGreaterThanEqual(child) => {
            first_range_of_greater_than_greater_than_equal(child)
        }
        Expression_AssignmentExpression_Operator::GreaterThanGreaterThanGreaterThanEqual(child) => {
            first_range_of_greater_than_greater_than_greater_than_equal(child)
        }
        Expression_AssignmentExpression_Operator::LessThanLessThanEqual(child) => {
            first_range_of_less_than_less_than_equal(child)
        }
        Expression_AssignmentExpression_Operator::MinusEqual(child) => {
            first_range_of_minus_equal(child)
        }
        Expression_AssignmentExpression_Operator::PercentEqual(child) => {
            first_range_of_percent_equal(child)
        }
        Expression_AssignmentExpression_Operator::PlusEqual(child) => {
            first_range_of_plus_equal(child)
        }
        Expression_AssignmentExpression_Operator::SlashEqual(child) => {
            first_range_of_slash_equal(child)
        }
    }
}

fn first_range_of_expression_equality_expression_operator(
    node: &Expression_EqualityExpression_Operator,
) -> Range<usize> {
    match node {
        Expression_EqualityExpression_Operator::BangEqual(child) => {
            first_range_of_bang_equal(child)
        }
        Expression_EqualityExpression_Operator::EqualEqual(child) => {
            first_range_of_equal_equal(child)
        }
    }
}

fn first_range_of_expression_inequality_expression_operator(
    node: &Expression_InequalityExpression_Operator,
) -> Range<usize> {
    match node {
        Expression_InequalityExpression_Operator::GreaterThan(child) => {
            first_range_of_greater_than(child)
        }
        Expression_InequalityExpression_Operator::GreaterThanEqual(child) => {
            first_range_of_greater_than_equal(child)
        }
        Expression_InequalityExpression_Operator::LessThan(child) => {
            first_range_of_less_than(child)
        }
        Expression_InequalityExpression_Operator::LessThanEqual(child) => {
            first_range_of_less_than_equal(child)
        }
    }
}

fn first_range_of_expression_multiplicative_expression_operator(
    node: &Expression_MultiplicativeExpression_Operator,
) -> Range<usize> {
    match node {
        Expression_MultiplicativeExpression_Operator::Asterisk(child) => {
            first_range_of_asterisk(child)
        }
        Expression_MultiplicativeExpression_Operator::Percent(child) => {
            first_range_of_percent(child)
        }
        Expression_MultiplicativeExpression_Operator::Slash(child) => first_range_of_slash(child),
    }
}

fn first_range_of_expression_postfix_expression_operator(
    node: &Expression_PostfixExpression_Operator,
) -> Range<usize> {
    match node {
        Expression_PostfixExpression_Operator::MinusMinus(child) => {
            first_range_of_minus_minus(child)
        }
        Expression_PostfixExpression_Operator::PlusPlus(child) => first_range_of_plus_plus(child),
    }
}

fn first_range_of_expression_prefix_expression_operator(
    node: &Expression_PrefixExpression_Operator,
) -> Range<usize> {
    match node {
        Expression_PrefixExpression_Operator::Bang(child) => first_range_of_bang(child),
        Expression_PrefixExpression_Operator::DeleteKeyword(child) => {
            first_range_of_delete_keyword(child)
        }
        Expression_PrefixExpression_Operator::Minus(child) => first_range_of_minus(child),
        Expression_PrefixExpression_Operator::MinusMinus(child) => {
            first_range_of_minus_minus(child)
        }
        Expression_PrefixExpression_Operator::PlusPlus(child) => first_range_of_plus_plus(child),
        Expression_PrefixExpression_Operator::Tilde(child) => first_range_of_tilde(child),
    }
}

fn first_range_of_expression_shift_expression_operator(
    node: &Expression_ShiftExpression_Operator,
) -> Range<usize> {
    match node {
        Expression_ShiftExpression_Operator::GreaterThanGreaterThan(child) => {
            first_range_of_greater_than_greater_than(child)
        }
        Expression_ShiftExpression_Operator::GreaterThanGreaterThanGreaterThan(child) => {
            first_range_of_greater_than_greater_than_greater_than(child)
        }
        Expression_ShiftExpression_Operator::LessThanLessThan(child) => {
            first_range_of_less_than_less_than(child)
        }
    }
}

fn first_range_of_fallback_function_attribute(node: &FallbackFunctionAttribute) -> Range<usize> {
    match node {
        FallbackFunctionAttribute::ModifierInvocation(child) => {
            first_range_of_modifier_invocation(child)
        }
        FallbackFunctionAttribute::OverrideSpecifier(child) => {
            first_range_of_override_specifier(child)
        }
        FallbackFunctionAttribute::ExternalKeyword(child) => first_range_of_external_keyword(child),
        FallbackFunctionAttribute::PayableKeyword(child) => first_range_of_payable_keyword(child),
        FallbackFunctionAttribute::PureKeyword(child) => first_range_of_pure_keyword(child),
        FallbackFunctionAttribute::ViewKeyword(child) => first_range_of_view_keyword(child),
        FallbackFunctionAttribute::VirtualKeyword(child) => first_range_of_virtual_keyword(child),
    }
}

fn first_range_of_for_statement_condition(node: &ForStatementCondition) -> Range<usize> {
    match node {
        ForStatementCondition::ExpressionStatement(child) => {
            first_range_of_expression_statement(child)
        }
        ForStatementCondition::Semicolon(child) => first_range_of_semicolon(child),
    }
}

fn first_range_of_for_statement_initialization(node: &ForStatementInitialization) -> Range<usize> {
    match node {
        ForStatementInitialization::VariableDeclarationStatement(child) => {
            first_range_of_variable_declaration_statement(child)
        }
        ForStatementInitialization::ExpressionStatement(child) => {
            first_range_of_expression_statement(child)
        }
        ForStatementInitialization::Semicolon(child) => first_range_of_semicolon(child),
    }
}

fn first_range_of_function_attribute(node: &FunctionAttribute) -> Range<usize> {
    match node {
        FunctionAttribute::ModifierInvocation(child) => first_range_of_modifier_invocation(child),
        FunctionAttribute::OverrideSpecifier(child) => first_range_of_override_specifier(child),
        FunctionAttribute::ExternalKeyword(child) => first_range_of_external_keyword(child),
        FunctionAttribute::InternalKeyword(child) => first_range_of_internal_keyword(child),
        FunctionAttribute::PayableKeyword(child) => first_range_of_payable_keyword(child),
        FunctionAttribute::PrivateKeyword(child) => first_range_of_private_keyword(child),
        FunctionAttribute::PublicKeyword(child) => first_range_of_public_keyword(child),
        FunctionAttribute::PureKeyword(child) => first_range_of_pure_keyword(child),
        FunctionAttribute::ViewKeyword(child) => first_range_of_view_keyword(child),
        FunctionAttribute::VirtualKeyword(child) => first_range_of_virtual_keyword(child),
    }
}

fn first_range_of_function_body(node: &FunctionBody) -> Range<usize> {
    match node {
        FunctionBody::Block(child) => first_range_of_block(child),
        FunctionBody::Semicolon(child) => first_range_of_semicolon(child),
    }
}

fn first_range_of_function_name(node: &FunctionName) -> Range<usize> {
    match node {
        FunctionName::Identifier(child) => first_range_of_identifier(child),
        FunctionName::FallbackKeyword(child) => first_range_of_fallback_keyword(child),
        FunctionName::ReceiveKeyword(child) => first_range_of_receive_keyword(child),
    }
}

fn first_range_of_function_type_attribute(node: &FunctionTypeAttribute) -> Range<usize> {
    match node {
        FunctionTypeAttribute::InternalKeyword(child) => first_range_of_internal_keyword(child),
        FunctionTypeAttribute::ExternalKeyword(child) => first_range_of_external_keyword(child),
        FunctionTypeAttribute::PrivateKeyword(child) => first_range_of_private_keyword(child),
        FunctionTypeAttribute::PublicKeyword(child) => first_range_of_public_keyword(child),
        FunctionTypeAttribute::PureKeyword(child) => first_range_of_pure_keyword(child),
        FunctionTypeAttribute::ViewKeyword(child) => first_range_of_view_keyword(child),
        FunctionTypeAttribute::PayableKeyword(child) => first_range_of_payable_keyword(child),
    }
}

fn first_range_of_identifier_path_element(node: &IdentifierPathElement) -> Range<usize> {
    match node {
        IdentifierPathElement::Identifier(child) => first_range_of_identifier(child),
        IdentifierPathElement::AddressKeyword(child) => first_range_of_address_keyword(child),
    }
}

fn first_range_of_import_clause(node: &ImportClause) -> Range<usize> {
    match node {
        ImportClause::PathImport(child) => first_range_of_path_import(child),
        ImportClause::NamedImport(child) => first_range_of_named_import(child),
        ImportClause::ImportDeconstruction(child) => first_range_of_import_deconstruction(child),
    }
}

fn first_range_of_mapping_key_type(node: &MappingKeyType) -> Range<usize> {
    match node {
        MappingKeyType::ElementaryType(child) => first_range_of_elementary_type(child),
        MappingKeyType::IdentifierPath(child) => first_range_of_identifier_path(child),
    }
}

fn first_range_of_modifier_attribute(node: &ModifierAttribute) -> Range<usize> {
    match node {
        ModifierAttribute::OverrideSpecifier(child) => first_range_of_override_specifier(child),
        ModifierAttribute::VirtualKeyword(child) => first_range_of_virtual_keyword(child),
    }
}

fn first_range_of_number_unit(node: &NumberUnit) -> Range<usize> {
    match node {
        NumberUnit::WeiKeyword(child) => first_range_of_wei_keyword(child),
        NumberUnit::GweiKeyword(child) => first_range_of_gwei_keyword(child),
        NumberUnit::EtherKeyword(child) => first_range_of_ether_keyword(child),
        NumberUnit::SecondsKeyword(child) => first_range_of_seconds_keyword(child),
        NumberUnit::MinutesKeyword(child) => first_range_of_minutes_keyword(child),
        NumberUnit::HoursKeyword(child) => first_range_of_hours_keyword(child),
        NumberUnit::DaysKeyword(child) => first_range_of_days_keyword(child),
        NumberUnit::WeeksKeyword(child) => first_range_of_weeks_keyword(child),
    }
}

fn first_range_of_pragma(node: &Pragma) -> Range<usize> {
    match node {
        Pragma::VersionPragma(child) => first_range_of_version_pragma(child),
        Pragma::AbicoderPragma(child) => first_range_of_abicoder_pragma(child),
        Pragma::ExperimentalPragma(child) => first_range_of_experimental_pragma(child),
    }
}

fn first_range_of_receive_function_attribute(node: &ReceiveFunctionAttribute) -> Range<usize> {
    match node {
        ReceiveFunctionAttribute::ModifierInvocation(child) => {
            first_range_of_modifier_invocation(child)
        }
        ReceiveFunctionAttribute::OverrideSpecifier(child) => {
            first_range_of_override_specifier(child)
        }
        ReceiveFunctionAttribute::ExternalKeyword(child) => first_range_of_external_keyword(child),
        ReceiveFunctionAttribute::PayableKeyword(child) => first_range_of_payable_keyword(child),
        ReceiveFunctionAttribute::VirtualKeyword(child) => first_range_of_virtual_keyword(child),
    }
}

fn first_range_of_source_unit_member(node: &SourceUnitMember) -> Range<usize> {
    match node {
        SourceUnitMember::PragmaDirective(child) => first_range_of_pragma_directive(child),
        SourceUnitMember::ImportDirective(child) => first_range_of_import_directive(child),
        SourceUnitMember::ContractDefinition(child) => first_range_of_contract_definition(child),
        SourceUnitMember::InterfaceDefinition(child) => first_range_of_interface_definition(child),
        SourceUnitMember::LibraryDefinition(child) => first_range_of_library_definition(child),
        SourceUnitMember::StructDefinition(child) => first_range_of_struct_definition(child),
        SourceUnitMember::EnumDefinition(child) => first_range_of_enum_definition(child),
        SourceUnitMember::FunctionDefinition(child) => first_range_of_function_definition(child),
        SourceUnitMember::ErrorDefinition(child) => first_range_of_error_definition(child),
        SourceUnitMember::UserDefinedValueTypeDefinition(child) => {
            first_range_of_user_defined_value_type_definition(child)
        }
        SourceUnitMember::UsingDirective(child) => first_range_of_using_directive(child),
        SourceUnitMember::EventDefinition(child) => first_range_of_event_definition(child),
        SourceUnitMember::ConstantDefinition(child) => first_range_of_constant_definition(child),
    }
}

fn first_range_of_state_variable_attribute(node: &StateVariableAttribute) -> Range<usize> {
    match node {
        StateVariableAttribute::OverrideSpecifier(child) => {
            first_range_of_override_specifier(child)
        }
        StateVariableAttribute::ConstantKeyword(child) => first_range_of_constant_keyword(child),
        StateVariableAttribute::InternalKeyword(child) => first_range_of_internal_keyword(child),
        StateVariableAttribute::PrivateKeyword(child) => first_range_of_private_keyword(child),
        StateVariableAttribute::PublicKeyword(child) => first_range_of_public_keyword(child),
        StateVariableAttribute::ImmutableKeyword(child) => first_range_of_immutable_keyword(child),
        StateVariableAttribute::TransientKeyword(child) => first_range_of_transient_keyword(child),
    }
}

fn first_range_of_statement(node: &Statement) -> Range<usize> {
    match node {
        Statement::IfStatement(child) => first_range_of_if_statement(child),
        Statement::ForStatement(child) => first_range_of_for_statement(child),
        Statement::WhileStatement(child) => first_range_of_while_statement(child),
        Statement::DoWhileStatement(child) => first_range_of_do_while_statement(child),
        Statement::ContinueStatement(child) => first_range_of_continue_statement(child),
        Statement::BreakStatement(child) => first_range_of_break_statement(child),
        Statement::ReturnStatement(child) => first_range_of_return_statement(child),
        Statement::EmitStatement(child) => first_range_of_emit_statement(child),
        Statement::TryStatement(child) => first_range_of_try_statement(child),
        Statement::RevertStatement(child) => first_range_of_revert_statement(child),
        Statement::AssemblyStatement(child) => first_range_of_assembly_statement(child),
        Statement::Block(child) => first_range_of_block(child),
        Statement::UncheckedBlock(child) => first_range_of_unchecked_block(child),
        Statement::VariableDeclarationStatement(child) => {
            first_range_of_variable_declaration_statement(child)
        }
        Statement::ExpressionStatement(child) => first_range_of_expression_statement(child),
    }
}

fn first_range_of_storage_location(node: &StorageLocation) -> Range<usize> {
    match node {
        StorageLocation::MemoryKeyword(child) => first_range_of_memory_keyword(child),
        StorageLocation::StorageKeyword(child) => first_range_of_storage_keyword(child),
        StorageLocation::CallDataKeyword(child) => first_range_of_call_data_keyword(child),
    }
}

fn first_range_of_string_expression(node: &StringExpression) -> Range<usize> {
    match node {
        StringExpression::StringLiterals(child) => first_range_of_string_literals(child),
        StringExpression::HexStringLiterals(child) => first_range_of_hex_string_literals(child),
        StringExpression::UnicodeStringLiterals(child) => {
            first_range_of_unicode_string_literals(child)
        }
    }
}

fn first_range_of_type_name(node: &TypeName) -> Range<usize> {
    match node {
        TypeName::ArrayTypeName(child) => first_range_of_array_type_name(child),
        TypeName::FunctionType(child) => first_range_of_function_type(child),
        TypeName::MappingType(child) => first_range_of_mapping_type(child),
        TypeName::ElementaryType(child) => first_range_of_elementary_type(child),
        TypeName::IdentifierPath(child) => first_range_of_identifier_path(child),
    }
}

fn first_range_of_using_clause(node: &UsingClause) -> Range<usize> {
    match node {
        UsingClause::IdentifierPath(child) => first_range_of_identifier_path(child),
        UsingClause::UsingDeconstruction(child) => first_range_of_using_deconstruction(child),
    }
}

fn first_range_of_using_operator(node: &UsingOperator) -> Range<usize> {
    match node {
        UsingOperator::Ampersand(child) => first_range_of_ampersand(child),
        UsingOperator::Asterisk(child) => first_range_of_asterisk(child),
        UsingOperator::BangEqual(child) => first_range_of_bang_equal(child),
        UsingOperator::Bar(child) => first_range_of_bar(child),
        UsingOperator::Caret(child) => first_range_of_caret(child),
        UsingOperator::EqualEqual(child) => first_range_of_equal_equal(child),
        UsingOperator::GreaterThan(child) => first_range_of_greater_than(child),
        UsingOperator::GreaterThanEqual(child) => first_range_of_greater_than_equal(child),
        UsingOperator::LessThan(child) => first_range_of_less_than(child),
        UsingOperator::LessThanEqual(child) => first_range_of_less_than_equal(child),
        UsingOperator::Minus(child) => first_range_of_minus(child),
        UsingOperator::Percent(child) => first_range_of_percent(child),
        UsingOperator::Plus(child) => first_range_of_plus(child),
        UsingOperator::Slash(child) => first_range_of_slash(child),
        UsingOperator::Tilde(child) => first_range_of_tilde(child),
    }
}

fn first_range_of_using_target(node: &UsingTarget) -> Range<usize> {
    match node {
        UsingTarget::TypeName(child) => first_range_of_type_name(child),
        UsingTarget::Asterisk(child) => first_range_of_asterisk(child),
    }
}

fn first_range_of_variable_declaration_target(node: &VariableDeclarationTarget) -> Range<usize> {
    match node {
        VariableDeclarationTarget::SingleTypedDeclaration(child) => {
            first_range_of_single_typed_declaration(child)
        }
        VariableDeclarationTarget::MultiTypedDeclaration(child) => {
            first_range_of_multi_typed_declaration(child)
        }
    }
}

fn first_range_of_version_expression(node: &VersionExpression) -> Range<usize> {
    match node {
        VersionExpression::VersionRange(child) => first_range_of_version_range(child),
        VersionExpression::VersionTerm(child) => first_range_of_version_term(child),
    }
}

fn first_range_of_version_literal(node: &VersionLiteral) -> Range<usize> {
    match node {
        VersionLiteral::SimpleVersionLiteral(child) => first_range_of_simple_version_literal(child),
        VersionLiteral::PragmaStringLiteral(child) => first_range_of_pragma_string_literal(child),
    }
}

fn first_range_of_version_operator(node: &VersionOperator) -> Range<usize> {
    match node {
        VersionOperator::PragmaCaret(child) => first_range_of_pragma_caret(child),
        VersionOperator::PragmaTilde(child) => first_range_of_pragma_tilde(child),
        VersionOperator::PragmaEqual(child) => first_range_of_pragma_equal(child),
        VersionOperator::PragmaLessThan(child) => first_range_of_pragma_less_than(child),
        VersionOperator::PragmaGreaterThan(child) => first_range_of_pragma_greater_than(child),
        VersionOperator::PragmaLessThanEqual(child) => first_range_of_pragma_less_than_equal(child),
        VersionOperator::PragmaGreaterThanEqual(child) => {
            first_range_of_pragma_greater_than_equal(child)
        }
    }
}

fn first_range_of_yul_expression(node: &YulExpression) -> Range<usize> {
    match node {
        YulExpression::YulFunctionCallExpression(child) => {
            first_range_of_yul_function_call_expression(child)
        }
        YulExpression::YulLiteral(child) => first_range_of_yul_literal(child),
        YulExpression::YulPath(child) => first_range_of_yul_path(child),
    }
}

fn first_range_of_yul_literal(node: &YulLiteral) -> Range<usize> {
    match node {
        YulLiteral::YulTrueKeyword(child) => first_range_of_yul_true_keyword(child),
        YulLiteral::YulFalseKeyword(child) => first_range_of_yul_false_keyword(child),
        YulLiteral::YulDecimalLiteral(child) => first_range_of_yul_decimal_literal(child),
        YulLiteral::YulHexLiteral(child) => first_range_of_yul_hex_literal(child),
        YulLiteral::YulHexStringLiteral(child) => first_range_of_yul_hex_string_literal(child),
        YulLiteral::YulStringLiteral(child) => first_range_of_yul_string_literal(child),
    }
}

fn first_range_of_yul_statement(node: &YulStatement) -> Range<usize> {
    match node {
        YulStatement::YulBlock(child) => first_range_of_yul_block(child),
        YulStatement::YulFunctionDefinition(child) => first_range_of_yul_function_definition(child),
        YulStatement::YulIfStatement(child) => first_range_of_yul_if_statement(child),
        YulStatement::YulForStatement(child) => first_range_of_yul_for_statement(child),
        YulStatement::YulSwitchStatement(child) => first_range_of_yul_switch_statement(child),
        YulStatement::YulLeaveStatement(child) => first_range_of_yul_leave_statement(child),
        YulStatement::YulBreakStatement(child) => first_range_of_yul_break_statement(child),
        YulStatement::YulContinueStatement(child) => first_range_of_yul_continue_statement(child),
        YulStatement::YulVariableAssignmentStatement(child) => {
            first_range_of_yul_variable_assignment_statement(child)
        }
        YulStatement::YulVariableDeclarationStatement(child) => {
            first_range_of_yul_variable_declaration_statement(child)
        }
        YulStatement::YulExpression(child) => first_range_of_yul_expression(child),
    }
}

fn first_range_of_yul_switch_case(node: &YulSwitchCase) -> Range<usize> {
    match node {
        YulSwitchCase::YulDefaultCase(child) => first_range_of_yul_default_case(child),
        YulSwitchCase::YulValueCase(child) => first_range_of_yul_value_case(child),
    }
}

fn first_range_of_array_values(node: &ArrayValues) -> Range<usize> {
    node.elements
        .first()
        .map_or(0..0, |child| first_range_of_expression(child))
}

fn first_range_of_call_options(node: &CallOptions) -> Range<usize> {
    node.elements
        .first()
        .map_or(0..0, |child| first_range_of_named_argument(child))
}

fn first_range_of_catch_clauses(node: &CatchClauses) -> Range<usize> {
    node.elements
        .first()
        .map_or(0..0, |child| first_range_of_catch_clause(child))
}

fn first_range_of_constructor_attributes(node: &ConstructorAttributes) -> Range<usize> {
    node.elements
        .first()
        .map_or(0..0, |child| first_range_of_constructor_attribute(child))
}

fn first_range_of_contract_members(node: &ContractMembers) -> Range<usize> {
    node.elements
        .first()
        .map_or(0..0, |child| first_range_of_contract_member(child))
}

fn first_range_of_contract_specifiers(node: &ContractSpecifiers) -> Range<usize> {
    node.elements
        .first()
        .map_or(0..0, |child| first_range_of_contract_specifier(child))
}

fn first_range_of_enum_members(node: &EnumMembers) -> Range<usize> {
    node.elements
        .first()
        .map_or(0..0, |child| first_range_of_identifier(child))
}

fn first_range_of_error_parameters(node: &ErrorParameters) -> Range<usize> {
    node.elements
        .first()
        .map_or(0..0, |child| first_range_of_error_parameter(child))
}

fn first_range_of_event_parameters(node: &EventParameters) -> Range<usize> {
    node.elements
        .first()
        .map_or(0..0, |child| first_range_of_event_parameter(child))
}

fn first_range_of_fallback_function_attributes(node: &FallbackFunctionAttributes) -> Range<usize> {
    node.elements.first().map_or(0..0, |child| {
        first_range_of_fallback_function_attribute(child)
    })
}

fn first_range_of_function_attributes(node: &FunctionAttributes) -> Range<usize> {
    node.elements
        .first()
        .map_or(0..0, |child| first_range_of_function_attribute(child))
}

fn first_range_of_function_type_attributes(node: &FunctionTypeAttributes) -> Range<usize> {
    node.elements
        .first()
        .map_or(0..0, |child| first_range_of_function_type_attribute(child))
}

fn first_range_of_hex_string_literals(node: &HexStringLiterals) -> Range<usize> {
    node.elements
        .first()
        .map_or(0..0, |child| first_range_of_hex_string_literal(child))
}

fn first_range_of_identifier_path(node: &IdentifierPath) -> Range<usize> {
    node.elements
        .first()
        .map_or(0..0, |child| first_range_of_identifier_path_element(child))
}

fn first_range_of_import_deconstruction_symbols(
    node: &ImportDeconstructionSymbols,
) -> Range<usize> {
    node.elements.first().map_or(0..0, |child| {
        first_range_of_import_deconstruction_symbol(child)
    })
}

fn first_range_of_inheritance_types(node: &InheritanceTypes) -> Range<usize> {
    node.elements
        .first()
        .map_or(0..0, |child| first_range_of_inheritance_type(child))
}

fn first_range_of_interface_members(node: &InterfaceMembers) -> Range<usize> {
    node.elements
        .first()
        .map_or(0..0, |child| first_range_of_contract_member(child))
}

fn first_range_of_library_members(node: &LibraryMembers) -> Range<usize> {
    node.elements
        .first()
        .map_or(0..0, |child| first_range_of_contract_member(child))
}

fn first_range_of_modifier_attributes(node: &ModifierAttributes) -> Range<usize> {
    node.elements
        .first()
        .map_or(0..0, |child| first_range_of_modifier_attribute(child))
}

fn first_range_of_multi_typed_declaration_elements(
    node: &MultiTypedDeclarationElements,
) -> Range<usize> {
    node.elements.first().map_or(0..0, |child| {
        first_range_of_multi_typed_declaration_element(child)
    })
}

fn first_range_of_named_arguments(node: &NamedArguments) -> Range<usize> {
    node.elements
        .first()
        .map_or(0..0, |child| first_range_of_named_argument(child))
}

fn first_range_of_override_paths(node: &OverridePaths) -> Range<usize> {
    node.elements
        .first()
        .map_or(0..0, |child| first_range_of_identifier_path(child))
}

fn first_range_of_parameters(node: &Parameters) -> Range<usize> {
    node.elements
        .first()
        .map_or(0..0, |child| first_range_of_parameter(child))
}

fn first_range_of_positional_arguments(node: &PositionalArguments) -> Range<usize> {
    node.elements
        .first()
        .map_or(0..0, |child| first_range_of_expression(child))
}

fn first_range_of_receive_function_attributes(node: &ReceiveFunctionAttributes) -> Range<usize> {
    node.elements.first().map_or(0..0, |child| {
        first_range_of_receive_function_attribute(child)
    })
}

fn first_range_of_simple_version_literal(node: &SimpleVersionLiteral) -> Range<usize> {
    node.elements
        .first()
        .map_or(0..0, |child| first_range_of_version_specifier(child))
}

fn first_range_of_source_unit_members(node: &SourceUnitMembers) -> Range<usize> {
    node.elements
        .first()
        .map_or(0..0, |child| first_range_of_source_unit_member(child))
}

fn first_range_of_state_variable_attributes(node: &StateVariableAttributes) -> Range<usize> {
    node.elements
        .first()
        .map_or(0..0, |child| first_range_of_state_variable_attribute(child))
}

fn first_range_of_statements(node: &Statements) -> Range<usize> {
    node.elements
        .first()
        .map_or(0..0, |child| first_range_of_statement(child))
}

fn first_range_of_string_literals(node: &StringLiterals) -> Range<usize> {
    node.elements
        .first()
        .map_or(0..0, |child| first_range_of_string_literal(child))
}

fn first_range_of_struct_members(node: &StructMembers) -> Range<usize> {
    node.elements
        .first()
        .map_or(0..0, |child| first_range_of_struct_member(child))
}

fn first_range_of_tuple_values(node: &TupleValues) -> Range<usize> {
    node.elements
        .first()
        .map_or(0..0, |child| first_range_of_tuple_value(child))
}

fn first_range_of_unicode_string_literals(node: &UnicodeStringLiterals) -> Range<usize> {
    node.elements
        .first()
        .map_or(0..0, |child| first_range_of_unicode_string_literal(child))
}

fn first_range_of_using_deconstruction_symbols(node: &UsingDeconstructionSymbols) -> Range<usize> {
    node.elements.first().map_or(0..0, |child| {
        first_range_of_using_deconstruction_symbol(child)
    })
}

fn first_range_of_version_expression_set(node: &VersionExpressionSet) -> Range<usize> {
    node.elements
        .first()
        .map_or(0..0, |child| first_range_of_version_expression(child))
}

fn first_range_of_version_expression_sets(node: &VersionExpressionSets) -> Range<usize> {
    node.elements
        .first()
        .map_or(0..0, |child| first_range_of_version_expression_set(child))
}

fn first_range_of_yul_arguments(node: &YulArguments) -> Range<usize> {
    node.elements
        .first()
        .map_or(0..0, |child| first_range_of_yul_expression(child))
}

fn first_range_of_yul_flags(node: &YulFlags) -> Range<usize> {
    node.elements
        .first()
        .map_or(0..0, |child| first_range_of_yul_string_literal(child))
}

fn first_range_of_yul_parameters(node: &YulParameters) -> Range<usize> {
    node.elements
        .first()
        .map_or(0..0, |child| first_range_of_yul_identifier(child))
}

fn first_range_of_yul_path(node: &YulPath) -> Range<usize> {
    node.elements
        .first()
        .map_or(0..0, |child| first_range_of_yul_identifier(child))
}

fn first_range_of_yul_paths(node: &YulPaths) -> Range<usize> {
    node.elements
        .first()
        .map_or(0..0, |child| first_range_of_yul_path(child))
}

fn first_range_of_yul_statements(node: &YulStatements) -> Range<usize> {
    node.elements
        .first()
        .map_or(0..0, |child| first_range_of_yul_statement(child))
}

fn first_range_of_yul_switch_cases(node: &YulSwitchCases) -> Range<usize> {
    node.elements
        .first()
        .map_or(0..0, |child| first_range_of_yul_switch_case(child))
}

fn first_range_of_yul_variable_names(node: &YulVariableNames) -> Range<usize> {
    node.elements
        .first()
        .map_or(0..0, |child| first_range_of_yul_identifier(child))
}

fn first_range_of_abi_encoder_v2_keyword(node: &ABIEncoderV2Keyword) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_abicoder_keyword(node: &AbicoderKeyword) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_abicoder_v1_keyword(node: &AbicoderV1Keyword) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_abicoder_v2_keyword(node: &AbicoderV2Keyword) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_abstract_keyword(node: &AbstractKeyword) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_address_keyword(node: &AddressKeyword) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_after_keyword(node: &AfterKeyword) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_alias_keyword(node: &AliasKeyword) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_ampersand(node: &Ampersand) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_ampersand_ampersand(node: &AmpersandAmpersand) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_ampersand_equal(node: &AmpersandEqual) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_anonymous_keyword(node: &AnonymousKeyword) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_apply_keyword(node: &ApplyKeyword) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_as_keyword(node: &AsKeyword) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_assembly_keyword(node: &AssemblyKeyword) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_asterisk(node: &Asterisk) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_asterisk_asterisk(node: &AsteriskAsterisk) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_asterisk_equal(node: &AsteriskEqual) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_at_keyword(node: &AtKeyword) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_auto_keyword(node: &AutoKeyword) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_bang(node: &Bang) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_bang_equal(node: &BangEqual) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_bar(node: &Bar) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_bar_bar(node: &BarBar) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_bar_equal(node: &BarEqual) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_bool_keyword(node: &BoolKeyword) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_break_keyword(node: &BreakKeyword) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_byte_keyword(node: &ByteKeyword) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_bytes_keyword(node: &BytesKeyword) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_call_data_keyword(node: &CallDataKeyword) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_caret(node: &Caret) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_caret_equal(node: &CaretEqual) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_case_keyword(node: &CaseKeyword) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_catch_keyword(node: &CatchKeyword) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_close_brace(node: &CloseBrace) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_close_bracket(node: &CloseBracket) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_close_paren(node: &CloseParen) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_colon(node: &Colon) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_comma(node: &Comma) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_constant_keyword(node: &ConstantKeyword) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_constructor_keyword(node: &ConstructorKeyword) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_continue_keyword(node: &ContinueKeyword) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_contract_keyword(node: &ContractKeyword) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_copy_of_keyword(node: &CopyOfKeyword) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_days_keyword(node: &DaysKeyword) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_decimal_literal(node: &DecimalLiteral) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_default_keyword(node: &DefaultKeyword) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_define_keyword(node: &DefineKeyword) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_delete_keyword(node: &DeleteKeyword) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_do_keyword(node: &DoKeyword) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_else_keyword(node: &ElseKeyword) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_emit_keyword(node: &EmitKeyword) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_enum_keyword(node: &EnumKeyword) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_equal(node: &Equal) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_equal_equal(node: &EqualEqual) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_equal_greater_than(node: &EqualGreaterThan) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_error_keyword(node: &ErrorKeyword) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_ether_keyword(node: &EtherKeyword) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_event_keyword(node: &EventKeyword) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_experimental_keyword(node: &ExperimentalKeyword) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_external_keyword(node: &ExternalKeyword) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_fallback_keyword(node: &FallbackKeyword) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_false_keyword(node: &FalseKeyword) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_final_keyword(node: &FinalKeyword) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_fixed_keyword(node: &FixedKeyword) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_for_keyword(node: &ForKeyword) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_from_keyword(node: &FromKeyword) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_function_keyword(node: &FunctionKeyword) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_global_keyword(node: &GlobalKeyword) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_greater_than(node: &GreaterThan) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_greater_than_equal(node: &GreaterThanEqual) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_greater_than_greater_than(node: &GreaterThanGreaterThan) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_greater_than_greater_than_equal(
    node: &GreaterThanGreaterThanEqual,
) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_greater_than_greater_than_greater_than(
    node: &GreaterThanGreaterThanGreaterThan,
) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_greater_than_greater_than_greater_than_equal(
    node: &GreaterThanGreaterThanGreaterThanEqual,
) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_gwei_keyword(node: &GweiKeyword) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_hex_keyword(node: &HexKeyword) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_hex_literal(node: &HexLiteral) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_hex_string_literal(node: &HexStringLiteral) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_hours_keyword(node: &HoursKeyword) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_identifier(node: &Identifier) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_if_keyword(node: &IfKeyword) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_immutable_keyword(node: &ImmutableKeyword) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_implements_keyword(node: &ImplementsKeyword) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_import_keyword(node: &ImportKeyword) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_in_keyword(node: &InKeyword) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_indexed_keyword(node: &IndexedKeyword) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_inline_keyword(node: &InlineKeyword) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_int_keyword(node: &IntKeyword) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_interface_keyword(node: &InterfaceKeyword) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_internal_keyword(node: &InternalKeyword) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_is_keyword(node: &IsKeyword) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_layout_keyword(node: &LayoutKeyword) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_less_than(node: &LessThan) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_less_than_equal(node: &LessThanEqual) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_less_than_less_than(node: &LessThanLessThan) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_less_than_less_than_equal(node: &LessThanLessThanEqual) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_let_keyword(node: &LetKeyword) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_library_keyword(node: &LibraryKeyword) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_macro_keyword(node: &MacroKeyword) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_mapping_keyword(node: &MappingKeyword) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_match_keyword(node: &MatchKeyword) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_memory_keyword(node: &MemoryKeyword) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_minus(node: &Minus) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_minus_equal(node: &MinusEqual) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_minus_minus(node: &MinusMinus) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_minutes_keyword(node: &MinutesKeyword) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_modifier_keyword(node: &ModifierKeyword) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_mutable_keyword(node: &MutableKeyword) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_new_keyword(node: &NewKeyword) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_null_keyword(node: &NullKeyword) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_of_keyword(node: &OfKeyword) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_open_brace(node: &OpenBrace) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_open_bracket(node: &OpenBracket) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_open_paren(node: &OpenParen) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_override_keyword(node: &OverrideKeyword) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_partial_keyword(node: &PartialKeyword) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_payable_keyword(node: &PayableKeyword) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_percent(node: &Percent) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_percent_equal(node: &PercentEqual) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_period(node: &Period) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_plus(node: &Plus) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_plus_equal(node: &PlusEqual) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_plus_plus(node: &PlusPlus) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_pragma_bar_bar(node: &PragmaBarBar) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_pragma_caret(node: &PragmaCaret) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_pragma_equal(node: &PragmaEqual) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_pragma_greater_than(node: &PragmaGreaterThan) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_pragma_greater_than_equal(node: &PragmaGreaterThanEqual) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_pragma_keyword(node: &PragmaKeyword) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_pragma_less_than(node: &PragmaLessThan) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_pragma_less_than_equal(node: &PragmaLessThanEqual) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_pragma_minus(node: &PragmaMinus) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_pragma_period(node: &PragmaPeriod) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_pragma_semicolon(node: &PragmaSemicolon) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_pragma_string_literal(node: &PragmaStringLiteral) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_pragma_tilde(node: &PragmaTilde) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_private_keyword(node: &PrivateKeyword) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_promise_keyword(node: &PromiseKeyword) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_public_keyword(node: &PublicKeyword) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_pure_keyword(node: &PureKeyword) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_question_mark(node: &QuestionMark) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_receive_keyword(node: &ReceiveKeyword) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_reference_keyword(node: &ReferenceKeyword) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_relocatable_keyword(node: &RelocatableKeyword) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_return_keyword(node: &ReturnKeyword) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_returns_keyword(node: &ReturnsKeyword) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_revert_keyword(node: &RevertKeyword) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_smt_checker_keyword(node: &SMTCheckerKeyword) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_sealed_keyword(node: &SealedKeyword) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_seconds_keyword(node: &SecondsKeyword) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_semicolon(node: &Semicolon) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_size_of_keyword(node: &SizeOfKeyword) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_slash(node: &Slash) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_slash_equal(node: &SlashEqual) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_solidity_keyword(node: &SolidityKeyword) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_static_keyword(node: &StaticKeyword) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_storage_keyword(node: &StorageKeyword) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_string_keyword(node: &StringKeyword) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_string_literal(node: &StringLiteral) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_struct_keyword(node: &StructKeyword) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_super_keyword(node: &SuperKeyword) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_supports_keyword(node: &SupportsKeyword) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_switch_keyword(node: &SwitchKeyword) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_this_keyword(node: &ThisKeyword) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_throw_keyword(node: &ThrowKeyword) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_tilde(node: &Tilde) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_transient_keyword(node: &TransientKeyword) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_true_keyword(node: &TrueKeyword) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_try_keyword(node: &TryKeyword) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_type_def_keyword(node: &TypeDefKeyword) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_type_keyword(node: &TypeKeyword) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_type_of_keyword(node: &TypeOfKeyword) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_ufixed_keyword(node: &UfixedKeyword) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_uint_keyword(node: &UintKeyword) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_unchecked_keyword(node: &UncheckedKeyword) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_unicode_string_literal(node: &UnicodeStringLiteral) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_using_keyword(node: &UsingKeyword) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_var_keyword(node: &VarKeyword) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_version_specifier(node: &VersionSpecifier) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_view_keyword(node: &ViewKeyword) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_virtual_keyword(node: &VirtualKeyword) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_weeks_keyword(node: &WeeksKeyword) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_wei_keyword(node: &WeiKeyword) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_while_keyword(node: &WhileKeyword) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_years_keyword(node: &YearsKeyword) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_yul_break_keyword(node: &YulBreakKeyword) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_yul_case_keyword(node: &YulCaseKeyword) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_yul_close_brace(node: &YulCloseBrace) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_yul_close_paren(node: &YulCloseParen) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_yul_colon_equal(node: &YulColonEqual) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_yul_comma(node: &YulComma) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_yul_continue_keyword(node: &YulContinueKeyword) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_yul_decimal_literal(node: &YulDecimalLiteral) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_yul_default_keyword(node: &YulDefaultKeyword) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_yul_false_keyword(node: &YulFalseKeyword) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_yul_for_keyword(node: &YulForKeyword) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_yul_function_keyword(node: &YulFunctionKeyword) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_yul_hex_keyword(node: &YulHexKeyword) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_yul_hex_literal(node: &YulHexLiteral) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_yul_hex_string_literal(node: &YulHexStringLiteral) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_yul_identifier(node: &YulIdentifier) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_yul_if_keyword(node: &YulIfKeyword) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_yul_leave_keyword(node: &YulLeaveKeyword) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_yul_let_keyword(node: &YulLetKeyword) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_yul_minus_greater_than(node: &YulMinusGreaterThan) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_yul_open_brace(node: &YulOpenBrace) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_yul_open_paren(node: &YulOpenParen) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_yul_period(node: &YulPeriod) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_yul_string_literal(node: &YulStringLiteral) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_yul_super_keyword(node: &YulSuperKeyword) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_yul_switch_keyword(node: &YulSwitchKeyword) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_yul_this_keyword(node: &YulThisKeyword) -> Range<usize> {
    node.range.clone()
}

fn first_range_of_yul_true_keyword(node: &YulTrueKeyword) -> Range<usize> {
    node.range.clone()
}
