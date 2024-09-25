use std::collections::HashMap;
use std::vec::Vec;

// Define a struct for Position
#[derive(Debug, Clone)]
pub struct Position {
    pub line: usize,
    pub column: usize,
}

// Define a struct for Location
#[derive(Debug, Clone)]
pub struct Location {
    pub start: Position,
    pub end: Position,
}

// Define an enum for CommentType
#[derive(Debug, Clone)]
pub enum CommentType {
    BlockComment,
    LineComment,
}

// Define a struct for Comment
#[derive(Debug, Clone)]
pub struct Comment {
    pub r#type: CommentType,
    pub value: String,
    pub range: Option<(usize, usize)>, // Optional range
    pub loc: Option<Location>,          // Optional location
}

// Define a base struct for AST nodes
#[derive(Debug, Clone)]
pub enum ASTNodeType {
    SourceUnit,
    PragmaDirective,
    ImportDirective,
    ContractDefinition,
    InheritanceSpecifier,
    StateVariableDeclaration,
    UsingForDeclaration,
    StructDefinition,
    ModifierDefinition,
    ModifierInvocation,
    FunctionDefinition,
    EventDefinition,
    CustomErrorDefinition,
    RevertStatement,
    EnumValue,
    EnumDefinition,
    VariableDeclaration,
    UserDefinedTypeName,
    Mapping,
    ArrayTypeName,
    FunctionTypeName,
    Block,
    ExpressionStatement,
    IfStatement,
    WhileStatement,
    ForStatement,
    InlineAssemblyStatement,
    DoWhileStatement,
    ContinueStatement,
    Break,
    Continue,
    BreakStatement,
    ReturnStatement,
    EmitStatement,
    ThrowStatement,
    VariableDeclarationStatement,
    ElementaryTypeName,
    FunctionCall,
    AssemblyBlock,
    AssemblyCall,
    AssemblyLocalDefinition,
    AssemblyAssignment,
    AssemblyStackAssignment,
    LabelDefinition,
    AssemblySwitch,
    AssemblyCase,
    AssemblyFunctionDefinition,
    AssemblyFor,
    AssemblyIf,
    TupleExpression,
    NameValueExpression,
    BooleanLiteral,
    NumberLiteral,
    Identifier,
    BinaryOperation,
    UnaryOperation,
    NewExpression,
    Conditional,
    StringLiteral,
    HexLiteral,
    HexNumber,
    DecimalNumber,
    MemberAccess,
    IndexAccess,
    IndexRangeAccess,
    NameValueList,
    UncheckedStatement,
    TryStatement,
    CatchClause,
    FileLevelConstant,
    AssemblyMemberAccess,
    TypeDefinition,
}

// Define a base struct for AST nodes
#[derive(Debug, Clone)]
pub struct BaseASTNode {
    pub node_type: ASTNodeType,
    pub comments: Option<Vec<Comment>>, // Optional comments
    pub range: Option<(usize, usize)>,   // Optional range
    pub loc: Option<Location>,            // Optional location
}

// Define a struct for SourceUnit
#[derive(Debug, Clone)]
pub struct SourceUnit {
    pub node_type: String, // Embedding BaseASTNode
    pub children: Vec<ASTNode>,// Children nodes, could be any AST node
    pub comments: Option<Vec<Comment>>, // Optional comments
    pub range: Option<(usize, usize)>,   // Optional range
    pub loc: Option<Location> 
}

// Define a struct for ContractDefinition
#[derive(Debug, Clone)]
pub struct ContractDefinition {
    pub node_type: String, // Should be "ContractDefinition"
    pub name: String,
    pub base_contracts: Vec<InheritanceSpecifier>,
    pub kind: String,
    pub sub_nodes: Vec<BaseASTNode>, // Sub-nodes of type BaseASTNode
    pub comments: Option<Vec<Comment>>, // Optional comments
    pub range: Option<(usize, usize)>,   // Optional range
    pub loc: Option<Location>,            // Optional location
}

// Define a struct for InheritanceSpecifier
#[derive(Debug, Clone)]
pub struct InheritanceSpecifier {
    pub node_type: String, // Should be "InheritanceSpecifier"
    pub base_name: UserDefinedTypeName,
    pub arguments: Vec<Expression>, // Arguments can be any Expression type
    pub comments: Option<Vec<Comment>>, // Optional comments
    pub range: Option<(usize, usize)>,   // Optional range
    pub loc: Option<Location>,            // Optional location
}

// Define a struct for UserDefinedTypeName
#[derive(Debug, Clone)]
pub struct UserDefinedTypeName {
    pub node_type: String, // Should be "UserDefinedTypeName"
    pub name_path: String,
    pub comments: Option<Vec<Comment>>, // Optional comments
    pub range: Option<(usize, usize)>,   // Optional range
    pub loc: Option<Location>,            // Optional location
}
#[derive(Debug, Clone)]
pub struct PragmaDirective {
    pub node_type: String, // Should be "PragmaDirective"
    pub name: String,
    pub value: String,
    pub comments: Option<Vec<Comment>>, // Optional comments
    pub range: Option<(usize, usize)>,   // Optional range
    pub loc: Option<Location>,            // Optional location
}

#[derive(Debug, Clone)]
pub struct ImportDirective {
    pub node_type: String, // Should be "ImportDirective"
    pub path: String,
    pub path_literal: StringLiteral,
    pub unit_alias: Option<String>, // Nullable
    pub unit_alias_identifier: Option<Identifier>, // Nullable
    pub symbol_aliases: Option<Vec<(String, Option<String>)>>, // Nullable
    pub symbol_aliases_identifiers: Option<Vec<(Identifier, Option<Identifier>)>>, // Nullable
    pub comments: Option<Vec<Comment>>, // Optional comments
    pub range: Option<(usize, usize)>,   // Optional range
    pub loc: Option<Location>,            // Optional location
}

