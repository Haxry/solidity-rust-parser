#![allow(nonstandard_style)]
// Generated from /mnt/c/Users/blbha/Downloads/Solidity.g4 by ANTLR 4.8
use antlr_rust::tree::{ParseTreeVisitor};
use super::solidityparser::*;

/**
 * This interface defines a complete generic visitor for a parse tree produced
 * by {@link SolidityParser}.
 */
pub trait SolidityVisitor<'input>: ParseTreeVisitor<'input,SolidityParserContextType>{
	/**
	 * Visit a parse tree produced by {@link SolidityParser#sourceUnit}.
	 * @param ctx the parse tree
	 */
	fn visit_sourceUnit(&mut self, ctx: &SourceUnitContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SolidityParser#pragmaDirective}.
	 * @param ctx the parse tree
	 */
	fn visit_pragmaDirective(&mut self, ctx: &PragmaDirectiveContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SolidityParser#pragmaName}.
	 * @param ctx the parse tree
	 */
	fn visit_pragmaName(&mut self, ctx: &PragmaNameContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SolidityParser#pragmaValue}.
	 * @param ctx the parse tree
	 */
	fn visit_pragmaValue(&mut self, ctx: &PragmaValueContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SolidityParser#version}.
	 * @param ctx the parse tree
	 */
	fn visit_version(&mut self, ctx: &VersionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SolidityParser#versionOperator}.
	 * @param ctx the parse tree
	 */
	fn visit_versionOperator(&mut self, ctx: &VersionOperatorContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SolidityParser#versionConstraint}.
	 * @param ctx the parse tree
	 */
	fn visit_versionConstraint(&mut self, ctx: &VersionConstraintContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SolidityParser#importDeclaration}.
	 * @param ctx the parse tree
	 */
	fn visit_importDeclaration(&mut self, ctx: &ImportDeclarationContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SolidityParser#importDirective}.
	 * @param ctx the parse tree
	 */
	fn visit_importDirective(&mut self, ctx: &ImportDirectiveContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SolidityParser#importPath}.
	 * @param ctx the parse tree
	 */
	fn visit_importPath(&mut self, ctx: &ImportPathContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SolidityParser#contractDefinition}.
	 * @param ctx the parse tree
	 */
	fn visit_contractDefinition(&mut self, ctx: &ContractDefinitionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SolidityParser#inheritanceSpecifier}.
	 * @param ctx the parse tree
	 */
	fn visit_inheritanceSpecifier(&mut self, ctx: &InheritanceSpecifierContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SolidityParser#contractPart}.
	 * @param ctx the parse tree
	 */
	fn visit_contractPart(&mut self, ctx: &ContractPartContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SolidityParser#stateVariableDeclaration}.
	 * @param ctx the parse tree
	 */
	fn visit_stateVariableDeclaration(&mut self, ctx: &StateVariableDeclarationContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SolidityParser#fileLevelConstant}.
	 * @param ctx the parse tree
	 */
	fn visit_fileLevelConstant(&mut self, ctx: &FileLevelConstantContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SolidityParser#customErrorDefinition}.
	 * @param ctx the parse tree
	 */
	fn visit_customErrorDefinition(&mut self, ctx: &CustomErrorDefinitionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SolidityParser#typeDefinition}.
	 * @param ctx the parse tree
	 */
	fn visit_typeDefinition(&mut self, ctx: &TypeDefinitionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SolidityParser#usingForDeclaration}.
	 * @param ctx the parse tree
	 */
	fn visit_usingForDeclaration(&mut self, ctx: &UsingForDeclarationContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SolidityParser#usingForObject}.
	 * @param ctx the parse tree
	 */
	fn visit_usingForObject(&mut self, ctx: &UsingForObjectContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SolidityParser#usingForObjectDirective}.
	 * @param ctx the parse tree
	 */
	fn visit_usingForObjectDirective(&mut self, ctx: &UsingForObjectDirectiveContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SolidityParser#userDefinableOperators}.
	 * @param ctx the parse tree
	 */
	fn visit_userDefinableOperators(&mut self, ctx: &UserDefinableOperatorsContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SolidityParser#structDefinition}.
	 * @param ctx the parse tree
	 */
	fn visit_structDefinition(&mut self, ctx: &StructDefinitionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SolidityParser#modifierDefinition}.
	 * @param ctx the parse tree
	 */
	fn visit_modifierDefinition(&mut self, ctx: &ModifierDefinitionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SolidityParser#modifierInvocation}.
	 * @param ctx the parse tree
	 */
	fn visit_modifierInvocation(&mut self, ctx: &ModifierInvocationContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SolidityParser#functionDefinition}.
	 * @param ctx the parse tree
	 */
	fn visit_functionDefinition(&mut self, ctx: &FunctionDefinitionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SolidityParser#functionDescriptor}.
	 * @param ctx the parse tree
	 */
	fn visit_functionDescriptor(&mut self, ctx: &FunctionDescriptorContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SolidityParser#returnParameters}.
	 * @param ctx the parse tree
	 */
	fn visit_returnParameters(&mut self, ctx: &ReturnParametersContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SolidityParser#modifierList}.
	 * @param ctx the parse tree
	 */
	fn visit_modifierList(&mut self, ctx: &ModifierListContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SolidityParser#eventDefinition}.
	 * @param ctx the parse tree
	 */
	fn visit_eventDefinition(&mut self, ctx: &EventDefinitionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SolidityParser#enumValue}.
	 * @param ctx the parse tree
	 */
	fn visit_enumValue(&mut self, ctx: &EnumValueContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SolidityParser#enumDefinition}.
	 * @param ctx the parse tree
	 */
	fn visit_enumDefinition(&mut self, ctx: &EnumDefinitionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SolidityParser#parameterList}.
	 * @param ctx the parse tree
	 */
	fn visit_parameterList(&mut self, ctx: &ParameterListContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SolidityParser#parameter}.
	 * @param ctx the parse tree
	 */
	fn visit_parameter(&mut self, ctx: &ParameterContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SolidityParser#eventParameterList}.
	 * @param ctx the parse tree
	 */
	fn visit_eventParameterList(&mut self, ctx: &EventParameterListContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SolidityParser#eventParameter}.
	 * @param ctx the parse tree
	 */
	fn visit_eventParameter(&mut self, ctx: &EventParameterContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SolidityParser#functionTypeParameterList}.
	 * @param ctx the parse tree
	 */
	fn visit_functionTypeParameterList(&mut self, ctx: &FunctionTypeParameterListContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SolidityParser#functionTypeParameter}.
	 * @param ctx the parse tree
	 */
	fn visit_functionTypeParameter(&mut self, ctx: &FunctionTypeParameterContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SolidityParser#variableDeclaration}.
	 * @param ctx the parse tree
	 */
	fn visit_variableDeclaration(&mut self, ctx: &VariableDeclarationContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SolidityParser#typeName}.
	 * @param ctx the parse tree
	 */
	fn visit_typeName(&mut self, ctx: &TypeNameContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SolidityParser#userDefinedTypeName}.
	 * @param ctx the parse tree
	 */
	fn visit_userDefinedTypeName(&mut self, ctx: &UserDefinedTypeNameContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SolidityParser#mappingKey}.
	 * @param ctx the parse tree
	 */
	fn visit_mappingKey(&mut self, ctx: &MappingKeyContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SolidityParser#mapping}.
	 * @param ctx the parse tree
	 */
	fn visit_mapping(&mut self, ctx: &MappingContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SolidityParser#mappingKeyName}.
	 * @param ctx the parse tree
	 */
	fn visit_mappingKeyName(&mut self, ctx: &MappingKeyNameContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SolidityParser#mappingValueName}.
	 * @param ctx the parse tree
	 */
	fn visit_mappingValueName(&mut self, ctx: &MappingValueNameContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SolidityParser#functionTypeName}.
	 * @param ctx the parse tree
	 */
	fn visit_functionTypeName(&mut self, ctx: &FunctionTypeNameContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SolidityParser#storageLocation}.
	 * @param ctx the parse tree
	 */
	fn visit_storageLocation(&mut self, ctx: &StorageLocationContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SolidityParser#stateMutability}.
	 * @param ctx the parse tree
	 */
	fn visit_stateMutability(&mut self, ctx: &StateMutabilityContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SolidityParser#block}.
	 * @param ctx the parse tree
	 */
	fn visit_block(&mut self, ctx: &BlockContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SolidityParser#statement}.
	 * @param ctx the parse tree
	 */
	fn visit_statement(&mut self, ctx: &StatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SolidityParser#expressionStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_expressionStatement(&mut self, ctx: &ExpressionStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SolidityParser#ifStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_ifStatement(&mut self, ctx: &IfStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SolidityParser#tryStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_tryStatement(&mut self, ctx: &TryStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SolidityParser#catchClause}.
	 * @param ctx the parse tree
	 */
	fn visit_catchClause(&mut self, ctx: &CatchClauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SolidityParser#whileStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_whileStatement(&mut self, ctx: &WhileStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SolidityParser#simpleStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_simpleStatement(&mut self, ctx: &SimpleStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SolidityParser#uncheckedStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_uncheckedStatement(&mut self, ctx: &UncheckedStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SolidityParser#forStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_forStatement(&mut self, ctx: &ForStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SolidityParser#inlineAssemblyStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_inlineAssemblyStatement(&mut self, ctx: &InlineAssemblyStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SolidityParser#inlineAssemblyStatementFlag}.
	 * @param ctx the parse tree
	 */
	fn visit_inlineAssemblyStatementFlag(&mut self, ctx: &InlineAssemblyStatementFlagContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SolidityParser#doWhileStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_doWhileStatement(&mut self, ctx: &DoWhileStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SolidityParser#continueStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_continueStatement(&mut self, ctx: &ContinueStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SolidityParser#breakStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_breakStatement(&mut self, ctx: &BreakStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SolidityParser#returnStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_returnStatement(&mut self, ctx: &ReturnStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SolidityParser#throwStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_throwStatement(&mut self, ctx: &ThrowStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SolidityParser#emitStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_emitStatement(&mut self, ctx: &EmitStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SolidityParser#revertStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_revertStatement(&mut self, ctx: &RevertStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SolidityParser#variableDeclarationStatement}.
	 * @param ctx the parse tree
	 */
	fn visit_variableDeclarationStatement(&mut self, ctx: &VariableDeclarationStatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SolidityParser#variableDeclarationList}.
	 * @param ctx the parse tree
	 */
	fn visit_variableDeclarationList(&mut self, ctx: &VariableDeclarationListContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SolidityParser#identifierList}.
	 * @param ctx the parse tree
	 */
	fn visit_identifierList(&mut self, ctx: &IdentifierListContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SolidityParser#elementaryTypeName}.
	 * @param ctx the parse tree
	 */
	fn visit_elementaryTypeName(&mut self, ctx: &ElementaryTypeNameContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SolidityParser#expression}.
	 * @param ctx the parse tree
	 */
	fn visit_expression(&mut self, ctx: &ExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SolidityParser#primaryExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_primaryExpression(&mut self, ctx: &PrimaryExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SolidityParser#expressionList}.
	 * @param ctx the parse tree
	 */
	fn visit_expressionList(&mut self, ctx: &ExpressionListContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SolidityParser#nameValueList}.
	 * @param ctx the parse tree
	 */
	fn visit_nameValueList(&mut self, ctx: &NameValueListContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SolidityParser#nameValue}.
	 * @param ctx the parse tree
	 */
	fn visit_nameValue(&mut self, ctx: &NameValueContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SolidityParser#functionCallArguments}.
	 * @param ctx the parse tree
	 */
	fn visit_functionCallArguments(&mut self, ctx: &FunctionCallArgumentsContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SolidityParser#functionCall}.
	 * @param ctx the parse tree
	 */
	fn visit_functionCall(&mut self, ctx: &FunctionCallContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SolidityParser#assemblyBlock}.
	 * @param ctx the parse tree
	 */
	fn visit_assemblyBlock(&mut self, ctx: &AssemblyBlockContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SolidityParser#assemblyItem}.
	 * @param ctx the parse tree
	 */
	fn visit_assemblyItem(&mut self, ctx: &AssemblyItemContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SolidityParser#assemblyExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_assemblyExpression(&mut self, ctx: &AssemblyExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SolidityParser#assemblyMember}.
	 * @param ctx the parse tree
	 */
	fn visit_assemblyMember(&mut self, ctx: &AssemblyMemberContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SolidityParser#assemblyCall}.
	 * @param ctx the parse tree
	 */
	fn visit_assemblyCall(&mut self, ctx: &AssemblyCallContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SolidityParser#assemblyLocalDefinition}.
	 * @param ctx the parse tree
	 */
	fn visit_assemblyLocalDefinition(&mut self, ctx: &AssemblyLocalDefinitionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SolidityParser#assemblyAssignment}.
	 * @param ctx the parse tree
	 */
	fn visit_assemblyAssignment(&mut self, ctx: &AssemblyAssignmentContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SolidityParser#assemblyIdentifierOrList}.
	 * @param ctx the parse tree
	 */
	fn visit_assemblyIdentifierOrList(&mut self, ctx: &AssemblyIdentifierOrListContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SolidityParser#assemblyIdentifierList}.
	 * @param ctx the parse tree
	 */
	fn visit_assemblyIdentifierList(&mut self, ctx: &AssemblyIdentifierListContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SolidityParser#assemblyStackAssignment}.
	 * @param ctx the parse tree
	 */
	fn visit_assemblyStackAssignment(&mut self, ctx: &AssemblyStackAssignmentContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SolidityParser#labelDefinition}.
	 * @param ctx the parse tree
	 */
	fn visit_labelDefinition(&mut self, ctx: &LabelDefinitionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SolidityParser#assemblySwitch}.
	 * @param ctx the parse tree
	 */
	fn visit_assemblySwitch(&mut self, ctx: &AssemblySwitchContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SolidityParser#assemblyCase}.
	 * @param ctx the parse tree
	 */
	fn visit_assemblyCase(&mut self, ctx: &AssemblyCaseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SolidityParser#assemblyFunctionDefinition}.
	 * @param ctx the parse tree
	 */
	fn visit_assemblyFunctionDefinition(&mut self, ctx: &AssemblyFunctionDefinitionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SolidityParser#assemblyFunctionReturns}.
	 * @param ctx the parse tree
	 */
	fn visit_assemblyFunctionReturns(&mut self, ctx: &AssemblyFunctionReturnsContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SolidityParser#assemblyFor}.
	 * @param ctx the parse tree
	 */
	fn visit_assemblyFor(&mut self, ctx: &AssemblyForContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SolidityParser#assemblyIf}.
	 * @param ctx the parse tree
	 */
	fn visit_assemblyIf(&mut self, ctx: &AssemblyIfContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SolidityParser#assemblyLiteral}.
	 * @param ctx the parse tree
	 */
	fn visit_assemblyLiteral(&mut self, ctx: &AssemblyLiteralContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SolidityParser#tupleExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_tupleExpression(&mut self, ctx: &TupleExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SolidityParser#numberLiteral}.
	 * @param ctx the parse tree
	 */
	fn visit_numberLiteral(&mut self, ctx: &NumberLiteralContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SolidityParser#identifier}.
	 * @param ctx the parse tree
	 */
	fn visit_identifier(&mut self, ctx: &IdentifierContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SolidityParser#hexLiteral}.
	 * @param ctx the parse tree
	 */
	fn visit_hexLiteral(&mut self, ctx: &HexLiteralContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SolidityParser#overrideSpecifier}.
	 * @param ctx the parse tree
	 */
	fn visit_overrideSpecifier(&mut self, ctx: &OverrideSpecifierContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link SolidityParser#stringLiteral}.
	 * @param ctx the parse tree
	 */
	fn visit_stringLiteral(&mut self, ctx: &StringLiteralContext<'input>) { self.visit_children(ctx) }


}