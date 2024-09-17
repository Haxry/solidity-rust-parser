#![allow(nonstandard_style)]
// Generated from antlr/Solidity.g4 by ANTLR 4.8
use antlr_rust::tree::ParseTreeListener;
use super::solidityparser::*;

pub trait SolidityListener<'input> : ParseTreeListener<'input,SolidityParserContextType>{

/**
 * Enter a parse tree produced by {@link SolidityParser#sourceUnit}.
 * @param ctx the parse tree
 */
fn enter_sourceUnit(&mut self, _ctx: &SourceUnitContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SolidityParser#sourceUnit}.
 * @param ctx the parse tree
 */
fn exit_sourceUnit(&mut self, _ctx: &SourceUnitContext<'input>) { }

/**
 * Enter a parse tree produced by {@link SolidityParser#pragmaDirective}.
 * @param ctx the parse tree
 */
fn enter_pragmaDirective(&mut self, _ctx: &PragmaDirectiveContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SolidityParser#pragmaDirective}.
 * @param ctx the parse tree
 */
fn exit_pragmaDirective(&mut self, _ctx: &PragmaDirectiveContext<'input>) { }

/**
 * Enter a parse tree produced by {@link SolidityParser#pragmaName}.
 * @param ctx the parse tree
 */
fn enter_pragmaName(&mut self, _ctx: &PragmaNameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SolidityParser#pragmaName}.
 * @param ctx the parse tree
 */
fn exit_pragmaName(&mut self, _ctx: &PragmaNameContext<'input>) { }

/**
 * Enter a parse tree produced by {@link SolidityParser#pragmaValue}.
 * @param ctx the parse tree
 */
fn enter_pragmaValue(&mut self, _ctx: &PragmaValueContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SolidityParser#pragmaValue}.
 * @param ctx the parse tree
 */
fn exit_pragmaValue(&mut self, _ctx: &PragmaValueContext<'input>) { }

/**
 * Enter a parse tree produced by {@link SolidityParser#version}.
 * @param ctx the parse tree
 */
fn enter_version(&mut self, _ctx: &VersionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SolidityParser#version}.
 * @param ctx the parse tree
 */
fn exit_version(&mut self, _ctx: &VersionContext<'input>) { }

/**
 * Enter a parse tree produced by {@link SolidityParser#versionOperator}.
 * @param ctx the parse tree
 */
fn enter_versionOperator(&mut self, _ctx: &VersionOperatorContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SolidityParser#versionOperator}.
 * @param ctx the parse tree
 */
fn exit_versionOperator(&mut self, _ctx: &VersionOperatorContext<'input>) { }

/**
 * Enter a parse tree produced by {@link SolidityParser#versionConstraint}.
 * @param ctx the parse tree
 */
fn enter_versionConstraint(&mut self, _ctx: &VersionConstraintContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SolidityParser#versionConstraint}.
 * @param ctx the parse tree
 */
fn exit_versionConstraint(&mut self, _ctx: &VersionConstraintContext<'input>) { }

/**
 * Enter a parse tree produced by {@link SolidityParser#importDeclaration}.
 * @param ctx the parse tree
 */
fn enter_importDeclaration(&mut self, _ctx: &ImportDeclarationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SolidityParser#importDeclaration}.
 * @param ctx the parse tree
 */
fn exit_importDeclaration(&mut self, _ctx: &ImportDeclarationContext<'input>) { }

/**
 * Enter a parse tree produced by {@link SolidityParser#importDirective}.
 * @param ctx the parse tree
 */
fn enter_importDirective(&mut self, _ctx: &ImportDirectiveContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SolidityParser#importDirective}.
 * @param ctx the parse tree
 */
fn exit_importDirective(&mut self, _ctx: &ImportDirectiveContext<'input>) { }

/**
 * Enter a parse tree produced by {@link SolidityParser#importPath}.
 * @param ctx the parse tree
 */
fn enter_importPath(&mut self, _ctx: &ImportPathContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SolidityParser#importPath}.
 * @param ctx the parse tree
 */
fn exit_importPath(&mut self, _ctx: &ImportPathContext<'input>) { }

/**
 * Enter a parse tree produced by {@link SolidityParser#contractDefinition}.
 * @param ctx the parse tree
 */
fn enter_contractDefinition(&mut self, _ctx: &ContractDefinitionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SolidityParser#contractDefinition}.
 * @param ctx the parse tree
 */
fn exit_contractDefinition(&mut self, _ctx: &ContractDefinitionContext<'input>) { }

/**
 * Enter a parse tree produced by {@link SolidityParser#inheritanceSpecifier}.
 * @param ctx the parse tree
 */
fn enter_inheritanceSpecifier(&mut self, _ctx: &InheritanceSpecifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SolidityParser#inheritanceSpecifier}.
 * @param ctx the parse tree
 */
fn exit_inheritanceSpecifier(&mut self, _ctx: &InheritanceSpecifierContext<'input>) { }

/**
 * Enter a parse tree produced by {@link SolidityParser#contractPart}.
 * @param ctx the parse tree
 */
fn enter_contractPart(&mut self, _ctx: &ContractPartContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SolidityParser#contractPart}.
 * @param ctx the parse tree
 */
fn exit_contractPart(&mut self, _ctx: &ContractPartContext<'input>) { }

/**
 * Enter a parse tree produced by {@link SolidityParser#stateVariableDeclaration}.
 * @param ctx the parse tree
 */
fn enter_stateVariableDeclaration(&mut self, _ctx: &StateVariableDeclarationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SolidityParser#stateVariableDeclaration}.
 * @param ctx the parse tree
 */
fn exit_stateVariableDeclaration(&mut self, _ctx: &StateVariableDeclarationContext<'input>) { }

/**
 * Enter a parse tree produced by {@link SolidityParser#fileLevelConstant}.
 * @param ctx the parse tree
 */
fn enter_fileLevelConstant(&mut self, _ctx: &FileLevelConstantContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SolidityParser#fileLevelConstant}.
 * @param ctx the parse tree
 */
fn exit_fileLevelConstant(&mut self, _ctx: &FileLevelConstantContext<'input>) { }

/**
 * Enter a parse tree produced by {@link SolidityParser#customErrorDefinition}.
 * @param ctx the parse tree
 */
fn enter_customErrorDefinition(&mut self, _ctx: &CustomErrorDefinitionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SolidityParser#customErrorDefinition}.
 * @param ctx the parse tree
 */
fn exit_customErrorDefinition(&mut self, _ctx: &CustomErrorDefinitionContext<'input>) { }

/**
 * Enter a parse tree produced by {@link SolidityParser#typeDefinition}.
 * @param ctx the parse tree
 */
fn enter_typeDefinition(&mut self, _ctx: &TypeDefinitionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SolidityParser#typeDefinition}.
 * @param ctx the parse tree
 */
fn exit_typeDefinition(&mut self, _ctx: &TypeDefinitionContext<'input>) { }

/**
 * Enter a parse tree produced by {@link SolidityParser#usingForDeclaration}.
 * @param ctx the parse tree
 */
fn enter_usingForDeclaration(&mut self, _ctx: &UsingForDeclarationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SolidityParser#usingForDeclaration}.
 * @param ctx the parse tree
 */
fn exit_usingForDeclaration(&mut self, _ctx: &UsingForDeclarationContext<'input>) { }

/**
 * Enter a parse tree produced by {@link SolidityParser#usingForObject}.
 * @param ctx the parse tree
 */
fn enter_usingForObject(&mut self, _ctx: &UsingForObjectContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SolidityParser#usingForObject}.
 * @param ctx the parse tree
 */
fn exit_usingForObject(&mut self, _ctx: &UsingForObjectContext<'input>) { }

/**
 * Enter a parse tree produced by {@link SolidityParser#usingForObjectDirective}.
 * @param ctx the parse tree
 */
fn enter_usingForObjectDirective(&mut self, _ctx: &UsingForObjectDirectiveContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SolidityParser#usingForObjectDirective}.
 * @param ctx the parse tree
 */
fn exit_usingForObjectDirective(&mut self, _ctx: &UsingForObjectDirectiveContext<'input>) { }

/**
 * Enter a parse tree produced by {@link SolidityParser#userDefinableOperators}.
 * @param ctx the parse tree
 */
fn enter_userDefinableOperators(&mut self, _ctx: &UserDefinableOperatorsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SolidityParser#userDefinableOperators}.
 * @param ctx the parse tree
 */
fn exit_userDefinableOperators(&mut self, _ctx: &UserDefinableOperatorsContext<'input>) { }

/**
 * Enter a parse tree produced by {@link SolidityParser#structDefinition}.
 * @param ctx the parse tree
 */
fn enter_structDefinition(&mut self, _ctx: &StructDefinitionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SolidityParser#structDefinition}.
 * @param ctx the parse tree
 */
fn exit_structDefinition(&mut self, _ctx: &StructDefinitionContext<'input>) { }

/**
 * Enter a parse tree produced by {@link SolidityParser#modifierDefinition}.
 * @param ctx the parse tree
 */
fn enter_modifierDefinition(&mut self, _ctx: &ModifierDefinitionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SolidityParser#modifierDefinition}.
 * @param ctx the parse tree
 */
fn exit_modifierDefinition(&mut self, _ctx: &ModifierDefinitionContext<'input>) { }

/**
 * Enter a parse tree produced by {@link SolidityParser#modifierInvocation}.
 * @param ctx the parse tree
 */
fn enter_modifierInvocation(&mut self, _ctx: &ModifierInvocationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SolidityParser#modifierInvocation}.
 * @param ctx the parse tree
 */
fn exit_modifierInvocation(&mut self, _ctx: &ModifierInvocationContext<'input>) { }

/**
 * Enter a parse tree produced by {@link SolidityParser#functionDefinition}.
 * @param ctx the parse tree
 */
fn enter_functionDefinition(&mut self, _ctx: &FunctionDefinitionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SolidityParser#functionDefinition}.
 * @param ctx the parse tree
 */
fn exit_functionDefinition(&mut self, _ctx: &FunctionDefinitionContext<'input>) { }

/**
 * Enter a parse tree produced by {@link SolidityParser#functionDescriptor}.
 * @param ctx the parse tree
 */
fn enter_functionDescriptor(&mut self, _ctx: &FunctionDescriptorContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SolidityParser#functionDescriptor}.
 * @param ctx the parse tree
 */
fn exit_functionDescriptor(&mut self, _ctx: &FunctionDescriptorContext<'input>) { }

/**
 * Enter a parse tree produced by {@link SolidityParser#returnParameters}.
 * @param ctx the parse tree
 */
fn enter_returnParameters(&mut self, _ctx: &ReturnParametersContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SolidityParser#returnParameters}.
 * @param ctx the parse tree
 */
fn exit_returnParameters(&mut self, _ctx: &ReturnParametersContext<'input>) { }

/**
 * Enter a parse tree produced by {@link SolidityParser#modifierList}.
 * @param ctx the parse tree
 */
fn enter_modifierList(&mut self, _ctx: &ModifierListContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SolidityParser#modifierList}.
 * @param ctx the parse tree
 */
fn exit_modifierList(&mut self, _ctx: &ModifierListContext<'input>) { }

/**
 * Enter a parse tree produced by {@link SolidityParser#eventDefinition}.
 * @param ctx the parse tree
 */
fn enter_eventDefinition(&mut self, _ctx: &EventDefinitionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SolidityParser#eventDefinition}.
 * @param ctx the parse tree
 */
fn exit_eventDefinition(&mut self, _ctx: &EventDefinitionContext<'input>) { }

/**
 * Enter a parse tree produced by {@link SolidityParser#enumValue}.
 * @param ctx the parse tree
 */
fn enter_enumValue(&mut self, _ctx: &EnumValueContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SolidityParser#enumValue}.
 * @param ctx the parse tree
 */
fn exit_enumValue(&mut self, _ctx: &EnumValueContext<'input>) { }

/**
 * Enter a parse tree produced by {@link SolidityParser#enumDefinition}.
 * @param ctx the parse tree
 */
fn enter_enumDefinition(&mut self, _ctx: &EnumDefinitionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SolidityParser#enumDefinition}.
 * @param ctx the parse tree
 */
fn exit_enumDefinition(&mut self, _ctx: &EnumDefinitionContext<'input>) { }

/**
 * Enter a parse tree produced by {@link SolidityParser#parameterList}.
 * @param ctx the parse tree
 */
fn enter_parameterList(&mut self, _ctx: &ParameterListContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SolidityParser#parameterList}.
 * @param ctx the parse tree
 */
fn exit_parameterList(&mut self, _ctx: &ParameterListContext<'input>) { }

/**
 * Enter a parse tree produced by {@link SolidityParser#parameter}.
 * @param ctx the parse tree
 */
fn enter_parameter(&mut self, _ctx: &ParameterContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SolidityParser#parameter}.
 * @param ctx the parse tree
 */
fn exit_parameter(&mut self, _ctx: &ParameterContext<'input>) { }

/**
 * Enter a parse tree produced by {@link SolidityParser#eventParameterList}.
 * @param ctx the parse tree
 */
fn enter_eventParameterList(&mut self, _ctx: &EventParameterListContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SolidityParser#eventParameterList}.
 * @param ctx the parse tree
 */
fn exit_eventParameterList(&mut self, _ctx: &EventParameterListContext<'input>) { }

/**
 * Enter a parse tree produced by {@link SolidityParser#eventParameter}.
 * @param ctx the parse tree
 */
fn enter_eventParameter(&mut self, _ctx: &EventParameterContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SolidityParser#eventParameter}.
 * @param ctx the parse tree
 */
fn exit_eventParameter(&mut self, _ctx: &EventParameterContext<'input>) { }

/**
 * Enter a parse tree produced by {@link SolidityParser#functionTypeParameterList}.
 * @param ctx the parse tree
 */
fn enter_functionTypeParameterList(&mut self, _ctx: &FunctionTypeParameterListContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SolidityParser#functionTypeParameterList}.
 * @param ctx the parse tree
 */
fn exit_functionTypeParameterList(&mut self, _ctx: &FunctionTypeParameterListContext<'input>) { }

/**
 * Enter a parse tree produced by {@link SolidityParser#functionTypeParameter}.
 * @param ctx the parse tree
 */
fn enter_functionTypeParameter(&mut self, _ctx: &FunctionTypeParameterContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SolidityParser#functionTypeParameter}.
 * @param ctx the parse tree
 */
fn exit_functionTypeParameter(&mut self, _ctx: &FunctionTypeParameterContext<'input>) { }

/**
 * Enter a parse tree produced by {@link SolidityParser#variableDeclaration}.
 * @param ctx the parse tree
 */
fn enter_variableDeclaration(&mut self, _ctx: &VariableDeclarationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SolidityParser#variableDeclaration}.
 * @param ctx the parse tree
 */
fn exit_variableDeclaration(&mut self, _ctx: &VariableDeclarationContext<'input>) { }

/**
 * Enter a parse tree produced by {@link SolidityParser#typeName}.
 * @param ctx the parse tree
 */
fn enter_typeName(&mut self, _ctx: &TypeNameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SolidityParser#typeName}.
 * @param ctx the parse tree
 */
fn exit_typeName(&mut self, _ctx: &TypeNameContext<'input>) { }

/**
 * Enter a parse tree produced by {@link SolidityParser#userDefinedTypeName}.
 * @param ctx the parse tree
 */
fn enter_userDefinedTypeName(&mut self, _ctx: &UserDefinedTypeNameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SolidityParser#userDefinedTypeName}.
 * @param ctx the parse tree
 */
fn exit_userDefinedTypeName(&mut self, _ctx: &UserDefinedTypeNameContext<'input>) { }

/**
 * Enter a parse tree produced by {@link SolidityParser#mappingKey}.
 * @param ctx the parse tree
 */
fn enter_mappingKey(&mut self, _ctx: &MappingKeyContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SolidityParser#mappingKey}.
 * @param ctx the parse tree
 */
fn exit_mappingKey(&mut self, _ctx: &MappingKeyContext<'input>) { }

/**
 * Enter a parse tree produced by {@link SolidityParser#mapping}.
 * @param ctx the parse tree
 */
fn enter_mapping(&mut self, _ctx: &MappingContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SolidityParser#mapping}.
 * @param ctx the parse tree
 */
fn exit_mapping(&mut self, _ctx: &MappingContext<'input>) { }

/**
 * Enter a parse tree produced by {@link SolidityParser#mappingKeyName}.
 * @param ctx the parse tree
 */
fn enter_mappingKeyName(&mut self, _ctx: &MappingKeyNameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SolidityParser#mappingKeyName}.
 * @param ctx the parse tree
 */
fn exit_mappingKeyName(&mut self, _ctx: &MappingKeyNameContext<'input>) { }

/**
 * Enter a parse tree produced by {@link SolidityParser#mappingValueName}.
 * @param ctx the parse tree
 */
fn enter_mappingValueName(&mut self, _ctx: &MappingValueNameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SolidityParser#mappingValueName}.
 * @param ctx the parse tree
 */
fn exit_mappingValueName(&mut self, _ctx: &MappingValueNameContext<'input>) { }

/**
 * Enter a parse tree produced by {@link SolidityParser#functionTypeName}.
 * @param ctx the parse tree
 */
fn enter_functionTypeName(&mut self, _ctx: &FunctionTypeNameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SolidityParser#functionTypeName}.
 * @param ctx the parse tree
 */
fn exit_functionTypeName(&mut self, _ctx: &FunctionTypeNameContext<'input>) { }

/**
 * Enter a parse tree produced by {@link SolidityParser#storageLocation}.
 * @param ctx the parse tree
 */
fn enter_storageLocation(&mut self, _ctx: &StorageLocationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SolidityParser#storageLocation}.
 * @param ctx the parse tree
 */
fn exit_storageLocation(&mut self, _ctx: &StorageLocationContext<'input>) { }

/**
 * Enter a parse tree produced by {@link SolidityParser#stateMutability}.
 * @param ctx the parse tree
 */
fn enter_stateMutability(&mut self, _ctx: &StateMutabilityContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SolidityParser#stateMutability}.
 * @param ctx the parse tree
 */
fn exit_stateMutability(&mut self, _ctx: &StateMutabilityContext<'input>) { }

/**
 * Enter a parse tree produced by {@link SolidityParser#block}.
 * @param ctx the parse tree
 */
fn enter_block(&mut self, _ctx: &BlockContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SolidityParser#block}.
 * @param ctx the parse tree
 */
fn exit_block(&mut self, _ctx: &BlockContext<'input>) { }

/**
 * Enter a parse tree produced by {@link SolidityParser#statement}.
 * @param ctx the parse tree
 */
fn enter_statement(&mut self, _ctx: &StatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SolidityParser#statement}.
 * @param ctx the parse tree
 */
fn exit_statement(&mut self, _ctx: &StatementContext<'input>) { }

/**
 * Enter a parse tree produced by {@link SolidityParser#expressionStatement}.
 * @param ctx the parse tree
 */
fn enter_expressionStatement(&mut self, _ctx: &ExpressionStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SolidityParser#expressionStatement}.
 * @param ctx the parse tree
 */
fn exit_expressionStatement(&mut self, _ctx: &ExpressionStatementContext<'input>) { }

/**
 * Enter a parse tree produced by {@link SolidityParser#ifStatement}.
 * @param ctx the parse tree
 */
fn enter_ifStatement(&mut self, _ctx: &IfStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SolidityParser#ifStatement}.
 * @param ctx the parse tree
 */
fn exit_ifStatement(&mut self, _ctx: &IfStatementContext<'input>) { }

/**
 * Enter a parse tree produced by {@link SolidityParser#tryStatement}.
 * @param ctx the parse tree
 */
fn enter_tryStatement(&mut self, _ctx: &TryStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SolidityParser#tryStatement}.
 * @param ctx the parse tree
 */
fn exit_tryStatement(&mut self, _ctx: &TryStatementContext<'input>) { }

/**
 * Enter a parse tree produced by {@link SolidityParser#catchClause}.
 * @param ctx the parse tree
 */
fn enter_catchClause(&mut self, _ctx: &CatchClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SolidityParser#catchClause}.
 * @param ctx the parse tree
 */
fn exit_catchClause(&mut self, _ctx: &CatchClauseContext<'input>) { }

/**
 * Enter a parse tree produced by {@link SolidityParser#whileStatement}.
 * @param ctx the parse tree
 */
fn enter_whileStatement(&mut self, _ctx: &WhileStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SolidityParser#whileStatement}.
 * @param ctx the parse tree
 */
fn exit_whileStatement(&mut self, _ctx: &WhileStatementContext<'input>) { }

/**
 * Enter a parse tree produced by {@link SolidityParser#simpleStatement}.
 * @param ctx the parse tree
 */
fn enter_simpleStatement(&mut self, _ctx: &SimpleStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SolidityParser#simpleStatement}.
 * @param ctx the parse tree
 */
fn exit_simpleStatement(&mut self, _ctx: &SimpleStatementContext<'input>) { }

/**
 * Enter a parse tree produced by {@link SolidityParser#uncheckedStatement}.
 * @param ctx the parse tree
 */
fn enter_uncheckedStatement(&mut self, _ctx: &UncheckedStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SolidityParser#uncheckedStatement}.
 * @param ctx the parse tree
 */
fn exit_uncheckedStatement(&mut self, _ctx: &UncheckedStatementContext<'input>) { }

/**
 * Enter a parse tree produced by {@link SolidityParser#forStatement}.
 * @param ctx the parse tree
 */
fn enter_forStatement(&mut self, _ctx: &ForStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SolidityParser#forStatement}.
 * @param ctx the parse tree
 */
fn exit_forStatement(&mut self, _ctx: &ForStatementContext<'input>) { }

/**
 * Enter a parse tree produced by {@link SolidityParser#inlineAssemblyStatement}.
 * @param ctx the parse tree
 */
fn enter_inlineAssemblyStatement(&mut self, _ctx: &InlineAssemblyStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SolidityParser#inlineAssemblyStatement}.
 * @param ctx the parse tree
 */
fn exit_inlineAssemblyStatement(&mut self, _ctx: &InlineAssemblyStatementContext<'input>) { }

/**
 * Enter a parse tree produced by {@link SolidityParser#inlineAssemblyStatementFlag}.
 * @param ctx the parse tree
 */
fn enter_inlineAssemblyStatementFlag(&mut self, _ctx: &InlineAssemblyStatementFlagContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SolidityParser#inlineAssemblyStatementFlag}.
 * @param ctx the parse tree
 */
fn exit_inlineAssemblyStatementFlag(&mut self, _ctx: &InlineAssemblyStatementFlagContext<'input>) { }

/**
 * Enter a parse tree produced by {@link SolidityParser#doWhileStatement}.
 * @param ctx the parse tree
 */
fn enter_doWhileStatement(&mut self, _ctx: &DoWhileStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SolidityParser#doWhileStatement}.
 * @param ctx the parse tree
 */
fn exit_doWhileStatement(&mut self, _ctx: &DoWhileStatementContext<'input>) { }

/**
 * Enter a parse tree produced by {@link SolidityParser#continueStatement}.
 * @param ctx the parse tree
 */
fn enter_continueStatement(&mut self, _ctx: &ContinueStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SolidityParser#continueStatement}.
 * @param ctx the parse tree
 */
fn exit_continueStatement(&mut self, _ctx: &ContinueStatementContext<'input>) { }

/**
 * Enter a parse tree produced by {@link SolidityParser#breakStatement}.
 * @param ctx the parse tree
 */
fn enter_breakStatement(&mut self, _ctx: &BreakStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SolidityParser#breakStatement}.
 * @param ctx the parse tree
 */
fn exit_breakStatement(&mut self, _ctx: &BreakStatementContext<'input>) { }

/**
 * Enter a parse tree produced by {@link SolidityParser#returnStatement}.
 * @param ctx the parse tree
 */
fn enter_returnStatement(&mut self, _ctx: &ReturnStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SolidityParser#returnStatement}.
 * @param ctx the parse tree
 */
fn exit_returnStatement(&mut self, _ctx: &ReturnStatementContext<'input>) { }

/**
 * Enter a parse tree produced by {@link SolidityParser#throwStatement}.
 * @param ctx the parse tree
 */
fn enter_throwStatement(&mut self, _ctx: &ThrowStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SolidityParser#throwStatement}.
 * @param ctx the parse tree
 */
fn exit_throwStatement(&mut self, _ctx: &ThrowStatementContext<'input>) { }

/**
 * Enter a parse tree produced by {@link SolidityParser#emitStatement}.
 * @param ctx the parse tree
 */
fn enter_emitStatement(&mut self, _ctx: &EmitStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SolidityParser#emitStatement}.
 * @param ctx the parse tree
 */
fn exit_emitStatement(&mut self, _ctx: &EmitStatementContext<'input>) { }

/**
 * Enter a parse tree produced by {@link SolidityParser#revertStatement}.
 * @param ctx the parse tree
 */
fn enter_revertStatement(&mut self, _ctx: &RevertStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SolidityParser#revertStatement}.
 * @param ctx the parse tree
 */
fn exit_revertStatement(&mut self, _ctx: &RevertStatementContext<'input>) { }

/**
 * Enter a parse tree produced by {@link SolidityParser#variableDeclarationStatement}.
 * @param ctx the parse tree
 */
fn enter_variableDeclarationStatement(&mut self, _ctx: &VariableDeclarationStatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SolidityParser#variableDeclarationStatement}.
 * @param ctx the parse tree
 */
fn exit_variableDeclarationStatement(&mut self, _ctx: &VariableDeclarationStatementContext<'input>) { }

/**
 * Enter a parse tree produced by {@link SolidityParser#variableDeclarationList}.
 * @param ctx the parse tree
 */
fn enter_variableDeclarationList(&mut self, _ctx: &VariableDeclarationListContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SolidityParser#variableDeclarationList}.
 * @param ctx the parse tree
 */
fn exit_variableDeclarationList(&mut self, _ctx: &VariableDeclarationListContext<'input>) { }

/**
 * Enter a parse tree produced by {@link SolidityParser#identifierList}.
 * @param ctx the parse tree
 */
fn enter_identifierList(&mut self, _ctx: &IdentifierListContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SolidityParser#identifierList}.
 * @param ctx the parse tree
 */
fn exit_identifierList(&mut self, _ctx: &IdentifierListContext<'input>) { }

/**
 * Enter a parse tree produced by {@link SolidityParser#elementaryTypeName}.
 * @param ctx the parse tree
 */
fn enter_elementaryTypeName(&mut self, _ctx: &ElementaryTypeNameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SolidityParser#elementaryTypeName}.
 * @param ctx the parse tree
 */
fn exit_elementaryTypeName(&mut self, _ctx: &ElementaryTypeNameContext<'input>) { }

/**
 * Enter a parse tree produced by {@link SolidityParser#expression}.
 * @param ctx the parse tree
 */
fn enter_expression(&mut self, _ctx: &ExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SolidityParser#expression}.
 * @param ctx the parse tree
 */
fn exit_expression(&mut self, _ctx: &ExpressionContext<'input>) { }

/**
 * Enter a parse tree produced by {@link SolidityParser#primaryExpression}.
 * @param ctx the parse tree
 */
fn enter_primaryExpression(&mut self, _ctx: &PrimaryExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SolidityParser#primaryExpression}.
 * @param ctx the parse tree
 */
fn exit_primaryExpression(&mut self, _ctx: &PrimaryExpressionContext<'input>) { }

/**
 * Enter a parse tree produced by {@link SolidityParser#expressionList}.
 * @param ctx the parse tree
 */
fn enter_expressionList(&mut self, _ctx: &ExpressionListContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SolidityParser#expressionList}.
 * @param ctx the parse tree
 */
fn exit_expressionList(&mut self, _ctx: &ExpressionListContext<'input>) { }

/**
 * Enter a parse tree produced by {@link SolidityParser#nameValueList}.
 * @param ctx the parse tree
 */
fn enter_nameValueList(&mut self, _ctx: &NameValueListContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SolidityParser#nameValueList}.
 * @param ctx the parse tree
 */
fn exit_nameValueList(&mut self, _ctx: &NameValueListContext<'input>) { }

/**
 * Enter a parse tree produced by {@link SolidityParser#nameValue}.
 * @param ctx the parse tree
 */
fn enter_nameValue(&mut self, _ctx: &NameValueContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SolidityParser#nameValue}.
 * @param ctx the parse tree
 */
fn exit_nameValue(&mut self, _ctx: &NameValueContext<'input>) { }

/**
 * Enter a parse tree produced by {@link SolidityParser#functionCallArguments}.
 * @param ctx the parse tree
 */
fn enter_functionCallArguments(&mut self, _ctx: &FunctionCallArgumentsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SolidityParser#functionCallArguments}.
 * @param ctx the parse tree
 */
fn exit_functionCallArguments(&mut self, _ctx: &FunctionCallArgumentsContext<'input>) { }

/**
 * Enter a parse tree produced by {@link SolidityParser#functionCall}.
 * @param ctx the parse tree
 */
fn enter_functionCall(&mut self, _ctx: &FunctionCallContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SolidityParser#functionCall}.
 * @param ctx the parse tree
 */
fn exit_functionCall(&mut self, _ctx: &FunctionCallContext<'input>) { }

/**
 * Enter a parse tree produced by {@link SolidityParser#assemblyBlock}.
 * @param ctx the parse tree
 */
fn enter_assemblyBlock(&mut self, _ctx: &AssemblyBlockContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SolidityParser#assemblyBlock}.
 * @param ctx the parse tree
 */
fn exit_assemblyBlock(&mut self, _ctx: &AssemblyBlockContext<'input>) { }

/**
 * Enter a parse tree produced by {@link SolidityParser#assemblyItem}.
 * @param ctx the parse tree
 */
fn enter_assemblyItem(&mut self, _ctx: &AssemblyItemContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SolidityParser#assemblyItem}.
 * @param ctx the parse tree
 */
fn exit_assemblyItem(&mut self, _ctx: &AssemblyItemContext<'input>) { }

/**
 * Enter a parse tree produced by {@link SolidityParser#assemblyExpression}.
 * @param ctx the parse tree
 */
fn enter_assemblyExpression(&mut self, _ctx: &AssemblyExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SolidityParser#assemblyExpression}.
 * @param ctx the parse tree
 */
fn exit_assemblyExpression(&mut self, _ctx: &AssemblyExpressionContext<'input>) { }

/**
 * Enter a parse tree produced by {@link SolidityParser#assemblyMember}.
 * @param ctx the parse tree
 */
fn enter_assemblyMember(&mut self, _ctx: &AssemblyMemberContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SolidityParser#assemblyMember}.
 * @param ctx the parse tree
 */
fn exit_assemblyMember(&mut self, _ctx: &AssemblyMemberContext<'input>) { }

/**
 * Enter a parse tree produced by {@link SolidityParser#assemblyCall}.
 * @param ctx the parse tree
 */
fn enter_assemblyCall(&mut self, _ctx: &AssemblyCallContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SolidityParser#assemblyCall}.
 * @param ctx the parse tree
 */
fn exit_assemblyCall(&mut self, _ctx: &AssemblyCallContext<'input>) { }

/**
 * Enter a parse tree produced by {@link SolidityParser#assemblyLocalDefinition}.
 * @param ctx the parse tree
 */
fn enter_assemblyLocalDefinition(&mut self, _ctx: &AssemblyLocalDefinitionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SolidityParser#assemblyLocalDefinition}.
 * @param ctx the parse tree
 */
fn exit_assemblyLocalDefinition(&mut self, _ctx: &AssemblyLocalDefinitionContext<'input>) { }

/**
 * Enter a parse tree produced by {@link SolidityParser#assemblyAssignment}.
 * @param ctx the parse tree
 */
fn enter_assemblyAssignment(&mut self, _ctx: &AssemblyAssignmentContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SolidityParser#assemblyAssignment}.
 * @param ctx the parse tree
 */
fn exit_assemblyAssignment(&mut self, _ctx: &AssemblyAssignmentContext<'input>) { }

/**
 * Enter a parse tree produced by {@link SolidityParser#assemblyIdentifierOrList}.
 * @param ctx the parse tree
 */
fn enter_assemblyIdentifierOrList(&mut self, _ctx: &AssemblyIdentifierOrListContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SolidityParser#assemblyIdentifierOrList}.
 * @param ctx the parse tree
 */
fn exit_assemblyIdentifierOrList(&mut self, _ctx: &AssemblyIdentifierOrListContext<'input>) { }

/**
 * Enter a parse tree produced by {@link SolidityParser#assemblyIdentifierList}.
 * @param ctx the parse tree
 */
fn enter_assemblyIdentifierList(&mut self, _ctx: &AssemblyIdentifierListContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SolidityParser#assemblyIdentifierList}.
 * @param ctx the parse tree
 */
fn exit_assemblyIdentifierList(&mut self, _ctx: &AssemblyIdentifierListContext<'input>) { }

/**
 * Enter a parse tree produced by {@link SolidityParser#assemblyStackAssignment}.
 * @param ctx the parse tree
 */
fn enter_assemblyStackAssignment(&mut self, _ctx: &AssemblyStackAssignmentContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SolidityParser#assemblyStackAssignment}.
 * @param ctx the parse tree
 */
fn exit_assemblyStackAssignment(&mut self, _ctx: &AssemblyStackAssignmentContext<'input>) { }

/**
 * Enter a parse tree produced by {@link SolidityParser#labelDefinition}.
 * @param ctx the parse tree
 */
fn enter_labelDefinition(&mut self, _ctx: &LabelDefinitionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SolidityParser#labelDefinition}.
 * @param ctx the parse tree
 */
fn exit_labelDefinition(&mut self, _ctx: &LabelDefinitionContext<'input>) { }

/**
 * Enter a parse tree produced by {@link SolidityParser#assemblySwitch}.
 * @param ctx the parse tree
 */
fn enter_assemblySwitch(&mut self, _ctx: &AssemblySwitchContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SolidityParser#assemblySwitch}.
 * @param ctx the parse tree
 */
fn exit_assemblySwitch(&mut self, _ctx: &AssemblySwitchContext<'input>) { }

/**
 * Enter a parse tree produced by {@link SolidityParser#assemblyCase}.
 * @param ctx the parse tree
 */
fn enter_assemblyCase(&mut self, _ctx: &AssemblyCaseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SolidityParser#assemblyCase}.
 * @param ctx the parse tree
 */
fn exit_assemblyCase(&mut self, _ctx: &AssemblyCaseContext<'input>) { }

/**
 * Enter a parse tree produced by {@link SolidityParser#assemblyFunctionDefinition}.
 * @param ctx the parse tree
 */
fn enter_assemblyFunctionDefinition(&mut self, _ctx: &AssemblyFunctionDefinitionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SolidityParser#assemblyFunctionDefinition}.
 * @param ctx the parse tree
 */
fn exit_assemblyFunctionDefinition(&mut self, _ctx: &AssemblyFunctionDefinitionContext<'input>) { }

/**
 * Enter a parse tree produced by {@link SolidityParser#assemblyFunctionReturns}.
 * @param ctx the parse tree
 */
fn enter_assemblyFunctionReturns(&mut self, _ctx: &AssemblyFunctionReturnsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SolidityParser#assemblyFunctionReturns}.
 * @param ctx the parse tree
 */
fn exit_assemblyFunctionReturns(&mut self, _ctx: &AssemblyFunctionReturnsContext<'input>) { }

/**
 * Enter a parse tree produced by {@link SolidityParser#assemblyFor}.
 * @param ctx the parse tree
 */
fn enter_assemblyFor(&mut self, _ctx: &AssemblyForContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SolidityParser#assemblyFor}.
 * @param ctx the parse tree
 */
fn exit_assemblyFor(&mut self, _ctx: &AssemblyForContext<'input>) { }

/**
 * Enter a parse tree produced by {@link SolidityParser#assemblyIf}.
 * @param ctx the parse tree
 */
fn enter_assemblyIf(&mut self, _ctx: &AssemblyIfContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SolidityParser#assemblyIf}.
 * @param ctx the parse tree
 */
fn exit_assemblyIf(&mut self, _ctx: &AssemblyIfContext<'input>) { }

/**
 * Enter a parse tree produced by {@link SolidityParser#assemblyLiteral}.
 * @param ctx the parse tree
 */
fn enter_assemblyLiteral(&mut self, _ctx: &AssemblyLiteralContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SolidityParser#assemblyLiteral}.
 * @param ctx the parse tree
 */
fn exit_assemblyLiteral(&mut self, _ctx: &AssemblyLiteralContext<'input>) { }

/**
 * Enter a parse tree produced by {@link SolidityParser#tupleExpression}.
 * @param ctx the parse tree
 */
fn enter_tupleExpression(&mut self, _ctx: &TupleExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SolidityParser#tupleExpression}.
 * @param ctx the parse tree
 */
fn exit_tupleExpression(&mut self, _ctx: &TupleExpressionContext<'input>) { }

/**
 * Enter a parse tree produced by {@link SolidityParser#numberLiteral}.
 * @param ctx the parse tree
 */
fn enter_numberLiteral(&mut self, _ctx: &NumberLiteralContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SolidityParser#numberLiteral}.
 * @param ctx the parse tree
 */
fn exit_numberLiteral(&mut self, _ctx: &NumberLiteralContext<'input>) { }

/**
 * Enter a parse tree produced by {@link SolidityParser#identifier}.
 * @param ctx the parse tree
 */
fn enter_identifier(&mut self, _ctx: &IdentifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SolidityParser#identifier}.
 * @param ctx the parse tree
 */
fn exit_identifier(&mut self, _ctx: &IdentifierContext<'input>) { }

/**
 * Enter a parse tree produced by {@link SolidityParser#hexLiteral}.
 * @param ctx the parse tree
 */
fn enter_hexLiteral(&mut self, _ctx: &HexLiteralContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SolidityParser#hexLiteral}.
 * @param ctx the parse tree
 */
fn exit_hexLiteral(&mut self, _ctx: &HexLiteralContext<'input>) { }

/**
 * Enter a parse tree produced by {@link SolidityParser#overrideSpecifier}.
 * @param ctx the parse tree
 */
fn enter_overrideSpecifier(&mut self, _ctx: &OverrideSpecifierContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SolidityParser#overrideSpecifier}.
 * @param ctx the parse tree
 */
fn exit_overrideSpecifier(&mut self, _ctx: &OverrideSpecifierContext<'input>) { }

/**
 * Enter a parse tree produced by {@link SolidityParser#stringLiteral}.
 * @param ctx the parse tree
 */
fn enter_stringLiteral(&mut self, _ctx: &StringLiteralContext<'input>) { }
/**
 * Exit a parse tree produced by {@link SolidityParser#stringLiteral}.
 * @param ctx the parse tree
 */
fn exit_stringLiteral(&mut self, _ctx: &StringLiteralContext<'input>) { }

}