#[derive(Debug, Clone)]
pub struct StateVariableDeclaration {
    pub node_type: String, // Should be "StateVariableDeclaration"
    pub variables: Vec<StateVariableDeclarationVariable>,
    pub initial_value: Option<Expression>, // Nullable
    pub comments: Option<Vec<Comment>>, // Optional comments
    pub range: Option<(usize, usize)>,   // Optional range
    pub loc: Option<Location>,            // Optional location
}

#[derive(Debug, Clone)]
pub struct FileLevelConstant {
    pub node_type: String, // Should be "FileLevelConstant"
    pub type_name: TypeName,
    pub name: String,
    pub initial_value: Expression, // Must be present
    pub is_declared_const: bool,
    pub is_immutable: bool,
    pub comments: Option<Vec<Comment>>, // Optional comments
    pub range: Option<(usize, usize)>,   // Optional range
    pub loc: Option<Location>,            // Optional location
}

#[derive(Debug, Clone)]
pub struct UsingForDeclaration {
    pub node_type: String, // Should be "UsingForDeclaration"
    pub type_name: Option<TypeName>, // Nullable
    pub functions: Vec<String>,
    pub operators: Vec<Option<String>>, // Array of Option<String>
    pub library_name: Option<String>, // Nullable
    pub is_global: bool,
    pub comments: Option<Vec<Comment>>, // Optional comments
    pub range: Option<(usize, usize)>,   // Optional range
    pub loc: Option<Location>,            // Optional location
}

#[derive(Debug, Clone)]
pub struct StructDefinition {
    pub node_type: String, // Should be "StructDefinition"
    pub name: String,
    pub members: Vec<VariableDeclaration>,
    pub comments: Option<Vec<Comment>>, // Optional comments
    pub range: Option<(usize, usize)>,   // Optional range
    pub loc: Option<Location>,            // Optional location
}

#[derive(Debug, Clone)]
pub struct ModifierDefinition {
    pub node_type: String, // Should be "ModifierDefinition"
    pub name: String,
    pub parameters: Option<Vec<VariableDeclaration>>, // Nullable
    pub is_virtual: bool,
    pub overrides: Option<Vec<UserDefinedTypeName>>, // Nullable
    pub body: Option<Box<Block>>, // Nullable
    pub comments: Option<Vec<Comment>>, // Optional comments
    pub range: Option<(usize, usize)>,   // Optional range
    pub loc: Option<Location>,            // Optional location
}

#[derive(Debug, Clone)]
pub struct ModifierInvocation {
    pub node_type: String, // Should be "ModifierInvocation"
    pub name: String,
    pub arguments: Option<Vec<Expression>>, // Nullable
    pub comments: Option<Vec<Comment>>, // Optional comments
    pub range: Option<(usize, usize)>,   // Optional range
    pub loc: Option<Location>,            // Optional location
}

#[derive(Debug, Clone)]
pub struct FunctionDefinition {
    pub node_type: String, // Should be "FunctionDefinition"
    pub name: Option<String>, // Nullable
    pub parameters: Vec<VariableDeclaration>,
    pub modifiers: Vec<ModifierInvocation>,
    pub state_mutability: Option<String>, // Can be 'pure', 'constant', 'payable', 'view', or null
    pub visibility: String, // 'default', 'external', 'internal', 'public', or 'private'
    pub return_parameters: Option<Vec<VariableDeclaration>>, // Nullable
    pub body: Option<Box<Block>>, // Nullable
    pub overrides: Option<Vec<UserDefinedTypeName>>, // Nullable
    pub is_constructor: bool,
    pub is_receive_ether: bool,
    pub is_fallback: bool,
    pub is_virtual: bool,
    pub comments: Option<Vec<Comment>>, // Optional comments
    pub range: Option<(usize, usize)>,   // Optional range
    pub loc: Option<Location>,            // Optional location
}

#[derive(Debug, Clone)]
pub struct CustomErrorDefinition {
    pub node_type: String, // Should be "CustomErrorDefinition"
    pub name: String,
    pub parameters: Vec<VariableDeclaration>,
    pub comments: Option<Vec<Comment>>, // Optional comments
    pub range: Option<(usize, usize)>,   // Optional range
    pub loc: Option<Location>,            // Optional location
}
#[derive(Debug, Clone)]
pub struct TypeDefinition {
    pub node_type: String, // Should be "TypeDefinition"
    pub name: String,
    pub definition: ElementaryTypeName,
    pub comments: Option<Vec<Comment>>, // Optional comments
    pub range: Option<(usize, usize)>,   // Optional range
    pub loc: Option<Location>,            // Optional location
}
#[derive(Debug, Clone)]
pub struct RevertStatement {
    pub node_type: String, // Should be "RevertStatement"
    pub revert_call: FunctionCall,
    pub comments: Option<Vec<Comment>>, // Optional comments
    pub range: Option<(usize, usize)>,   // Optional range
    pub loc: Option<Location>,            // Optional location
}
#[derive(Debug, Clone)]
pub struct EventDefinition {
    pub node_type: String, // Should be "EventDefinition"
    pub name: String,
    pub parameters: Vec<VariableDeclaration>,
    pub is_anonymous: bool,
    pub comments: Option<Vec<Comment>>, // Optional comments
    pub range: Option<(usize, usize)>,   // Optional range
    pub loc: Option<Location>,            // Optional location
}
#[derive(Debug, Clone)]
pub struct EnumValue {
    pub node_type: String, // Should be "EnumValue"
    pub name: String,
    pub comments: Option<Vec<Comment>>, // Optional comments
    pub range: Option<(usize, usize)>,   // Optional range
    pub loc: Option<Location>,            // Optional location
}
#[derive(Debug, Clone)]
pub struct EnumDefinition {
    pub node_type: String, // Should be "EnumDefinition"
    pub name: String,
    pub members: Vec<EnumValue>,
    pub comments: Option<Vec<Comment>>, // Optional comments
    pub range: Option<(usize, usize)>,   // Optional range
    pub loc: Option<Location>,            // Optional location
}
#[derive(Debug, Clone)]
pub struct VariableDeclaration {
    pub node_type: String, // Should be "VariableDeclaration"
    pub is_indexed: bool,
    pub is_state_var: bool,
    pub type_name: Option<TypeName>, // Nullable
    pub name: Option<String>,         // Nullable
    pub identifier: Option<Identifier>, // Nullable
    pub is_declared_const: Option<bool>, // Nullable
    pub storage_location: Option<String>, // Nullable
    pub expression: Option<Expression>, // Nullable
    pub visibility: Option<String>,     // Nullable, could be 'public', 'private', 'internal', or 'default'
    pub comments: Option<Vec<Comment>>, // Optional comments
    pub range: Option<(usize, usize)>,   // Optional range
    pub loc: Option<Location>,            // Optional location
}
// StateVariableDeclarationVariable extends VariableDeclaration
#[derive(Debug, Clone)]
pub struct StateVariableDeclarationVariable {
    pub node_type: String, // Should be "VariableDeclaration"
    pub is_indexed: bool,
    pub is_state_var: bool,
    pub type_name: Option<TypeName>, // Nullable
    pub name: Option<String>,        // Nullable
    pub identifier: Option<Identifier>, // Nullable
    pub is_declared_const: Option<bool>, // Nullable
    pub storage_location: Option<String>, // Nullable
    pub expression: Option<Expression>, // Nullable
    pub visibility: Option<String>,    // Nullable, could be 'public', 'private', 'internal', or 'default'
    pub overrides: Option<Vec<UserDefinedTypeName>>, // Nullable
    pub is_immutable: bool,
    pub comments: Option<Vec<Comment>>, // Optional comments
    pub range: Option<(usize, usize)>,   // Optional range
    pub loc: Option<Location>,            // Optional location
}

// ArrayTypeName
#[derive(Debug, Clone)]
pub struct ArrayTypeName {
    pub node_type: String, // Should be "ArrayTypeName"
    pub base_type_name: TypeName,
    pub length: Option<Expression>, // Nullable
    pub comments: Option<Vec<Comment>>, // Optional comments
    pub range: Option<(usize, usize)>,   // Optional range
    pub loc: Option<Location>,            // Optional location
}

// Mapping
#[derive(Debug, Clone)]
pub struct Mapping {
    pub node_type: String, // Should be "Mapping"
    pub key_type: MappingKeyType, // Either ElementaryTypeName or UserDefinedTypeName
    pub key_name: Option<Identifier>, // Nullable
    pub value_type: Box<TypeName>,
    pub value_name: Option<Identifier>, // Nullable
    pub comments: Option<Vec<Comment>>, // Optional comments
    pub range: Option<(usize, usize)>,   // Optional range
    pub loc: Option<Location>,            // Optional location
}

// Enum for keyType, can be either ElementaryTypeName or UserDefinedTypeName
#[derive(Debug, Clone)]
pub enum MappingKeyType {
    ElementaryTypeName(ElementaryTypeName),
    UserDefinedTypeName(UserDefinedTypeName),
}

// FunctionTypeName#[derive(Debug, Clone)]
#[derive(Debug, Clone)]
pub struct FunctionTypeName {
    pub node_type: String, // Should be "FunctionTypeName"
    pub parameter_types: Vec<VariableDeclaration>,
    pub return_types: Vec<VariableDeclaration>,
    pub visibility: String,
    pub state_mutability: Option<String>, // Nullable
    pub comments: Option<Vec<Comment>>, // Optional comments
    pub range: Option<(usize, usize)>,   // Optional range
    pub loc: Option<Location>,            // Optional location
}

// Block
#[derive(Debug, Clone)]
pub struct Block {
    pub node_type: String, // Should be "Block"
    pub statements: Vec<ASTNode>, // Contains various AST nodes
    pub comments: Option<Vec<Comment>>, // Optional comments
    pub range: Option<(usize, usize)>,   // Optional range
    pub loc: Option<Location>,            // Optional location
}

// ExpressionStatement
#[derive(Debug, Clone)]
pub struct ExpressionStatement {
    pub node_type: String, // Should be "ExpressionStatement"
    pub expression: Option<Expression>, // Nullable
    pub comments: Option<Vec<Comment>>, // Optional comments
    pub range: Option<(usize, usize)>,   // Optional range
    pub loc: Option<Location>,            // Optional location
}

// IfStatement
#[derive(Debug, Clone)]
pub struct IfStatement {
    pub node_type: String, // Should be "IfStatement"
    pub condition: Expression,
    pub true_body: ASTNode,
    pub false_body: Option<ASTNode>, // Nullable
    pub comments: Option<Vec<Comment>>, // Optional comments
    pub range: Option<(usize, usize)>,   // Optional range
    pub loc: Option<Location>,            // Optional location
}

// UncheckedStatement
#[derive(Debug, Clone)]
pub struct UncheckedStatement {
    pub node_type: String, // Should be "UncheckedStatement"
    pub block: Block,
    pub comments: Option<Vec<Comment>>, // Optional comments
    pub range: Option<(usize, usize)>,   // Optional range
    pub loc: Option<Location>,            // Optional location
}

// TryStatement
#[derive(Debug, Clone)]
pub struct TryStatement {
    pub node_type: String, // Should be "TryStatement"
    pub expression: Expression,
    pub return_parameters: Option<Vec<VariableDeclaration>>, // Nullable
    pub body: Block,
    pub catch_clauses: Vec<CatchClause>,
    pub comments: Option<Vec<Comment>>, // Optional comments
    pub range: Option<(usize, usize)>,   // Optional range
    pub loc: Option<Location>,            // Optional location
}

// CatchClause
#[derive(Debug, Clone)]
pub struct CatchClause {
    pub node_type: String, // Should be "CatchClause"
    pub is_reason_string_type: bool,
    pub kind: Option<String>, // Nullable
    pub parameters: Option<Vec<VariableDeclaration>>, // Nullable
    pub body: Block,
    pub comments: Option<Vec<Comment>>, // Optional comments
    pub range: Option<(usize, usize)>,   // Optional range
    pub loc: Option<Location>,            // Optional location
}

// WhileStatement
#[derive(Debug, Clone)]
pub struct WhileStatement {
    pub node_type: String, // Should be "WhileStatement"
    pub condition: Expression,
    pub body: ASTNode,
    pub comments: Option<Vec<Comment>>, // Optional comments
    pub range: Option<(usize, usize)>,   // Optional range
    pub loc: Option<Location>,            // Optional location
}

// ForStatement
#[derive(Debug, Clone)]
pub struct ForStatement {
    pub node_type: String, // Should be "ForStatement"
    pub init_expression: Option<ASTNode>, // Nullable
    pub condition_expression: Option<Expression>, // Optional
    pub loop_expression: ExpressionStatement,
    pub body: ASTNode,
    pub comments: Option<Vec<Comment>>, // Optional comments
    pub range: Option<(usize, usize)>,   // Optional range
    pub loc: Option<Location>,            // Optional location
}

// InlineAssemblyStatement
#[derive(Debug, Clone)]
pub struct InlineAssemblyStatement {
    pub node_type: String, // Should be "InlineAssemblyStatement"
    pub language: Option<String>, // Nullable
    pub flags: Vec<String>,
    pub body: AssemblyBlock,
    pub comments: Option<Vec<Comment>>, // Optional comments
    pub range: Option<(usize, usize)>,   // Optional range
    pub loc: Option<Location>,            // Optional location
}

// DoWhileStatement
#[derive(Debug, Clone)]
pub struct DoWhileStatement {
    pub node_type: String, // Should be "DoWhileStatement"
    pub condition: Expression,
    pub body: ASTNode,
    pub comments: Option<Vec<Comment>>, // Optional comments
    pub range: Option<(usize, usize)>,   // Optional range
    pub loc: Option<Location>,            // Optional location
}

// ContinueStatement
#[derive(Debug, Clone)]
pub struct ContinueStatement {
    pub node_type: String, // Should be "ContinueStatement"
    pub comments: Option<Vec<Comment>>, // Optional comments
    pub range: Option<(usize, usize)>,   // Optional range
    pub loc: Option<Location>,            // Optional location
}

// BreakStatement
#[derive(Debug, Clone)]
pub struct BreakStatement {
    pub node_type: String, // Should be "BreakStatement"
    pub comments: Option<Vec<Comment>>, // Optional comments
    pub range: Option<(usize, usize)>,   // Optional range
    pub loc: Option<Location>,            // Optional location
}

// ReturnStatement
#[derive(Debug, Clone)]
pub struct ReturnStatement {
    pub node_type: String, // Should be "ReturnStatement"
    pub expression: Option<Expression>, // Nullable
    pub comments: Option<Vec<Comment>>, // Optional comments
    pub range: Option<(usize, usize)>,   // Optional range
    pub loc: Option<Location>,            // Optional location
}

// EmitStatement
#[derive(Debug, Clone)]
pub struct EmitStatement {
    pub node_type: String, // Should be "EmitStatement"
    pub event_call: FunctionCall,
    pub comments: Option<Vec<Comment>>, // Optional comments
    pub range: Option<(usize, usize)>,   // Optional range
    pub loc: Option<Location>,            // Optional location
}

// ThrowStatement
#[derive(Debug, Clone)]
pub struct ThrowStatement {
    pub node_type: String, // Should be "ThrowStatement"
    pub comments: Option<Vec<Comment>>, // Optional comments
    pub range: Option<(usize, usize)>,   // Optional range
    pub loc: Option<Location>,            // Optional location
}

// VariableDeclarationStatement
#[derive(Debug, Clone)]
pub struct VariableDeclarationStatement {
    pub node_type: String, // Should be "VariableDeclarationStatement"
    pub variables: Vec<Option<ASTNode>>,
    pub initial_value: Option<Expression>, // Nullable
    pub comments: Option<Vec<Comment>>,    // Optional comments
    pub range: Option<(usize, usize)>,     // Optional range
    pub loc: Option<Location>,             // Optional location
}

// ElementaryTypeName
#[derive(Debug, Clone)]
pub struct ElementaryTypeName {
    pub node_type: String, // Should be "ElementaryTypeName"
    pub name: String,
    pub state_mutability: Option<String>,  // Nullable
    pub comments: Option<Vec<Comment>>,    // Optional comments
    pub range: Option<(usize, usize)>,     // Optional range
    pub loc: Option<Location>,             // Optional location
}

// FunctionCall
#[derive(Debug, Clone)]
pub struct FunctionCall {
    pub node_type: String, // Should be "FunctionCall"
    pub expression: Expression,
    pub arguments: Vec<Expression>,
    pub names: Vec<String>,
    pub identifiers: Vec<Identifier>,
    pub comments: Option<Vec<Comment>>,    // Optional comments
    pub range: Option<(usize, usize)>,     // Optional range
    pub loc: Option<Location>,             // Optional location
}

// AssemblyBlock
#[derive(Debug, Clone)]
pub struct AssemblyBlock {
    pub node_type: String, // Should be "AssemblyBlock"
    pub operations: Vec<AssemblyItem>,
    pub comments: Option<Vec<Comment>>,    // Optional comments
    pub range: Option<(usize, usize)>,     // Optional range
    pub loc: Option<Location>,             // Optional location
}
#[derive(Debug, Clone)]
// AssemblyCall
pub struct AssemblyCall {
    pub node_type: String, // Should be "AssemblyCall"
    pub function_name: String,
    pub arguments: Vec<AssemblyExpression>,
    pub comments: Option<Vec<Comment>>,    // Optional comments
    pub range: Option<(usize, usize)>,     // Optional range
    pub loc: Option<Location>,             // Optional location
}
#[derive(Debug, Clone)]
// AssemblyLocalDefinition
pub struct AssemblyLocalDefinition {
    pub node_type: String, // Should be "AssemblyLocalDefinition"
    pub names: Vec<AssemblyLocalNames>,  // Either Identifiers or AssemblyMemberAccesses
    pub expression: Option<AssemblyExpression>, // Nullable
    pub comments: Option<Vec<Comment>>,    // Optional comments
    pub range: Option<(usize, usize)>,     // Optional range
    pub loc: Option<Location>,             // Optional location
}
#[derive(Debug, Clone)]
// Enum for names in AssemblyLocalDefinition
pub enum AssemblyLocalNames {
    Identifier(Identifier),
    AssemblyMemberAccess(AssemblyMemberAccess),
}
#[derive(Debug, Clone)]
// AssemblyAssignment
pub struct AssemblyAssignment {
    pub node_type: String, // Should be "AssemblyAssignment"
    pub names: Vec<AssemblyLocalNames>, // Either Identifiers or AssemblyMemberAccesses
    pub expression: AssemblyExpression,
    pub comments: Option<Vec<Comment>>,    // Optional comments
    pub range: Option<(usize, usize)>,     // Optional range
    pub loc: Option<Location>,             // Optional location
}
#[derive(Debug, Clone)]
// AssemblyStackAssignment
pub struct AssemblyStackAssignment {
    pub node_type: String, // Should be "AssemblyStackAssignment"
    pub name: String,
    pub expression: AssemblyExpression,
    pub comments: Option<Vec<Comment>>,    // Optional comments
    pub range: Option<(usize, usize)>,     // Optional range
    pub loc: Option<Location>,             // Optional location
}
#[derive(Debug, Clone)]
// LabelDefinition
pub struct LabelDefinition {
    pub node_type: String, // Should be "LabelDefinition"
    pub name: String,
    pub comments: Option<Vec<Comment>>,    // Optional comments
    pub range: Option<(usize, usize)>,     // Optional range
    pub loc: Option<Location>,             // Optional location
}
#[derive(Debug, Clone)]
// AssemblySwitch
pub struct AssemblySwitch {
    pub node_type: String, // Should be "AssemblySwitch"
    pub expression: AssemblyExpression,
    pub cases: Vec<AssemblyCase>,
    pub comments: Option<Vec<Comment>>,    // Optional comments
    pub range: Option<(usize, usize)>,     // Optional range
    pub loc: Option<Location>,             // Optional location
}
#[derive(Debug, Clone)]
// AssemblyCase
pub struct AssemblyCase {
    pub node_type: String, // Should be "AssemblyCase"
    pub value: Option<AssemblyLiteral>,    // Nullable
    pub block: AssemblyBlock,
    pub is_default: bool,                  // To represent the 'default' case
    pub comments: Option<Vec<Comment>>,    // Optional comments
    pub range: Option<(usize, usize)>,     // Optional range
    pub loc: Option<Location>,             // Optional location
}
#[derive(Debug, Clone)]
// AssemblyFunctionDefinition
pub struct AssemblyFunctionDefinition {
    pub node_type: String, // Should be "AssemblyFunctionDefinition"
    pub name: String,
    pub arguments: Vec<Identifier>,
    pub return_arguments: Vec<Identifier>,
    pub body: AssemblyBlock,
    pub comments: Option<Vec<Comment>>,    // Optional comments
    pub range: Option<(usize, usize)>,     // Optional range
    pub loc: Option<Location>,             // Optional location
}
#[derive(Debug, Clone)]
// AssemblyFor
pub struct AssemblyFor {
    pub node_type: String, // Should be "AssemblyFor"
    pub pre: AssemblyPreCondition,
    pub condition: AssemblyExpression,
    pub post: AssemblyPreCondition,
    pub body: AssemblyBlock,
    pub comments: Option<Vec<Comment>>,    // Optional comments
    pub range: Option<(usize, usize)>,     // Optional range
    pub loc: Option<Location>,             // Optional location
}
#[derive(Debug, Clone)]
// Enum for pre/post conditions in AssemblyFor
pub enum AssemblyPreCondition {
    Block(AssemblyBlock),
    Expression(AssemblyExpression),
}
#[derive(Debug, Clone)]
// AssemblyIf
pub struct AssemblyIf {
    pub node_type: String, // Should be "AssemblyIf"
    pub condition: AssemblyExpression,
    pub body: AssemblyBlock,
    pub comments: Option<Vec<Comment>>,    // Optional comments
    pub range: Option<(usize, usize)>,     // Optional range
    pub loc: Option<Location>,             // Optional location
}
#[derive(Debug, Clone)]
// Enum for AssemblyLiteral
pub enum AssemblyLiteral {
    StringLiteral(StringLiteral),
    BooleanLiteral(BooleanLiteral),
    DecimalNumber(DecimalNumber),
    HexNumber(HexNumber),
    HexLiteral(HexLiteral),
}
#[derive(Debug, Clone)]
// AssemblyMemberAccess
pub struct AssemblyMemberAccess {
    pub node_type: String, // Should be "AssemblyMemberAccess"
    pub expression: Identifier,
    pub member_name: Identifier,
    pub comments: Option<Vec<Comment>>,    // Optional comments
    pub range: Option<(usize, usize)>,     // Optional range
    pub loc: Option<Location>,             // Optional location
}
#[derive(Debug, Clone)]
// NewExpression
pub struct NewExpression {
    pub node_type: String, // Should be "NewExpression"
    pub type_name: TypeName,
    pub comments: Option<Vec<Comment>>,    // Optional comments
    pub range: Option<(usize, usize)>,     // Optional range
    pub loc: Option<Location>,             // Optional location
}
#[derive(Debug, Clone)]
// TupleExpression
pub struct TupleExpression {
    pub node_type: String, // Should be "TupleExpression"
    pub components: Vec<Option<ASTNode>>, // Nullable components
    pub is_array: bool,
    pub comments: Option<Vec<Comment>>,    // Optional comments
    pub range: Option<(usize, usize)>,     // Optional range
    pub loc: Option<Location>,             // Optional location
}
#[derive(Debug, Clone)]
// NameValueExpression
pub struct NameValueExpression {
    pub node_type: String, // Should be "NameValueExpression"
    pub expression: Expression,
    pub arguments: NameValueList,
    pub comments: Option<Vec<Comment>>,    // Optional comments
    pub range: Option<(usize, usize)>,     // Optional range
    pub loc: Option<Location>,             // Optional location
}
#[derive(Debug, Clone)]
// NumberLiteral
pub struct NumberLiteral {
    pub node_type: String, // Should be "NumberLiteral"
    pub number: String,
    pub subdenomination: Option<Subdenomination>,  // Nullable
    pub comments: Option<Vec<Comment>>,            // Optional comments
    pub range: Option<(usize, usize)>,             // Optional range
    pub loc: Option<Location>,                     // Optional location
}
#[derive(Debug, Clone)]
pub enum Subdenomination {
    Wei,
    Szabo,
    Finney,
    Ether,
    Seconds,
    Minutes,
    Hours,
    Days,
    Weeks,
    Years,
}
#[derive(Debug, Clone)]
// BooleanLiteral
pub struct BooleanLiteral {
    pub node_type: String, // Should be "BooleanLiteral"
    pub value: bool,
    pub comments: Option<Vec<Comment>>,    // Optional comments
    pub range: Option<(usize, usize)>,     // Optional range
    pub loc: Option<Location>,             // Optional location
}
#[derive(Debug, Clone)]
// HexLiteral
pub struct HexLiteral {
    pub node_type: String, // Should be "HexLiteral"
    pub value: String,
    pub parts: Vec<String>,
    pub comments: Option<Vec<Comment>>,    // Optional comments
    pub range: Option<(usize, usize)>,     // Optional range
    pub loc: Option<Location>,             // Optional location
}
#[derive(Debug, Clone)]
// StringLiteral
pub struct StringLiteral {
    pub node_type: String, // Should be "StringLiteral"
    pub value: String,
    pub parts: Vec<String>,
    pub is_unicode: Vec<bool>, // Represents isUnicode as an array of booleans
    pub comments: Option<Vec<Comment>>,    // Optional comments
    pub range: Option<(usize, usize)>,     // Optional range
    pub loc: Option<Location>,             // Optional location
}
#[derive(Debug, Clone)]
// Identifier
pub struct Identifier {
    pub node_type: String, // Should be "Identifier"
    pub name: String,
    pub comments: Option<Vec<Comment>>,    // Optional comments
    pub range: Option<(usize, usize)>,     // Optional range
    pub loc: Option<Location>,             // Optional location
}

// BinOp (Binary Operators)
#[derive(Clone, Debug)]
pub enum BinOp {
    Add,        // '+'
    Subtract,   // '-'
    Multiply,   // '*'
    Divide,     // '/'
    Exponent,   // '**'
    Modulus,    // '%'
    ShiftLeft,  // '<<'
    ShiftRight, // '>>'
    And,        // '&&'
    Or,         // '||'
    BitAnd,     // '&'
    BitXor,     // '^'
    LessThan,   // '<'
    GreaterThan,// '>'
    LessOrEqual,// '<='
    GreaterOrEqual, // '>='
    Equal,      // '=='
    NotEqual,   // '!='
    Assign,     // '='
    BitXorAssign,// '^='
    BitAndAssign,// '&='
    ShiftLeftAssign,// '<<='
    ShiftRightAssign,// '>>='
    AddAssign,  // '+='
    SubtractAssign,// '-='
    MultiplyAssign,// '*='
    DivideAssign,// '/='
    ModulusAssign,// '%='
    BitOr,      // '|'
    BitOrAssign,// '|='
}

// UnaryOp (Unary Operators)
#[derive(Clone, Debug)]
pub enum UnaryOp {
    Negate,     // '-'
    Positive,   // '+'
    Increment,  // '++'
    Decrement,  // '--'
    BitwiseNot, // '~'
    After,      // 'after'
    Delete,     // 'delete'
    Not,        // '!'
}
#[derive(Debug, Clone)]
// BinaryOperation
pub struct BinaryOperation {
    pub node_type: String, // Should be "BinaryOperation"
    pub left:Box<Expression>,
    pub right: Box<Expression>,
    pub operator: BinOp,
    pub comments: Option<Vec<Comment>>,    // Optional comments
    pub range: Option<(usize, usize)>,     // Optional range
    pub loc: Option<Location>,             // Optional location
}
#[derive(Debug, Clone)]
// UnaryOperation
pub struct UnaryOperation {
    pub node_type: String, // Should be "UnaryOperation"
    pub operator: UnaryOp,
    pub sub_expression: Expression,
    pub is_prefix: bool,
    pub comments: Option<Vec<Comment>>,    // Optional comments
    pub range: Option<(usize, usize)>,     // Optional range
    pub loc: Option<Location>,             // Optional location
}
#[derive(Debug, Clone)]
// Conditional
pub struct Conditional {
    pub node_type: String, // Should be "Conditional"
    pub condition: Box<Expression>,
    pub true_expression: Box<Expression>,
    pub false_expression: Box<Expression>,
    pub comments: Option<Vec<Comment>>,    // Optional comments
    pub range: Option<(usize, usize)>,     // Optional range
    pub loc: Option<Location>,             // Optional location
}
#[derive(Debug, Clone)]
// IndexAccess
pub struct IndexAccess {
    pub node_type: String, // Should be "IndexAccess"
    pub base: Box<Expression>,
    pub index: Box<Expression>,
    pub comments: Option<Vec<Comment>>,    // Optional comments
    pub range: Option<(usize, usize)>,     // Optional range
    pub loc: Option<Location>,             // Optional location
}
#[derive(Debug, Clone)]
pub struct IndexRangeAccess {
    pub node_type: String,                 // Should be "IndexRangeAccess"
    pub base: Box<Expression>,                  // Base expression
    pub index_start: Option<Box<Expression>>,   // Optional start index
    pub index_end: Option<Box<Expression>>,     // Optional end index
    pub comments: Option<Vec<Comment>>,    // Optional comments
    pub range: Option<(usize, usize)>,     // Optional range
    pub loc: Option<Location>,             // Optional location
}
#[derive(Debug, Clone)]
// MemberAccess struct
pub struct MemberAccess {
    pub node_type: String,                 // Should be "MemberAccess"
    pub expression: Box<Expression>,            // Expression being accessed
    pub member_name: String,               // Name of the member being accessed
    pub comments: Option<Vec<Comment>>,    // Optional comments
    pub range: Option<(usize, usize)>,     // Optional range
    pub loc: Option<Location>,             // Optional location
}
#[derive(Debug, Clone)]
// HexNumber struct
pub struct HexNumber {
    pub node_type: String,                 // Should be "HexNumber"
    pub value: String,                     // The hex number value
    pub comments: Option<Vec<Comment>>,    // Optional comments
    pub range: Option<(usize, usize)>,     // Optional range
    pub loc: Option<Location>,             // Optional location
}
#[derive(Debug, Clone)]
// DecimalNumber struct
pub struct DecimalNumber {
    pub node_type: String,                 // Should be "DecimalNumber"
    pub value: String,                     // The decimal number value
    pub comments: Option<Vec<Comment>>,    // Optional comments
    pub range: Option<(usize, usize)>,     // Optional range
    pub loc: Option<Location>,             // Optional location
}
#[derive(Debug, Clone)]
// NameValueList struct
pub struct NameValueList {
    pub node_type: String,                 // Should be "NameValueList"
    pub names: Vec<String>,                // List of names
    pub identifiers: Vec<Identifier>,      // List of identifiers
    pub arguments: Vec<Expression>,        // List of expressions (arguments)
    pub comments: Option<Vec<Comment>>,    // Optional comments
    pub range: Option<(usize, usize)>,     // Optional range
    pub loc: Option<Location>,             // Optional location
}

// ASTNode Enum in Rust
#[derive(Debug, Clone)]
pub enum ASTNode {
    SourceUnit(Box<SourceUnit>),
    PragmaDirective(Box<PragmaDirective>),
    ImportDirective(Box<ImportDirective>),
    ContractDefinition(Box<ContractDefinition>),
    InheritanceSpecifier(Box<InheritanceSpecifier>),
    StateVariableDeclaration(Box<StateVariableDeclaration>),
    UsingForDeclaration(Box<UsingForDeclaration>),
    StructDefinition(Box<StructDefinition>),
    ModifierDefinition(Box<ModifierDefinition>),
    ModifierInvocation(Box<ModifierInvocation>),
    FunctionDefinition(Box<FunctionDefinition>),
    EventDefinition(Box<EventDefinition>),
    CustomErrorDefinition(Box<CustomErrorDefinition>),
    EnumValue(Box<EnumValue>),
    EnumDefinition(Box<EnumDefinition>),
    VariableDeclaration(Box<VariableDeclaration>),
    TypeName(Box<TypeName>),
    UserDefinedTypeName(Box<UserDefinedTypeName>),
    Mapping(Box<Mapping>),
    FunctionTypeName(Box<FunctionTypeName>),
    Block(Box<Block>),
    Statement(Box<Statement>),
    ElementaryTypeName(Box<ElementaryTypeName>),
    AssemblyBlock(Box<AssemblyBlock>),
    AssemblyCall(Box<AssemblyCall>),
    AssemblyLocalDefinition(Box<AssemblyLocalDefinition>),
    AssemblyAssignment(Box<AssemblyAssignment>),
    AssemblyStackAssignment(Box<AssemblyStackAssignment>),
    LabelDefinition(Box<LabelDefinition>),
    AssemblySwitch(Box<AssemblySwitch>),
    AssemblyCase(Box<AssemblyCase>),
    AssemblyFunctionDefinition(Box<AssemblyFunctionDefinition>),
    AssemblyFor(Box<AssemblyFor>),
    AssemblyIf(Box<AssemblyIf>),
    AssemblyLiteral(Box<AssemblyLiteral>),
    TupleExpression(Box<TupleExpression>),
    BinaryOperation(Box<BinaryOperation>),
    Conditional(Box<Conditional>),
    IndexAccess(Box<IndexAccess>),
    IndexRangeAccess(Box<IndexRangeAccess>),
    AssemblyItem(Box<AssemblyItem>),
    Expression(Box<Expression>),
    NameValueList(Box<NameValueList>),
    AssemblyMemberAccess(Box<AssemblyMemberAccess>),
    CatchClause(Box<CatchClause>),
    FileLevelConstant(Box<FileLevelConstant>),
    TypeDefinition(Box<TypeDefinition>),
}

#[derive(Debug, Clone)]
// AssemblyItem Enum in Rust
pub enum AssemblyItem {
    Identifier(Identifier),
    AssemblyBlock(AssemblyBlock),
    AssemblyExpression(AssemblyExpression),
    AssemblyLocalDefinition(AssemblyLocalDefinition),
    AssemblyAssignment(AssemblyAssignment),
    AssemblyStackAssignment(AssemblyStackAssignment),
    LabelDefinition(LabelDefinition),
    AssemblySwitch(AssemblySwitch),
    AssemblyFunctionDefinition(AssemblyFunctionDefinition),
    AssemblyFor(AssemblyFor),
    AssemblyIf(AssemblyIf),
    Break(BreakStatement),
    Continue(ContinueStatement),
    NumberLiteral(NumberLiteral),
    StringLiteral(StringLiteral),
    HexNumber(HexNumber),
    HexLiteral(HexLiteral),
    DecimalNumber(DecimalNumber),
}
#[derive(Debug, Clone)]
// AssemblyExpression Enum in Rust
pub enum AssemblyExpression {
    AssemblyCall(AssemblyCall),
    AssemblyLiteral(AssemblyLiteral),
}
#[derive(Debug, Clone)]
// Expression Enum in Rust
pub enum Expression {
    IndexAccess(Box<IndexAccess>),
    IndexRangeAccess(Box<IndexRangeAccess>),
    TupleExpression(Box<TupleExpression>),
    BinaryOperation(Box<BinaryOperation>),
    Conditional(Box<Conditional>),
    MemberAccess(Box<MemberAccess>),
    FunctionCall(Box<FunctionCall>),
    UnaryOperation(Box<UnaryOperation>),
    NewExpression(Box<NewExpression>),
    PrimaryExpression(Box<PrimaryExpression>),
    NameValueExpression(Box<NameValueExpression>),
}
#[derive(Debug, Clone)]
// PrimaryExpression Enum in Rust
pub enum PrimaryExpression {
    BooleanLiteral(BooleanLiteral),
    HexLiteral(HexLiteral),
    StringLiteral(StringLiteral),
    NumberLiteral(NumberLiteral),
    Identifier(Identifier),
    TupleExpression(TupleExpression),
    TypeName(TypeName),
}
#[derive(Debug, Clone)]
//#[derive(Debug, Clone)] SimpleStatement Enum in Rust
pub enum SimpleStatement {
    VariableDeclarationStatement(VariableDeclarationStatement),
    ExpressionStatement(ExpressionStatement),
}
#[derive(Debug, Clone)]
// TypeName Enum in Rust
pub enum TypeName {
    ElementaryTypeName(ElementaryTypeName),
    UserDefinedTypeName(UserDefinedTypeName),
    Mapping(Box<Mapping>),
    ArrayTypeName(Box<ArrayTypeName>),
    FunctionTypeName(FunctionTypeName),
}
#[derive(Debug, Clone)]
// Statement Enum in Rust
pub enum Statement {
    IfStatement(IfStatement),
    WhileStatement(WhileStatement),
    ForStatement(ForStatement),
    Block(Block),
    InlineAssemblyStatement(InlineAssemblyStatement),
    DoWhileStatement(DoWhileStatement),
    ContinueStatement(ContinueStatement),
    BreakStatement(BreakStatement),
    ReturnStatement(ReturnStatement),
    EmitStatement(EmitStatement),
    ThrowStatement(ThrowStatement),
    SimpleStatement(SimpleStatement),
    VariableDeclarationStatement(VariableDeclarationStatement),
    UncheckedStatement(UncheckedStatement),
    TryStatement(TryStatement),
    RevertStatement(RevertStatement),
}
#[derive(Debug, Clone)]
// ASTVisitor structs for enter and exit functions
pub struct ASTVisitorEnter {
    pub source_unit: Option<Box<dyn Fn(&SourceUnit, Option<&ASTNode>)>>,
    pub pragma_directive: Option<Box<dyn Fn(&PragmaDirective, Option<&ASTNode>)>>,
    // Continue adding for each ASTNode type...
}
#[derive(Debug, Clone)]
pub struct ASTVisitorExit {
    pub source_unit_exit: Option<Box<dyn Fn(&SourceUnit, Option<&ASTNode>)>>,
    pub pragma_directive_exit: Option<Box<dyn Fn(&PragmaDirective, Option<&ASTNode>)>>,
    // Continue adding for each ASTNode type...
}
#[derive(Debug, Clone)]
// ASTVisitor combining enter and exit
pub struct ASTVisitor {
    pub enter: ASTVisitorEnter,
    pub exit: ASTVisitorExit,
}









