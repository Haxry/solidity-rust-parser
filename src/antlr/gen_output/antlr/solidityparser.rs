// Generated from antlr/Solidity.g4 by ANTLR 4.8
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
use antlr_rust::PredictionContextCache;
use antlr_rust::parser::{Parser, BaseParser, ParserRecog, ParserNodeType};
use antlr_rust::token_stream::TokenStream;
use antlr_rust::TokenSource;
use antlr_rust::parser_atn_simulator::ParserATNSimulator;
use antlr_rust::errors::*;
use antlr_rust::rule_context::{BaseRuleContext, CustomRuleContext, RuleContext};
use antlr_rust::recognizer::{Recognizer,Actions};
use antlr_rust::atn_deserializer::ATNDeserializer;
use antlr_rust::dfa::DFA;
use antlr_rust::atn::{ATN, INVALID_ALT};
use antlr_rust::error_strategy::{ErrorStrategy, DefaultErrorStrategy};
use antlr_rust::parser_rule_context::{BaseParserRuleContext, ParserRuleContext,cast,cast_mut};
use antlr_rust::tree::*;
use antlr_rust::token::{TOKEN_EOF,OwningToken,Token};
use antlr_rust::int_stream::EOF;
use antlr_rust::vocabulary::{Vocabulary,VocabularyImpl};
use antlr_rust::token_factory::{CommonTokenFactory,TokenFactory, TokenAware};
use super::soliditylistener::*;
use antlr_rust::lazy_static;
use antlr_rust::{TidAble,TidExt};

use std::marker::PhantomData;
use std::sync::Arc;
use std::rc::Rc;
use std::convert::TryFrom;
use std::cell::RefCell;
use std::ops::{DerefMut, Deref};
use std::borrow::{Borrow,BorrowMut};
use std::any::{Any,TypeId};

		pub const T__0:isize=1; 
		pub const T__1:isize=2; 
		pub const T__2:isize=3; 
		pub const T__3:isize=4; 
		pub const T__4:isize=5; 
		pub const T__5:isize=6; 
		pub const T__6:isize=7; 
		pub const T__7:isize=8; 
		pub const T__8:isize=9; 
		pub const T__9:isize=10; 
		pub const T__10:isize=11; 
		pub const T__11:isize=12; 
		pub const T__12:isize=13; 
		pub const T__13:isize=14; 
		pub const T__14:isize=15; 
		pub const T__15:isize=16; 
		pub const T__16:isize=17; 
		pub const T__17:isize=18; 
		pub const T__18:isize=19; 
		pub const T__19:isize=20; 
		pub const T__20:isize=21; 
		pub const T__21:isize=22; 
		pub const T__22:isize=23; 
		pub const T__23:isize=24; 
		pub const T__24:isize=25; 
		pub const T__25:isize=26; 
		pub const T__26:isize=27; 
		pub const T__27:isize=28; 
		pub const T__28:isize=29; 
		pub const T__29:isize=30; 
		pub const T__30:isize=31; 
		pub const T__31:isize=32; 
		pub const T__32:isize=33; 
		pub const T__33:isize=34; 
		pub const T__34:isize=35; 
		pub const T__35:isize=36; 
		pub const T__36:isize=37; 
		pub const T__37:isize=38; 
		pub const T__38:isize=39; 
		pub const T__39:isize=40; 
		pub const T__40:isize=41; 
		pub const T__41:isize=42; 
		pub const T__42:isize=43; 
		pub const T__43:isize=44; 
		pub const T__44:isize=45; 
		pub const T__45:isize=46; 
		pub const T__46:isize=47; 
		pub const T__47:isize=48; 
		pub const T__48:isize=49; 
		pub const T__49:isize=50; 
		pub const T__50:isize=51; 
		pub const T__51:isize=52; 
		pub const T__52:isize=53; 
		pub const T__53:isize=54; 
		pub const T__54:isize=55; 
		pub const T__55:isize=56; 
		pub const T__56:isize=57; 
		pub const T__57:isize=58; 
		pub const T__58:isize=59; 
		pub const T__59:isize=60; 
		pub const T__60:isize=61; 
		pub const T__61:isize=62; 
		pub const T__62:isize=63; 
		pub const T__63:isize=64; 
		pub const T__64:isize=65; 
		pub const T__65:isize=66; 
		pub const T__66:isize=67; 
		pub const T__67:isize=68; 
		pub const T__68:isize=69; 
		pub const T__69:isize=70; 
		pub const T__70:isize=71; 
		pub const T__71:isize=72; 
		pub const T__72:isize=73; 
		pub const T__73:isize=74; 
		pub const T__74:isize=75; 
		pub const T__75:isize=76; 
		pub const T__76:isize=77; 
		pub const T__77:isize=78; 
		pub const T__78:isize=79; 
		pub const T__79:isize=80; 
		pub const T__80:isize=81; 
		pub const T__81:isize=82; 
		pub const T__82:isize=83; 
		pub const T__83:isize=84; 
		pub const T__84:isize=85; 
		pub const T__85:isize=86; 
		pub const T__86:isize=87; 
		pub const T__87:isize=88; 
		pub const T__88:isize=89; 
		pub const T__89:isize=90; 
		pub const T__90:isize=91; 
		pub const T__91:isize=92; 
		pub const T__92:isize=93; 
		pub const T__93:isize=94; 
		pub const T__94:isize=95; 
		pub const T__95:isize=96; 
		pub const Int:isize=97; 
		pub const Uint:isize=98; 
		pub const Byte:isize=99; 
		pub const Fixed:isize=100; 
		pub const Ufixed:isize=101; 
		pub const BooleanLiteral:isize=102; 
		pub const DecimalNumber:isize=103; 
		pub const HexNumber:isize=104; 
		pub const NumberUnit:isize=105; 
		pub const HexLiteralFragment:isize=106; 
		pub const ReservedKeyword:isize=107; 
		pub const AnonymousKeyword:isize=108; 
		pub const BreakKeyword:isize=109; 
		pub const ConstantKeyword:isize=110; 
		pub const ImmutableKeyword:isize=111; 
		pub const ContinueKeyword:isize=112; 
		pub const LeaveKeyword:isize=113; 
		pub const ExternalKeyword:isize=114; 
		pub const IndexedKeyword:isize=115; 
		pub const InternalKeyword:isize=116; 
		pub const PayableKeyword:isize=117; 
		pub const PrivateKeyword:isize=118; 
		pub const PublicKeyword:isize=119; 
		pub const VirtualKeyword:isize=120; 
		pub const PureKeyword:isize=121; 
		pub const TypeKeyword:isize=122; 
		pub const ViewKeyword:isize=123; 
		pub const GlobalKeyword:isize=124; 
		pub const ConstructorKeyword:isize=125; 
		pub const FallbackKeyword:isize=126; 
		pub const ReceiveKeyword:isize=127; 
		pub const Identifier:isize=128; 
		pub const StringLiteralFragment:isize=129; 
		pub const VersionLiteral:isize=130; 
		pub const WS:isize=131; 
		pub const COMMENT:isize=132; 
		pub const LINE_COMMENT:isize=133;
	pub const RULE_sourceUnit:usize = 0; 
	pub const RULE_pragmaDirective:usize = 1; 
	pub const RULE_pragmaName:usize = 2; 
	pub const RULE_pragmaValue:usize = 3; 
	pub const RULE_version:usize = 4; 
	pub const RULE_versionOperator:usize = 5; 
	pub const RULE_versionConstraint:usize = 6; 
	pub const RULE_importDeclaration:usize = 7; 
	pub const RULE_importDirective:usize = 8; 
	pub const RULE_importPath:usize = 9; 
	pub const RULE_contractDefinition:usize = 10; 
	pub const RULE_inheritanceSpecifier:usize = 11; 
	pub const RULE_contractPart:usize = 12; 
	pub const RULE_stateVariableDeclaration:usize = 13; 
	pub const RULE_fileLevelConstant:usize = 14; 
	pub const RULE_customErrorDefinition:usize = 15; 
	pub const RULE_typeDefinition:usize = 16; 
	pub const RULE_usingForDeclaration:usize = 17; 
	pub const RULE_usingForObject:usize = 18; 
	pub const RULE_usingForObjectDirective:usize = 19; 
	pub const RULE_userDefinableOperators:usize = 20; 
	pub const RULE_structDefinition:usize = 21; 
	pub const RULE_modifierDefinition:usize = 22; 
	pub const RULE_modifierInvocation:usize = 23; 
	pub const RULE_functionDefinition:usize = 24; 
	pub const RULE_functionDescriptor:usize = 25; 
	pub const RULE_returnParameters:usize = 26; 
	pub const RULE_modifierList:usize = 27; 
	pub const RULE_eventDefinition:usize = 28; 
	pub const RULE_enumValue:usize = 29; 
	pub const RULE_enumDefinition:usize = 30; 
	pub const RULE_parameterList:usize = 31; 
	pub const RULE_parameter:usize = 32; 
	pub const RULE_eventParameterList:usize = 33; 
	pub const RULE_eventParameter:usize = 34; 
	pub const RULE_functionTypeParameterList:usize = 35; 
	pub const RULE_functionTypeParameter:usize = 36; 
	pub const RULE_variableDeclaration:usize = 37; 
	pub const RULE_typeName:usize = 38; 
	pub const RULE_userDefinedTypeName:usize = 39; 
	pub const RULE_mappingKey:usize = 40; 
	pub const RULE_mapping:usize = 41; 
	pub const RULE_mappingKeyName:usize = 42; 
	pub const RULE_mappingValueName:usize = 43; 
	pub const RULE_functionTypeName:usize = 44; 
	pub const RULE_storageLocation:usize = 45; 
	pub const RULE_stateMutability:usize = 46; 
	pub const RULE_block:usize = 47; 
	pub const RULE_statement:usize = 48; 
	pub const RULE_expressionStatement:usize = 49; 
	pub const RULE_ifStatement:usize = 50; 
	pub const RULE_tryStatement:usize = 51; 
	pub const RULE_catchClause:usize = 52; 
	pub const RULE_whileStatement:usize = 53; 
	pub const RULE_simpleStatement:usize = 54; 
	pub const RULE_uncheckedStatement:usize = 55; 
	pub const RULE_forStatement:usize = 56; 
	pub const RULE_inlineAssemblyStatement:usize = 57; 
	pub const RULE_inlineAssemblyStatementFlag:usize = 58; 
	pub const RULE_doWhileStatement:usize = 59; 
	pub const RULE_continueStatement:usize = 60; 
	pub const RULE_breakStatement:usize = 61; 
	pub const RULE_returnStatement:usize = 62; 
	pub const RULE_throwStatement:usize = 63; 
	pub const RULE_emitStatement:usize = 64; 
	pub const RULE_revertStatement:usize = 65; 
	pub const RULE_variableDeclarationStatement:usize = 66; 
	pub const RULE_variableDeclarationList:usize = 67; 
	pub const RULE_identifierList:usize = 68; 
	pub const RULE_elementaryTypeName:usize = 69; 
	pub const RULE_expression:usize = 70; 
	pub const RULE_primaryExpression:usize = 71; 
	pub const RULE_expressionList:usize = 72; 
	pub const RULE_nameValueList:usize = 73; 
	pub const RULE_nameValue:usize = 74; 
	pub const RULE_functionCallArguments:usize = 75; 
	pub const RULE_functionCall:usize = 76; 
	pub const RULE_assemblyBlock:usize = 77; 
	pub const RULE_assemblyItem:usize = 78; 
	pub const RULE_assemblyExpression:usize = 79; 
	pub const RULE_assemblyMember:usize = 80; 
	pub const RULE_assemblyCall:usize = 81; 
	pub const RULE_assemblyLocalDefinition:usize = 82; 
	pub const RULE_assemblyAssignment:usize = 83; 
	pub const RULE_assemblyIdentifierOrList:usize = 84; 
	pub const RULE_assemblyIdentifierList:usize = 85; 
	pub const RULE_assemblyStackAssignment:usize = 86; 
	pub const RULE_labelDefinition:usize = 87; 
	pub const RULE_assemblySwitch:usize = 88; 
	pub const RULE_assemblyCase:usize = 89; 
	pub const RULE_assemblyFunctionDefinition:usize = 90; 
	pub const RULE_assemblyFunctionReturns:usize = 91; 
	pub const RULE_assemblyFor:usize = 92; 
	pub const RULE_assemblyIf:usize = 93; 
	pub const RULE_assemblyLiteral:usize = 94; 
	pub const RULE_tupleExpression:usize = 95; 
	pub const RULE_numberLiteral:usize = 96; 
	pub const RULE_identifier:usize = 97; 
	pub const RULE_hexLiteral:usize = 98; 
	pub const RULE_overrideSpecifier:usize = 99; 
	pub const RULE_stringLiteral:usize = 100;
	pub const ruleNames: [&'static str; 101] =  [
		"sourceUnit", "pragmaDirective", "pragmaName", "pragmaValue", "version", 
		"versionOperator", "versionConstraint", "importDeclaration", "importDirective", 
		"importPath", "contractDefinition", "inheritanceSpecifier", "contractPart", 
		"stateVariableDeclaration", "fileLevelConstant", "customErrorDefinition", 
		"typeDefinition", "usingForDeclaration", "usingForObject", "usingForObjectDirective", 
		"userDefinableOperators", "structDefinition", "modifierDefinition", "modifierInvocation", 
		"functionDefinition", "functionDescriptor", "returnParameters", "modifierList", 
		"eventDefinition", "enumValue", "enumDefinition", "parameterList", "parameter", 
		"eventParameterList", "eventParameter", "functionTypeParameterList", "functionTypeParameter", 
		"variableDeclaration", "typeName", "userDefinedTypeName", "mappingKey", 
		"mapping", "mappingKeyName", "mappingValueName", "functionTypeName", "storageLocation", 
		"stateMutability", "block", "statement", "expressionStatement", "ifStatement", 
		"tryStatement", "catchClause", "whileStatement", "simpleStatement", "uncheckedStatement", 
		"forStatement", "inlineAssemblyStatement", "inlineAssemblyStatementFlag", 
		"doWhileStatement", "continueStatement", "breakStatement", "returnStatement", 
		"throwStatement", "emitStatement", "revertStatement", "variableDeclarationStatement", 
		"variableDeclarationList", "identifierList", "elementaryTypeName", "expression", 
		"primaryExpression", "expressionList", "nameValueList", "nameValue", "functionCallArguments", 
		"functionCall", "assemblyBlock", "assemblyItem", "assemblyExpression", 
		"assemblyMember", "assemblyCall", "assemblyLocalDefinition", "assemblyAssignment", 
		"assemblyIdentifierOrList", "assemblyIdentifierList", "assemblyStackAssignment", 
		"labelDefinition", "assemblySwitch", "assemblyCase", "assemblyFunctionDefinition", 
		"assemblyFunctionReturns", "assemblyFor", "assemblyIf", "assemblyLiteral", 
		"tupleExpression", "numberLiteral", "identifier", "hexLiteral", "overrideSpecifier", 
		"stringLiteral"
	];


	pub const _LITERAL_NAMES: [Option<&'static str>;128] = [
		None, Some("'pragma'"), Some("';'"), Some("'*'"), Some("'||'"), Some("'^'"), 
		Some("'~'"), Some("'>='"), Some("'>'"), Some("'<'"), Some("'<='"), Some("'='"), 
		Some("'as'"), Some("'import'"), Some("'from'"), Some("'{'"), Some("','"), 
		Some("'}'"), Some("'abstract'"), Some("'contract'"), Some("'interface'"), 
		Some("'library'"), Some("'is'"), Some("'('"), Some("')'"), Some("'error'"), 
		Some("'using'"), Some("'for'"), Some("'|'"), Some("'&'"), Some("'+'"), 
		Some("'-'"), Some("'/'"), Some("'%'"), Some("'=='"), Some("'!='"), Some("'struct'"), 
		Some("'modifier'"), Some("'function'"), Some("'returns'"), Some("'event'"), 
		Some("'enum'"), Some("'['"), Some("']'"), Some("'address'"), Some("'.'"), 
		Some("'mapping'"), Some("'=>'"), Some("'memory'"), Some("'storage'"), 
		Some("'calldata'"), Some("'if'"), Some("'else'"), Some("'try'"), Some("'catch'"), 
		Some("'while'"), Some("'unchecked'"), Some("'assembly'"), Some("'do'"), 
		Some("'return'"), Some("'throw'"), Some("'emit'"), Some("'revert'"), Some("'var'"), 
		Some("'bool'"), Some("'string'"), Some("'byte'"), Some("'++'"), Some("'--'"), 
		Some("'new'"), Some("':'"), Some("'delete'"), Some("'!'"), Some("'**'"), 
		Some("'<<'"), Some("'>>'"), Some("'&&'"), Some("'?'"), Some("'|='"), Some("'^='"), 
		Some("'&='"), Some("'<<='"), Some("'>>='"), Some("'+='"), Some("'-='"), 
		Some("'*='"), Some("'/='"), Some("'%='"), Some("'let'"), Some("':='"), 
		Some("'=:'"), Some("'switch'"), Some("'case'"), Some("'default'"), Some("'->'"), 
		Some("'callback'"), Some("'override'"), None, None, None, None, None, 
		None, None, None, None, None, None, Some("'anonymous'"), Some("'break'"), 
		Some("'constant'"), Some("'immutable'"), Some("'continue'"), Some("'leave'"), 
		Some("'external'"), Some("'indexed'"), Some("'internal'"), Some("'payable'"), 
		Some("'private'"), Some("'public'"), Some("'virtual'"), Some("'pure'"), 
		Some("'type'"), Some("'view'"), Some("'global'"), Some("'constructor'"), 
		Some("'fallback'"), Some("'receive'")
	];
	pub const _SYMBOLIC_NAMES: [Option<&'static str>;134]  = [
		None, None, None, None, None, None, None, None, None, None, None, None, 
		None, None, None, None, None, None, None, None, None, None, None, None, 
		None, None, None, None, None, None, None, None, None, None, None, None, 
		None, None, None, None, None, None, None, None, None, None, None, None, 
		None, None, None, None, None, None, None, None, None, None, None, None, 
		None, None, None, None, None, None, None, None, None, None, None, None, 
		None, None, None, None, None, None, None, None, None, None, None, None, 
		None, None, None, None, None, None, None, None, None, None, None, None, 
		None, Some("Int"), Some("Uint"), Some("Byte"), Some("Fixed"), Some("Ufixed"), 
		Some("BooleanLiteral"), Some("DecimalNumber"), Some("HexNumber"), Some("NumberUnit"), 
		Some("HexLiteralFragment"), Some("ReservedKeyword"), Some("AnonymousKeyword"), 
		Some("BreakKeyword"), Some("ConstantKeyword"), Some("ImmutableKeyword"), 
		Some("ContinueKeyword"), Some("LeaveKeyword"), Some("ExternalKeyword"), 
		Some("IndexedKeyword"), Some("InternalKeyword"), Some("PayableKeyword"), 
		Some("PrivateKeyword"), Some("PublicKeyword"), Some("VirtualKeyword"), 
		Some("PureKeyword"), Some("TypeKeyword"), Some("ViewKeyword"), Some("GlobalKeyword"), 
		Some("ConstructorKeyword"), Some("FallbackKeyword"), Some("ReceiveKeyword"), 
		Some("Identifier"), Some("StringLiteralFragment"), Some("VersionLiteral"), 
		Some("WS"), Some("COMMENT"), Some("LINE_COMMENT")
	];
	lazy_static!{
	    static ref _shared_context_cache: Arc<PredictionContextCache> = Arc::new(PredictionContextCache::new());
		static ref VOCABULARY: Box<dyn Vocabulary> = Box::new(VocabularyImpl::new(_LITERAL_NAMES.iter(), _SYMBOLIC_NAMES.iter(), None));
	}


type BaseParserType<'input, I> =
	BaseParser<'input,SolidityParserExt, I, SolidityParserContextType , dyn SolidityListener<'input> + 'input >;

type TokenType<'input> = <LocalTokenFactory<'input> as TokenFactory<'input>>::Tok;
pub type LocalTokenFactory<'input> = CommonTokenFactory;

pub type SolidityTreeWalker<'input,'a> =
	ParseTreeWalker<'input, 'a, SolidityParserContextType , dyn SolidityListener<'input> + 'a>;

/// Parser for Solidity grammar
pub struct SolidityParser<'input,I,H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	base:BaseParserType<'input,I>,
	interpreter:Arc<ParserATNSimulator>,
	_shared_context_cache: Box<PredictionContextCache>,
    pub err_handler: H,
}

impl<'input, I, H> SolidityParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn get_serialized_atn() -> &'static str { _serializedATN }

    pub fn set_error_strategy(&mut self, strategy: H) {
        self.err_handler = strategy
    }

    pub fn with_strategy(input: I, strategy: H) -> Self {
		antlr_rust::recognizer::check_version("0","2");
		let interpreter = Arc::new(ParserATNSimulator::new(
			_ATN.clone(),
			_decision_to_DFA.clone(),
			_shared_context_cache.clone(),
		));
		Self {
			base: BaseParser::new_base_parser(
				input,
				Arc::clone(&interpreter),
				SolidityParserExt{
				}
			),
			interpreter,
            _shared_context_cache: Box::new(PredictionContextCache::new()),
            err_handler: strategy,
        }
    }

}

type DynStrategy<'input,I> = Box<dyn ErrorStrategy<'input,BaseParserType<'input,I>> + 'input>;

impl<'input, I> SolidityParser<'input, I, DynStrategy<'input,I>>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
    pub fn with_dyn_strategy(input: I) -> Self{
    	Self::with_strategy(input,Box::new(DefaultErrorStrategy::new()))
    }
}

impl<'input, I> SolidityParser<'input, I, DefaultErrorStrategy<'input,SolidityParserContextType>>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
    pub fn new(input: I) -> Self{
    	Self::with_strategy(input,DefaultErrorStrategy::new())
    }
}

/// Trait for monomorphized trait object that corresponds to the nodes of parse tree generated for SolidityParser
pub trait SolidityParserContext<'input>:
	for<'x> Listenable<dyn SolidityListener<'input> + 'x > + 
	ParserRuleContext<'input, TF=LocalTokenFactory<'input>, Ctx=SolidityParserContextType>
{}

impl<'input> SolidityParserContext<'input> for TerminalNode<'input,SolidityParserContextType> {}
impl<'input> SolidityParserContext<'input> for ErrorNode<'input,SolidityParserContextType> {}

#[antlr_rust::impl_tid]
impl<'input> antlr_rust::TidAble<'input> for dyn SolidityParserContext<'input> + 'input{}

#[antlr_rust::impl_tid]
impl<'input> antlr_rust::TidAble<'input> for dyn SolidityListener<'input> + 'input{}

pub struct SolidityParserContextType;
antlr_rust::type_id!{SolidityParserContextType}

impl<'input> ParserNodeType<'input> for SolidityParserContextType{
	type TF = LocalTokenFactory<'input>;
	type Type = dyn SolidityParserContext<'input> + 'input;
}

impl<'input, I, H> Deref for SolidityParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
    type Target = BaseParserType<'input,I>;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl<'input, I, H> DerefMut for SolidityParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

pub struct SolidityParserExt{
}

impl SolidityParserExt{
}


impl<'input> TokenAware<'input> for SolidityParserExt{
	type TF = LocalTokenFactory<'input>;
}

impl<'input,I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>> ParserRecog<'input, BaseParserType<'input,I>> for SolidityParserExt{}

impl<'input,I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>> Actions<'input, BaseParserType<'input,I>> for SolidityParserExt{
	fn get_grammar_file_name(&self) -> & str{ "Solidity.g4"}

   	fn get_rule_names(&self) -> &[& str] {&ruleNames}

   	fn get_vocabulary(&self) -> &dyn Vocabulary { &**VOCABULARY }
	fn sempred(_localctx: Option<&(dyn SolidityParserContext<'input> + 'input)>, rule_index: isize, pred_index: isize,
			   recog:&mut BaseParserType<'input,I>
	)->bool{
		match rule_index {
					38 => SolidityParser::<'input,I,_>::typeName_sempred(_localctx.and_then(|x|x.downcast_ref()), pred_index, recog),
					70 => SolidityParser::<'input,I,_>::expression_sempred(_localctx.and_then(|x|x.downcast_ref()), pred_index, recog),
			_ => true
		}
	}
}

impl<'input, I> SolidityParser<'input, I, DefaultErrorStrategy<'input,SolidityParserContextType>>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	fn typeName_sempred(_localctx: Option<&TypeNameContext<'input>>, pred_index:isize,
						recog:&mut <Self as Deref>::Target
		) -> bool {
		match pred_index {
				0=>{
					recog.precpred(None, 3)
				}
			_ => true
		}
	}
	fn expression_sempred(_localctx: Option<&ExpressionContext<'input>>, pred_index:isize,
						recog:&mut <Self as Deref>::Target
		) -> bool {
		match pred_index {
				1=>{
					recog.precpred(None, 14)
				}
				2=>{
					recog.precpred(None, 13)
				}
				3=>{
					recog.precpred(None, 12)
				}
				4=>{
					recog.precpred(None, 11)
				}
				5=>{
					recog.precpred(None, 10)
				}
				6=>{
					recog.precpred(None, 9)
				}
				7=>{
					recog.precpred(None, 8)
				}
				8=>{
					recog.precpred(None, 7)
				}
				9=>{
					recog.precpred(None, 6)
				}
				10=>{
					recog.precpred(None, 5)
				}
				11=>{
					recog.precpred(None, 4)
				}
				12=>{
					recog.precpred(None, 3)
				}
				13=>{
					recog.precpred(None, 2)
				}
				14=>{
					recog.precpred(None, 27)
				}
				15=>{
					recog.precpred(None, 25)
				}
				16=>{
					recog.precpred(None, 24)
				}
				17=>{
					recog.precpred(None, 23)
				}
				18=>{
					recog.precpred(None, 22)
				}
				19=>{
					recog.precpred(None, 21)
				}
			_ => true
		}
	}
}
//------------------- sourceUnit ----------------
pub type SourceUnitContextAll<'input> = SourceUnitContext<'input>;


pub type SourceUnitContext<'input> = BaseParserRuleContext<'input,SourceUnitContextExt<'input>>;

#[derive(Clone)]
pub struct SourceUnitContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SolidityParserContext<'input> for SourceUnitContext<'input>{}

impl<'input,'a> Listenable<dyn SolidityListener<'input> + 'a> for SourceUnitContext<'input>{
	fn enter(&self,listener: &mut (dyn SolidityListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_sourceUnit(self);
	}
}

impl<'input> CustomRuleContext<'input> for SourceUnitContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SolidityParserContextType;
	fn get_rule_index(&self) -> usize { RULE_sourceUnit }
	//fn type_rule_index() -> usize where Self: Sized { RULE_sourceUnit }
}
antlr_rust::type_id!{SourceUnitContextExt<'a>}

impl<'input> SourceUnitContextExt<'input>{
	fn new(parent: Option<Rc<dyn SolidityParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<SourceUnitContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,SourceUnitContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait SourceUnitContextAttrs<'input>: SolidityParserContext<'input> + BorrowMut<SourceUnitContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token EOF
/// Returns `None` if there is no child corresponding to token EOF
fn EOF(&self) -> Option<Rc<TerminalNode<'input,SolidityParserContextType>>> where Self:Sized{
	self.get_token(EOF, 0)
}
fn pragmaDirective_all(&self) ->  Vec<Rc<PragmaDirectiveContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn pragmaDirective(&self, i: usize) -> Option<Rc<PragmaDirectiveContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn importDirective_all(&self) ->  Vec<Rc<ImportDirectiveContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn importDirective(&self, i: usize) -> Option<Rc<ImportDirectiveContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn contractDefinition_all(&self) ->  Vec<Rc<ContractDefinitionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn contractDefinition(&self, i: usize) -> Option<Rc<ContractDefinitionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn enumDefinition_all(&self) ->  Vec<Rc<EnumDefinitionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn enumDefinition(&self, i: usize) -> Option<Rc<EnumDefinitionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn eventDefinition_all(&self) ->  Vec<Rc<EventDefinitionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn eventDefinition(&self, i: usize) -> Option<Rc<EventDefinitionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn structDefinition_all(&self) ->  Vec<Rc<StructDefinitionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn structDefinition(&self, i: usize) -> Option<Rc<StructDefinitionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn functionDefinition_all(&self) ->  Vec<Rc<FunctionDefinitionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn functionDefinition(&self, i: usize) -> Option<Rc<FunctionDefinitionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn fileLevelConstant_all(&self) ->  Vec<Rc<FileLevelConstantContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn fileLevelConstant(&self, i: usize) -> Option<Rc<FileLevelConstantContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn customErrorDefinition_all(&self) ->  Vec<Rc<CustomErrorDefinitionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn customErrorDefinition(&self, i: usize) -> Option<Rc<CustomErrorDefinitionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn typeDefinition_all(&self) ->  Vec<Rc<TypeDefinitionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn typeDefinition(&self, i: usize) -> Option<Rc<TypeDefinitionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn usingForDeclaration_all(&self) ->  Vec<Rc<UsingForDeclarationContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn usingForDeclaration(&self, i: usize) -> Option<Rc<UsingForDeclarationContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> SourceUnitContextAttrs<'input> for SourceUnitContext<'input>{}

impl<'input, I, H> SolidityParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{#![feature(try_blocks)]
	pub fn sourceUnit(&mut self,)
	-> Result<Rc<SourceUnitContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = SourceUnitContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 0, RULE_sourceUnit);
        let mut _localctx: Rc<SourceUnitContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(215);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while ((((_la - 1)) & !0x3f) == 0 && ((1usize << (_la - 1)) & ((1usize << (T__0 - 1)) | (1usize << (T__12 - 1)) | (1usize << (T__13 - 1)) | (1usize << (T__17 - 1)) | (1usize << (T__18 - 1)) | (1usize << (T__19 - 1)) | (1usize << (T__20 - 1)) | (1usize << (T__24 - 1)) | (1usize << (T__25 - 1)) | (1usize << (T__35 - 1)) | (1usize << (T__37 - 1)) | (1usize << (T__39 - 1)) | (1usize << (T__40 - 1)) | (1usize << (T__43 - 1)) | (1usize << (T__45 - 1)) | (1usize << (T__49 - 1)) | (1usize << (T__61 - 1)) | (1usize << (T__62 - 1)) | (1usize << (T__63 - 1)))) != 0) || ((((_la - 65)) & !0x3f) == 0 && ((1usize << (_la - 65)) & ((1usize << (T__64 - 65)) | (1usize << (T__65 - 65)) | (1usize << (T__94 - 65)) | (1usize << (Int - 65)) | (1usize << (Uint - 65)) | (1usize << (Byte - 65)) | (1usize << (Fixed - 65)) | (1usize << (Ufixed - 65)) | (1usize << (LeaveKeyword - 65)) | (1usize << (PayableKeyword - 65)) | (1usize << (TypeKeyword - 65)) | (1usize << (GlobalKeyword - 65)) | (1usize << (ConstructorKeyword - 65)) | (1usize << (FallbackKeyword - 65)) | (1usize << (ReceiveKeyword - 65)) | (1usize << (Identifier - 65)))) != 0) {
				{
				recog.base.set_state(213);
				recog.err_handler.sync(&mut recog.base)?;
				match  recog.interpreter.adaptive_predict(0,&mut recog.base)? {
					1 =>{
						{
						/*InvokeRule pragmaDirective*/
						recog.base.set_state(202);
						recog.pragmaDirective()?;

						}
					}
				,
					2 =>{
						{
						/*InvokeRule importDirective*/
						recog.base.set_state(203);
						recog.importDirective()?;

						}
					}
				,
					3 =>{
						{
						/*InvokeRule contractDefinition*/
						recog.base.set_state(204);
						recog.contractDefinition()?;

						}
					}
				,
					4 =>{
						{
						/*InvokeRule enumDefinition*/
						recog.base.set_state(205);
						recog.enumDefinition()?;

						}
					}
				,
					5 =>{
						{
						/*InvokeRule eventDefinition*/
						recog.base.set_state(206);
						recog.eventDefinition()?;

						}
					}
				,
					6 =>{
						{
						/*InvokeRule structDefinition*/
						recog.base.set_state(207);
						recog.structDefinition()?;

						}
					}
				,
					7 =>{
						{
						/*InvokeRule functionDefinition*/
						recog.base.set_state(208);
						recog.functionDefinition()?;

						}
					}
				,
					8 =>{
						{
						/*InvokeRule fileLevelConstant*/
						recog.base.set_state(209);
						recog.fileLevelConstant()?;

						}
					}
				,
					9 =>{
						{
						/*InvokeRule customErrorDefinition*/
						recog.base.set_state(210);
						recog.customErrorDefinition()?;

						}
					}
				,
					10 =>{
						{
						/*InvokeRule typeDefinition*/
						recog.base.set_state(211);
						recog.typeDefinition()?;

						}
					}
				,
					11 =>{
						{
						/*InvokeRule usingForDeclaration*/
						recog.base.set_state(212);
						recog.usingForDeclaration()?;

						}
					}

					_ => {}
				}
				}
				recog.base.set_state(217);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(218);
			recog.base.match_token(EOF,&mut recog.err_handler)?;

			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- pragmaDirective ----------------
pub type PragmaDirectiveContextAll<'input> = PragmaDirectiveContext<'input>;


pub type PragmaDirectiveContext<'input> = BaseParserRuleContext<'input,PragmaDirectiveContextExt<'input>>;

#[derive(Clone)]
pub struct PragmaDirectiveContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SolidityParserContext<'input> for PragmaDirectiveContext<'input>{}

impl<'input,'a> Listenable<dyn SolidityListener<'input> + 'a> for PragmaDirectiveContext<'input>{
	fn enter(&self,listener: &mut (dyn SolidityListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_pragmaDirective(self);
	}
}

impl<'input> CustomRuleContext<'input> for PragmaDirectiveContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SolidityParserContextType;
	fn get_rule_index(&self) -> usize { RULE_pragmaDirective }
	//fn type_rule_index() -> usize where Self: Sized { RULE_pragmaDirective }
}
antlr_rust::type_id!{PragmaDirectiveContextExt<'a>}

impl<'input> PragmaDirectiveContextExt<'input>{
	fn new(parent: Option<Rc<dyn SolidityParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<PragmaDirectiveContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,PragmaDirectiveContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait PragmaDirectiveContextAttrs<'input>: SolidityParserContext<'input> + BorrowMut<PragmaDirectiveContextExt<'input>>{

fn pragmaName(&self) -> Option<Rc<PragmaNameContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn pragmaValue(&self) -> Option<Rc<PragmaValueContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> PragmaDirectiveContextAttrs<'input> for PragmaDirectiveContext<'input>{}

impl<'input, I, H> SolidityParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn pragmaDirective(&mut self,)
	-> Result<Rc<PragmaDirectiveContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = PragmaDirectiveContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 2, RULE_pragmaDirective);
        let mut _localctx: Rc<PragmaDirectiveContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(220);
			recog.base.match_token(T__0,&mut recog.err_handler)?;

			/*InvokeRule pragmaName*/
			recog.base.set_state(221);
			recog.pragmaName()?;

			/*InvokeRule pragmaValue*/
			recog.base.set_state(222);
			recog.pragmaValue()?;

			recog.base.set_state(223);
			recog.base.match_token(T__1,&mut recog.err_handler)?;

			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- pragmaName ----------------
pub type PragmaNameContextAll<'input> = PragmaNameContext<'input>;


pub type PragmaNameContext<'input> = BaseParserRuleContext<'input,PragmaNameContextExt<'input>>;

#[derive(Clone)]
pub struct PragmaNameContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SolidityParserContext<'input> for PragmaNameContext<'input>{}

impl<'input,'a> Listenable<dyn SolidityListener<'input> + 'a> for PragmaNameContext<'input>{
	fn enter(&self,listener: &mut (dyn SolidityListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_pragmaName(self);
	}
}

impl<'input> CustomRuleContext<'input> for PragmaNameContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SolidityParserContextType;
	fn get_rule_index(&self) -> usize { RULE_pragmaName }
	//fn type_rule_index() -> usize where Self: Sized { RULE_pragmaName }
}
antlr_rust::type_id!{PragmaNameContextExt<'a>}

impl<'input> PragmaNameContextExt<'input>{
	fn new(parent: Option<Rc<dyn SolidityParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<PragmaNameContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,PragmaNameContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait PragmaNameContextAttrs<'input>: SolidityParserContext<'input> + BorrowMut<PragmaNameContextExt<'input>>{

fn identifier(&self) -> Option<Rc<IdentifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> PragmaNameContextAttrs<'input> for PragmaNameContext<'input>{}

impl<'input, I, H> SolidityParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn pragmaName(&mut self,)
	-> Result<Rc<PragmaNameContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = PragmaNameContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 4, RULE_pragmaName);
        let mut _localctx: Rc<PragmaNameContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule identifier*/
			recog.base.set_state(225);
			recog.identifier()?;

			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- pragmaValue ----------------
pub type PragmaValueContextAll<'input> = PragmaValueContext<'input>;


pub type PragmaValueContext<'input> = BaseParserRuleContext<'input,PragmaValueContextExt<'input>>;

#[derive(Clone)]
pub struct PragmaValueContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SolidityParserContext<'input> for PragmaValueContext<'input>{}

impl<'input,'a> Listenable<dyn SolidityListener<'input> + 'a> for PragmaValueContext<'input>{
	fn enter(&self,listener: &mut (dyn SolidityListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_pragmaValue(self);
	}
}

impl<'input> CustomRuleContext<'input> for PragmaValueContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SolidityParserContextType;
	fn get_rule_index(&self) -> usize { RULE_pragmaValue }
	//fn type_rule_index() -> usize where Self: Sized { RULE_pragmaValue }
}
antlr_rust::type_id!{PragmaValueContextExt<'a>}

impl<'input> PragmaValueContextExt<'input>{
	fn new(parent: Option<Rc<dyn SolidityParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<PragmaValueContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,PragmaValueContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait PragmaValueContextAttrs<'input>: SolidityParserContext<'input> + BorrowMut<PragmaValueContextExt<'input>>{

fn version(&self) -> Option<Rc<VersionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> PragmaValueContextAttrs<'input> for PragmaValueContext<'input>{}

impl<'input, I, H> SolidityParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn pragmaValue(&mut self,)
	-> Result<Rc<PragmaValueContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = PragmaValueContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 6, RULE_pragmaValue);
        let mut _localctx: Rc<PragmaValueContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			recog.base.set_state(230);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(2,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(227);
					recog.base.match_token(T__2,&mut recog.err_handler)?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule version*/
					recog.base.set_state(228);
					recog.version()?;

					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					/*InvokeRule expression*/
					recog.base.set_state(229);
					recog.expression_rec(0)?;

					}
				}

				_ => {}
			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- version ----------------
pub type VersionContextAll<'input> = VersionContext<'input>;


pub type VersionContext<'input> = BaseParserRuleContext<'input,VersionContextExt<'input>>;

#[derive(Clone)]
pub struct VersionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SolidityParserContext<'input> for VersionContext<'input>{}

impl<'input,'a> Listenable<dyn SolidityListener<'input> + 'a> for VersionContext<'input>{
	fn enter(&self,listener: &mut (dyn SolidityListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_version(self);
	}
}

impl<'input> CustomRuleContext<'input> for VersionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SolidityParserContextType;
	fn get_rule_index(&self) -> usize { RULE_version }
	//fn type_rule_index() -> usize where Self: Sized { RULE_version }
}
antlr_rust::type_id!{VersionContextExt<'a>}

impl<'input> VersionContextExt<'input>{
	fn new(parent: Option<Rc<dyn SolidityParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<VersionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,VersionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait VersionContextAttrs<'input>: SolidityParserContext<'input> + BorrowMut<VersionContextExt<'input>>{

fn versionConstraint_all(&self) ->  Vec<Rc<VersionConstraintContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn versionConstraint(&self, i: usize) -> Option<Rc<VersionConstraintContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> VersionContextAttrs<'input> for VersionContext<'input>{}

impl<'input, I, H> SolidityParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn version(&mut self,)
	-> Result<Rc<VersionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = VersionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 8, RULE_version);
        let mut _localctx: Rc<VersionContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule versionConstraint*/
			recog.base.set_state(232);
			recog.versionConstraint()?;

			recog.base.set_state(239);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while (((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << T__3) | (1usize << T__4) | (1usize << T__5) | (1usize << T__6) | (1usize << T__7) | (1usize << T__8) | (1usize << T__9) | (1usize << T__10))) != 0) || _la==DecimalNumber || _la==VersionLiteral {
				{
				{
				recog.base.set_state(234);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				if _la==T__3 {
					{
					recog.base.set_state(233);
					recog.base.match_token(T__3,&mut recog.err_handler)?;

					}
				}

				/*InvokeRule versionConstraint*/
				recog.base.set_state(236);
				recog.versionConstraint()?;

				}
				}
				recog.base.set_state(241);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- versionOperator ----------------
pub type VersionOperatorContextAll<'input> = VersionOperatorContext<'input>;


pub type VersionOperatorContext<'input> = BaseParserRuleContext<'input,VersionOperatorContextExt<'input>>;

#[derive(Clone)]
pub struct VersionOperatorContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SolidityParserContext<'input> for VersionOperatorContext<'input>{}

impl<'input,'a> Listenable<dyn SolidityListener<'input> + 'a> for VersionOperatorContext<'input>{
	fn enter(&self,listener: &mut (dyn SolidityListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_versionOperator(self);
	}
}

impl<'input> CustomRuleContext<'input> for VersionOperatorContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SolidityParserContextType;
	fn get_rule_index(&self) -> usize { RULE_versionOperator }
	//fn type_rule_index() -> usize where Self: Sized { RULE_versionOperator }
}
antlr_rust::type_id!{VersionOperatorContextExt<'a>}

impl<'input> VersionOperatorContextExt<'input>{
	fn new(parent: Option<Rc<dyn SolidityParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<VersionOperatorContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,VersionOperatorContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait VersionOperatorContextAttrs<'input>: SolidityParserContext<'input> + BorrowMut<VersionOperatorContextExt<'input>>{


}

impl<'input> VersionOperatorContextAttrs<'input> for VersionOperatorContext<'input>{}

impl<'input, I, H> SolidityParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn versionOperator(&mut self,)
	-> Result<Rc<VersionOperatorContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = VersionOperatorContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 10, RULE_versionOperator);
        let mut _localctx: Rc<VersionOperatorContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(242);
			_la = recog.base.input.la(1);
			if { !((((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << T__4) | (1usize << T__5) | (1usize << T__6) | (1usize << T__7) | (1usize << T__8) | (1usize << T__9) | (1usize << T__10))) != 0)) } {
				recog.err_handler.recover_inline(&mut recog.base)?;

			}
			else {
				if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler);
			}
			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- versionConstraint ----------------
pub type VersionConstraintContextAll<'input> = VersionConstraintContext<'input>;


pub type VersionConstraintContext<'input> = BaseParserRuleContext<'input,VersionConstraintContextExt<'input>>;

#[derive(Clone)]
pub struct VersionConstraintContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SolidityParserContext<'input> for VersionConstraintContext<'input>{}

impl<'input,'a> Listenable<dyn SolidityListener<'input> + 'a> for VersionConstraintContext<'input>{
	fn enter(&self,listener: &mut (dyn SolidityListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_versionConstraint(self);
	}
}

impl<'input> CustomRuleContext<'input> for VersionConstraintContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SolidityParserContextType;
	fn get_rule_index(&self) -> usize { RULE_versionConstraint }
	//fn type_rule_index() -> usize where Self: Sized { RULE_versionConstraint }
}
antlr_rust::type_id!{VersionConstraintContextExt<'a>}

impl<'input> VersionConstraintContextExt<'input>{
	fn new(parent: Option<Rc<dyn SolidityParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<VersionConstraintContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,VersionConstraintContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait VersionConstraintContextAttrs<'input>: SolidityParserContext<'input> + BorrowMut<VersionConstraintContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token VersionLiteral
/// Returns `None` if there is no child corresponding to token VersionLiteral
fn VersionLiteral(&self) -> Option<Rc<TerminalNode<'input,SolidityParserContextType>>> where Self:Sized{
	self.get_token(VersionLiteral, 0)
}
fn versionOperator(&self) -> Option<Rc<VersionOperatorContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token DecimalNumber
/// Returns `None` if there is no child corresponding to token DecimalNumber
fn DecimalNumber(&self) -> Option<Rc<TerminalNode<'input,SolidityParserContextType>>> where Self:Sized{
	self.get_token(DecimalNumber, 0)
}

}

impl<'input> VersionConstraintContextAttrs<'input> for VersionConstraintContext<'input>{}

impl<'input, I, H> SolidityParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn versionConstraint(&mut self,)
	-> Result<Rc<VersionConstraintContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = VersionConstraintContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 12, RULE_versionConstraint);
        let mut _localctx: Rc<VersionConstraintContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			recog.base.set_state(252);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(7,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(245);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if (((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << T__4) | (1usize << T__5) | (1usize << T__6) | (1usize << T__7) | (1usize << T__8) | (1usize << T__9) | (1usize << T__10))) != 0) {
						{
						/*InvokeRule versionOperator*/
						recog.base.set_state(244);
						recog.versionOperator()?;

						}
					}

					recog.base.set_state(247);
					recog.base.match_token(VersionLiteral,&mut recog.err_handler)?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(249);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if (((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << T__4) | (1usize << T__5) | (1usize << T__6) | (1usize << T__7) | (1usize << T__8) | (1usize << T__9) | (1usize << T__10))) != 0) {
						{
						/*InvokeRule versionOperator*/
						recog.base.set_state(248);
						recog.versionOperator()?;

						}
					}

					recog.base.set_state(251);
					recog.base.match_token(DecimalNumber,&mut recog.err_handler)?;

					}
				}

				_ => {}
			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- importDeclaration ----------------
pub type ImportDeclarationContextAll<'input> = ImportDeclarationContext<'input>;


pub type ImportDeclarationContext<'input> = BaseParserRuleContext<'input,ImportDeclarationContextExt<'input>>;

#[derive(Clone)]
pub struct ImportDeclarationContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SolidityParserContext<'input> for ImportDeclarationContext<'input>{}

impl<'input,'a> Listenable<dyn SolidityListener<'input> + 'a> for ImportDeclarationContext<'input>{
	fn enter(&self,listener: &mut (dyn SolidityListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_importDeclaration(self);
	}
}

impl<'input> CustomRuleContext<'input> for ImportDeclarationContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SolidityParserContextType;
	fn get_rule_index(&self) -> usize { RULE_importDeclaration }
	//fn type_rule_index() -> usize where Self: Sized { RULE_importDeclaration }
}
antlr_rust::type_id!{ImportDeclarationContextExt<'a>}

impl<'input> ImportDeclarationContextExt<'input>{
	fn new(parent: Option<Rc<dyn SolidityParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ImportDeclarationContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ImportDeclarationContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ImportDeclarationContextAttrs<'input>: SolidityParserContext<'input> + BorrowMut<ImportDeclarationContextExt<'input>>{

fn identifier_all(&self) ->  Vec<Rc<IdentifierContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn identifier(&self, i: usize) -> Option<Rc<IdentifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> ImportDeclarationContextAttrs<'input> for ImportDeclarationContext<'input>{}

impl<'input, I, H> SolidityParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn importDeclaration(&mut self,)
	-> Result<Rc<ImportDeclarationContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ImportDeclarationContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 14, RULE_importDeclaration);
        let mut _localctx: Rc<ImportDeclarationContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule identifier*/
			recog.base.set_state(254);
			recog.identifier()?;

			recog.base.set_state(257);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==T__11 {
				{
				recog.base.set_state(255);
				recog.base.match_token(T__11,&mut recog.err_handler)?;

				/*InvokeRule identifier*/
				recog.base.set_state(256);
				recog.identifier()?;

				}
			}

			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- importDirective ----------------
pub type ImportDirectiveContextAll<'input> = ImportDirectiveContext<'input>;


pub type ImportDirectiveContext<'input> = BaseParserRuleContext<'input,ImportDirectiveContextExt<'input>>;

#[derive(Clone)]
pub struct ImportDirectiveContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SolidityParserContext<'input> for ImportDirectiveContext<'input>{}

impl<'input,'a> Listenable<dyn SolidityListener<'input> + 'a> for ImportDirectiveContext<'input>{
	fn enter(&self,listener: &mut (dyn SolidityListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_importDirective(self);
	}
}

impl<'input> CustomRuleContext<'input> for ImportDirectiveContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SolidityParserContextType;
	fn get_rule_index(&self) -> usize { RULE_importDirective }
	//fn type_rule_index() -> usize where Self: Sized { RULE_importDirective }
}
antlr_rust::type_id!{ImportDirectiveContextExt<'a>}

impl<'input> ImportDirectiveContextExt<'input>{
	fn new(parent: Option<Rc<dyn SolidityParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ImportDirectiveContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ImportDirectiveContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ImportDirectiveContextAttrs<'input>: SolidityParserContext<'input> + BorrowMut<ImportDirectiveContextExt<'input>>{

fn importPath(&self) -> Option<Rc<ImportPathContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn identifier_all(&self) ->  Vec<Rc<IdentifierContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn identifier(&self, i: usize) -> Option<Rc<IdentifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn importDeclaration_all(&self) ->  Vec<Rc<ImportDeclarationContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn importDeclaration(&self, i: usize) -> Option<Rc<ImportDeclarationContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> ImportDirectiveContextAttrs<'input> for ImportDirectiveContext<'input>{}

impl<'input, I, H> SolidityParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn importDirective(&mut self,)
	-> Result<Rc<ImportDirectiveContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ImportDirectiveContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 16, RULE_importDirective);
        let mut _localctx: Rc<ImportDirectiveContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			recog.base.set_state(295);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(13,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(259);
					recog.base.match_token(T__12,&mut recog.err_handler)?;

					/*InvokeRule importPath*/
					recog.base.set_state(260);
					recog.importPath()?;

					recog.base.set_state(263);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==T__11 {
						{
						recog.base.set_state(261);
						recog.base.match_token(T__11,&mut recog.err_handler)?;

						/*InvokeRule identifier*/
						recog.base.set_state(262);
						recog.identifier()?;

						}
					}

					recog.base.set_state(265);
					recog.base.match_token(T__1,&mut recog.err_handler)?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(267);
					recog.base.match_token(T__12,&mut recog.err_handler)?;

					recog.base.set_state(270);
					recog.err_handler.sync(&mut recog.base)?;
					match recog.base.input.la(1) {
					 T__2 
						=> {
							{
							recog.base.set_state(268);
							recog.base.match_token(T__2,&mut recog.err_handler)?;

							}
						}

					 T__13 | T__24 | T__43 | T__49 | T__61 | T__94 | LeaveKeyword | PayableKeyword |
					 GlobalKeyword | ConstructorKeyword | ReceiveKeyword | Identifier 
						=> {
							{
							/*InvokeRule identifier*/
							recog.base.set_state(269);
							recog.identifier()?;

							}
						}

						_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
					}
					recog.base.set_state(274);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==T__11 {
						{
						recog.base.set_state(272);
						recog.base.match_token(T__11,&mut recog.err_handler)?;

						/*InvokeRule identifier*/
						recog.base.set_state(273);
						recog.identifier()?;

						}
					}

					recog.base.set_state(276);
					recog.base.match_token(T__13,&mut recog.err_handler)?;

					/*InvokeRule importPath*/
					recog.base.set_state(277);
					recog.importPath()?;

					recog.base.set_state(278);
					recog.base.match_token(T__1,&mut recog.err_handler)?;

					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					recog.base.set_state(280);
					recog.base.match_token(T__12,&mut recog.err_handler)?;

					recog.base.set_state(281);
					recog.base.match_token(T__14,&mut recog.err_handler)?;

					/*InvokeRule importDeclaration*/
					recog.base.set_state(282);
					recog.importDeclaration()?;

					recog.base.set_state(287);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					while _la==T__15 {
						{
						{
						recog.base.set_state(283);
						recog.base.match_token(T__15,&mut recog.err_handler)?;

						/*InvokeRule importDeclaration*/
						recog.base.set_state(284);
						recog.importDeclaration()?;

						}
						}
						recog.base.set_state(289);
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
					}
					recog.base.set_state(290);
					recog.base.match_token(T__16,&mut recog.err_handler)?;

					recog.base.set_state(291);
					recog.base.match_token(T__13,&mut recog.err_handler)?;

					/*InvokeRule importPath*/
					recog.base.set_state(292);
					recog.importPath()?;

					recog.base.set_state(293);
					recog.base.match_token(T__1,&mut recog.err_handler)?;

					}
				}

				_ => {}
			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- importPath ----------------
pub type ImportPathContextAll<'input> = ImportPathContext<'input>;


pub type ImportPathContext<'input> = BaseParserRuleContext<'input,ImportPathContextExt<'input>>;

#[derive(Clone)]
pub struct ImportPathContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SolidityParserContext<'input> for ImportPathContext<'input>{}

impl<'input,'a> Listenable<dyn SolidityListener<'input> + 'a> for ImportPathContext<'input>{
	fn enter(&self,listener: &mut (dyn SolidityListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_importPath(self);
	}
}

impl<'input> CustomRuleContext<'input> for ImportPathContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SolidityParserContextType;
	fn get_rule_index(&self) -> usize { RULE_importPath }
	//fn type_rule_index() -> usize where Self: Sized { RULE_importPath }
}
antlr_rust::type_id!{ImportPathContextExt<'a>}

impl<'input> ImportPathContextExt<'input>{
	fn new(parent: Option<Rc<dyn SolidityParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ImportPathContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ImportPathContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ImportPathContextAttrs<'input>: SolidityParserContext<'input> + BorrowMut<ImportPathContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token StringLiteralFragment
/// Returns `None` if there is no child corresponding to token StringLiteralFragment
fn StringLiteralFragment(&self) -> Option<Rc<TerminalNode<'input,SolidityParserContextType>>> where Self:Sized{
	self.get_token(StringLiteralFragment, 0)
}

}

impl<'input> ImportPathContextAttrs<'input> for ImportPathContext<'input>{}

impl<'input, I, H> SolidityParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn importPath(&mut self,)
	-> Result<Rc<ImportPathContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ImportPathContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 18, RULE_importPath);
        let mut _localctx: Rc<ImportPathContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(297);
			recog.base.match_token(StringLiteralFragment,&mut recog.err_handler)?;

			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- contractDefinition ----------------
pub type ContractDefinitionContextAll<'input> = ContractDefinitionContext<'input>;


pub type ContractDefinitionContext<'input> = BaseParserRuleContext<'input,ContractDefinitionContextExt<'input>>;

#[derive(Clone)]
pub struct ContractDefinitionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SolidityParserContext<'input> for ContractDefinitionContext<'input>{}

impl<'input,'a> Listenable<dyn SolidityListener<'input> + 'a> for ContractDefinitionContext<'input>{
	fn enter(&self,listener: &mut (dyn SolidityListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_contractDefinition(self);
	}
}

impl<'input> CustomRuleContext<'input> for ContractDefinitionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SolidityParserContextType;
	fn get_rule_index(&self) -> usize { RULE_contractDefinition }
	//fn type_rule_index() -> usize where Self: Sized { RULE_contractDefinition }
}
antlr_rust::type_id!{ContractDefinitionContextExt<'a>}

impl<'input> ContractDefinitionContextExt<'input>{
	fn new(parent: Option<Rc<dyn SolidityParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ContractDefinitionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ContractDefinitionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ContractDefinitionContextAttrs<'input>: SolidityParserContext<'input> + BorrowMut<ContractDefinitionContextExt<'input>>{

fn identifier(&self) -> Option<Rc<IdentifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn inheritanceSpecifier_all(&self) ->  Vec<Rc<InheritanceSpecifierContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn inheritanceSpecifier(&self, i: usize) -> Option<Rc<InheritanceSpecifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn contractPart_all(&self) ->  Vec<Rc<ContractPartContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn contractPart(&self, i: usize) -> Option<Rc<ContractPartContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> ContractDefinitionContextAttrs<'input> for ContractDefinitionContext<'input>{}

impl<'input, I, H> SolidityParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn contractDefinition(&mut self,)
	-> Result<Rc<ContractDefinitionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ContractDefinitionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 20, RULE_contractDefinition);
        let mut _localctx: Rc<ContractDefinitionContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(300);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==T__17 {
				{
				recog.base.set_state(299);
				recog.base.match_token(T__17,&mut recog.err_handler)?;

				}
			}

			recog.base.set_state(302);
			_la = recog.base.input.la(1);
			if { !((((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << T__18) | (1usize << T__19) | (1usize << T__20))) != 0)) } {
				recog.err_handler.recover_inline(&mut recog.base)?;

			}
			else {
				if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler);
			}
			/*InvokeRule identifier*/
			recog.base.set_state(303);
			recog.identifier()?;

			recog.base.set_state(313);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==T__21 {
				{
				recog.base.set_state(304);
				recog.base.match_token(T__21,&mut recog.err_handler)?;

				/*InvokeRule inheritanceSpecifier*/
				recog.base.set_state(305);
				recog.inheritanceSpecifier()?;

				recog.base.set_state(310);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				while _la==T__15 {
					{
					{
					recog.base.set_state(306);
					recog.base.match_token(T__15,&mut recog.err_handler)?;

					/*InvokeRule inheritanceSpecifier*/
					recog.base.set_state(307);
					recog.inheritanceSpecifier()?;

					}
					}
					recog.base.set_state(312);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
				}
				}
			}

			recog.base.set_state(315);
			recog.base.match_token(T__14,&mut recog.err_handler)?;

			recog.base.set_state(319);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while ((((_la - 14)) & !0x3f) == 0 && ((1usize << (_la - 14)) & ((1usize << (T__13 - 14)) | (1usize << (T__24 - 14)) | (1usize << (T__25 - 14)) | (1usize << (T__35 - 14)) | (1usize << (T__36 - 14)) | (1usize << (T__37 - 14)) | (1usize << (T__39 - 14)) | (1usize << (T__40 - 14)) | (1usize << (T__43 - 14)) | (1usize << (T__45 - 14)) | (1usize << (T__49 - 14)) | (1usize << (T__61 - 14)) | (1usize << (T__62 - 14)) | (1usize << (T__63 - 14)) | (1usize << (T__64 - 14)) | (1usize << (T__65 - 14)))) != 0) || ((((_la - 95)) & !0x3f) == 0 && ((1usize << (_la - 95)) & ((1usize << (T__94 - 95)) | (1usize << (Int - 95)) | (1usize << (Uint - 95)) | (1usize << (Byte - 95)) | (1usize << (Fixed - 95)) | (1usize << (Ufixed - 95)) | (1usize << (LeaveKeyword - 95)) | (1usize << (PayableKeyword - 95)) | (1usize << (TypeKeyword - 95)) | (1usize << (GlobalKeyword - 95)) | (1usize << (ConstructorKeyword - 95)) | (1usize << (FallbackKeyword - 95)) | (1usize << (ReceiveKeyword - 95)) | (1usize << (Identifier - 95)))) != 0) {
				{
				{
				/*InvokeRule contractPart*/
				recog.base.set_state(316);
				recog.contractPart()?;

				}
				}
				recog.base.set_state(321);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(322);
			recog.base.match_token(T__16,&mut recog.err_handler)?;

			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- inheritanceSpecifier ----------------
pub type InheritanceSpecifierContextAll<'input> = InheritanceSpecifierContext<'input>;


pub type InheritanceSpecifierContext<'input> = BaseParserRuleContext<'input,InheritanceSpecifierContextExt<'input>>;

#[derive(Clone)]
pub struct InheritanceSpecifierContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SolidityParserContext<'input> for InheritanceSpecifierContext<'input>{}

impl<'input,'a> Listenable<dyn SolidityListener<'input> + 'a> for InheritanceSpecifierContext<'input>{
	fn enter(&self,listener: &mut (dyn SolidityListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_inheritanceSpecifier(self);
	}
}

impl<'input> CustomRuleContext<'input> for InheritanceSpecifierContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SolidityParserContextType;
	fn get_rule_index(&self) -> usize { RULE_inheritanceSpecifier }
	//fn type_rule_index() -> usize where Self: Sized { RULE_inheritanceSpecifier }
}
antlr_rust::type_id!{InheritanceSpecifierContextExt<'a>}

impl<'input> InheritanceSpecifierContextExt<'input>{
	fn new(parent: Option<Rc<dyn SolidityParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<InheritanceSpecifierContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,InheritanceSpecifierContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait InheritanceSpecifierContextAttrs<'input>: SolidityParserContext<'input> + BorrowMut<InheritanceSpecifierContextExt<'input>>{

fn userDefinedTypeName(&self) -> Option<Rc<UserDefinedTypeNameContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn expressionList(&self) -> Option<Rc<ExpressionListContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> InheritanceSpecifierContextAttrs<'input> for InheritanceSpecifierContext<'input>{}

impl<'input, I, H> SolidityParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn inheritanceSpecifier(&mut self,)
	-> Result<Rc<InheritanceSpecifierContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = InheritanceSpecifierContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 22, RULE_inheritanceSpecifier);
        let mut _localctx: Rc<InheritanceSpecifierContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule userDefinedTypeName*/
			recog.base.set_state(324);
			recog.userDefinedTypeName()?;

			recog.base.set_state(330);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==T__22 {
				{
				recog.base.set_state(325);
				recog.base.match_token(T__22,&mut recog.err_handler)?;

				recog.base.set_state(327);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				if ((((_la - 6)) & !0x3f) == 0 && ((1usize << (_la - 6)) & ((1usize << (T__5 - 6)) | (1usize << (T__13 - 6)) | (1usize << (T__22 - 6)) | (1usize << (T__24 - 6)) | (1usize << (T__29 - 6)) | (1usize << (T__30 - 6)) | (1usize << (T__37 - 6)) | (1usize << (T__41 - 6)) | (1usize << (T__43 - 6)) | (1usize << (T__45 - 6)) | (1usize << (T__49 - 6)) | (1usize << (T__61 - 6)) | (1usize << (T__62 - 6)) | (1usize << (T__63 - 6)) | (1usize << (T__64 - 6)) | (1usize << (T__65 - 6)) | (1usize << (T__66 - 6)) | (1usize << (T__67 - 6)) | (1usize << (T__68 - 6)))) != 0) || ((((_la - 71)) & !0x3f) == 0 && ((1usize << (_la - 71)) & ((1usize << (T__70 - 71)) | (1usize << (T__71 - 71)) | (1usize << (T__94 - 71)) | (1usize << (Int - 71)) | (1usize << (Uint - 71)) | (1usize << (Byte - 71)) | (1usize << (Fixed - 71)) | (1usize << (Ufixed - 71)) | (1usize << (BooleanLiteral - 71)) | (1usize << (DecimalNumber - 71)) | (1usize << (HexNumber - 71)) | (1usize << (HexLiteralFragment - 71)) | (1usize << (LeaveKeyword - 71)) | (1usize << (PayableKeyword - 71)) | (1usize << (TypeKeyword - 71)) | (1usize << (GlobalKeyword - 71)) | (1usize << (ConstructorKeyword - 71)) | (1usize << (ReceiveKeyword - 71)) | (1usize << (Identifier - 71)) | (1usize << (StringLiteralFragment - 71)))) != 0) {
					{
					/*InvokeRule expressionList*/
					recog.base.set_state(326);
					recog.expressionList()?;

					}
				}

				recog.base.set_state(329);
				recog.base.match_token(T__23,&mut recog.err_handler)?;

				}
			}

			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- contractPart ----------------
pub type ContractPartContextAll<'input> = ContractPartContext<'input>;


pub type ContractPartContext<'input> = BaseParserRuleContext<'input,ContractPartContextExt<'input>>;

#[derive(Clone)]
pub struct ContractPartContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SolidityParserContext<'input> for ContractPartContext<'input>{}

impl<'input,'a> Listenable<dyn SolidityListener<'input> + 'a> for ContractPartContext<'input>{
	fn enter(&self,listener: &mut (dyn SolidityListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_contractPart(self);
	}
}

impl<'input> CustomRuleContext<'input> for ContractPartContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SolidityParserContextType;
	fn get_rule_index(&self) -> usize { RULE_contractPart }
	//fn type_rule_index() -> usize where Self: Sized { RULE_contractPart }
}
antlr_rust::type_id!{ContractPartContextExt<'a>}

impl<'input> ContractPartContextExt<'input>{
	fn new(parent: Option<Rc<dyn SolidityParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ContractPartContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ContractPartContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ContractPartContextAttrs<'input>: SolidityParserContext<'input> + BorrowMut<ContractPartContextExt<'input>>{

fn stateVariableDeclaration(&self) -> Option<Rc<StateVariableDeclarationContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn usingForDeclaration(&self) -> Option<Rc<UsingForDeclarationContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn structDefinition(&self) -> Option<Rc<StructDefinitionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn modifierDefinition(&self) -> Option<Rc<ModifierDefinitionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn functionDefinition(&self) -> Option<Rc<FunctionDefinitionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn eventDefinition(&self) -> Option<Rc<EventDefinitionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn enumDefinition(&self) -> Option<Rc<EnumDefinitionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn customErrorDefinition(&self) -> Option<Rc<CustomErrorDefinitionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn typeDefinition(&self) -> Option<Rc<TypeDefinitionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> ContractPartContextAttrs<'input> for ContractPartContext<'input>{}

impl<'input, I, H> SolidityParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn contractPart(&mut self,)
	-> Result<Rc<ContractPartContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ContractPartContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 24, RULE_contractPart);
        let mut _localctx: Rc<ContractPartContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			recog.base.set_state(341);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(20,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule stateVariableDeclaration*/
					recog.base.set_state(332);
					recog.stateVariableDeclaration()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule usingForDeclaration*/
					recog.base.set_state(333);
					recog.usingForDeclaration()?;

					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					/*InvokeRule structDefinition*/
					recog.base.set_state(334);
					recog.structDefinition()?;

					}
				}
			,
				4 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 4);
					recog.base.enter_outer_alt(None, 4);
					{
					/*InvokeRule modifierDefinition*/
					recog.base.set_state(335);
					recog.modifierDefinition()?;

					}
				}
			,
				5 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 5);
					recog.base.enter_outer_alt(None, 5);
					{
					/*InvokeRule functionDefinition*/
					recog.base.set_state(336);
					recog.functionDefinition()?;

					}
				}
			,
				6 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 6);
					recog.base.enter_outer_alt(None, 6);
					{
					/*InvokeRule eventDefinition*/
					recog.base.set_state(337);
					recog.eventDefinition()?;

					}
				}
			,
				7 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 7);
					recog.base.enter_outer_alt(None, 7);
					{
					/*InvokeRule enumDefinition*/
					recog.base.set_state(338);
					recog.enumDefinition()?;

					}
				}
			,
				8 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 8);
					recog.base.enter_outer_alt(None, 8);
					{
					/*InvokeRule customErrorDefinition*/
					recog.base.set_state(339);
					recog.customErrorDefinition()?;

					}
				}
			,
				9 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 9);
					recog.base.enter_outer_alt(None, 9);
					{
					/*InvokeRule typeDefinition*/
					recog.base.set_state(340);
					recog.typeDefinition()?;

					}
				}

				_ => {}
			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- stateVariableDeclaration ----------------
pub type StateVariableDeclarationContextAll<'input> = StateVariableDeclarationContext<'input>;


pub type StateVariableDeclarationContext<'input> = BaseParserRuleContext<'input,StateVariableDeclarationContextExt<'input>>;

#[derive(Clone)]
pub struct StateVariableDeclarationContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SolidityParserContext<'input> for StateVariableDeclarationContext<'input>{}

impl<'input,'a> Listenable<dyn SolidityListener<'input> + 'a> for StateVariableDeclarationContext<'input>{
	fn enter(&self,listener: &mut (dyn SolidityListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_stateVariableDeclaration(self);
	}
}

impl<'input> CustomRuleContext<'input> for StateVariableDeclarationContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SolidityParserContextType;
	fn get_rule_index(&self) -> usize { RULE_stateVariableDeclaration }
	//fn type_rule_index() -> usize where Self: Sized { RULE_stateVariableDeclaration }
}
antlr_rust::type_id!{StateVariableDeclarationContextExt<'a>}

impl<'input> StateVariableDeclarationContextExt<'input>{
	fn new(parent: Option<Rc<dyn SolidityParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<StateVariableDeclarationContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,StateVariableDeclarationContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait StateVariableDeclarationContextAttrs<'input>: SolidityParserContext<'input> + BorrowMut<StateVariableDeclarationContextExt<'input>>{

fn typeName(&self) -> Option<Rc<TypeNameContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn identifier(&self) -> Option<Rc<IdentifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves all `TerminalNode`s corresponding to token PublicKeyword in current rule
fn PublicKeyword_all(&self) -> Vec<Rc<TerminalNode<'input,SolidityParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token PublicKeyword, starting from 0.
/// Returns `None` if number of children corresponding to token PublicKeyword is less or equal than `i`.
fn PublicKeyword(&self, i: usize) -> Option<Rc<TerminalNode<'input,SolidityParserContextType>>> where Self:Sized{
	self.get_token(PublicKeyword, i)
}
/// Retrieves all `TerminalNode`s corresponding to token InternalKeyword in current rule
fn InternalKeyword_all(&self) -> Vec<Rc<TerminalNode<'input,SolidityParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token InternalKeyword, starting from 0.
/// Returns `None` if number of children corresponding to token InternalKeyword is less or equal than `i`.
fn InternalKeyword(&self, i: usize) -> Option<Rc<TerminalNode<'input,SolidityParserContextType>>> where Self:Sized{
	self.get_token(InternalKeyword, i)
}
/// Retrieves all `TerminalNode`s corresponding to token PrivateKeyword in current rule
fn PrivateKeyword_all(&self) -> Vec<Rc<TerminalNode<'input,SolidityParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token PrivateKeyword, starting from 0.
/// Returns `None` if number of children corresponding to token PrivateKeyword is less or equal than `i`.
fn PrivateKeyword(&self, i: usize) -> Option<Rc<TerminalNode<'input,SolidityParserContextType>>> where Self:Sized{
	self.get_token(PrivateKeyword, i)
}
/// Retrieves all `TerminalNode`s corresponding to token ConstantKeyword in current rule
fn ConstantKeyword_all(&self) -> Vec<Rc<TerminalNode<'input,SolidityParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token ConstantKeyword, starting from 0.
/// Returns `None` if number of children corresponding to token ConstantKeyword is less or equal than `i`.
fn ConstantKeyword(&self, i: usize) -> Option<Rc<TerminalNode<'input,SolidityParserContextType>>> where Self:Sized{
	self.get_token(ConstantKeyword, i)
}
/// Retrieves all `TerminalNode`s corresponding to token ImmutableKeyword in current rule
fn ImmutableKeyword_all(&self) -> Vec<Rc<TerminalNode<'input,SolidityParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token ImmutableKeyword, starting from 0.
/// Returns `None` if number of children corresponding to token ImmutableKeyword is less or equal than `i`.
fn ImmutableKeyword(&self, i: usize) -> Option<Rc<TerminalNode<'input,SolidityParserContextType>>> where Self:Sized{
	self.get_token(ImmutableKeyword, i)
}
fn overrideSpecifier_all(&self) ->  Vec<Rc<OverrideSpecifierContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn overrideSpecifier(&self, i: usize) -> Option<Rc<OverrideSpecifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> StateVariableDeclarationContextAttrs<'input> for StateVariableDeclarationContext<'input>{}

impl<'input, I, H> SolidityParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn stateVariableDeclaration(&mut self,)
	-> Result<Rc<StateVariableDeclarationContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = StateVariableDeclarationContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 26, RULE_stateVariableDeclaration);
        let mut _localctx: Rc<StateVariableDeclarationContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule typeName*/
			recog.base.set_state(343);
			recog.typeName_rec(0)?;

			recog.base.set_state(352);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while ((((_la - 96)) & !0x3f) == 0 && ((1usize << (_la - 96)) & ((1usize << (T__95 - 96)) | (1usize << (ConstantKeyword - 96)) | (1usize << (ImmutableKeyword - 96)) | (1usize << (InternalKeyword - 96)) | (1usize << (PrivateKeyword - 96)) | (1usize << (PublicKeyword - 96)))) != 0) {
				{
				recog.base.set_state(350);
				recog.err_handler.sync(&mut recog.base)?;
				match recog.base.input.la(1) {
				 PublicKeyword 
					=> {
						{
						recog.base.set_state(344);
						recog.base.match_token(PublicKeyword,&mut recog.err_handler)?;

						}
					}

				 InternalKeyword 
					=> {
						{
						recog.base.set_state(345);
						recog.base.match_token(InternalKeyword,&mut recog.err_handler)?;

						}
					}

				 PrivateKeyword 
					=> {
						{
						recog.base.set_state(346);
						recog.base.match_token(PrivateKeyword,&mut recog.err_handler)?;

						}
					}

				 ConstantKeyword 
					=> {
						{
						recog.base.set_state(347);
						recog.base.match_token(ConstantKeyword,&mut recog.err_handler)?;

						}
					}

				 ImmutableKeyword 
					=> {
						{
						recog.base.set_state(348);
						recog.base.match_token(ImmutableKeyword,&mut recog.err_handler)?;

						}
					}

				 T__95 
					=> {
						{
						/*InvokeRule overrideSpecifier*/
						recog.base.set_state(349);
						recog.overrideSpecifier()?;

						}
					}

					_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
				}
				}
				recog.base.set_state(354);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			/*InvokeRule identifier*/
			recog.base.set_state(355);
			recog.identifier()?;

			recog.base.set_state(358);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==T__10 {
				{
				recog.base.set_state(356);
				recog.base.match_token(T__10,&mut recog.err_handler)?;

				/*InvokeRule expression*/
				recog.base.set_state(357);
				recog.expression_rec(0)?;

				}
			}

			recog.base.set_state(360);
			recog.base.match_token(T__1,&mut recog.err_handler)?;

			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- fileLevelConstant ----------------
pub type FileLevelConstantContextAll<'input> = FileLevelConstantContext<'input>;


pub type FileLevelConstantContext<'input> = BaseParserRuleContext<'input,FileLevelConstantContextExt<'input>>;

#[derive(Clone)]
pub struct FileLevelConstantContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SolidityParserContext<'input> for FileLevelConstantContext<'input>{}

impl<'input,'a> Listenable<dyn SolidityListener<'input> + 'a> for FileLevelConstantContext<'input>{
	fn enter(&self,listener: &mut (dyn SolidityListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_fileLevelConstant(self);
	}
}

impl<'input> CustomRuleContext<'input> for FileLevelConstantContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SolidityParserContextType;
	fn get_rule_index(&self) -> usize { RULE_fileLevelConstant }
	//fn type_rule_index() -> usize where Self: Sized { RULE_fileLevelConstant }
}
antlr_rust::type_id!{FileLevelConstantContextExt<'a>}

impl<'input> FileLevelConstantContextExt<'input>{
	fn new(parent: Option<Rc<dyn SolidityParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<FileLevelConstantContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,FileLevelConstantContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait FileLevelConstantContextAttrs<'input>: SolidityParserContext<'input> + BorrowMut<FileLevelConstantContextExt<'input>>{

fn typeName(&self) -> Option<Rc<TypeNameContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token ConstantKeyword
/// Returns `None` if there is no child corresponding to token ConstantKeyword
fn ConstantKeyword(&self) -> Option<Rc<TerminalNode<'input,SolidityParserContextType>>> where Self:Sized{
	self.get_token(ConstantKeyword, 0)
}
fn identifier(&self) -> Option<Rc<IdentifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> FileLevelConstantContextAttrs<'input> for FileLevelConstantContext<'input>{}

impl<'input, I, H> SolidityParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn fileLevelConstant(&mut self,)
	-> Result<Rc<FileLevelConstantContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = FileLevelConstantContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 28, RULE_fileLevelConstant);
        let mut _localctx: Rc<FileLevelConstantContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule typeName*/
			recog.base.set_state(362);
			recog.typeName_rec(0)?;

			recog.base.set_state(363);
			recog.base.match_token(ConstantKeyword,&mut recog.err_handler)?;

			/*InvokeRule identifier*/
			recog.base.set_state(364);
			recog.identifier()?;

			recog.base.set_state(365);
			recog.base.match_token(T__10,&mut recog.err_handler)?;

			/*InvokeRule expression*/
			recog.base.set_state(366);
			recog.expression_rec(0)?;

			recog.base.set_state(367);
			recog.base.match_token(T__1,&mut recog.err_handler)?;

			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- customErrorDefinition ----------------
pub type CustomErrorDefinitionContextAll<'input> = CustomErrorDefinitionContext<'input>;


pub type CustomErrorDefinitionContext<'input> = BaseParserRuleContext<'input,CustomErrorDefinitionContextExt<'input>>;

#[derive(Clone)]
pub struct CustomErrorDefinitionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SolidityParserContext<'input> for CustomErrorDefinitionContext<'input>{}

impl<'input,'a> Listenable<dyn SolidityListener<'input> + 'a> for CustomErrorDefinitionContext<'input>{
	fn enter(&self,listener: &mut (dyn SolidityListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_customErrorDefinition(self);
	}
}

impl<'input> CustomRuleContext<'input> for CustomErrorDefinitionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SolidityParserContextType;
	fn get_rule_index(&self) -> usize { RULE_customErrorDefinition }
	//fn type_rule_index() -> usize where Self: Sized { RULE_customErrorDefinition }
}
antlr_rust::type_id!{CustomErrorDefinitionContextExt<'a>}

impl<'input> CustomErrorDefinitionContextExt<'input>{
	fn new(parent: Option<Rc<dyn SolidityParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<CustomErrorDefinitionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,CustomErrorDefinitionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait CustomErrorDefinitionContextAttrs<'input>: SolidityParserContext<'input> + BorrowMut<CustomErrorDefinitionContextExt<'input>>{

fn identifier(&self) -> Option<Rc<IdentifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn parameterList(&self) -> Option<Rc<ParameterListContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> CustomErrorDefinitionContextAttrs<'input> for CustomErrorDefinitionContext<'input>{}

impl<'input, I, H> SolidityParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn customErrorDefinition(&mut self,)
	-> Result<Rc<CustomErrorDefinitionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = CustomErrorDefinitionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 30, RULE_customErrorDefinition);
        let mut _localctx: Rc<CustomErrorDefinitionContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(369);
			recog.base.match_token(T__24,&mut recog.err_handler)?;

			/*InvokeRule identifier*/
			recog.base.set_state(370);
			recog.identifier()?;

			/*InvokeRule parameterList*/
			recog.base.set_state(371);
			recog.parameterList()?;

			recog.base.set_state(372);
			recog.base.match_token(T__1,&mut recog.err_handler)?;

			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- typeDefinition ----------------
pub type TypeDefinitionContextAll<'input> = TypeDefinitionContext<'input>;


pub type TypeDefinitionContext<'input> = BaseParserRuleContext<'input,TypeDefinitionContextExt<'input>>;

#[derive(Clone)]
pub struct TypeDefinitionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SolidityParserContext<'input> for TypeDefinitionContext<'input>{}

impl<'input,'a> Listenable<dyn SolidityListener<'input> + 'a> for TypeDefinitionContext<'input>{
	fn enter(&self,listener: &mut (dyn SolidityListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_typeDefinition(self);
	}
}

impl<'input> CustomRuleContext<'input> for TypeDefinitionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SolidityParserContextType;
	fn get_rule_index(&self) -> usize { RULE_typeDefinition }
	//fn type_rule_index() -> usize where Self: Sized { RULE_typeDefinition }
}
antlr_rust::type_id!{TypeDefinitionContextExt<'a>}

impl<'input> TypeDefinitionContextExt<'input>{
	fn new(parent: Option<Rc<dyn SolidityParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<TypeDefinitionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,TypeDefinitionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait TypeDefinitionContextAttrs<'input>: SolidityParserContext<'input> + BorrowMut<TypeDefinitionContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token TypeKeyword
/// Returns `None` if there is no child corresponding to token TypeKeyword
fn TypeKeyword(&self) -> Option<Rc<TerminalNode<'input,SolidityParserContextType>>> where Self:Sized{
	self.get_token(TypeKeyword, 0)
}
fn identifier(&self) -> Option<Rc<IdentifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn elementaryTypeName(&self) -> Option<Rc<ElementaryTypeNameContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> TypeDefinitionContextAttrs<'input> for TypeDefinitionContext<'input>{}

impl<'input, I, H> SolidityParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn typeDefinition(&mut self,)
	-> Result<Rc<TypeDefinitionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = TypeDefinitionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 32, RULE_typeDefinition);
        let mut _localctx: Rc<TypeDefinitionContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(374);
			recog.base.match_token(TypeKeyword,&mut recog.err_handler)?;

			/*InvokeRule identifier*/
			recog.base.set_state(375);
			recog.identifier()?;

			recog.base.set_state(376);
			recog.base.match_token(T__21,&mut recog.err_handler)?;

			/*InvokeRule elementaryTypeName*/
			recog.base.set_state(377);
			recog.elementaryTypeName()?;

			recog.base.set_state(378);
			recog.base.match_token(T__1,&mut recog.err_handler)?;

			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- usingForDeclaration ----------------
pub type UsingForDeclarationContextAll<'input> = UsingForDeclarationContext<'input>;


pub type UsingForDeclarationContext<'input> = BaseParserRuleContext<'input,UsingForDeclarationContextExt<'input>>;

#[derive(Clone)]
pub struct UsingForDeclarationContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SolidityParserContext<'input> for UsingForDeclarationContext<'input>{}

impl<'input,'a> Listenable<dyn SolidityListener<'input> + 'a> for UsingForDeclarationContext<'input>{
	fn enter(&self,listener: &mut (dyn SolidityListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_usingForDeclaration(self);
	}
}

impl<'input> CustomRuleContext<'input> for UsingForDeclarationContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SolidityParserContextType;
	fn get_rule_index(&self) -> usize { RULE_usingForDeclaration }
	//fn type_rule_index() -> usize where Self: Sized { RULE_usingForDeclaration }
}
antlr_rust::type_id!{UsingForDeclarationContextExt<'a>}

impl<'input> UsingForDeclarationContextExt<'input>{
	fn new(parent: Option<Rc<dyn SolidityParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<UsingForDeclarationContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,UsingForDeclarationContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait UsingForDeclarationContextAttrs<'input>: SolidityParserContext<'input> + BorrowMut<UsingForDeclarationContextExt<'input>>{

fn usingForObject(&self) -> Option<Rc<UsingForObjectContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn typeName(&self) -> Option<Rc<TypeNameContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token GlobalKeyword
/// Returns `None` if there is no child corresponding to token GlobalKeyword
fn GlobalKeyword(&self) -> Option<Rc<TerminalNode<'input,SolidityParserContextType>>> where Self:Sized{
	self.get_token(GlobalKeyword, 0)
}

}

impl<'input> UsingForDeclarationContextAttrs<'input> for UsingForDeclarationContext<'input>{}

impl<'input, I, H> SolidityParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn usingForDeclaration(&mut self,)
	-> Result<Rc<UsingForDeclarationContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = UsingForDeclarationContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 34, RULE_usingForDeclaration);
        let mut _localctx: Rc<UsingForDeclarationContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(380);
			recog.base.match_token(T__25,&mut recog.err_handler)?;

			/*InvokeRule usingForObject*/
			recog.base.set_state(381);
			recog.usingForObject()?;

			recog.base.set_state(382);
			recog.base.match_token(T__26,&mut recog.err_handler)?;

			recog.base.set_state(385);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 T__2 
				=> {
					{
					recog.base.set_state(383);
					recog.base.match_token(T__2,&mut recog.err_handler)?;

					}
				}

			 T__13 | T__24 | T__37 | T__43 | T__45 | T__49 | T__61 | T__62 | T__63 |
			 T__64 | T__65 | T__94 | Int | Uint | Byte | Fixed | Ufixed | LeaveKeyword |
			 PayableKeyword | GlobalKeyword | ConstructorKeyword | ReceiveKeyword |
			 Identifier 
				=> {
					{
					/*InvokeRule typeName*/
					recog.base.set_state(384);
					recog.typeName_rec(0)?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			recog.base.set_state(388);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==GlobalKeyword {
				{
				recog.base.set_state(387);
				recog.base.match_token(GlobalKeyword,&mut recog.err_handler)?;

				}
			}

			recog.base.set_state(390);
			recog.base.match_token(T__1,&mut recog.err_handler)?;

			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- usingForObject ----------------
pub type UsingForObjectContextAll<'input> = UsingForObjectContext<'input>;


pub type UsingForObjectContext<'input> = BaseParserRuleContext<'input,UsingForObjectContextExt<'input>>;

#[derive(Clone)]
pub struct UsingForObjectContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SolidityParserContext<'input> for UsingForObjectContext<'input>{}

impl<'input,'a> Listenable<dyn SolidityListener<'input> + 'a> for UsingForObjectContext<'input>{
	fn enter(&self,listener: &mut (dyn SolidityListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_usingForObject(self);
	}
}

impl<'input> CustomRuleContext<'input> for UsingForObjectContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SolidityParserContextType;
	fn get_rule_index(&self) -> usize { RULE_usingForObject }
	//fn type_rule_index() -> usize where Self: Sized { RULE_usingForObject }
}
antlr_rust::type_id!{UsingForObjectContextExt<'a>}

impl<'input> UsingForObjectContextExt<'input>{
	fn new(parent: Option<Rc<dyn SolidityParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<UsingForObjectContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,UsingForObjectContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait UsingForObjectContextAttrs<'input>: SolidityParserContext<'input> + BorrowMut<UsingForObjectContextExt<'input>>{

fn userDefinedTypeName(&self) -> Option<Rc<UserDefinedTypeNameContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn usingForObjectDirective_all(&self) ->  Vec<Rc<UsingForObjectDirectiveContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn usingForObjectDirective(&self, i: usize) -> Option<Rc<UsingForObjectDirectiveContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> UsingForObjectContextAttrs<'input> for UsingForObjectContext<'input>{}

impl<'input, I, H> SolidityParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn usingForObject(&mut self,)
	-> Result<Rc<UsingForObjectContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = UsingForObjectContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 36, RULE_usingForObject);
        let mut _localctx: Rc<UsingForObjectContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			recog.base.set_state(404);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 T__13 | T__24 | T__43 | T__49 | T__61 | T__94 | LeaveKeyword | PayableKeyword |
			 GlobalKeyword | ConstructorKeyword | ReceiveKeyword | Identifier 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule userDefinedTypeName*/
					recog.base.set_state(392);
					recog.userDefinedTypeName()?;

					}
				}

			 T__14 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(393);
					recog.base.match_token(T__14,&mut recog.err_handler)?;

					/*InvokeRule usingForObjectDirective*/
					recog.base.set_state(394);
					recog.usingForObjectDirective()?;

					recog.base.set_state(399);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					while _la==T__15 {
						{
						{
						recog.base.set_state(395);
						recog.base.match_token(T__15,&mut recog.err_handler)?;

						/*InvokeRule usingForObjectDirective*/
						recog.base.set_state(396);
						recog.usingForObjectDirective()?;

						}
						}
						recog.base.set_state(401);
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
					}
					recog.base.set_state(402);
					recog.base.match_token(T__16,&mut recog.err_handler)?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- usingForObjectDirective ----------------
pub type UsingForObjectDirectiveContextAll<'input> = UsingForObjectDirectiveContext<'input>;


pub type UsingForObjectDirectiveContext<'input> = BaseParserRuleContext<'input,UsingForObjectDirectiveContextExt<'input>>;

#[derive(Clone)]
pub struct UsingForObjectDirectiveContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SolidityParserContext<'input> for UsingForObjectDirectiveContext<'input>{}

impl<'input,'a> Listenable<dyn SolidityListener<'input> + 'a> for UsingForObjectDirectiveContext<'input>{
	fn enter(&self,listener: &mut (dyn SolidityListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_usingForObjectDirective(self);
	}
}

impl<'input> CustomRuleContext<'input> for UsingForObjectDirectiveContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SolidityParserContextType;
	fn get_rule_index(&self) -> usize { RULE_usingForObjectDirective }
	//fn type_rule_index() -> usize where Self: Sized { RULE_usingForObjectDirective }
}
antlr_rust::type_id!{UsingForObjectDirectiveContextExt<'a>}

impl<'input> UsingForObjectDirectiveContextExt<'input>{
	fn new(parent: Option<Rc<dyn SolidityParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<UsingForObjectDirectiveContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,UsingForObjectDirectiveContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait UsingForObjectDirectiveContextAttrs<'input>: SolidityParserContext<'input> + BorrowMut<UsingForObjectDirectiveContextExt<'input>>{

fn userDefinedTypeName(&self) -> Option<Rc<UserDefinedTypeNameContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn userDefinableOperators(&self) -> Option<Rc<UserDefinableOperatorsContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> UsingForObjectDirectiveContextAttrs<'input> for UsingForObjectDirectiveContext<'input>{}

impl<'input, I, H> SolidityParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn usingForObjectDirective(&mut self,)
	-> Result<Rc<UsingForObjectDirectiveContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = UsingForObjectDirectiveContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 38, RULE_usingForObjectDirective);
        let mut _localctx: Rc<UsingForObjectDirectiveContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule userDefinedTypeName*/
			recog.base.set_state(406);
			recog.userDefinedTypeName()?;

			recog.base.set_state(409);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==T__11 {
				{
				recog.base.set_state(407);
				recog.base.match_token(T__11,&mut recog.err_handler)?;

				/*InvokeRule userDefinableOperators*/
				recog.base.set_state(408);
				recog.userDefinableOperators()?;

				}
			}

			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- userDefinableOperators ----------------
pub type UserDefinableOperatorsContextAll<'input> = UserDefinableOperatorsContext<'input>;


pub type UserDefinableOperatorsContext<'input> = BaseParserRuleContext<'input,UserDefinableOperatorsContextExt<'input>>;

#[derive(Clone)]
pub struct UserDefinableOperatorsContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SolidityParserContext<'input> for UserDefinableOperatorsContext<'input>{}

impl<'input,'a> Listenable<dyn SolidityListener<'input> + 'a> for UserDefinableOperatorsContext<'input>{
	fn enter(&self,listener: &mut (dyn SolidityListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_userDefinableOperators(self);
	}
}

impl<'input> CustomRuleContext<'input> for UserDefinableOperatorsContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SolidityParserContextType;
	fn get_rule_index(&self) -> usize { RULE_userDefinableOperators }
	//fn type_rule_index() -> usize where Self: Sized { RULE_userDefinableOperators }
}
antlr_rust::type_id!{UserDefinableOperatorsContextExt<'a>}

impl<'input> UserDefinableOperatorsContextExt<'input>{
	fn new(parent: Option<Rc<dyn SolidityParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<UserDefinableOperatorsContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,UserDefinableOperatorsContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait UserDefinableOperatorsContextAttrs<'input>: SolidityParserContext<'input> + BorrowMut<UserDefinableOperatorsContextExt<'input>>{


}

impl<'input> UserDefinableOperatorsContextAttrs<'input> for UserDefinableOperatorsContext<'input>{}

impl<'input, I, H> SolidityParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn userDefinableOperators(&mut self,)
	-> Result<Rc<UserDefinableOperatorsContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = UserDefinableOperatorsContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 40, RULE_userDefinableOperators);
        let mut _localctx: Rc<UserDefinableOperatorsContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(411);
			_la = recog.base.input.la(1);
			if { !((((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << T__2) | (1usize << T__4) | (1usize << T__5) | (1usize << T__6) | (1usize << T__7) | (1usize << T__8) | (1usize << T__9) | (1usize << T__27) | (1usize << T__28) | (1usize << T__29) | (1usize << T__30) | (1usize << T__31) | (1usize << T__32) | (1usize << T__33) | (1usize << T__34))) != 0)) } {
				recog.err_handler.recover_inline(&mut recog.base)?;

			}
			else {
				if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler);
			}
			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- structDefinition ----------------
pub type StructDefinitionContextAll<'input> = StructDefinitionContext<'input>;


pub type StructDefinitionContext<'input> = BaseParserRuleContext<'input,StructDefinitionContextExt<'input>>;

#[derive(Clone)]
pub struct StructDefinitionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SolidityParserContext<'input> for StructDefinitionContext<'input>{}

impl<'input,'a> Listenable<dyn SolidityListener<'input> + 'a> for StructDefinitionContext<'input>{
	fn enter(&self,listener: &mut (dyn SolidityListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_structDefinition(self);
	}
}

impl<'input> CustomRuleContext<'input> for StructDefinitionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SolidityParserContextType;
	fn get_rule_index(&self) -> usize { RULE_structDefinition }
	//fn type_rule_index() -> usize where Self: Sized { RULE_structDefinition }
}
antlr_rust::type_id!{StructDefinitionContextExt<'a>}

impl<'input> StructDefinitionContextExt<'input>{
	fn new(parent: Option<Rc<dyn SolidityParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<StructDefinitionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,StructDefinitionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait StructDefinitionContextAttrs<'input>: SolidityParserContext<'input> + BorrowMut<StructDefinitionContextExt<'input>>{

fn identifier(&self) -> Option<Rc<IdentifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn variableDeclaration_all(&self) ->  Vec<Rc<VariableDeclarationContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn variableDeclaration(&self, i: usize) -> Option<Rc<VariableDeclarationContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> StructDefinitionContextAttrs<'input> for StructDefinitionContext<'input>{}

impl<'input, I, H> SolidityParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn structDefinition(&mut self,)
	-> Result<Rc<StructDefinitionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = StructDefinitionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 42, RULE_structDefinition);
        let mut _localctx: Rc<StructDefinitionContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(413);
			recog.base.match_token(T__35,&mut recog.err_handler)?;

			/*InvokeRule identifier*/
			recog.base.set_state(414);
			recog.identifier()?;

			recog.base.set_state(415);
			recog.base.match_token(T__14,&mut recog.err_handler)?;

			recog.base.set_state(426);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if ((((_la - 14)) & !0x3f) == 0 && ((1usize << (_la - 14)) & ((1usize << (T__13 - 14)) | (1usize << (T__24 - 14)) | (1usize << (T__37 - 14)) | (1usize << (T__43 - 14)) | (1usize << (T__45 - 14)) | (1usize << (T__49 - 14)) | (1usize << (T__61 - 14)) | (1usize << (T__62 - 14)) | (1usize << (T__63 - 14)) | (1usize << (T__64 - 14)) | (1usize << (T__65 - 14)))) != 0) || ((((_la - 95)) & !0x3f) == 0 && ((1usize << (_la - 95)) & ((1usize << (T__94 - 95)) | (1usize << (Int - 95)) | (1usize << (Uint - 95)) | (1usize << (Byte - 95)) | (1usize << (Fixed - 95)) | (1usize << (Ufixed - 95)) | (1usize << (LeaveKeyword - 95)) | (1usize << (PayableKeyword - 95)) | (1usize << (GlobalKeyword - 95)) | (1usize << (ConstructorKeyword - 95)) | (1usize << (ReceiveKeyword - 95)) | (1usize << (Identifier - 95)))) != 0) {
				{
				/*InvokeRule variableDeclaration*/
				recog.base.set_state(416);
				recog.variableDeclaration()?;

				recog.base.set_state(417);
				recog.base.match_token(T__1,&mut recog.err_handler)?;

				recog.base.set_state(423);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				while ((((_la - 14)) & !0x3f) == 0 && ((1usize << (_la - 14)) & ((1usize << (T__13 - 14)) | (1usize << (T__24 - 14)) | (1usize << (T__37 - 14)) | (1usize << (T__43 - 14)) | (1usize << (T__45 - 14)) | (1usize << (T__49 - 14)) | (1usize << (T__61 - 14)) | (1usize << (T__62 - 14)) | (1usize << (T__63 - 14)) | (1usize << (T__64 - 14)) | (1usize << (T__65 - 14)))) != 0) || ((((_la - 95)) & !0x3f) == 0 && ((1usize << (_la - 95)) & ((1usize << (T__94 - 95)) | (1usize << (Int - 95)) | (1usize << (Uint - 95)) | (1usize << (Byte - 95)) | (1usize << (Fixed - 95)) | (1usize << (Ufixed - 95)) | (1usize << (LeaveKeyword - 95)) | (1usize << (PayableKeyword - 95)) | (1usize << (GlobalKeyword - 95)) | (1usize << (ConstructorKeyword - 95)) | (1usize << (ReceiveKeyword - 95)) | (1usize << (Identifier - 95)))) != 0) {
					{
					{
					/*InvokeRule variableDeclaration*/
					recog.base.set_state(418);
					recog.variableDeclaration()?;

					recog.base.set_state(419);
					recog.base.match_token(T__1,&mut recog.err_handler)?;

					}
					}
					recog.base.set_state(425);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
				}
				}
			}

			recog.base.set_state(428);
			recog.base.match_token(T__16,&mut recog.err_handler)?;

			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- modifierDefinition ----------------
pub type ModifierDefinitionContextAll<'input> = ModifierDefinitionContext<'input>;


pub type ModifierDefinitionContext<'input> = BaseParserRuleContext<'input,ModifierDefinitionContextExt<'input>>;

#[derive(Clone)]
pub struct ModifierDefinitionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SolidityParserContext<'input> for ModifierDefinitionContext<'input>{}

impl<'input,'a> Listenable<dyn SolidityListener<'input> + 'a> for ModifierDefinitionContext<'input>{
	fn enter(&self,listener: &mut (dyn SolidityListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_modifierDefinition(self);
	}
}

impl<'input> CustomRuleContext<'input> for ModifierDefinitionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SolidityParserContextType;
	fn get_rule_index(&self) -> usize { RULE_modifierDefinition }
	//fn type_rule_index() -> usize where Self: Sized { RULE_modifierDefinition }
}
antlr_rust::type_id!{ModifierDefinitionContextExt<'a>}

impl<'input> ModifierDefinitionContextExt<'input>{
	fn new(parent: Option<Rc<dyn SolidityParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ModifierDefinitionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ModifierDefinitionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ModifierDefinitionContextAttrs<'input>: SolidityParserContext<'input> + BorrowMut<ModifierDefinitionContextExt<'input>>{

fn identifier(&self) -> Option<Rc<IdentifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn block(&self) -> Option<Rc<BlockContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn parameterList(&self) -> Option<Rc<ParameterListContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves all `TerminalNode`s corresponding to token VirtualKeyword in current rule
fn VirtualKeyword_all(&self) -> Vec<Rc<TerminalNode<'input,SolidityParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token VirtualKeyword, starting from 0.
/// Returns `None` if number of children corresponding to token VirtualKeyword is less or equal than `i`.
fn VirtualKeyword(&self, i: usize) -> Option<Rc<TerminalNode<'input,SolidityParserContextType>>> where Self:Sized{
	self.get_token(VirtualKeyword, i)
}
fn overrideSpecifier_all(&self) ->  Vec<Rc<OverrideSpecifierContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn overrideSpecifier(&self, i: usize) -> Option<Rc<OverrideSpecifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> ModifierDefinitionContextAttrs<'input> for ModifierDefinitionContext<'input>{}

impl<'input, I, H> SolidityParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn modifierDefinition(&mut self,)
	-> Result<Rc<ModifierDefinitionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ModifierDefinitionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 44, RULE_modifierDefinition);
        let mut _localctx: Rc<ModifierDefinitionContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(430);
			recog.base.match_token(T__36,&mut recog.err_handler)?;

			/*InvokeRule identifier*/
			recog.base.set_state(431);
			recog.identifier()?;

			recog.base.set_state(433);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==T__22 {
				{
				/*InvokeRule parameterList*/
				recog.base.set_state(432);
				recog.parameterList()?;

				}
			}

			recog.base.set_state(439);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==T__95 || _la==VirtualKeyword {
				{
				recog.base.set_state(437);
				recog.err_handler.sync(&mut recog.base)?;
				match recog.base.input.la(1) {
				 VirtualKeyword 
					=> {
						{
						recog.base.set_state(435);
						recog.base.match_token(VirtualKeyword,&mut recog.err_handler)?;

						}
					}

				 T__95 
					=> {
						{
						/*InvokeRule overrideSpecifier*/
						recog.base.set_state(436);
						recog.overrideSpecifier()?;

						}
					}

					_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
				}
				}
				recog.base.set_state(441);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(444);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 T__1 
				=> {
					{
					recog.base.set_state(442);
					recog.base.match_token(T__1,&mut recog.err_handler)?;

					}
				}

			 T__14 
				=> {
					{
					/*InvokeRule block*/
					recog.base.set_state(443);
					recog.block()?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- modifierInvocation ----------------
pub type ModifierInvocationContextAll<'input> = ModifierInvocationContext<'input>;


pub type ModifierInvocationContext<'input> = BaseParserRuleContext<'input,ModifierInvocationContextExt<'input>>;

#[derive(Clone)]
pub struct ModifierInvocationContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SolidityParserContext<'input> for ModifierInvocationContext<'input>{}

impl<'input,'a> Listenable<dyn SolidityListener<'input> + 'a> for ModifierInvocationContext<'input>{
	fn enter(&self,listener: &mut (dyn SolidityListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_modifierInvocation(self);
	}
}

impl<'input> CustomRuleContext<'input> for ModifierInvocationContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SolidityParserContextType;
	fn get_rule_index(&self) -> usize { RULE_modifierInvocation }
	//fn type_rule_index() -> usize where Self: Sized { RULE_modifierInvocation }
}
antlr_rust::type_id!{ModifierInvocationContextExt<'a>}

impl<'input> ModifierInvocationContextExt<'input>{
	fn new(parent: Option<Rc<dyn SolidityParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ModifierInvocationContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ModifierInvocationContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ModifierInvocationContextAttrs<'input>: SolidityParserContext<'input> + BorrowMut<ModifierInvocationContextExt<'input>>{

fn identifier(&self) -> Option<Rc<IdentifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn expressionList(&self) -> Option<Rc<ExpressionListContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> ModifierInvocationContextAttrs<'input> for ModifierInvocationContext<'input>{}

impl<'input, I, H> SolidityParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn modifierInvocation(&mut self,)
	-> Result<Rc<ModifierInvocationContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ModifierInvocationContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 46, RULE_modifierInvocation);
        let mut _localctx: Rc<ModifierInvocationContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule identifier*/
			recog.base.set_state(446);
			recog.identifier()?;

			recog.base.set_state(452);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==T__22 {
				{
				recog.base.set_state(447);
				recog.base.match_token(T__22,&mut recog.err_handler)?;

				recog.base.set_state(449);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				if ((((_la - 6)) & !0x3f) == 0 && ((1usize << (_la - 6)) & ((1usize << (T__5 - 6)) | (1usize << (T__13 - 6)) | (1usize << (T__22 - 6)) | (1usize << (T__24 - 6)) | (1usize << (T__29 - 6)) | (1usize << (T__30 - 6)) | (1usize << (T__37 - 6)) | (1usize << (T__41 - 6)) | (1usize << (T__43 - 6)) | (1usize << (T__45 - 6)) | (1usize << (T__49 - 6)) | (1usize << (T__61 - 6)) | (1usize << (T__62 - 6)) | (1usize << (T__63 - 6)) | (1usize << (T__64 - 6)) | (1usize << (T__65 - 6)) | (1usize << (T__66 - 6)) | (1usize << (T__67 - 6)) | (1usize << (T__68 - 6)))) != 0) || ((((_la - 71)) & !0x3f) == 0 && ((1usize << (_la - 71)) & ((1usize << (T__70 - 71)) | (1usize << (T__71 - 71)) | (1usize << (T__94 - 71)) | (1usize << (Int - 71)) | (1usize << (Uint - 71)) | (1usize << (Byte - 71)) | (1usize << (Fixed - 71)) | (1usize << (Ufixed - 71)) | (1usize << (BooleanLiteral - 71)) | (1usize << (DecimalNumber - 71)) | (1usize << (HexNumber - 71)) | (1usize << (HexLiteralFragment - 71)) | (1usize << (LeaveKeyword - 71)) | (1usize << (PayableKeyword - 71)) | (1usize << (TypeKeyword - 71)) | (1usize << (GlobalKeyword - 71)) | (1usize << (ConstructorKeyword - 71)) | (1usize << (ReceiveKeyword - 71)) | (1usize << (Identifier - 71)) | (1usize << (StringLiteralFragment - 71)))) != 0) {
					{
					/*InvokeRule expressionList*/
					recog.base.set_state(448);
					recog.expressionList()?;

					}
				}

				recog.base.set_state(451);
				recog.base.match_token(T__23,&mut recog.err_handler)?;

				}
			}

			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- functionDefinition ----------------
pub type FunctionDefinitionContextAll<'input> = FunctionDefinitionContext<'input>;


pub type FunctionDefinitionContext<'input> = BaseParserRuleContext<'input,FunctionDefinitionContextExt<'input>>;

#[derive(Clone)]
pub struct FunctionDefinitionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SolidityParserContext<'input> for FunctionDefinitionContext<'input>{}

impl<'input,'a> Listenable<dyn SolidityListener<'input> + 'a> for FunctionDefinitionContext<'input>{
	fn enter(&self,listener: &mut (dyn SolidityListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_functionDefinition(self);
	}
}

impl<'input> CustomRuleContext<'input> for FunctionDefinitionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SolidityParserContextType;
	fn get_rule_index(&self) -> usize { RULE_functionDefinition }
	//fn type_rule_index() -> usize where Self: Sized { RULE_functionDefinition }
}
antlr_rust::type_id!{FunctionDefinitionContextExt<'a>}

impl<'input> FunctionDefinitionContextExt<'input>{
	fn new(parent: Option<Rc<dyn SolidityParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<FunctionDefinitionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,FunctionDefinitionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait FunctionDefinitionContextAttrs<'input>: SolidityParserContext<'input> + BorrowMut<FunctionDefinitionContextExt<'input>>{

fn functionDescriptor(&self) -> Option<Rc<FunctionDescriptorContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn parameterList(&self) -> Option<Rc<ParameterListContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn modifierList(&self) -> Option<Rc<ModifierListContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn block(&self) -> Option<Rc<BlockContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn returnParameters(&self) -> Option<Rc<ReturnParametersContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> FunctionDefinitionContextAttrs<'input> for FunctionDefinitionContext<'input>{}

impl<'input, I, H> SolidityParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn functionDefinition(&mut self,)
	-> Result<Rc<FunctionDefinitionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = FunctionDefinitionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 48, RULE_functionDefinition);
        let mut _localctx: Rc<FunctionDefinitionContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule functionDescriptor*/
			recog.base.set_state(454);
			recog.functionDescriptor()?;

			/*InvokeRule parameterList*/
			recog.base.set_state(455);
			recog.parameterList()?;

			/*InvokeRule modifierList*/
			recog.base.set_state(456);
			recog.modifierList()?;

			recog.base.set_state(458);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==T__38 {
				{
				/*InvokeRule returnParameters*/
				recog.base.set_state(457);
				recog.returnParameters()?;

				}
			}

			recog.base.set_state(462);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 T__1 
				=> {
					{
					recog.base.set_state(460);
					recog.base.match_token(T__1,&mut recog.err_handler)?;

					}
				}

			 T__14 
				=> {
					{
					/*InvokeRule block*/
					recog.base.set_state(461);
					recog.block()?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- functionDescriptor ----------------
pub type FunctionDescriptorContextAll<'input> = FunctionDescriptorContext<'input>;


pub type FunctionDescriptorContext<'input> = BaseParserRuleContext<'input,FunctionDescriptorContextExt<'input>>;

#[derive(Clone)]
pub struct FunctionDescriptorContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SolidityParserContext<'input> for FunctionDescriptorContext<'input>{}

impl<'input,'a> Listenable<dyn SolidityListener<'input> + 'a> for FunctionDescriptorContext<'input>{
	fn enter(&self,listener: &mut (dyn SolidityListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_functionDescriptor(self);
	}
}

impl<'input> CustomRuleContext<'input> for FunctionDescriptorContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SolidityParserContextType;
	fn get_rule_index(&self) -> usize { RULE_functionDescriptor }
	//fn type_rule_index() -> usize where Self: Sized { RULE_functionDescriptor }
}
antlr_rust::type_id!{FunctionDescriptorContextExt<'a>}

impl<'input> FunctionDescriptorContextExt<'input>{
	fn new(parent: Option<Rc<dyn SolidityParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<FunctionDescriptorContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,FunctionDescriptorContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait FunctionDescriptorContextAttrs<'input>: SolidityParserContext<'input> + BorrowMut<FunctionDescriptorContextExt<'input>>{

fn identifier(&self) -> Option<Rc<IdentifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token ConstructorKeyword
/// Returns `None` if there is no child corresponding to token ConstructorKeyword
fn ConstructorKeyword(&self) -> Option<Rc<TerminalNode<'input,SolidityParserContextType>>> where Self:Sized{
	self.get_token(ConstructorKeyword, 0)
}
/// Retrieves first TerminalNode corresponding to token FallbackKeyword
/// Returns `None` if there is no child corresponding to token FallbackKeyword
fn FallbackKeyword(&self) -> Option<Rc<TerminalNode<'input,SolidityParserContextType>>> where Self:Sized{
	self.get_token(FallbackKeyword, 0)
}
/// Retrieves first TerminalNode corresponding to token ReceiveKeyword
/// Returns `None` if there is no child corresponding to token ReceiveKeyword
fn ReceiveKeyword(&self) -> Option<Rc<TerminalNode<'input,SolidityParserContextType>>> where Self:Sized{
	self.get_token(ReceiveKeyword, 0)
}

}

impl<'input> FunctionDescriptorContextAttrs<'input> for FunctionDescriptorContext<'input>{}

impl<'input, I, H> SolidityParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn functionDescriptor(&mut self,)
	-> Result<Rc<FunctionDescriptorContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = FunctionDescriptorContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 50, RULE_functionDescriptor);
        let mut _localctx: Rc<FunctionDescriptorContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			recog.base.set_state(471);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 T__37 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(464);
					recog.base.match_token(T__37,&mut recog.err_handler)?;

					recog.base.set_state(466);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if (((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << T__13) | (1usize << T__24) | (1usize << T__43) | (1usize << T__49) | (1usize << T__61))) != 0) || ((((_la - 95)) & !0x3f) == 0 && ((1usize << (_la - 95)) & ((1usize << (T__94 - 95)) | (1usize << (LeaveKeyword - 95)) | (1usize << (PayableKeyword - 95)) | (1usize << (GlobalKeyword - 95)) | (1usize << (ConstructorKeyword - 95)) | (1usize << (ReceiveKeyword - 95)) | (1usize << (Identifier - 95)))) != 0) {
						{
						/*InvokeRule identifier*/
						recog.base.set_state(465);
						recog.identifier()?;

						}
					}

					}
				}

			 ConstructorKeyword 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(468);
					recog.base.match_token(ConstructorKeyword,&mut recog.err_handler)?;

					}
				}

			 FallbackKeyword 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					recog.base.set_state(469);
					recog.base.match_token(FallbackKeyword,&mut recog.err_handler)?;

					}
				}

			 ReceiveKeyword 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 4);
					recog.base.enter_outer_alt(None, 4);
					{
					recog.base.set_state(470);
					recog.base.match_token(ReceiveKeyword,&mut recog.err_handler)?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- returnParameters ----------------
pub type ReturnParametersContextAll<'input> = ReturnParametersContext<'input>;


pub type ReturnParametersContext<'input> = BaseParserRuleContext<'input,ReturnParametersContextExt<'input>>;

#[derive(Clone)]
pub struct ReturnParametersContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SolidityParserContext<'input> for ReturnParametersContext<'input>{}

impl<'input,'a> Listenable<dyn SolidityListener<'input> + 'a> for ReturnParametersContext<'input>{
	fn enter(&self,listener: &mut (dyn SolidityListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_returnParameters(self);
	}
}

impl<'input> CustomRuleContext<'input> for ReturnParametersContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SolidityParserContextType;
	fn get_rule_index(&self) -> usize { RULE_returnParameters }
	//fn type_rule_index() -> usize where Self: Sized { RULE_returnParameters }
}
antlr_rust::type_id!{ReturnParametersContextExt<'a>}

impl<'input> ReturnParametersContextExt<'input>{
	fn new(parent: Option<Rc<dyn SolidityParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ReturnParametersContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ReturnParametersContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ReturnParametersContextAttrs<'input>: SolidityParserContext<'input> + BorrowMut<ReturnParametersContextExt<'input>>{

fn parameterList(&self) -> Option<Rc<ParameterListContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> ReturnParametersContextAttrs<'input> for ReturnParametersContext<'input>{}

impl<'input, I, H> SolidityParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn returnParameters(&mut self,)
	-> Result<Rc<ReturnParametersContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ReturnParametersContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 52, RULE_returnParameters);
        let mut _localctx: Rc<ReturnParametersContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(473);
			recog.base.match_token(T__38,&mut recog.err_handler)?;

			/*InvokeRule parameterList*/
			recog.base.set_state(474);
			recog.parameterList()?;

			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- modifierList ----------------
pub type ModifierListContextAll<'input> = ModifierListContext<'input>;


pub type ModifierListContext<'input> = BaseParserRuleContext<'input,ModifierListContextExt<'input>>;

#[derive(Clone)]
pub struct ModifierListContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SolidityParserContext<'input> for ModifierListContext<'input>{}

impl<'input,'a> Listenable<dyn SolidityListener<'input> + 'a> for ModifierListContext<'input>{
	fn enter(&self,listener: &mut (dyn SolidityListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_modifierList(self);
	}
}

impl<'input> CustomRuleContext<'input> for ModifierListContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SolidityParserContextType;
	fn get_rule_index(&self) -> usize { RULE_modifierList }
	//fn type_rule_index() -> usize where Self: Sized { RULE_modifierList }
}
antlr_rust::type_id!{ModifierListContextExt<'a>}

impl<'input> ModifierListContextExt<'input>{
	fn new(parent: Option<Rc<dyn SolidityParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ModifierListContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ModifierListContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ModifierListContextAttrs<'input>: SolidityParserContext<'input> + BorrowMut<ModifierListContextExt<'input>>{

/// Retrieves all `TerminalNode`s corresponding to token ExternalKeyword in current rule
fn ExternalKeyword_all(&self) -> Vec<Rc<TerminalNode<'input,SolidityParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token ExternalKeyword, starting from 0.
/// Returns `None` if number of children corresponding to token ExternalKeyword is less or equal than `i`.
fn ExternalKeyword(&self, i: usize) -> Option<Rc<TerminalNode<'input,SolidityParserContextType>>> where Self:Sized{
	self.get_token(ExternalKeyword, i)
}
/// Retrieves all `TerminalNode`s corresponding to token PublicKeyword in current rule
fn PublicKeyword_all(&self) -> Vec<Rc<TerminalNode<'input,SolidityParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token PublicKeyword, starting from 0.
/// Returns `None` if number of children corresponding to token PublicKeyword is less or equal than `i`.
fn PublicKeyword(&self, i: usize) -> Option<Rc<TerminalNode<'input,SolidityParserContextType>>> where Self:Sized{
	self.get_token(PublicKeyword, i)
}
/// Retrieves all `TerminalNode`s corresponding to token InternalKeyword in current rule
fn InternalKeyword_all(&self) -> Vec<Rc<TerminalNode<'input,SolidityParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token InternalKeyword, starting from 0.
/// Returns `None` if number of children corresponding to token InternalKeyword is less or equal than `i`.
fn InternalKeyword(&self, i: usize) -> Option<Rc<TerminalNode<'input,SolidityParserContextType>>> where Self:Sized{
	self.get_token(InternalKeyword, i)
}
/// Retrieves all `TerminalNode`s corresponding to token PrivateKeyword in current rule
fn PrivateKeyword_all(&self) -> Vec<Rc<TerminalNode<'input,SolidityParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token PrivateKeyword, starting from 0.
/// Returns `None` if number of children corresponding to token PrivateKeyword is less or equal than `i`.
fn PrivateKeyword(&self, i: usize) -> Option<Rc<TerminalNode<'input,SolidityParserContextType>>> where Self:Sized{
	self.get_token(PrivateKeyword, i)
}
/// Retrieves all `TerminalNode`s corresponding to token VirtualKeyword in current rule
fn VirtualKeyword_all(&self) -> Vec<Rc<TerminalNode<'input,SolidityParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token VirtualKeyword, starting from 0.
/// Returns `None` if number of children corresponding to token VirtualKeyword is less or equal than `i`.
fn VirtualKeyword(&self, i: usize) -> Option<Rc<TerminalNode<'input,SolidityParserContextType>>> where Self:Sized{
	self.get_token(VirtualKeyword, i)
}
fn stateMutability_all(&self) ->  Vec<Rc<StateMutabilityContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn stateMutability(&self, i: usize) -> Option<Rc<StateMutabilityContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn modifierInvocation_all(&self) ->  Vec<Rc<ModifierInvocationContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn modifierInvocation(&self, i: usize) -> Option<Rc<ModifierInvocationContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn overrideSpecifier_all(&self) ->  Vec<Rc<OverrideSpecifierContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn overrideSpecifier(&self, i: usize) -> Option<Rc<OverrideSpecifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> ModifierListContextAttrs<'input> for ModifierListContext<'input>{}

impl<'input, I, H> SolidityParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn modifierList(&mut self,)
	-> Result<Rc<ModifierListContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ModifierListContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 54, RULE_modifierList);
        let mut _localctx: Rc<ModifierListContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(486);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while (((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << T__13) | (1usize << T__24) | (1usize << T__43) | (1usize << T__49) | (1usize << T__61))) != 0) || ((((_la - 95)) & !0x3f) == 0 && ((1usize << (_la - 95)) & ((1usize << (T__94 - 95)) | (1usize << (T__95 - 95)) | (1usize << (ConstantKeyword - 95)) | (1usize << (LeaveKeyword - 95)) | (1usize << (ExternalKeyword - 95)) | (1usize << (InternalKeyword - 95)) | (1usize << (PayableKeyword - 95)) | (1usize << (PrivateKeyword - 95)) | (1usize << (PublicKeyword - 95)) | (1usize << (VirtualKeyword - 95)) | (1usize << (PureKeyword - 95)) | (1usize << (ViewKeyword - 95)) | (1usize << (GlobalKeyword - 95)) | (1usize << (ConstructorKeyword - 95)) | (1usize << (ReceiveKeyword - 95)) | (1usize << (Identifier - 95)))) != 0) {
				{
				recog.base.set_state(484);
				recog.err_handler.sync(&mut recog.base)?;
				match  recog.interpreter.adaptive_predict(41,&mut recog.base)? {
					1 =>{
						{
						recog.base.set_state(476);
						recog.base.match_token(ExternalKeyword,&mut recog.err_handler)?;

						}
					}
				,
					2 =>{
						{
						recog.base.set_state(477);
						recog.base.match_token(PublicKeyword,&mut recog.err_handler)?;

						}
					}
				,
					3 =>{
						{
						recog.base.set_state(478);
						recog.base.match_token(InternalKeyword,&mut recog.err_handler)?;

						}
					}
				,
					4 =>{
						{
						recog.base.set_state(479);
						recog.base.match_token(PrivateKeyword,&mut recog.err_handler)?;

						}
					}
				,
					5 =>{
						{
						recog.base.set_state(480);
						recog.base.match_token(VirtualKeyword,&mut recog.err_handler)?;

						}
					}
				,
					6 =>{
						{
						/*InvokeRule stateMutability*/
						recog.base.set_state(481);
						recog.stateMutability()?;

						}
					}
				,
					7 =>{
						{
						/*InvokeRule modifierInvocation*/
						recog.base.set_state(482);
						recog.modifierInvocation()?;

						}
					}
				,
					8 =>{
						{
						/*InvokeRule overrideSpecifier*/
						recog.base.set_state(483);
						recog.overrideSpecifier()?;

						}
					}

					_ => {}
				}
				}
				recog.base.set_state(488);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- eventDefinition ----------------
pub type EventDefinitionContextAll<'input> = EventDefinitionContext<'input>;


pub type EventDefinitionContext<'input> = BaseParserRuleContext<'input,EventDefinitionContextExt<'input>>;

#[derive(Clone)]
pub struct EventDefinitionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SolidityParserContext<'input> for EventDefinitionContext<'input>{}

impl<'input,'a> Listenable<dyn SolidityListener<'input> + 'a> for EventDefinitionContext<'input>{
	fn enter(&self,listener: &mut (dyn SolidityListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_eventDefinition(self);
	}
}

impl<'input> CustomRuleContext<'input> for EventDefinitionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SolidityParserContextType;
	fn get_rule_index(&self) -> usize { RULE_eventDefinition }
	//fn type_rule_index() -> usize where Self: Sized { RULE_eventDefinition }
}
antlr_rust::type_id!{EventDefinitionContextExt<'a>}

impl<'input> EventDefinitionContextExt<'input>{
	fn new(parent: Option<Rc<dyn SolidityParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<EventDefinitionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,EventDefinitionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait EventDefinitionContextAttrs<'input>: SolidityParserContext<'input> + BorrowMut<EventDefinitionContextExt<'input>>{

fn identifier(&self) -> Option<Rc<IdentifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn eventParameterList(&self) -> Option<Rc<EventParameterListContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token AnonymousKeyword
/// Returns `None` if there is no child corresponding to token AnonymousKeyword
fn AnonymousKeyword(&self) -> Option<Rc<TerminalNode<'input,SolidityParserContextType>>> where Self:Sized{
	self.get_token(AnonymousKeyword, 0)
}

}

impl<'input> EventDefinitionContextAttrs<'input> for EventDefinitionContext<'input>{}

impl<'input, I, H> SolidityParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn eventDefinition(&mut self,)
	-> Result<Rc<EventDefinitionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = EventDefinitionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 56, RULE_eventDefinition);
        let mut _localctx: Rc<EventDefinitionContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(489);
			recog.base.match_token(T__39,&mut recog.err_handler)?;

			/*InvokeRule identifier*/
			recog.base.set_state(490);
			recog.identifier()?;

			/*InvokeRule eventParameterList*/
			recog.base.set_state(491);
			recog.eventParameterList()?;

			recog.base.set_state(493);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==AnonymousKeyword {
				{
				recog.base.set_state(492);
				recog.base.match_token(AnonymousKeyword,&mut recog.err_handler)?;

				}
			}

			recog.base.set_state(495);
			recog.base.match_token(T__1,&mut recog.err_handler)?;

			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- enumValue ----------------
pub type EnumValueContextAll<'input> = EnumValueContext<'input>;


pub type EnumValueContext<'input> = BaseParserRuleContext<'input,EnumValueContextExt<'input>>;

#[derive(Clone)]
pub struct EnumValueContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SolidityParserContext<'input> for EnumValueContext<'input>{}

impl<'input,'a> Listenable<dyn SolidityListener<'input> + 'a> for EnumValueContext<'input>{
	fn enter(&self,listener: &mut (dyn SolidityListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_enumValue(self);
	}
}

impl<'input> CustomRuleContext<'input> for EnumValueContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SolidityParserContextType;
	fn get_rule_index(&self) -> usize { RULE_enumValue }
	//fn type_rule_index() -> usize where Self: Sized { RULE_enumValue }
}
antlr_rust::type_id!{EnumValueContextExt<'a>}

impl<'input> EnumValueContextExt<'input>{
	fn new(parent: Option<Rc<dyn SolidityParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<EnumValueContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,EnumValueContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait EnumValueContextAttrs<'input>: SolidityParserContext<'input> + BorrowMut<EnumValueContextExt<'input>>{

fn identifier(&self) -> Option<Rc<IdentifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> EnumValueContextAttrs<'input> for EnumValueContext<'input>{}

impl<'input, I, H> SolidityParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn enumValue(&mut self,)
	-> Result<Rc<EnumValueContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = EnumValueContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 58, RULE_enumValue);
        let mut _localctx: Rc<EnumValueContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule identifier*/
			recog.base.set_state(497);
			recog.identifier()?;

			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- enumDefinition ----------------
pub type EnumDefinitionContextAll<'input> = EnumDefinitionContext<'input>;


pub type EnumDefinitionContext<'input> = BaseParserRuleContext<'input,EnumDefinitionContextExt<'input>>;

#[derive(Clone)]
pub struct EnumDefinitionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SolidityParserContext<'input> for EnumDefinitionContext<'input>{}

impl<'input,'a> Listenable<dyn SolidityListener<'input> + 'a> for EnumDefinitionContext<'input>{
	fn enter(&self,listener: &mut (dyn SolidityListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_enumDefinition(self);
	}
}

impl<'input> CustomRuleContext<'input> for EnumDefinitionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SolidityParserContextType;
	fn get_rule_index(&self) -> usize { RULE_enumDefinition }
	//fn type_rule_index() -> usize where Self: Sized { RULE_enumDefinition }
}
antlr_rust::type_id!{EnumDefinitionContextExt<'a>}

impl<'input> EnumDefinitionContextExt<'input>{
	fn new(parent: Option<Rc<dyn SolidityParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<EnumDefinitionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,EnumDefinitionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait EnumDefinitionContextAttrs<'input>: SolidityParserContext<'input> + BorrowMut<EnumDefinitionContextExt<'input>>{

fn identifier(&self) -> Option<Rc<IdentifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn enumValue_all(&self) ->  Vec<Rc<EnumValueContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn enumValue(&self, i: usize) -> Option<Rc<EnumValueContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> EnumDefinitionContextAttrs<'input> for EnumDefinitionContext<'input>{}

impl<'input, I, H> SolidityParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn enumDefinition(&mut self,)
	-> Result<Rc<EnumDefinitionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = EnumDefinitionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 60, RULE_enumDefinition);
        let mut _localctx: Rc<EnumDefinitionContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(499);
			recog.base.match_token(T__40,&mut recog.err_handler)?;

			/*InvokeRule identifier*/
			recog.base.set_state(500);
			recog.identifier()?;

			recog.base.set_state(501);
			recog.base.match_token(T__14,&mut recog.err_handler)?;

			recog.base.set_state(503);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if (((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << T__13) | (1usize << T__24) | (1usize << T__43) | (1usize << T__49) | (1usize << T__61))) != 0) || ((((_la - 95)) & !0x3f) == 0 && ((1usize << (_la - 95)) & ((1usize << (T__94 - 95)) | (1usize << (LeaveKeyword - 95)) | (1usize << (PayableKeyword - 95)) | (1usize << (GlobalKeyword - 95)) | (1usize << (ConstructorKeyword - 95)) | (1usize << (ReceiveKeyword - 95)) | (1usize << (Identifier - 95)))) != 0) {
				{
				/*InvokeRule enumValue*/
				recog.base.set_state(502);
				recog.enumValue()?;

				}
			}

			recog.base.set_state(509);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==T__15 {
				{
				{
				recog.base.set_state(505);
				recog.base.match_token(T__15,&mut recog.err_handler)?;

				/*InvokeRule enumValue*/
				recog.base.set_state(506);
				recog.enumValue()?;

				}
				}
				recog.base.set_state(511);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(512);
			recog.base.match_token(T__16,&mut recog.err_handler)?;

			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- parameterList ----------------
pub type ParameterListContextAll<'input> = ParameterListContext<'input>;


pub type ParameterListContext<'input> = BaseParserRuleContext<'input,ParameterListContextExt<'input>>;

#[derive(Clone)]
pub struct ParameterListContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SolidityParserContext<'input> for ParameterListContext<'input>{}

impl<'input,'a> Listenable<dyn SolidityListener<'input> + 'a> for ParameterListContext<'input>{
	fn enter(&self,listener: &mut (dyn SolidityListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_parameterList(self);
	}
}

impl<'input> CustomRuleContext<'input> for ParameterListContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SolidityParserContextType;
	fn get_rule_index(&self) -> usize { RULE_parameterList }
	//fn type_rule_index() -> usize where Self: Sized { RULE_parameterList }
}
antlr_rust::type_id!{ParameterListContextExt<'a>}

impl<'input> ParameterListContextExt<'input>{
	fn new(parent: Option<Rc<dyn SolidityParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ParameterListContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ParameterListContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ParameterListContextAttrs<'input>: SolidityParserContext<'input> + BorrowMut<ParameterListContextExt<'input>>{

fn parameter_all(&self) ->  Vec<Rc<ParameterContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn parameter(&self, i: usize) -> Option<Rc<ParameterContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> ParameterListContextAttrs<'input> for ParameterListContext<'input>{}

impl<'input, I, H> SolidityParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn parameterList(&mut self,)
	-> Result<Rc<ParameterListContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ParameterListContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 62, RULE_parameterList);
        let mut _localctx: Rc<ParameterListContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(514);
			recog.base.match_token(T__22,&mut recog.err_handler)?;

			recog.base.set_state(523);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if ((((_la - 14)) & !0x3f) == 0 && ((1usize << (_la - 14)) & ((1usize << (T__13 - 14)) | (1usize << (T__24 - 14)) | (1usize << (T__37 - 14)) | (1usize << (T__43 - 14)) | (1usize << (T__45 - 14)) | (1usize << (T__49 - 14)) | (1usize << (T__61 - 14)) | (1usize << (T__62 - 14)) | (1usize << (T__63 - 14)) | (1usize << (T__64 - 14)) | (1usize << (T__65 - 14)))) != 0) || ((((_la - 95)) & !0x3f) == 0 && ((1usize << (_la - 95)) & ((1usize << (T__94 - 95)) | (1usize << (Int - 95)) | (1usize << (Uint - 95)) | (1usize << (Byte - 95)) | (1usize << (Fixed - 95)) | (1usize << (Ufixed - 95)) | (1usize << (LeaveKeyword - 95)) | (1usize << (PayableKeyword - 95)) | (1usize << (GlobalKeyword - 95)) | (1usize << (ConstructorKeyword - 95)) | (1usize << (ReceiveKeyword - 95)) | (1usize << (Identifier - 95)))) != 0) {
				{
				/*InvokeRule parameter*/
				recog.base.set_state(515);
				recog.parameter()?;

				recog.base.set_state(520);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				while _la==T__15 {
					{
					{
					recog.base.set_state(516);
					recog.base.match_token(T__15,&mut recog.err_handler)?;

					/*InvokeRule parameter*/
					recog.base.set_state(517);
					recog.parameter()?;

					}
					}
					recog.base.set_state(522);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
				}
				}
			}

			recog.base.set_state(525);
			recog.base.match_token(T__23,&mut recog.err_handler)?;

			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- parameter ----------------
pub type ParameterContextAll<'input> = ParameterContext<'input>;


pub type ParameterContext<'input> = BaseParserRuleContext<'input,ParameterContextExt<'input>>;

#[derive(Clone)]
pub struct ParameterContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SolidityParserContext<'input> for ParameterContext<'input>{}

impl<'input,'a> Listenable<dyn SolidityListener<'input> + 'a> for ParameterContext<'input>{
	fn enter(&self,listener: &mut (dyn SolidityListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_parameter(self);
	}
}

impl<'input> CustomRuleContext<'input> for ParameterContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SolidityParserContextType;
	fn get_rule_index(&self) -> usize { RULE_parameter }
	//fn type_rule_index() -> usize where Self: Sized { RULE_parameter }
}
antlr_rust::type_id!{ParameterContextExt<'a>}

impl<'input> ParameterContextExt<'input>{
	fn new(parent: Option<Rc<dyn SolidityParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ParameterContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ParameterContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ParameterContextAttrs<'input>: SolidityParserContext<'input> + BorrowMut<ParameterContextExt<'input>>{

fn typeName(&self) -> Option<Rc<TypeNameContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn storageLocation(&self) -> Option<Rc<StorageLocationContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn identifier(&self) -> Option<Rc<IdentifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> ParameterContextAttrs<'input> for ParameterContext<'input>{}

impl<'input, I, H> SolidityParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn parameter(&mut self,)
	-> Result<Rc<ParameterContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ParameterContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 64, RULE_parameter);
        let mut _localctx: Rc<ParameterContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule typeName*/
			recog.base.set_state(527);
			recog.typeName_rec(0)?;

			recog.base.set_state(529);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(48,&mut recog.base)? {
				x if x == 1=>{
					{
					/*InvokeRule storageLocation*/
					recog.base.set_state(528);
					recog.storageLocation()?;

					}
				}

				_ => {}
			}
			recog.base.set_state(532);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if (((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << T__13) | (1usize << T__24) | (1usize << T__43) | (1usize << T__49) | (1usize << T__61))) != 0) || ((((_la - 95)) & !0x3f) == 0 && ((1usize << (_la - 95)) & ((1usize << (T__94 - 95)) | (1usize << (LeaveKeyword - 95)) | (1usize << (PayableKeyword - 95)) | (1usize << (GlobalKeyword - 95)) | (1usize << (ConstructorKeyword - 95)) | (1usize << (ReceiveKeyword - 95)) | (1usize << (Identifier - 95)))) != 0) {
				{
				/*InvokeRule identifier*/
				recog.base.set_state(531);
				recog.identifier()?;

				}
			}

			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- eventParameterList ----------------
pub type EventParameterListContextAll<'input> = EventParameterListContext<'input>;


pub type EventParameterListContext<'input> = BaseParserRuleContext<'input,EventParameterListContextExt<'input>>;

#[derive(Clone)]
pub struct EventParameterListContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SolidityParserContext<'input> for EventParameterListContext<'input>{}

impl<'input,'a> Listenable<dyn SolidityListener<'input> + 'a> for EventParameterListContext<'input>{
	fn enter(&self,listener: &mut (dyn SolidityListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_eventParameterList(self);
	}
}

impl<'input> CustomRuleContext<'input> for EventParameterListContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SolidityParserContextType;
	fn get_rule_index(&self) -> usize { RULE_eventParameterList }
	//fn type_rule_index() -> usize where Self: Sized { RULE_eventParameterList }
}
antlr_rust::type_id!{EventParameterListContextExt<'a>}

impl<'input> EventParameterListContextExt<'input>{
	fn new(parent: Option<Rc<dyn SolidityParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<EventParameterListContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,EventParameterListContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait EventParameterListContextAttrs<'input>: SolidityParserContext<'input> + BorrowMut<EventParameterListContextExt<'input>>{

fn eventParameter_all(&self) ->  Vec<Rc<EventParameterContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn eventParameter(&self, i: usize) -> Option<Rc<EventParameterContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> EventParameterListContextAttrs<'input> for EventParameterListContext<'input>{}

impl<'input, I, H> SolidityParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn eventParameterList(&mut self,)
	-> Result<Rc<EventParameterListContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = EventParameterListContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 66, RULE_eventParameterList);
        let mut _localctx: Rc<EventParameterListContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(534);
			recog.base.match_token(T__22,&mut recog.err_handler)?;

			recog.base.set_state(543);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if ((((_la - 14)) & !0x3f) == 0 && ((1usize << (_la - 14)) & ((1usize << (T__13 - 14)) | (1usize << (T__24 - 14)) | (1usize << (T__37 - 14)) | (1usize << (T__43 - 14)) | (1usize << (T__45 - 14)) | (1usize << (T__49 - 14)) | (1usize << (T__61 - 14)) | (1usize << (T__62 - 14)) | (1usize << (T__63 - 14)) | (1usize << (T__64 - 14)) | (1usize << (T__65 - 14)))) != 0) || ((((_la - 95)) & !0x3f) == 0 && ((1usize << (_la - 95)) & ((1usize << (T__94 - 95)) | (1usize << (Int - 95)) | (1usize << (Uint - 95)) | (1usize << (Byte - 95)) | (1usize << (Fixed - 95)) | (1usize << (Ufixed - 95)) | (1usize << (LeaveKeyword - 95)) | (1usize << (PayableKeyword - 95)) | (1usize << (GlobalKeyword - 95)) | (1usize << (ConstructorKeyword - 95)) | (1usize << (ReceiveKeyword - 95)) | (1usize << (Identifier - 95)))) != 0) {
				{
				/*InvokeRule eventParameter*/
				recog.base.set_state(535);
				recog.eventParameter()?;

				recog.base.set_state(540);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				while _la==T__15 {
					{
					{
					recog.base.set_state(536);
					recog.base.match_token(T__15,&mut recog.err_handler)?;

					/*InvokeRule eventParameter*/
					recog.base.set_state(537);
					recog.eventParameter()?;

					}
					}
					recog.base.set_state(542);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
				}
				}
			}

			recog.base.set_state(545);
			recog.base.match_token(T__23,&mut recog.err_handler)?;

			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- eventParameter ----------------
pub type EventParameterContextAll<'input> = EventParameterContext<'input>;


pub type EventParameterContext<'input> = BaseParserRuleContext<'input,EventParameterContextExt<'input>>;

#[derive(Clone)]
pub struct EventParameterContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SolidityParserContext<'input> for EventParameterContext<'input>{}

impl<'input,'a> Listenable<dyn SolidityListener<'input> + 'a> for EventParameterContext<'input>{
	fn enter(&self,listener: &mut (dyn SolidityListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_eventParameter(self);
	}
}

impl<'input> CustomRuleContext<'input> for EventParameterContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SolidityParserContextType;
	fn get_rule_index(&self) -> usize { RULE_eventParameter }
	//fn type_rule_index() -> usize where Self: Sized { RULE_eventParameter }
}
antlr_rust::type_id!{EventParameterContextExt<'a>}

impl<'input> EventParameterContextExt<'input>{
	fn new(parent: Option<Rc<dyn SolidityParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<EventParameterContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,EventParameterContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait EventParameterContextAttrs<'input>: SolidityParserContext<'input> + BorrowMut<EventParameterContextExt<'input>>{

fn typeName(&self) -> Option<Rc<TypeNameContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token IndexedKeyword
/// Returns `None` if there is no child corresponding to token IndexedKeyword
fn IndexedKeyword(&self) -> Option<Rc<TerminalNode<'input,SolidityParserContextType>>> where Self:Sized{
	self.get_token(IndexedKeyword, 0)
}
fn identifier(&self) -> Option<Rc<IdentifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> EventParameterContextAttrs<'input> for EventParameterContext<'input>{}

impl<'input, I, H> SolidityParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn eventParameter(&mut self,)
	-> Result<Rc<EventParameterContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = EventParameterContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 68, RULE_eventParameter);
        let mut _localctx: Rc<EventParameterContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule typeName*/
			recog.base.set_state(547);
			recog.typeName_rec(0)?;

			recog.base.set_state(549);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==IndexedKeyword {
				{
				recog.base.set_state(548);
				recog.base.match_token(IndexedKeyword,&mut recog.err_handler)?;

				}
			}

			recog.base.set_state(552);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if (((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << T__13) | (1usize << T__24) | (1usize << T__43) | (1usize << T__49) | (1usize << T__61))) != 0) || ((((_la - 95)) & !0x3f) == 0 && ((1usize << (_la - 95)) & ((1usize << (T__94 - 95)) | (1usize << (LeaveKeyword - 95)) | (1usize << (PayableKeyword - 95)) | (1usize << (GlobalKeyword - 95)) | (1usize << (ConstructorKeyword - 95)) | (1usize << (ReceiveKeyword - 95)) | (1usize << (Identifier - 95)))) != 0) {
				{
				/*InvokeRule identifier*/
				recog.base.set_state(551);
				recog.identifier()?;

				}
			}

			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- functionTypeParameterList ----------------
pub type FunctionTypeParameterListContextAll<'input> = FunctionTypeParameterListContext<'input>;


pub type FunctionTypeParameterListContext<'input> = BaseParserRuleContext<'input,FunctionTypeParameterListContextExt<'input>>;

#[derive(Clone)]
pub struct FunctionTypeParameterListContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SolidityParserContext<'input> for FunctionTypeParameterListContext<'input>{}

impl<'input,'a> Listenable<dyn SolidityListener<'input> + 'a> for FunctionTypeParameterListContext<'input>{
	fn enter(&self,listener: &mut (dyn SolidityListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_functionTypeParameterList(self);
	}
}

impl<'input> CustomRuleContext<'input> for FunctionTypeParameterListContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SolidityParserContextType;
	fn get_rule_index(&self) -> usize { RULE_functionTypeParameterList }
	//fn type_rule_index() -> usize where Self: Sized { RULE_functionTypeParameterList }
}
antlr_rust::type_id!{FunctionTypeParameterListContextExt<'a>}

impl<'input> FunctionTypeParameterListContextExt<'input>{
	fn new(parent: Option<Rc<dyn SolidityParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<FunctionTypeParameterListContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,FunctionTypeParameterListContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait FunctionTypeParameterListContextAttrs<'input>: SolidityParserContext<'input> + BorrowMut<FunctionTypeParameterListContextExt<'input>>{

fn functionTypeParameter_all(&self) ->  Vec<Rc<FunctionTypeParameterContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn functionTypeParameter(&self, i: usize) -> Option<Rc<FunctionTypeParameterContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> FunctionTypeParameterListContextAttrs<'input> for FunctionTypeParameterListContext<'input>{}

impl<'input, I, H> SolidityParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn functionTypeParameterList(&mut self,)
	-> Result<Rc<FunctionTypeParameterListContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = FunctionTypeParameterListContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 70, RULE_functionTypeParameterList);
        let mut _localctx: Rc<FunctionTypeParameterListContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(554);
			recog.base.match_token(T__22,&mut recog.err_handler)?;

			recog.base.set_state(563);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if ((((_la - 14)) & !0x3f) == 0 && ((1usize << (_la - 14)) & ((1usize << (T__13 - 14)) | (1usize << (T__24 - 14)) | (1usize << (T__37 - 14)) | (1usize << (T__43 - 14)) | (1usize << (T__45 - 14)) | (1usize << (T__49 - 14)) | (1usize << (T__61 - 14)) | (1usize << (T__62 - 14)) | (1usize << (T__63 - 14)) | (1usize << (T__64 - 14)) | (1usize << (T__65 - 14)))) != 0) || ((((_la - 95)) & !0x3f) == 0 && ((1usize << (_la - 95)) & ((1usize << (T__94 - 95)) | (1usize << (Int - 95)) | (1usize << (Uint - 95)) | (1usize << (Byte - 95)) | (1usize << (Fixed - 95)) | (1usize << (Ufixed - 95)) | (1usize << (LeaveKeyword - 95)) | (1usize << (PayableKeyword - 95)) | (1usize << (GlobalKeyword - 95)) | (1usize << (ConstructorKeyword - 95)) | (1usize << (ReceiveKeyword - 95)) | (1usize << (Identifier - 95)))) != 0) {
				{
				/*InvokeRule functionTypeParameter*/
				recog.base.set_state(555);
				recog.functionTypeParameter()?;

				recog.base.set_state(560);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				while _la==T__15 {
					{
					{
					recog.base.set_state(556);
					recog.base.match_token(T__15,&mut recog.err_handler)?;

					/*InvokeRule functionTypeParameter*/
					recog.base.set_state(557);
					recog.functionTypeParameter()?;

					}
					}
					recog.base.set_state(562);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
				}
				}
			}

			recog.base.set_state(565);
			recog.base.match_token(T__23,&mut recog.err_handler)?;

			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- functionTypeParameter ----------------
pub type FunctionTypeParameterContextAll<'input> = FunctionTypeParameterContext<'input>;


pub type FunctionTypeParameterContext<'input> = BaseParserRuleContext<'input,FunctionTypeParameterContextExt<'input>>;

#[derive(Clone)]
pub struct FunctionTypeParameterContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SolidityParserContext<'input> for FunctionTypeParameterContext<'input>{}

impl<'input,'a> Listenable<dyn SolidityListener<'input> + 'a> for FunctionTypeParameterContext<'input>{
	fn enter(&self,listener: &mut (dyn SolidityListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_functionTypeParameter(self);
	}
}

impl<'input> CustomRuleContext<'input> for FunctionTypeParameterContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SolidityParserContextType;
	fn get_rule_index(&self) -> usize { RULE_functionTypeParameter }
	//fn type_rule_index() -> usize where Self: Sized { RULE_functionTypeParameter }
}
antlr_rust::type_id!{FunctionTypeParameterContextExt<'a>}

impl<'input> FunctionTypeParameterContextExt<'input>{
	fn new(parent: Option<Rc<dyn SolidityParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<FunctionTypeParameterContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,FunctionTypeParameterContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait FunctionTypeParameterContextAttrs<'input>: SolidityParserContext<'input> + BorrowMut<FunctionTypeParameterContextExt<'input>>{

fn typeName(&self) -> Option<Rc<TypeNameContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn storageLocation(&self) -> Option<Rc<StorageLocationContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> FunctionTypeParameterContextAttrs<'input> for FunctionTypeParameterContext<'input>{}

impl<'input, I, H> SolidityParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn functionTypeParameter(&mut self,)
	-> Result<Rc<FunctionTypeParameterContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = FunctionTypeParameterContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 72, RULE_functionTypeParameter);
        let mut _localctx: Rc<FunctionTypeParameterContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule typeName*/
			recog.base.set_state(567);
			recog.typeName_rec(0)?;

			recog.base.set_state(569);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if (((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << T__47) | (1usize << T__48) | (1usize << T__49))) != 0) {
				{
				/*InvokeRule storageLocation*/
				recog.base.set_state(568);
				recog.storageLocation()?;

				}
			}

			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- variableDeclaration ----------------
pub type VariableDeclarationContextAll<'input> = VariableDeclarationContext<'input>;


pub type VariableDeclarationContext<'input> = BaseParserRuleContext<'input,VariableDeclarationContextExt<'input>>;

#[derive(Clone)]
pub struct VariableDeclarationContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SolidityParserContext<'input> for VariableDeclarationContext<'input>{}

impl<'input,'a> Listenable<dyn SolidityListener<'input> + 'a> for VariableDeclarationContext<'input>{
	fn enter(&self,listener: &mut (dyn SolidityListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_variableDeclaration(self);
	}
}

impl<'input> CustomRuleContext<'input> for VariableDeclarationContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SolidityParserContextType;
	fn get_rule_index(&self) -> usize { RULE_variableDeclaration }
	//fn type_rule_index() -> usize where Self: Sized { RULE_variableDeclaration }
}
antlr_rust::type_id!{VariableDeclarationContextExt<'a>}

impl<'input> VariableDeclarationContextExt<'input>{
	fn new(parent: Option<Rc<dyn SolidityParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<VariableDeclarationContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,VariableDeclarationContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait VariableDeclarationContextAttrs<'input>: SolidityParserContext<'input> + BorrowMut<VariableDeclarationContextExt<'input>>{

fn typeName(&self) -> Option<Rc<TypeNameContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn identifier(&self) -> Option<Rc<IdentifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn storageLocation(&self) -> Option<Rc<StorageLocationContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> VariableDeclarationContextAttrs<'input> for VariableDeclarationContext<'input>{}

impl<'input, I, H> SolidityParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn variableDeclaration(&mut self,)
	-> Result<Rc<VariableDeclarationContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = VariableDeclarationContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 74, RULE_variableDeclaration);
        let mut _localctx: Rc<VariableDeclarationContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule typeName*/
			recog.base.set_state(571);
			recog.typeName_rec(0)?;

			recog.base.set_state(573);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(57,&mut recog.base)? {
				x if x == 1=>{
					{
					/*InvokeRule storageLocation*/
					recog.base.set_state(572);
					recog.storageLocation()?;

					}
				}

				_ => {}
			}
			/*InvokeRule identifier*/
			recog.base.set_state(575);
			recog.identifier()?;

			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- typeName ----------------
pub type TypeNameContextAll<'input> = TypeNameContext<'input>;


pub type TypeNameContext<'input> = BaseParserRuleContext<'input,TypeNameContextExt<'input>>;

#[derive(Clone)]
pub struct TypeNameContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SolidityParserContext<'input> for TypeNameContext<'input>{}

impl<'input,'a> Listenable<dyn SolidityListener<'input> + 'a> for TypeNameContext<'input>{
	fn enter(&self,listener: &mut (dyn SolidityListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_typeName(self);
	}
}

impl<'input> CustomRuleContext<'input> for TypeNameContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SolidityParserContextType;
	fn get_rule_index(&self) -> usize { RULE_typeName }
	//fn type_rule_index() -> usize where Self: Sized { RULE_typeName }
}
antlr_rust::type_id!{TypeNameContextExt<'a>}

impl<'input> TypeNameContextExt<'input>{
	fn new(parent: Option<Rc<dyn SolidityParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<TypeNameContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,TypeNameContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait TypeNameContextAttrs<'input>: SolidityParserContext<'input> + BorrowMut<TypeNameContextExt<'input>>{

fn elementaryTypeName(&self) -> Option<Rc<ElementaryTypeNameContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn userDefinedTypeName(&self) -> Option<Rc<UserDefinedTypeNameContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn mapping(&self) -> Option<Rc<MappingContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn functionTypeName(&self) -> Option<Rc<FunctionTypeNameContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token PayableKeyword
/// Returns `None` if there is no child corresponding to token PayableKeyword
fn PayableKeyword(&self) -> Option<Rc<TerminalNode<'input,SolidityParserContextType>>> where Self:Sized{
	self.get_token(PayableKeyword, 0)
}
fn typeName(&self) -> Option<Rc<TypeNameContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> TypeNameContextAttrs<'input> for TypeNameContext<'input>{}

impl<'input, I, H> SolidityParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn  typeName(&mut self,)
	-> Result<Rc<TypeNameContextAll<'input>>,ANTLRError> {
		self.typeName_rec(0)
	}

	fn typeName_rec(&mut self, _p: isize)
	-> Result<Rc<TypeNameContextAll<'input>>,ANTLRError> {
		let recog = self;
		let _parentctx = recog.ctx.take();
		let _parentState = recog.base.get_state();
		let mut _localctx = TypeNameContextExt::new(_parentctx.clone(), recog.base.get_state());
		recog.base.enter_recursion_rule(_localctx.clone(), 76, RULE_typeName, _p);
	    let mut _localctx: Rc<TypeNameContextAll> = _localctx;
        let mut _prevctx = _localctx.clone();
		let _startState = 76;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {
			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(584);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(58,&mut recog.base)? {
				1 =>{
					{
					/*InvokeRule elementaryTypeName*/
					recog.base.set_state(578);
					recog.elementaryTypeName()?;

					}
				}
			,
				2 =>{
					{
					/*InvokeRule userDefinedTypeName*/
					recog.base.set_state(579);
					recog.userDefinedTypeName()?;

					}
				}
			,
				3 =>{
					{
					/*InvokeRule mapping*/
					recog.base.set_state(580);
					recog.mapping()?;

					}
				}
			,
				4 =>{
					{
					/*InvokeRule functionTypeName*/
					recog.base.set_state(581);
					recog.functionTypeName()?;

					}
				}
			,
				5 =>{
					{
					recog.base.set_state(582);
					recog.base.match_token(T__43,&mut recog.err_handler)?;

					recog.base.set_state(583);
					recog.base.match_token(PayableKeyword,&mut recog.err_handler)?;

					}
				}

				_ => {}
			}

			let tmp = recog.input.lt(-1).cloned();
			recog.ctx.as_ref().unwrap().set_stop(tmp);
			recog.base.set_state(594);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(60,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					recog.trigger_exit_rule_event();
					_prevctx = _localctx.clone();
					{
					{
					/*recRuleAltStartAction*/
					let mut tmp = TypeNameContextExt::new(_parentctx.clone(), _parentState);
					recog.push_new_recursion_context(tmp.clone(), _startState, RULE_typeName);
					_localctx = tmp;
					recog.base.set_state(586);
					if !({recog.precpred(None, 3)}) {
						Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 3)".to_owned()), None))?;
					}
					recog.base.set_state(587);
					recog.base.match_token(T__41,&mut recog.err_handler)?;

					recog.base.set_state(589);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if ((((_la - 6)) & !0x3f) == 0 && ((1usize << (_la - 6)) & ((1usize << (T__5 - 6)) | (1usize << (T__13 - 6)) | (1usize << (T__22 - 6)) | (1usize << (T__24 - 6)) | (1usize << (T__29 - 6)) | (1usize << (T__30 - 6)) | (1usize << (T__37 - 6)) | (1usize << (T__41 - 6)) | (1usize << (T__43 - 6)) | (1usize << (T__45 - 6)) | (1usize << (T__49 - 6)) | (1usize << (T__61 - 6)) | (1usize << (T__62 - 6)) | (1usize << (T__63 - 6)) | (1usize << (T__64 - 6)) | (1usize << (T__65 - 6)) | (1usize << (T__66 - 6)) | (1usize << (T__67 - 6)) | (1usize << (T__68 - 6)))) != 0) || ((((_la - 71)) & !0x3f) == 0 && ((1usize << (_la - 71)) & ((1usize << (T__70 - 71)) | (1usize << (T__71 - 71)) | (1usize << (T__94 - 71)) | (1usize << (Int - 71)) | (1usize << (Uint - 71)) | (1usize << (Byte - 71)) | (1usize << (Fixed - 71)) | (1usize << (Ufixed - 71)) | (1usize << (BooleanLiteral - 71)) | (1usize << (DecimalNumber - 71)) | (1usize << (HexNumber - 71)) | (1usize << (HexLiteralFragment - 71)) | (1usize << (LeaveKeyword - 71)) | (1usize << (PayableKeyword - 71)) | (1usize << (TypeKeyword - 71)) | (1usize << (GlobalKeyword - 71)) | (1usize << (ConstructorKeyword - 71)) | (1usize << (ReceiveKeyword - 71)) | (1usize << (Identifier - 71)) | (1usize << (StringLiteralFragment - 71)))) != 0) {
						{
						/*InvokeRule expression*/
						recog.base.set_state(588);
						recog.expression_rec(0)?;

						}
					}

					recog.base.set_state(591);
					recog.base.match_token(T__42,&mut recog.err_handler)?;

					}
					} 
				}
				recog.base.set_state(596);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(60,&mut recog.base)?;
			}
			}
		};
		match result {
		Ok(_) => {},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re)=>{
			//_localctx.exception = re;
			recog.err_handler.report_error(&mut recog.base, re);
	        recog.err_handler.recover(&mut recog.base, re)?;}
		}
		recog.base.unroll_recursion_context(_parentctx);

		Ok(_localctx)
	}
}
//------------------- userDefinedTypeName ----------------
pub type UserDefinedTypeNameContextAll<'input> = UserDefinedTypeNameContext<'input>;


pub type UserDefinedTypeNameContext<'input> = BaseParserRuleContext<'input,UserDefinedTypeNameContextExt<'input>>;

#[derive(Clone)]
pub struct UserDefinedTypeNameContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SolidityParserContext<'input> for UserDefinedTypeNameContext<'input>{}

impl<'input,'a> Listenable<dyn SolidityListener<'input> + 'a> for UserDefinedTypeNameContext<'input>{
	fn enter(&self,listener: &mut (dyn SolidityListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_userDefinedTypeName(self);
	}
}

impl<'input> CustomRuleContext<'input> for UserDefinedTypeNameContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SolidityParserContextType;
	fn get_rule_index(&self) -> usize { RULE_userDefinedTypeName }
	//fn type_rule_index() -> usize where Self: Sized { RULE_userDefinedTypeName }
}
antlr_rust::type_id!{UserDefinedTypeNameContextExt<'a>}

impl<'input> UserDefinedTypeNameContextExt<'input>{
	fn new(parent: Option<Rc<dyn SolidityParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<UserDefinedTypeNameContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,UserDefinedTypeNameContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait UserDefinedTypeNameContextAttrs<'input>: SolidityParserContext<'input> + BorrowMut<UserDefinedTypeNameContextExt<'input>>{

fn identifier_all(&self) ->  Vec<Rc<IdentifierContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn identifier(&self, i: usize) -> Option<Rc<IdentifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> UserDefinedTypeNameContextAttrs<'input> for UserDefinedTypeNameContext<'input>{}

impl<'input, I, H> SolidityParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn userDefinedTypeName(&mut self,)
	-> Result<Rc<UserDefinedTypeNameContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = UserDefinedTypeNameContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 78, RULE_userDefinedTypeName);
        let mut _localctx: Rc<UserDefinedTypeNameContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule identifier*/
			recog.base.set_state(597);
			recog.identifier()?;

			recog.base.set_state(602);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(61,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					{
					{
					recog.base.set_state(598);
					recog.base.match_token(T__44,&mut recog.err_handler)?;

					/*InvokeRule identifier*/
					recog.base.set_state(599);
					recog.identifier()?;

					}
					} 
				}
				recog.base.set_state(604);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(61,&mut recog.base)?;
			}
			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- mappingKey ----------------
pub type MappingKeyContextAll<'input> = MappingKeyContext<'input>;


pub type MappingKeyContext<'input> = BaseParserRuleContext<'input,MappingKeyContextExt<'input>>;

#[derive(Clone)]
pub struct MappingKeyContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SolidityParserContext<'input> for MappingKeyContext<'input>{}

impl<'input,'a> Listenable<dyn SolidityListener<'input> + 'a> for MappingKeyContext<'input>{
	fn enter(&self,listener: &mut (dyn SolidityListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_mappingKey(self);
	}
}

impl<'input> CustomRuleContext<'input> for MappingKeyContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SolidityParserContextType;
	fn get_rule_index(&self) -> usize { RULE_mappingKey }
	//fn type_rule_index() -> usize where Self: Sized { RULE_mappingKey }
}
antlr_rust::type_id!{MappingKeyContextExt<'a>}

impl<'input> MappingKeyContextExt<'input>{
	fn new(parent: Option<Rc<dyn SolidityParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<MappingKeyContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,MappingKeyContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait MappingKeyContextAttrs<'input>: SolidityParserContext<'input> + BorrowMut<MappingKeyContextExt<'input>>{

fn elementaryTypeName(&self) -> Option<Rc<ElementaryTypeNameContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn userDefinedTypeName(&self) -> Option<Rc<UserDefinedTypeNameContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> MappingKeyContextAttrs<'input> for MappingKeyContext<'input>{}

impl<'input, I, H> SolidityParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn mappingKey(&mut self,)
	-> Result<Rc<MappingKeyContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = MappingKeyContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 80, RULE_mappingKey);
        let mut _localctx: Rc<MappingKeyContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			recog.base.set_state(607);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(62,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule elementaryTypeName*/
					recog.base.set_state(605);
					recog.elementaryTypeName()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule userDefinedTypeName*/
					recog.base.set_state(606);
					recog.userDefinedTypeName()?;

					}
				}

				_ => {}
			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- mapping ----------------
pub type MappingContextAll<'input> = MappingContext<'input>;


pub type MappingContext<'input> = BaseParserRuleContext<'input,MappingContextExt<'input>>;

#[derive(Clone)]
pub struct MappingContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SolidityParserContext<'input> for MappingContext<'input>{}

impl<'input,'a> Listenable<dyn SolidityListener<'input> + 'a> for MappingContext<'input>{
	fn enter(&self,listener: &mut (dyn SolidityListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_mapping(self);
	}
}

impl<'input> CustomRuleContext<'input> for MappingContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SolidityParserContextType;
	fn get_rule_index(&self) -> usize { RULE_mapping }
	//fn type_rule_index() -> usize where Self: Sized { RULE_mapping }
}
antlr_rust::type_id!{MappingContextExt<'a>}

impl<'input> MappingContextExt<'input>{
	fn new(parent: Option<Rc<dyn SolidityParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<MappingContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,MappingContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait MappingContextAttrs<'input>: SolidityParserContext<'input> + BorrowMut<MappingContextExt<'input>>{

fn mappingKey(&self) -> Option<Rc<MappingKeyContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn typeName(&self) -> Option<Rc<TypeNameContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn mappingKeyName(&self) -> Option<Rc<MappingKeyNameContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn mappingValueName(&self) -> Option<Rc<MappingValueNameContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> MappingContextAttrs<'input> for MappingContext<'input>{}

impl<'input, I, H> SolidityParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn mapping(&mut self,)
	-> Result<Rc<MappingContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = MappingContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 82, RULE_mapping);
        let mut _localctx: Rc<MappingContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(609);
			recog.base.match_token(T__45,&mut recog.err_handler)?;

			recog.base.set_state(610);
			recog.base.match_token(T__22,&mut recog.err_handler)?;

			/*InvokeRule mappingKey*/
			recog.base.set_state(611);
			recog.mappingKey()?;

			recog.base.set_state(613);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if (((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << T__13) | (1usize << T__24) | (1usize << T__43) | (1usize << T__49) | (1usize << T__61))) != 0) || ((((_la - 95)) & !0x3f) == 0 && ((1usize << (_la - 95)) & ((1usize << (T__94 - 95)) | (1usize << (LeaveKeyword - 95)) | (1usize << (PayableKeyword - 95)) | (1usize << (GlobalKeyword - 95)) | (1usize << (ConstructorKeyword - 95)) | (1usize << (ReceiveKeyword - 95)) | (1usize << (Identifier - 95)))) != 0) {
				{
				/*InvokeRule mappingKeyName*/
				recog.base.set_state(612);
				recog.mappingKeyName()?;

				}
			}

			recog.base.set_state(615);
			recog.base.match_token(T__46,&mut recog.err_handler)?;

			/*InvokeRule typeName*/
			recog.base.set_state(616);
			recog.typeName_rec(0)?;

			recog.base.set_state(618);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if (((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << T__13) | (1usize << T__24) | (1usize << T__43) | (1usize << T__49) | (1usize << T__61))) != 0) || ((((_la - 95)) & !0x3f) == 0 && ((1usize << (_la - 95)) & ((1usize << (T__94 - 95)) | (1usize << (LeaveKeyword - 95)) | (1usize << (PayableKeyword - 95)) | (1usize << (GlobalKeyword - 95)) | (1usize << (ConstructorKeyword - 95)) | (1usize << (ReceiveKeyword - 95)) | (1usize << (Identifier - 95)))) != 0) {
				{
				/*InvokeRule mappingValueName*/
				recog.base.set_state(617);
				recog.mappingValueName()?;

				}
			}

			recog.base.set_state(620);
			recog.base.match_token(T__23,&mut recog.err_handler)?;

			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- mappingKeyName ----------------
pub type MappingKeyNameContextAll<'input> = MappingKeyNameContext<'input>;


pub type MappingKeyNameContext<'input> = BaseParserRuleContext<'input,MappingKeyNameContextExt<'input>>;

#[derive(Clone)]
pub struct MappingKeyNameContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SolidityParserContext<'input> for MappingKeyNameContext<'input>{}

impl<'input,'a> Listenable<dyn SolidityListener<'input> + 'a> for MappingKeyNameContext<'input>{
	fn enter(&self,listener: &mut (dyn SolidityListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_mappingKeyName(self);
	}
}

impl<'input> CustomRuleContext<'input> for MappingKeyNameContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SolidityParserContextType;
	fn get_rule_index(&self) -> usize { RULE_mappingKeyName }
	//fn type_rule_index() -> usize where Self: Sized { RULE_mappingKeyName }
}
antlr_rust::type_id!{MappingKeyNameContextExt<'a>}

impl<'input> MappingKeyNameContextExt<'input>{
	fn new(parent: Option<Rc<dyn SolidityParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<MappingKeyNameContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,MappingKeyNameContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait MappingKeyNameContextAttrs<'input>: SolidityParserContext<'input> + BorrowMut<MappingKeyNameContextExt<'input>>{

fn identifier(&self) -> Option<Rc<IdentifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> MappingKeyNameContextAttrs<'input> for MappingKeyNameContext<'input>{}

impl<'input, I, H> SolidityParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn mappingKeyName(&mut self,)
	-> Result<Rc<MappingKeyNameContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = MappingKeyNameContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 84, RULE_mappingKeyName);
        let mut _localctx: Rc<MappingKeyNameContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule identifier*/
			recog.base.set_state(622);
			recog.identifier()?;

			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- mappingValueName ----------------
pub type MappingValueNameContextAll<'input> = MappingValueNameContext<'input>;


pub type MappingValueNameContext<'input> = BaseParserRuleContext<'input,MappingValueNameContextExt<'input>>;

#[derive(Clone)]
pub struct MappingValueNameContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SolidityParserContext<'input> for MappingValueNameContext<'input>{}

impl<'input,'a> Listenable<dyn SolidityListener<'input> + 'a> for MappingValueNameContext<'input>{
	fn enter(&self,listener: &mut (dyn SolidityListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_mappingValueName(self);
	}
}

impl<'input> CustomRuleContext<'input> for MappingValueNameContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SolidityParserContextType;
	fn get_rule_index(&self) -> usize { RULE_mappingValueName }
	//fn type_rule_index() -> usize where Self: Sized { RULE_mappingValueName }
}
antlr_rust::type_id!{MappingValueNameContextExt<'a>}

impl<'input> MappingValueNameContextExt<'input>{
	fn new(parent: Option<Rc<dyn SolidityParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<MappingValueNameContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,MappingValueNameContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait MappingValueNameContextAttrs<'input>: SolidityParserContext<'input> + BorrowMut<MappingValueNameContextExt<'input>>{

fn identifier(&self) -> Option<Rc<IdentifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> MappingValueNameContextAttrs<'input> for MappingValueNameContext<'input>{}

impl<'input, I, H> SolidityParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn mappingValueName(&mut self,)
	-> Result<Rc<MappingValueNameContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = MappingValueNameContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 86, RULE_mappingValueName);
        let mut _localctx: Rc<MappingValueNameContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule identifier*/
			recog.base.set_state(624);
			recog.identifier()?;

			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- functionTypeName ----------------
pub type FunctionTypeNameContextAll<'input> = FunctionTypeNameContext<'input>;


pub type FunctionTypeNameContext<'input> = BaseParserRuleContext<'input,FunctionTypeNameContextExt<'input>>;

#[derive(Clone)]
pub struct FunctionTypeNameContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SolidityParserContext<'input> for FunctionTypeNameContext<'input>{}

impl<'input,'a> Listenable<dyn SolidityListener<'input> + 'a> for FunctionTypeNameContext<'input>{
	fn enter(&self,listener: &mut (dyn SolidityListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_functionTypeName(self);
	}
}

impl<'input> CustomRuleContext<'input> for FunctionTypeNameContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SolidityParserContextType;
	fn get_rule_index(&self) -> usize { RULE_functionTypeName }
	//fn type_rule_index() -> usize where Self: Sized { RULE_functionTypeName }
}
antlr_rust::type_id!{FunctionTypeNameContextExt<'a>}

impl<'input> FunctionTypeNameContextExt<'input>{
	fn new(parent: Option<Rc<dyn SolidityParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<FunctionTypeNameContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,FunctionTypeNameContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait FunctionTypeNameContextAttrs<'input>: SolidityParserContext<'input> + BorrowMut<FunctionTypeNameContextExt<'input>>{

fn functionTypeParameterList_all(&self) ->  Vec<Rc<FunctionTypeParameterListContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn functionTypeParameterList(&self, i: usize) -> Option<Rc<FunctionTypeParameterListContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token InternalKeyword in current rule
fn InternalKeyword_all(&self) -> Vec<Rc<TerminalNode<'input,SolidityParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token InternalKeyword, starting from 0.
/// Returns `None` if number of children corresponding to token InternalKeyword is less or equal than `i`.
fn InternalKeyword(&self, i: usize) -> Option<Rc<TerminalNode<'input,SolidityParserContextType>>> where Self:Sized{
	self.get_token(InternalKeyword, i)
}
/// Retrieves all `TerminalNode`s corresponding to token ExternalKeyword in current rule
fn ExternalKeyword_all(&self) -> Vec<Rc<TerminalNode<'input,SolidityParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token ExternalKeyword, starting from 0.
/// Returns `None` if number of children corresponding to token ExternalKeyword is less or equal than `i`.
fn ExternalKeyword(&self, i: usize) -> Option<Rc<TerminalNode<'input,SolidityParserContextType>>> where Self:Sized{
	self.get_token(ExternalKeyword, i)
}
fn stateMutability_all(&self) ->  Vec<Rc<StateMutabilityContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn stateMutability(&self, i: usize) -> Option<Rc<StateMutabilityContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> FunctionTypeNameContextAttrs<'input> for FunctionTypeNameContext<'input>{}

impl<'input, I, H> SolidityParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn functionTypeName(&mut self,)
	-> Result<Rc<FunctionTypeNameContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = FunctionTypeNameContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 88, RULE_functionTypeName);
        let mut _localctx: Rc<FunctionTypeNameContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(626);
			recog.base.match_token(T__37,&mut recog.err_handler)?;

			/*InvokeRule functionTypeParameterList*/
			recog.base.set_state(627);
			recog.functionTypeParameterList()?;

			recog.base.set_state(633);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(66,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					{
					recog.base.set_state(631);
					recog.err_handler.sync(&mut recog.base)?;
					match recog.base.input.la(1) {
					 InternalKeyword 
						=> {
							{
							recog.base.set_state(628);
							recog.base.match_token(InternalKeyword,&mut recog.err_handler)?;

							}
						}

					 ExternalKeyword 
						=> {
							{
							recog.base.set_state(629);
							recog.base.match_token(ExternalKeyword,&mut recog.err_handler)?;

							}
						}

					 ConstantKeyword | PayableKeyword | PureKeyword | ViewKeyword 
						=> {
							{
							/*InvokeRule stateMutability*/
							recog.base.set_state(630);
							recog.stateMutability()?;

							}
						}

						_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
					}
					} 
				}
				recog.base.set_state(635);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(66,&mut recog.base)?;
			}
			recog.base.set_state(638);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(67,&mut recog.base)? {
				x if x == 1=>{
					{
					recog.base.set_state(636);
					recog.base.match_token(T__38,&mut recog.err_handler)?;

					/*InvokeRule functionTypeParameterList*/
					recog.base.set_state(637);
					recog.functionTypeParameterList()?;

					}
				}

				_ => {}
			}
			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- storageLocation ----------------
pub type StorageLocationContextAll<'input> = StorageLocationContext<'input>;


pub type StorageLocationContext<'input> = BaseParserRuleContext<'input,StorageLocationContextExt<'input>>;

#[derive(Clone)]
pub struct StorageLocationContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SolidityParserContext<'input> for StorageLocationContext<'input>{}

impl<'input,'a> Listenable<dyn SolidityListener<'input> + 'a> for StorageLocationContext<'input>{
	fn enter(&self,listener: &mut (dyn SolidityListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_storageLocation(self);
	}
}

impl<'input> CustomRuleContext<'input> for StorageLocationContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SolidityParserContextType;
	fn get_rule_index(&self) -> usize { RULE_storageLocation }
	//fn type_rule_index() -> usize where Self: Sized { RULE_storageLocation }
}
antlr_rust::type_id!{StorageLocationContextExt<'a>}

impl<'input> StorageLocationContextExt<'input>{
	fn new(parent: Option<Rc<dyn SolidityParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<StorageLocationContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,StorageLocationContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait StorageLocationContextAttrs<'input>: SolidityParserContext<'input> + BorrowMut<StorageLocationContextExt<'input>>{


}

impl<'input> StorageLocationContextAttrs<'input> for StorageLocationContext<'input>{}

impl<'input, I, H> SolidityParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn storageLocation(&mut self,)
	-> Result<Rc<StorageLocationContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = StorageLocationContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 90, RULE_storageLocation);
        let mut _localctx: Rc<StorageLocationContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(640);
			_la = recog.base.input.la(1);
			if { !((((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << T__47) | (1usize << T__48) | (1usize << T__49))) != 0)) } {
				recog.err_handler.recover_inline(&mut recog.base)?;

			}
			else {
				if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler);
			}
			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- stateMutability ----------------
pub type StateMutabilityContextAll<'input> = StateMutabilityContext<'input>;


pub type StateMutabilityContext<'input> = BaseParserRuleContext<'input,StateMutabilityContextExt<'input>>;

#[derive(Clone)]
pub struct StateMutabilityContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SolidityParserContext<'input> for StateMutabilityContext<'input>{}

impl<'input,'a> Listenable<dyn SolidityListener<'input> + 'a> for StateMutabilityContext<'input>{
	fn enter(&self,listener: &mut (dyn SolidityListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_stateMutability(self);
	}
}

impl<'input> CustomRuleContext<'input> for StateMutabilityContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SolidityParserContextType;
	fn get_rule_index(&self) -> usize { RULE_stateMutability }
	//fn type_rule_index() -> usize where Self: Sized { RULE_stateMutability }
}
antlr_rust::type_id!{StateMutabilityContextExt<'a>}

impl<'input> StateMutabilityContextExt<'input>{
	fn new(parent: Option<Rc<dyn SolidityParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<StateMutabilityContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,StateMutabilityContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait StateMutabilityContextAttrs<'input>: SolidityParserContext<'input> + BorrowMut<StateMutabilityContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token PureKeyword
/// Returns `None` if there is no child corresponding to token PureKeyword
fn PureKeyword(&self) -> Option<Rc<TerminalNode<'input,SolidityParserContextType>>> where Self:Sized{
	self.get_token(PureKeyword, 0)
}
/// Retrieves first TerminalNode corresponding to token ConstantKeyword
/// Returns `None` if there is no child corresponding to token ConstantKeyword
fn ConstantKeyword(&self) -> Option<Rc<TerminalNode<'input,SolidityParserContextType>>> where Self:Sized{
	self.get_token(ConstantKeyword, 0)
}
/// Retrieves first TerminalNode corresponding to token ViewKeyword
/// Returns `None` if there is no child corresponding to token ViewKeyword
fn ViewKeyword(&self) -> Option<Rc<TerminalNode<'input,SolidityParserContextType>>> where Self:Sized{
	self.get_token(ViewKeyword, 0)
}
/// Retrieves first TerminalNode corresponding to token PayableKeyword
/// Returns `None` if there is no child corresponding to token PayableKeyword
fn PayableKeyword(&self) -> Option<Rc<TerminalNode<'input,SolidityParserContextType>>> where Self:Sized{
	self.get_token(PayableKeyword, 0)
}

}

impl<'input> StateMutabilityContextAttrs<'input> for StateMutabilityContext<'input>{}

impl<'input, I, H> SolidityParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn stateMutability(&mut self,)
	-> Result<Rc<StateMutabilityContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = StateMutabilityContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 92, RULE_stateMutability);
        let mut _localctx: Rc<StateMutabilityContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(642);
			_la = recog.base.input.la(1);
			if { !(((((_la - 110)) & !0x3f) == 0 && ((1usize << (_la - 110)) & ((1usize << (ConstantKeyword - 110)) | (1usize << (PayableKeyword - 110)) | (1usize << (PureKeyword - 110)) | (1usize << (ViewKeyword - 110)))) != 0)) } {
				recog.err_handler.recover_inline(&mut recog.base)?;

			}
			else {
				if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler);
			}
			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- block ----------------
pub type BlockContextAll<'input> = BlockContext<'input>;


pub type BlockContext<'input> = BaseParserRuleContext<'input,BlockContextExt<'input>>;

#[derive(Clone)]
pub struct BlockContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SolidityParserContext<'input> for BlockContext<'input>{}

impl<'input,'a> Listenable<dyn SolidityListener<'input> + 'a> for BlockContext<'input>{
	fn enter(&self,listener: &mut (dyn SolidityListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_block(self);
	}
}

impl<'input> CustomRuleContext<'input> for BlockContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SolidityParserContextType;
	fn get_rule_index(&self) -> usize { RULE_block }
	//fn type_rule_index() -> usize where Self: Sized { RULE_block }
}
antlr_rust::type_id!{BlockContextExt<'a>}

impl<'input> BlockContextExt<'input>{
	fn new(parent: Option<Rc<dyn SolidityParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<BlockContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,BlockContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait BlockContextAttrs<'input>: SolidityParserContext<'input> + BorrowMut<BlockContextExt<'input>>{

fn statement_all(&self) ->  Vec<Rc<StatementContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn statement(&self, i: usize) -> Option<Rc<StatementContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> BlockContextAttrs<'input> for BlockContext<'input>{}

impl<'input, I, H> SolidityParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn block(&mut self,)
	-> Result<Rc<BlockContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = BlockContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 94, RULE_block);
        let mut _localctx: Rc<BlockContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(644);
			recog.base.match_token(T__14,&mut recog.err_handler)?;

			recog.base.set_state(648);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while ((((_la - 6)) & !0x3f) == 0 && ((1usize << (_la - 6)) & ((1usize << (T__5 - 6)) | (1usize << (T__13 - 6)) | (1usize << (T__14 - 6)) | (1usize << (T__22 - 6)) | (1usize << (T__24 - 6)) | (1usize << (T__26 - 6)) | (1usize << (T__29 - 6)) | (1usize << (T__30 - 6)) | (1usize << (T__37 - 6)) | (1usize << (T__41 - 6)) | (1usize << (T__43 - 6)) | (1usize << (T__45 - 6)) | (1usize << (T__49 - 6)) | (1usize << (T__50 - 6)) | (1usize << (T__52 - 6)) | (1usize << (T__54 - 6)) | (1usize << (T__55 - 6)) | (1usize << (T__56 - 6)) | (1usize << (T__57 - 6)) | (1usize << (T__58 - 6)) | (1usize << (T__59 - 6)) | (1usize << (T__60 - 6)) | (1usize << (T__61 - 6)) | (1usize << (T__62 - 6)) | (1usize << (T__63 - 6)) | (1usize << (T__64 - 6)) | (1usize << (T__65 - 6)) | (1usize << (T__66 - 6)) | (1usize << (T__67 - 6)) | (1usize << (T__68 - 6)))) != 0) || ((((_la - 71)) & !0x3f) == 0 && ((1usize << (_la - 71)) & ((1usize << (T__70 - 71)) | (1usize << (T__71 - 71)) | (1usize << (T__94 - 71)) | (1usize << (Int - 71)) | (1usize << (Uint - 71)) | (1usize << (Byte - 71)) | (1usize << (Fixed - 71)) | (1usize << (Ufixed - 71)) | (1usize << (BooleanLiteral - 71)) | (1usize << (DecimalNumber - 71)) | (1usize << (HexNumber - 71)) | (1usize << (HexLiteralFragment - 71)) | (1usize << (BreakKeyword - 71)) | (1usize << (ContinueKeyword - 71)) | (1usize << (LeaveKeyword - 71)) | (1usize << (PayableKeyword - 71)) | (1usize << (TypeKeyword - 71)) | (1usize << (GlobalKeyword - 71)) | (1usize << (ConstructorKeyword - 71)) | (1usize << (ReceiveKeyword - 71)) | (1usize << (Identifier - 71)) | (1usize << (StringLiteralFragment - 71)))) != 0) {
				{
				{
				/*InvokeRule statement*/
				recog.base.set_state(645);
				recog.statement()?;

				}
				}
				recog.base.set_state(650);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(651);
			recog.base.match_token(T__16,&mut recog.err_handler)?;

			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- statement ----------------
pub type StatementContextAll<'input> = StatementContext<'input>;


pub type StatementContext<'input> = BaseParserRuleContext<'input,StatementContextExt<'input>>;

#[derive(Clone)]
pub struct StatementContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SolidityParserContext<'input> for StatementContext<'input>{}

impl<'input,'a> Listenable<dyn SolidityListener<'input> + 'a> for StatementContext<'input>{
	fn enter(&self,listener: &mut (dyn SolidityListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_statement(self);
	}
}

impl<'input> CustomRuleContext<'input> for StatementContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SolidityParserContextType;
	fn get_rule_index(&self) -> usize { RULE_statement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_statement }
}
antlr_rust::type_id!{StatementContextExt<'a>}

impl<'input> StatementContextExt<'input>{
	fn new(parent: Option<Rc<dyn SolidityParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<StatementContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,StatementContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait StatementContextAttrs<'input>: SolidityParserContext<'input> + BorrowMut<StatementContextExt<'input>>{

fn ifStatement(&self) -> Option<Rc<IfStatementContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn tryStatement(&self) -> Option<Rc<TryStatementContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn whileStatement(&self) -> Option<Rc<WhileStatementContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn forStatement(&self) -> Option<Rc<ForStatementContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn block(&self) -> Option<Rc<BlockContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn inlineAssemblyStatement(&self) -> Option<Rc<InlineAssemblyStatementContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn doWhileStatement(&self) -> Option<Rc<DoWhileStatementContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn continueStatement(&self) -> Option<Rc<ContinueStatementContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn breakStatement(&self) -> Option<Rc<BreakStatementContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn returnStatement(&self) -> Option<Rc<ReturnStatementContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn throwStatement(&self) -> Option<Rc<ThrowStatementContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn emitStatement(&self) -> Option<Rc<EmitStatementContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn simpleStatement(&self) -> Option<Rc<SimpleStatementContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn uncheckedStatement(&self) -> Option<Rc<UncheckedStatementContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn revertStatement(&self) -> Option<Rc<RevertStatementContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> StatementContextAttrs<'input> for StatementContext<'input>{}

impl<'input, I, H> SolidityParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn statement(&mut self,)
	-> Result<Rc<StatementContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = StatementContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 96, RULE_statement);
        let mut _localctx: Rc<StatementContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			recog.base.set_state(668);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(69,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule ifStatement*/
					recog.base.set_state(653);
					recog.ifStatement()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule tryStatement*/
					recog.base.set_state(654);
					recog.tryStatement()?;

					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					/*InvokeRule whileStatement*/
					recog.base.set_state(655);
					recog.whileStatement()?;

					}
				}
			,
				4 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 4);
					recog.base.enter_outer_alt(None, 4);
					{
					/*InvokeRule forStatement*/
					recog.base.set_state(656);
					recog.forStatement()?;

					}
				}
			,
				5 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 5);
					recog.base.enter_outer_alt(None, 5);
					{
					/*InvokeRule block*/
					recog.base.set_state(657);
					recog.block()?;

					}
				}
			,
				6 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 6);
					recog.base.enter_outer_alt(None, 6);
					{
					/*InvokeRule inlineAssemblyStatement*/
					recog.base.set_state(658);
					recog.inlineAssemblyStatement()?;

					}
				}
			,
				7 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 7);
					recog.base.enter_outer_alt(None, 7);
					{
					/*InvokeRule doWhileStatement*/
					recog.base.set_state(659);
					recog.doWhileStatement()?;

					}
				}
			,
				8 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 8);
					recog.base.enter_outer_alt(None, 8);
					{
					/*InvokeRule continueStatement*/
					recog.base.set_state(660);
					recog.continueStatement()?;

					}
				}
			,
				9 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 9);
					recog.base.enter_outer_alt(None, 9);
					{
					/*InvokeRule breakStatement*/
					recog.base.set_state(661);
					recog.breakStatement()?;

					}
				}
			,
				10 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 10);
					recog.base.enter_outer_alt(None, 10);
					{
					/*InvokeRule returnStatement*/
					recog.base.set_state(662);
					recog.returnStatement()?;

					}
				}
			,
				11 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 11);
					recog.base.enter_outer_alt(None, 11);
					{
					/*InvokeRule throwStatement*/
					recog.base.set_state(663);
					recog.throwStatement()?;

					}
				}
			,
				12 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 12);
					recog.base.enter_outer_alt(None, 12);
					{
					/*InvokeRule emitStatement*/
					recog.base.set_state(664);
					recog.emitStatement()?;

					}
				}
			,
				13 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 13);
					recog.base.enter_outer_alt(None, 13);
					{
					/*InvokeRule simpleStatement*/
					recog.base.set_state(665);
					recog.simpleStatement()?;

					}
				}
			,
				14 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 14);
					recog.base.enter_outer_alt(None, 14);
					{
					/*InvokeRule uncheckedStatement*/
					recog.base.set_state(666);
					recog.uncheckedStatement()?;

					}
				}
			,
				15 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 15);
					recog.base.enter_outer_alt(None, 15);
					{
					/*InvokeRule revertStatement*/
					recog.base.set_state(667);
					recog.revertStatement()?;

					}
				}

				_ => {}
			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- expressionStatement ----------------
pub type ExpressionStatementContextAll<'input> = ExpressionStatementContext<'input>;


pub type ExpressionStatementContext<'input> = BaseParserRuleContext<'input,ExpressionStatementContextExt<'input>>;

#[derive(Clone)]
pub struct ExpressionStatementContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SolidityParserContext<'input> for ExpressionStatementContext<'input>{}

impl<'input,'a> Listenable<dyn SolidityListener<'input> + 'a> for ExpressionStatementContext<'input>{
	fn enter(&self,listener: &mut (dyn SolidityListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_expressionStatement(self);
	}
}

impl<'input> CustomRuleContext<'input> for ExpressionStatementContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SolidityParserContextType;
	fn get_rule_index(&self) -> usize { RULE_expressionStatement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_expressionStatement }
}
antlr_rust::type_id!{ExpressionStatementContextExt<'a>}

impl<'input> ExpressionStatementContextExt<'input>{
	fn new(parent: Option<Rc<dyn SolidityParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ExpressionStatementContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ExpressionStatementContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ExpressionStatementContextAttrs<'input>: SolidityParserContext<'input> + BorrowMut<ExpressionStatementContextExt<'input>>{

fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> ExpressionStatementContextAttrs<'input> for ExpressionStatementContext<'input>{}

impl<'input, I, H> SolidityParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn expressionStatement(&mut self,)
	-> Result<Rc<ExpressionStatementContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ExpressionStatementContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 98, RULE_expressionStatement);
        let mut _localctx: Rc<ExpressionStatementContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule expression*/
			recog.base.set_state(670);
			recog.expression_rec(0)?;

			recog.base.set_state(671);
			recog.base.match_token(T__1,&mut recog.err_handler)?;

			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- ifStatement ----------------
pub type IfStatementContextAll<'input> = IfStatementContext<'input>;


pub type IfStatementContext<'input> = BaseParserRuleContext<'input,IfStatementContextExt<'input>>;

#[derive(Clone)]
pub struct IfStatementContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SolidityParserContext<'input> for IfStatementContext<'input>{}

impl<'input,'a> Listenable<dyn SolidityListener<'input> + 'a> for IfStatementContext<'input>{
	fn enter(&self,listener: &mut (dyn SolidityListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_ifStatement(self);
	}
}

impl<'input> CustomRuleContext<'input> for IfStatementContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SolidityParserContextType;
	fn get_rule_index(&self) -> usize { RULE_ifStatement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_ifStatement }
}
antlr_rust::type_id!{IfStatementContextExt<'a>}

impl<'input> IfStatementContextExt<'input>{
	fn new(parent: Option<Rc<dyn SolidityParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<IfStatementContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,IfStatementContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait IfStatementContextAttrs<'input>: SolidityParserContext<'input> + BorrowMut<IfStatementContextExt<'input>>{

fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn statement_all(&self) ->  Vec<Rc<StatementContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn statement(&self, i: usize) -> Option<Rc<StatementContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> IfStatementContextAttrs<'input> for IfStatementContext<'input>{}

impl<'input, I, H> SolidityParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn ifStatement(&mut self,)
	-> Result<Rc<IfStatementContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = IfStatementContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 100, RULE_ifStatement);
        let mut _localctx: Rc<IfStatementContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(673);
			recog.base.match_token(T__50,&mut recog.err_handler)?;

			recog.base.set_state(674);
			recog.base.match_token(T__22,&mut recog.err_handler)?;

			/*InvokeRule expression*/
			recog.base.set_state(675);
			recog.expression_rec(0)?;

			recog.base.set_state(676);
			recog.base.match_token(T__23,&mut recog.err_handler)?;

			/*InvokeRule statement*/
			recog.base.set_state(677);
			recog.statement()?;

			recog.base.set_state(680);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(70,&mut recog.base)? {
				x if x == 1=>{
					{
					recog.base.set_state(678);
					recog.base.match_token(T__51,&mut recog.err_handler)?;

					/*InvokeRule statement*/
					recog.base.set_state(679);
					recog.statement()?;

					}
				}

				_ => {}
			}
			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- tryStatement ----------------
pub type TryStatementContextAll<'input> = TryStatementContext<'input>;


pub type TryStatementContext<'input> = BaseParserRuleContext<'input,TryStatementContextExt<'input>>;

#[derive(Clone)]
pub struct TryStatementContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SolidityParserContext<'input> for TryStatementContext<'input>{}

impl<'input,'a> Listenable<dyn SolidityListener<'input> + 'a> for TryStatementContext<'input>{
	fn enter(&self,listener: &mut (dyn SolidityListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_tryStatement(self);
	}
}

impl<'input> CustomRuleContext<'input> for TryStatementContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SolidityParserContextType;
	fn get_rule_index(&self) -> usize { RULE_tryStatement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_tryStatement }
}
antlr_rust::type_id!{TryStatementContextExt<'a>}

impl<'input> TryStatementContextExt<'input>{
	fn new(parent: Option<Rc<dyn SolidityParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<TryStatementContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,TryStatementContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait TryStatementContextAttrs<'input>: SolidityParserContext<'input> + BorrowMut<TryStatementContextExt<'input>>{

fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn block(&self) -> Option<Rc<BlockContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn returnParameters(&self) -> Option<Rc<ReturnParametersContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn catchClause_all(&self) ->  Vec<Rc<CatchClauseContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn catchClause(&self, i: usize) -> Option<Rc<CatchClauseContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> TryStatementContextAttrs<'input> for TryStatementContext<'input>{}

impl<'input, I, H> SolidityParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn tryStatement(&mut self,)
	-> Result<Rc<TryStatementContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = TryStatementContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 102, RULE_tryStatement);
        let mut _localctx: Rc<TryStatementContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(682);
			recog.base.match_token(T__52,&mut recog.err_handler)?;

			/*InvokeRule expression*/
			recog.base.set_state(683);
			recog.expression_rec(0)?;

			recog.base.set_state(685);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==T__38 {
				{
				/*InvokeRule returnParameters*/
				recog.base.set_state(684);
				recog.returnParameters()?;

				}
			}

			/*InvokeRule block*/
			recog.base.set_state(687);
			recog.block()?;

			recog.base.set_state(689); 
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			loop {
				{
				{
				/*InvokeRule catchClause*/
				recog.base.set_state(688);
				recog.catchClause()?;

				}
				}
				recog.base.set_state(691); 
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				if !(_la==T__53) {break}
			}
			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- catchClause ----------------
pub type CatchClauseContextAll<'input> = CatchClauseContext<'input>;


pub type CatchClauseContext<'input> = BaseParserRuleContext<'input,CatchClauseContextExt<'input>>;

#[derive(Clone)]
pub struct CatchClauseContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SolidityParserContext<'input> for CatchClauseContext<'input>{}

impl<'input,'a> Listenable<dyn SolidityListener<'input> + 'a> for CatchClauseContext<'input>{
	fn enter(&self,listener: &mut (dyn SolidityListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_catchClause(self);
	}
}

impl<'input> CustomRuleContext<'input> for CatchClauseContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SolidityParserContextType;
	fn get_rule_index(&self) -> usize { RULE_catchClause }
	//fn type_rule_index() -> usize where Self: Sized { RULE_catchClause }
}
antlr_rust::type_id!{CatchClauseContextExt<'a>}

impl<'input> CatchClauseContextExt<'input>{
	fn new(parent: Option<Rc<dyn SolidityParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<CatchClauseContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,CatchClauseContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait CatchClauseContextAttrs<'input>: SolidityParserContext<'input> + BorrowMut<CatchClauseContextExt<'input>>{

fn block(&self) -> Option<Rc<BlockContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn parameterList(&self) -> Option<Rc<ParameterListContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn identifier(&self) -> Option<Rc<IdentifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> CatchClauseContextAttrs<'input> for CatchClauseContext<'input>{}

impl<'input, I, H> SolidityParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn catchClause(&mut self,)
	-> Result<Rc<CatchClauseContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = CatchClauseContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 104, RULE_catchClause);
        let mut _localctx: Rc<CatchClauseContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(693);
			recog.base.match_token(T__53,&mut recog.err_handler)?;

			recog.base.set_state(698);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if (((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << T__13) | (1usize << T__22) | (1usize << T__24) | (1usize << T__43) | (1usize << T__49) | (1usize << T__61))) != 0) || ((((_la - 95)) & !0x3f) == 0 && ((1usize << (_la - 95)) & ((1usize << (T__94 - 95)) | (1usize << (LeaveKeyword - 95)) | (1usize << (PayableKeyword - 95)) | (1usize << (GlobalKeyword - 95)) | (1usize << (ConstructorKeyword - 95)) | (1usize << (ReceiveKeyword - 95)) | (1usize << (Identifier - 95)))) != 0) {
				{
				recog.base.set_state(695);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				if (((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << T__13) | (1usize << T__24) | (1usize << T__43) | (1usize << T__49) | (1usize << T__61))) != 0) || ((((_la - 95)) & !0x3f) == 0 && ((1usize << (_la - 95)) & ((1usize << (T__94 - 95)) | (1usize << (LeaveKeyword - 95)) | (1usize << (PayableKeyword - 95)) | (1usize << (GlobalKeyword - 95)) | (1usize << (ConstructorKeyword - 95)) | (1usize << (ReceiveKeyword - 95)) | (1usize << (Identifier - 95)))) != 0) {
					{
					/*InvokeRule identifier*/
					recog.base.set_state(694);
					recog.identifier()?;

					}
				}

				/*InvokeRule parameterList*/
				recog.base.set_state(697);
				recog.parameterList()?;

				}
			}

			/*InvokeRule block*/
			recog.base.set_state(700);
			recog.block()?;

			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- whileStatement ----------------
pub type WhileStatementContextAll<'input> = WhileStatementContext<'input>;


pub type WhileStatementContext<'input> = BaseParserRuleContext<'input,WhileStatementContextExt<'input>>;

#[derive(Clone)]
pub struct WhileStatementContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SolidityParserContext<'input> for WhileStatementContext<'input>{}

impl<'input,'a> Listenable<dyn SolidityListener<'input> + 'a> for WhileStatementContext<'input>{
	fn enter(&self,listener: &mut (dyn SolidityListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_whileStatement(self);
	}
}

impl<'input> CustomRuleContext<'input> for WhileStatementContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SolidityParserContextType;
	fn get_rule_index(&self) -> usize { RULE_whileStatement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_whileStatement }
}
antlr_rust::type_id!{WhileStatementContextExt<'a>}

impl<'input> WhileStatementContextExt<'input>{
	fn new(parent: Option<Rc<dyn SolidityParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<WhileStatementContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,WhileStatementContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait WhileStatementContextAttrs<'input>: SolidityParserContext<'input> + BorrowMut<WhileStatementContextExt<'input>>{

fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn statement(&self) -> Option<Rc<StatementContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> WhileStatementContextAttrs<'input> for WhileStatementContext<'input>{}

impl<'input, I, H> SolidityParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn whileStatement(&mut self,)
	-> Result<Rc<WhileStatementContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = WhileStatementContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 106, RULE_whileStatement);
        let mut _localctx: Rc<WhileStatementContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(702);
			recog.base.match_token(T__54,&mut recog.err_handler)?;

			recog.base.set_state(703);
			recog.base.match_token(T__22,&mut recog.err_handler)?;

			/*InvokeRule expression*/
			recog.base.set_state(704);
			recog.expression_rec(0)?;

			recog.base.set_state(705);
			recog.base.match_token(T__23,&mut recog.err_handler)?;

			/*InvokeRule statement*/
			recog.base.set_state(706);
			recog.statement()?;

			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- simpleStatement ----------------
pub type SimpleStatementContextAll<'input> = SimpleStatementContext<'input>;


pub type SimpleStatementContext<'input> = BaseParserRuleContext<'input,SimpleStatementContextExt<'input>>;

#[derive(Clone)]
pub struct SimpleStatementContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SolidityParserContext<'input> for SimpleStatementContext<'input>{}

impl<'input,'a> Listenable<dyn SolidityListener<'input> + 'a> for SimpleStatementContext<'input>{
	fn enter(&self,listener: &mut (dyn SolidityListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_simpleStatement(self);
	}
}

impl<'input> CustomRuleContext<'input> for SimpleStatementContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SolidityParserContextType;
	fn get_rule_index(&self) -> usize { RULE_simpleStatement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_simpleStatement }
}
antlr_rust::type_id!{SimpleStatementContextExt<'a>}

impl<'input> SimpleStatementContextExt<'input>{
	fn new(parent: Option<Rc<dyn SolidityParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<SimpleStatementContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,SimpleStatementContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait SimpleStatementContextAttrs<'input>: SolidityParserContext<'input> + BorrowMut<SimpleStatementContextExt<'input>>{

fn variableDeclarationStatement(&self) -> Option<Rc<VariableDeclarationStatementContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn expressionStatement(&self) -> Option<Rc<ExpressionStatementContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> SimpleStatementContextAttrs<'input> for SimpleStatementContext<'input>{}

impl<'input, I, H> SolidityParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn simpleStatement(&mut self,)
	-> Result<Rc<SimpleStatementContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = SimpleStatementContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 108, RULE_simpleStatement);
        let mut _localctx: Rc<SimpleStatementContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(710);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(75,&mut recog.base)? {
				1 =>{
					{
					/*InvokeRule variableDeclarationStatement*/
					recog.base.set_state(708);
					recog.variableDeclarationStatement()?;

					}
				}
			,
				2 =>{
					{
					/*InvokeRule expressionStatement*/
					recog.base.set_state(709);
					recog.expressionStatement()?;

					}
				}

				_ => {}
			}
			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- uncheckedStatement ----------------
pub type UncheckedStatementContextAll<'input> = UncheckedStatementContext<'input>;


pub type UncheckedStatementContext<'input> = BaseParserRuleContext<'input,UncheckedStatementContextExt<'input>>;

#[derive(Clone)]
pub struct UncheckedStatementContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SolidityParserContext<'input> for UncheckedStatementContext<'input>{}

impl<'input,'a> Listenable<dyn SolidityListener<'input> + 'a> for UncheckedStatementContext<'input>{
	fn enter(&self,listener: &mut (dyn SolidityListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_uncheckedStatement(self);
	}
}

impl<'input> CustomRuleContext<'input> for UncheckedStatementContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SolidityParserContextType;
	fn get_rule_index(&self) -> usize { RULE_uncheckedStatement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_uncheckedStatement }
}
antlr_rust::type_id!{UncheckedStatementContextExt<'a>}

impl<'input> UncheckedStatementContextExt<'input>{
	fn new(parent: Option<Rc<dyn SolidityParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<UncheckedStatementContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,UncheckedStatementContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait UncheckedStatementContextAttrs<'input>: SolidityParserContext<'input> + BorrowMut<UncheckedStatementContextExt<'input>>{

fn block(&self) -> Option<Rc<BlockContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> UncheckedStatementContextAttrs<'input> for UncheckedStatementContext<'input>{}

impl<'input, I, H> SolidityParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn uncheckedStatement(&mut self,)
	-> Result<Rc<UncheckedStatementContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = UncheckedStatementContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 110, RULE_uncheckedStatement);
        let mut _localctx: Rc<UncheckedStatementContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(712);
			recog.base.match_token(T__55,&mut recog.err_handler)?;

			/*InvokeRule block*/
			recog.base.set_state(713);
			recog.block()?;

			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- forStatement ----------------
pub type ForStatementContextAll<'input> = ForStatementContext<'input>;


pub type ForStatementContext<'input> = BaseParserRuleContext<'input,ForStatementContextExt<'input>>;

#[derive(Clone)]
pub struct ForStatementContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SolidityParserContext<'input> for ForStatementContext<'input>{}

impl<'input,'a> Listenable<dyn SolidityListener<'input> + 'a> for ForStatementContext<'input>{
	fn enter(&self,listener: &mut (dyn SolidityListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_forStatement(self);
	}
}

impl<'input> CustomRuleContext<'input> for ForStatementContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SolidityParserContextType;
	fn get_rule_index(&self) -> usize { RULE_forStatement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_forStatement }
}
antlr_rust::type_id!{ForStatementContextExt<'a>}

impl<'input> ForStatementContextExt<'input>{
	fn new(parent: Option<Rc<dyn SolidityParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ForStatementContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ForStatementContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ForStatementContextAttrs<'input>: SolidityParserContext<'input> + BorrowMut<ForStatementContextExt<'input>>{

fn statement(&self) -> Option<Rc<StatementContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn simpleStatement(&self) -> Option<Rc<SimpleStatementContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn expressionStatement(&self) -> Option<Rc<ExpressionStatementContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> ForStatementContextAttrs<'input> for ForStatementContext<'input>{}

impl<'input, I, H> SolidityParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn forStatement(&mut self,)
	-> Result<Rc<ForStatementContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ForStatementContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 112, RULE_forStatement);
        let mut _localctx: Rc<ForStatementContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(715);
			recog.base.match_token(T__26,&mut recog.err_handler)?;

			recog.base.set_state(716);
			recog.base.match_token(T__22,&mut recog.err_handler)?;

			recog.base.set_state(719);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 T__5 | T__13 | T__22 | T__24 | T__29 | T__30 | T__37 | T__41 | T__43 |
			 T__45 | T__49 | T__61 | T__62 | T__63 | T__64 | T__65 | T__66 | T__67 |
			 T__68 | T__70 | T__71 | T__94 | Int | Uint | Byte | Fixed | Ufixed |
			 BooleanLiteral | DecimalNumber | HexNumber | HexLiteralFragment | LeaveKeyword |
			 PayableKeyword | TypeKeyword | GlobalKeyword | ConstructorKeyword | ReceiveKeyword |
			 Identifier | StringLiteralFragment 
				=> {
					{
					/*InvokeRule simpleStatement*/
					recog.base.set_state(717);
					recog.simpleStatement()?;

					}
				}

			 T__1 
				=> {
					{
					recog.base.set_state(718);
					recog.base.match_token(T__1,&mut recog.err_handler)?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			recog.base.set_state(723);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 T__5 | T__13 | T__22 | T__24 | T__29 | T__30 | T__37 | T__41 | T__43 |
			 T__45 | T__49 | T__61 | T__62 | T__63 | T__64 | T__65 | T__66 | T__67 |
			 T__68 | T__70 | T__71 | T__94 | Int | Uint | Byte | Fixed | Ufixed |
			 BooleanLiteral | DecimalNumber | HexNumber | HexLiteralFragment | LeaveKeyword |
			 PayableKeyword | TypeKeyword | GlobalKeyword | ConstructorKeyword | ReceiveKeyword |
			 Identifier | StringLiteralFragment 
				=> {
					{
					/*InvokeRule expressionStatement*/
					recog.base.set_state(721);
					recog.expressionStatement()?;

					}
				}

			 T__1 
				=> {
					{
					recog.base.set_state(722);
					recog.base.match_token(T__1,&mut recog.err_handler)?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			recog.base.set_state(726);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if ((((_la - 6)) & !0x3f) == 0 && ((1usize << (_la - 6)) & ((1usize << (T__5 - 6)) | (1usize << (T__13 - 6)) | (1usize << (T__22 - 6)) | (1usize << (T__24 - 6)) | (1usize << (T__29 - 6)) | (1usize << (T__30 - 6)) | (1usize << (T__37 - 6)) | (1usize << (T__41 - 6)) | (1usize << (T__43 - 6)) | (1usize << (T__45 - 6)) | (1usize << (T__49 - 6)) | (1usize << (T__61 - 6)) | (1usize << (T__62 - 6)) | (1usize << (T__63 - 6)) | (1usize << (T__64 - 6)) | (1usize << (T__65 - 6)) | (1usize << (T__66 - 6)) | (1usize << (T__67 - 6)) | (1usize << (T__68 - 6)))) != 0) || ((((_la - 71)) & !0x3f) == 0 && ((1usize << (_la - 71)) & ((1usize << (T__70 - 71)) | (1usize << (T__71 - 71)) | (1usize << (T__94 - 71)) | (1usize << (Int - 71)) | (1usize << (Uint - 71)) | (1usize << (Byte - 71)) | (1usize << (Fixed - 71)) | (1usize << (Ufixed - 71)) | (1usize << (BooleanLiteral - 71)) | (1usize << (DecimalNumber - 71)) | (1usize << (HexNumber - 71)) | (1usize << (HexLiteralFragment - 71)) | (1usize << (LeaveKeyword - 71)) | (1usize << (PayableKeyword - 71)) | (1usize << (TypeKeyword - 71)) | (1usize << (GlobalKeyword - 71)) | (1usize << (ConstructorKeyword - 71)) | (1usize << (ReceiveKeyword - 71)) | (1usize << (Identifier - 71)) | (1usize << (StringLiteralFragment - 71)))) != 0) {
				{
				/*InvokeRule expression*/
				recog.base.set_state(725);
				recog.expression_rec(0)?;

				}
			}

			recog.base.set_state(728);
			recog.base.match_token(T__23,&mut recog.err_handler)?;

			/*InvokeRule statement*/
			recog.base.set_state(729);
			recog.statement()?;

			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- inlineAssemblyStatement ----------------
pub type InlineAssemblyStatementContextAll<'input> = InlineAssemblyStatementContext<'input>;


pub type InlineAssemblyStatementContext<'input> = BaseParserRuleContext<'input,InlineAssemblyStatementContextExt<'input>>;

#[derive(Clone)]
pub struct InlineAssemblyStatementContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SolidityParserContext<'input> for InlineAssemblyStatementContext<'input>{}

impl<'input,'a> Listenable<dyn SolidityListener<'input> + 'a> for InlineAssemblyStatementContext<'input>{
	fn enter(&self,listener: &mut (dyn SolidityListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_inlineAssemblyStatement(self);
	}
}

impl<'input> CustomRuleContext<'input> for InlineAssemblyStatementContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SolidityParserContextType;
	fn get_rule_index(&self) -> usize { RULE_inlineAssemblyStatement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_inlineAssemblyStatement }
}
antlr_rust::type_id!{InlineAssemblyStatementContextExt<'a>}

impl<'input> InlineAssemblyStatementContextExt<'input>{
	fn new(parent: Option<Rc<dyn SolidityParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<InlineAssemblyStatementContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,InlineAssemblyStatementContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait InlineAssemblyStatementContextAttrs<'input>: SolidityParserContext<'input> + BorrowMut<InlineAssemblyStatementContextExt<'input>>{

fn assemblyBlock(&self) -> Option<Rc<AssemblyBlockContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token StringLiteralFragment
/// Returns `None` if there is no child corresponding to token StringLiteralFragment
fn StringLiteralFragment(&self) -> Option<Rc<TerminalNode<'input,SolidityParserContextType>>> where Self:Sized{
	self.get_token(StringLiteralFragment, 0)
}
fn inlineAssemblyStatementFlag(&self) -> Option<Rc<InlineAssemblyStatementFlagContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> InlineAssemblyStatementContextAttrs<'input> for InlineAssemblyStatementContext<'input>{}

impl<'input, I, H> SolidityParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn inlineAssemblyStatement(&mut self,)
	-> Result<Rc<InlineAssemblyStatementContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = InlineAssemblyStatementContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 114, RULE_inlineAssemblyStatement);
        let mut _localctx: Rc<InlineAssemblyStatementContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(731);
			recog.base.match_token(T__56,&mut recog.err_handler)?;

			recog.base.set_state(733);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==StringLiteralFragment {
				{
				recog.base.set_state(732);
				recog.base.match_token(StringLiteralFragment,&mut recog.err_handler)?;

				}
			}

			recog.base.set_state(739);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==T__22 {
				{
				recog.base.set_state(735);
				recog.base.match_token(T__22,&mut recog.err_handler)?;

				/*InvokeRule inlineAssemblyStatementFlag*/
				recog.base.set_state(736);
				recog.inlineAssemblyStatementFlag()?;

				recog.base.set_state(737);
				recog.base.match_token(T__23,&mut recog.err_handler)?;

				}
			}

			/*InvokeRule assemblyBlock*/
			recog.base.set_state(741);
			recog.assemblyBlock()?;

			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- inlineAssemblyStatementFlag ----------------
pub type InlineAssemblyStatementFlagContextAll<'input> = InlineAssemblyStatementFlagContext<'input>;


pub type InlineAssemblyStatementFlagContext<'input> = BaseParserRuleContext<'input,InlineAssemblyStatementFlagContextExt<'input>>;

#[derive(Clone)]
pub struct InlineAssemblyStatementFlagContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SolidityParserContext<'input> for InlineAssemblyStatementFlagContext<'input>{}

impl<'input,'a> Listenable<dyn SolidityListener<'input> + 'a> for InlineAssemblyStatementFlagContext<'input>{
	fn enter(&self,listener: &mut (dyn SolidityListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_inlineAssemblyStatementFlag(self);
	}
}

impl<'input> CustomRuleContext<'input> for InlineAssemblyStatementFlagContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SolidityParserContextType;
	fn get_rule_index(&self) -> usize { RULE_inlineAssemblyStatementFlag }
	//fn type_rule_index() -> usize where Self: Sized { RULE_inlineAssemblyStatementFlag }
}
antlr_rust::type_id!{InlineAssemblyStatementFlagContextExt<'a>}

impl<'input> InlineAssemblyStatementFlagContextExt<'input>{
	fn new(parent: Option<Rc<dyn SolidityParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<InlineAssemblyStatementFlagContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,InlineAssemblyStatementFlagContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait InlineAssemblyStatementFlagContextAttrs<'input>: SolidityParserContext<'input> + BorrowMut<InlineAssemblyStatementFlagContextExt<'input>>{

fn stringLiteral(&self) -> Option<Rc<StringLiteralContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> InlineAssemblyStatementFlagContextAttrs<'input> for InlineAssemblyStatementFlagContext<'input>{}

impl<'input, I, H> SolidityParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn inlineAssemblyStatementFlag(&mut self,)
	-> Result<Rc<InlineAssemblyStatementFlagContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = InlineAssemblyStatementFlagContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 116, RULE_inlineAssemblyStatementFlag);
        let mut _localctx: Rc<InlineAssemblyStatementFlagContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule stringLiteral*/
			recog.base.set_state(743);
			recog.stringLiteral()?;

			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- doWhileStatement ----------------
pub type DoWhileStatementContextAll<'input> = DoWhileStatementContext<'input>;


pub type DoWhileStatementContext<'input> = BaseParserRuleContext<'input,DoWhileStatementContextExt<'input>>;

#[derive(Clone)]
pub struct DoWhileStatementContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SolidityParserContext<'input> for DoWhileStatementContext<'input>{}

impl<'input,'a> Listenable<dyn SolidityListener<'input> + 'a> for DoWhileStatementContext<'input>{
	fn enter(&self,listener: &mut (dyn SolidityListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_doWhileStatement(self);
	}
}

impl<'input> CustomRuleContext<'input> for DoWhileStatementContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SolidityParserContextType;
	fn get_rule_index(&self) -> usize { RULE_doWhileStatement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_doWhileStatement }
}
antlr_rust::type_id!{DoWhileStatementContextExt<'a>}

impl<'input> DoWhileStatementContextExt<'input>{
	fn new(parent: Option<Rc<dyn SolidityParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<DoWhileStatementContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,DoWhileStatementContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait DoWhileStatementContextAttrs<'input>: SolidityParserContext<'input> + BorrowMut<DoWhileStatementContextExt<'input>>{

fn statement(&self) -> Option<Rc<StatementContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> DoWhileStatementContextAttrs<'input> for DoWhileStatementContext<'input>{}

impl<'input, I, H> SolidityParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn doWhileStatement(&mut self,)
	-> Result<Rc<DoWhileStatementContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = DoWhileStatementContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 118, RULE_doWhileStatement);
        let mut _localctx: Rc<DoWhileStatementContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(745);
			recog.base.match_token(T__57,&mut recog.err_handler)?;

			/*InvokeRule statement*/
			recog.base.set_state(746);
			recog.statement()?;

			recog.base.set_state(747);
			recog.base.match_token(T__54,&mut recog.err_handler)?;

			recog.base.set_state(748);
			recog.base.match_token(T__22,&mut recog.err_handler)?;

			/*InvokeRule expression*/
			recog.base.set_state(749);
			recog.expression_rec(0)?;

			recog.base.set_state(750);
			recog.base.match_token(T__23,&mut recog.err_handler)?;

			recog.base.set_state(751);
			recog.base.match_token(T__1,&mut recog.err_handler)?;

			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- continueStatement ----------------
pub type ContinueStatementContextAll<'input> = ContinueStatementContext<'input>;


pub type ContinueStatementContext<'input> = BaseParserRuleContext<'input,ContinueStatementContextExt<'input>>;

#[derive(Clone)]
pub struct ContinueStatementContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SolidityParserContext<'input> for ContinueStatementContext<'input>{}

impl<'input,'a> Listenable<dyn SolidityListener<'input> + 'a> for ContinueStatementContext<'input>{
	fn enter(&self,listener: &mut (dyn SolidityListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_continueStatement(self);
	}
}

impl<'input> CustomRuleContext<'input> for ContinueStatementContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SolidityParserContextType;
	fn get_rule_index(&self) -> usize { RULE_continueStatement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_continueStatement }
}
antlr_rust::type_id!{ContinueStatementContextExt<'a>}

impl<'input> ContinueStatementContextExt<'input>{
	fn new(parent: Option<Rc<dyn SolidityParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ContinueStatementContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ContinueStatementContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ContinueStatementContextAttrs<'input>: SolidityParserContext<'input> + BorrowMut<ContinueStatementContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token ContinueKeyword
/// Returns `None` if there is no child corresponding to token ContinueKeyword
fn ContinueKeyword(&self) -> Option<Rc<TerminalNode<'input,SolidityParserContextType>>> where Self:Sized{
	self.get_token(ContinueKeyword, 0)
}

}

impl<'input> ContinueStatementContextAttrs<'input> for ContinueStatementContext<'input>{}

impl<'input, I, H> SolidityParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn continueStatement(&mut self,)
	-> Result<Rc<ContinueStatementContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ContinueStatementContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 120, RULE_continueStatement);
        let mut _localctx: Rc<ContinueStatementContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(753);
			recog.base.match_token(ContinueKeyword,&mut recog.err_handler)?;

			recog.base.set_state(754);
			recog.base.match_token(T__1,&mut recog.err_handler)?;

			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- breakStatement ----------------
pub type BreakStatementContextAll<'input> = BreakStatementContext<'input>;


pub type BreakStatementContext<'input> = BaseParserRuleContext<'input,BreakStatementContextExt<'input>>;

#[derive(Clone)]
pub struct BreakStatementContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SolidityParserContext<'input> for BreakStatementContext<'input>{}

impl<'input,'a> Listenable<dyn SolidityListener<'input> + 'a> for BreakStatementContext<'input>{
	fn enter(&self,listener: &mut (dyn SolidityListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_breakStatement(self);
	}
}

impl<'input> CustomRuleContext<'input> for BreakStatementContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SolidityParserContextType;
	fn get_rule_index(&self) -> usize { RULE_breakStatement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_breakStatement }
}
antlr_rust::type_id!{BreakStatementContextExt<'a>}

impl<'input> BreakStatementContextExt<'input>{
	fn new(parent: Option<Rc<dyn SolidityParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<BreakStatementContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,BreakStatementContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait BreakStatementContextAttrs<'input>: SolidityParserContext<'input> + BorrowMut<BreakStatementContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token BreakKeyword
/// Returns `None` if there is no child corresponding to token BreakKeyword
fn BreakKeyword(&self) -> Option<Rc<TerminalNode<'input,SolidityParserContextType>>> where Self:Sized{
	self.get_token(BreakKeyword, 0)
}

}

impl<'input> BreakStatementContextAttrs<'input> for BreakStatementContext<'input>{}

impl<'input, I, H> SolidityParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn breakStatement(&mut self,)
	-> Result<Rc<BreakStatementContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = BreakStatementContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 122, RULE_breakStatement);
        let mut _localctx: Rc<BreakStatementContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(756);
			recog.base.match_token(BreakKeyword,&mut recog.err_handler)?;

			recog.base.set_state(757);
			recog.base.match_token(T__1,&mut recog.err_handler)?;

			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- returnStatement ----------------
pub type ReturnStatementContextAll<'input> = ReturnStatementContext<'input>;


pub type ReturnStatementContext<'input> = BaseParserRuleContext<'input,ReturnStatementContextExt<'input>>;

#[derive(Clone)]
pub struct ReturnStatementContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SolidityParserContext<'input> for ReturnStatementContext<'input>{}

impl<'input,'a> Listenable<dyn SolidityListener<'input> + 'a> for ReturnStatementContext<'input>{
	fn enter(&self,listener: &mut (dyn SolidityListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_returnStatement(self);
	}
}

impl<'input> CustomRuleContext<'input> for ReturnStatementContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SolidityParserContextType;
	fn get_rule_index(&self) -> usize { RULE_returnStatement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_returnStatement }
}
antlr_rust::type_id!{ReturnStatementContextExt<'a>}

impl<'input> ReturnStatementContextExt<'input>{
	fn new(parent: Option<Rc<dyn SolidityParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ReturnStatementContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ReturnStatementContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ReturnStatementContextAttrs<'input>: SolidityParserContext<'input> + BorrowMut<ReturnStatementContextExt<'input>>{

fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> ReturnStatementContextAttrs<'input> for ReturnStatementContext<'input>{}

impl<'input, I, H> SolidityParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn returnStatement(&mut self,)
	-> Result<Rc<ReturnStatementContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ReturnStatementContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 124, RULE_returnStatement);
        let mut _localctx: Rc<ReturnStatementContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(759);
			recog.base.match_token(T__58,&mut recog.err_handler)?;

			recog.base.set_state(761);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if ((((_la - 6)) & !0x3f) == 0 && ((1usize << (_la - 6)) & ((1usize << (T__5 - 6)) | (1usize << (T__13 - 6)) | (1usize << (T__22 - 6)) | (1usize << (T__24 - 6)) | (1usize << (T__29 - 6)) | (1usize << (T__30 - 6)) | (1usize << (T__37 - 6)) | (1usize << (T__41 - 6)) | (1usize << (T__43 - 6)) | (1usize << (T__45 - 6)) | (1usize << (T__49 - 6)) | (1usize << (T__61 - 6)) | (1usize << (T__62 - 6)) | (1usize << (T__63 - 6)) | (1usize << (T__64 - 6)) | (1usize << (T__65 - 6)) | (1usize << (T__66 - 6)) | (1usize << (T__67 - 6)) | (1usize << (T__68 - 6)))) != 0) || ((((_la - 71)) & !0x3f) == 0 && ((1usize << (_la - 71)) & ((1usize << (T__70 - 71)) | (1usize << (T__71 - 71)) | (1usize << (T__94 - 71)) | (1usize << (Int - 71)) | (1usize << (Uint - 71)) | (1usize << (Byte - 71)) | (1usize << (Fixed - 71)) | (1usize << (Ufixed - 71)) | (1usize << (BooleanLiteral - 71)) | (1usize << (DecimalNumber - 71)) | (1usize << (HexNumber - 71)) | (1usize << (HexLiteralFragment - 71)) | (1usize << (LeaveKeyword - 71)) | (1usize << (PayableKeyword - 71)) | (1usize << (TypeKeyword - 71)) | (1usize << (GlobalKeyword - 71)) | (1usize << (ConstructorKeyword - 71)) | (1usize << (ReceiveKeyword - 71)) | (1usize << (Identifier - 71)) | (1usize << (StringLiteralFragment - 71)))) != 0) {
				{
				/*InvokeRule expression*/
				recog.base.set_state(760);
				recog.expression_rec(0)?;

				}
			}

			recog.base.set_state(763);
			recog.base.match_token(T__1,&mut recog.err_handler)?;

			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- throwStatement ----------------
pub type ThrowStatementContextAll<'input> = ThrowStatementContext<'input>;


pub type ThrowStatementContext<'input> = BaseParserRuleContext<'input,ThrowStatementContextExt<'input>>;

#[derive(Clone)]
pub struct ThrowStatementContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SolidityParserContext<'input> for ThrowStatementContext<'input>{}

impl<'input,'a> Listenable<dyn SolidityListener<'input> + 'a> for ThrowStatementContext<'input>{
	fn enter(&self,listener: &mut (dyn SolidityListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_throwStatement(self);
	}
}

impl<'input> CustomRuleContext<'input> for ThrowStatementContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SolidityParserContextType;
	fn get_rule_index(&self) -> usize { RULE_throwStatement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_throwStatement }
}
antlr_rust::type_id!{ThrowStatementContextExt<'a>}

impl<'input> ThrowStatementContextExt<'input>{
	fn new(parent: Option<Rc<dyn SolidityParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ThrowStatementContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ThrowStatementContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ThrowStatementContextAttrs<'input>: SolidityParserContext<'input> + BorrowMut<ThrowStatementContextExt<'input>>{


}

impl<'input> ThrowStatementContextAttrs<'input> for ThrowStatementContext<'input>{}

impl<'input, I, H> SolidityParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn throwStatement(&mut self,)
	-> Result<Rc<ThrowStatementContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ThrowStatementContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 126, RULE_throwStatement);
        let mut _localctx: Rc<ThrowStatementContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(765);
			recog.base.match_token(T__59,&mut recog.err_handler)?;

			recog.base.set_state(766);
			recog.base.match_token(T__1,&mut recog.err_handler)?;

			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- emitStatement ----------------
pub type EmitStatementContextAll<'input> = EmitStatementContext<'input>;


pub type EmitStatementContext<'input> = BaseParserRuleContext<'input,EmitStatementContextExt<'input>>;

#[derive(Clone)]
pub struct EmitStatementContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SolidityParserContext<'input> for EmitStatementContext<'input>{}

impl<'input,'a> Listenable<dyn SolidityListener<'input> + 'a> for EmitStatementContext<'input>{
	fn enter(&self,listener: &mut (dyn SolidityListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_emitStatement(self);
	}
}

impl<'input> CustomRuleContext<'input> for EmitStatementContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SolidityParserContextType;
	fn get_rule_index(&self) -> usize { RULE_emitStatement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_emitStatement }
}
antlr_rust::type_id!{EmitStatementContextExt<'a>}

impl<'input> EmitStatementContextExt<'input>{
	fn new(parent: Option<Rc<dyn SolidityParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<EmitStatementContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,EmitStatementContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait EmitStatementContextAttrs<'input>: SolidityParserContext<'input> + BorrowMut<EmitStatementContextExt<'input>>{

fn functionCall(&self) -> Option<Rc<FunctionCallContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> EmitStatementContextAttrs<'input> for EmitStatementContext<'input>{}

impl<'input, I, H> SolidityParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn emitStatement(&mut self,)
	-> Result<Rc<EmitStatementContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = EmitStatementContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 128, RULE_emitStatement);
        let mut _localctx: Rc<EmitStatementContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(768);
			recog.base.match_token(T__60,&mut recog.err_handler)?;

			/*InvokeRule functionCall*/
			recog.base.set_state(769);
			recog.functionCall()?;

			recog.base.set_state(770);
			recog.base.match_token(T__1,&mut recog.err_handler)?;

			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- revertStatement ----------------
pub type RevertStatementContextAll<'input> = RevertStatementContext<'input>;


pub type RevertStatementContext<'input> = BaseParserRuleContext<'input,RevertStatementContextExt<'input>>;

#[derive(Clone)]
pub struct RevertStatementContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SolidityParserContext<'input> for RevertStatementContext<'input>{}

impl<'input,'a> Listenable<dyn SolidityListener<'input> + 'a> for RevertStatementContext<'input>{
	fn enter(&self,listener: &mut (dyn SolidityListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_revertStatement(self);
	}
}

impl<'input> CustomRuleContext<'input> for RevertStatementContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SolidityParserContextType;
	fn get_rule_index(&self) -> usize { RULE_revertStatement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_revertStatement }
}
antlr_rust::type_id!{RevertStatementContextExt<'a>}

impl<'input> RevertStatementContextExt<'input>{
	fn new(parent: Option<Rc<dyn SolidityParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<RevertStatementContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,RevertStatementContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait RevertStatementContextAttrs<'input>: SolidityParserContext<'input> + BorrowMut<RevertStatementContextExt<'input>>{

fn functionCall(&self) -> Option<Rc<FunctionCallContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> RevertStatementContextAttrs<'input> for RevertStatementContext<'input>{}

impl<'input, I, H> SolidityParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn revertStatement(&mut self,)
	-> Result<Rc<RevertStatementContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = RevertStatementContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 130, RULE_revertStatement);
        let mut _localctx: Rc<RevertStatementContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(772);
			recog.base.match_token(T__61,&mut recog.err_handler)?;

			/*InvokeRule functionCall*/
			recog.base.set_state(773);
			recog.functionCall()?;

			recog.base.set_state(774);
			recog.base.match_token(T__1,&mut recog.err_handler)?;

			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- variableDeclarationStatement ----------------
pub type VariableDeclarationStatementContextAll<'input> = VariableDeclarationStatementContext<'input>;


pub type VariableDeclarationStatementContext<'input> = BaseParserRuleContext<'input,VariableDeclarationStatementContextExt<'input>>;

#[derive(Clone)]
pub struct VariableDeclarationStatementContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SolidityParserContext<'input> for VariableDeclarationStatementContext<'input>{}

impl<'input,'a> Listenable<dyn SolidityListener<'input> + 'a> for VariableDeclarationStatementContext<'input>{
	fn enter(&self,listener: &mut (dyn SolidityListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_variableDeclarationStatement(self);
	}
}

impl<'input> CustomRuleContext<'input> for VariableDeclarationStatementContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SolidityParserContextType;
	fn get_rule_index(&self) -> usize { RULE_variableDeclarationStatement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_variableDeclarationStatement }
}
antlr_rust::type_id!{VariableDeclarationStatementContextExt<'a>}

impl<'input> VariableDeclarationStatementContextExt<'input>{
	fn new(parent: Option<Rc<dyn SolidityParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<VariableDeclarationStatementContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,VariableDeclarationStatementContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait VariableDeclarationStatementContextAttrs<'input>: SolidityParserContext<'input> + BorrowMut<VariableDeclarationStatementContextExt<'input>>{

fn identifierList(&self) -> Option<Rc<IdentifierListContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn variableDeclaration(&self) -> Option<Rc<VariableDeclarationContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn variableDeclarationList(&self) -> Option<Rc<VariableDeclarationListContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> VariableDeclarationStatementContextAttrs<'input> for VariableDeclarationStatementContext<'input>{}

impl<'input, I, H> SolidityParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn variableDeclarationStatement(&mut self,)
	-> Result<Rc<VariableDeclarationStatementContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = VariableDeclarationStatementContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 132, RULE_variableDeclarationStatement);
        let mut _localctx: Rc<VariableDeclarationStatementContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(783);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(82,&mut recog.base)? {
				1 =>{
					{
					recog.base.set_state(776);
					recog.base.match_token(T__62,&mut recog.err_handler)?;

					/*InvokeRule identifierList*/
					recog.base.set_state(777);
					recog.identifierList()?;

					}
				}
			,
				2 =>{
					{
					/*InvokeRule variableDeclaration*/
					recog.base.set_state(778);
					recog.variableDeclaration()?;

					}
				}
			,
				3 =>{
					{
					recog.base.set_state(779);
					recog.base.match_token(T__22,&mut recog.err_handler)?;

					/*InvokeRule variableDeclarationList*/
					recog.base.set_state(780);
					recog.variableDeclarationList()?;

					recog.base.set_state(781);
					recog.base.match_token(T__23,&mut recog.err_handler)?;

					}
				}

				_ => {}
			}
			recog.base.set_state(787);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==T__10 {
				{
				recog.base.set_state(785);
				recog.base.match_token(T__10,&mut recog.err_handler)?;

				/*InvokeRule expression*/
				recog.base.set_state(786);
				recog.expression_rec(0)?;

				}
			}

			recog.base.set_state(789);
			recog.base.match_token(T__1,&mut recog.err_handler)?;

			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- variableDeclarationList ----------------
pub type VariableDeclarationListContextAll<'input> = VariableDeclarationListContext<'input>;


pub type VariableDeclarationListContext<'input> = BaseParserRuleContext<'input,VariableDeclarationListContextExt<'input>>;

#[derive(Clone)]
pub struct VariableDeclarationListContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SolidityParserContext<'input> for VariableDeclarationListContext<'input>{}

impl<'input,'a> Listenable<dyn SolidityListener<'input> + 'a> for VariableDeclarationListContext<'input>{
	fn enter(&self,listener: &mut (dyn SolidityListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_variableDeclarationList(self);
	}
}

impl<'input> CustomRuleContext<'input> for VariableDeclarationListContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SolidityParserContextType;
	fn get_rule_index(&self) -> usize { RULE_variableDeclarationList }
	//fn type_rule_index() -> usize where Self: Sized { RULE_variableDeclarationList }
}
antlr_rust::type_id!{VariableDeclarationListContextExt<'a>}

impl<'input> VariableDeclarationListContextExt<'input>{
	fn new(parent: Option<Rc<dyn SolidityParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<VariableDeclarationListContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,VariableDeclarationListContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait VariableDeclarationListContextAttrs<'input>: SolidityParserContext<'input> + BorrowMut<VariableDeclarationListContextExt<'input>>{

fn variableDeclaration_all(&self) ->  Vec<Rc<VariableDeclarationContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn variableDeclaration(&self, i: usize) -> Option<Rc<VariableDeclarationContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> VariableDeclarationListContextAttrs<'input> for VariableDeclarationListContext<'input>{}

impl<'input, I, H> SolidityParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn variableDeclarationList(&mut self,)
	-> Result<Rc<VariableDeclarationListContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = VariableDeclarationListContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 134, RULE_variableDeclarationList);
        let mut _localctx: Rc<VariableDeclarationListContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(792);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if ((((_la - 14)) & !0x3f) == 0 && ((1usize << (_la - 14)) & ((1usize << (T__13 - 14)) | (1usize << (T__24 - 14)) | (1usize << (T__37 - 14)) | (1usize << (T__43 - 14)) | (1usize << (T__45 - 14)) | (1usize << (T__49 - 14)) | (1usize << (T__61 - 14)) | (1usize << (T__62 - 14)) | (1usize << (T__63 - 14)) | (1usize << (T__64 - 14)) | (1usize << (T__65 - 14)))) != 0) || ((((_la - 95)) & !0x3f) == 0 && ((1usize << (_la - 95)) & ((1usize << (T__94 - 95)) | (1usize << (Int - 95)) | (1usize << (Uint - 95)) | (1usize << (Byte - 95)) | (1usize << (Fixed - 95)) | (1usize << (Ufixed - 95)) | (1usize << (LeaveKeyword - 95)) | (1usize << (PayableKeyword - 95)) | (1usize << (GlobalKeyword - 95)) | (1usize << (ConstructorKeyword - 95)) | (1usize << (ReceiveKeyword - 95)) | (1usize << (Identifier - 95)))) != 0) {
				{
				/*InvokeRule variableDeclaration*/
				recog.base.set_state(791);
				recog.variableDeclaration()?;

				}
			}

			recog.base.set_state(800);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==T__15 {
				{
				{
				recog.base.set_state(794);
				recog.base.match_token(T__15,&mut recog.err_handler)?;

				recog.base.set_state(796);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				if ((((_la - 14)) & !0x3f) == 0 && ((1usize << (_la - 14)) & ((1usize << (T__13 - 14)) | (1usize << (T__24 - 14)) | (1usize << (T__37 - 14)) | (1usize << (T__43 - 14)) | (1usize << (T__45 - 14)) | (1usize << (T__49 - 14)) | (1usize << (T__61 - 14)) | (1usize << (T__62 - 14)) | (1usize << (T__63 - 14)) | (1usize << (T__64 - 14)) | (1usize << (T__65 - 14)))) != 0) || ((((_la - 95)) & !0x3f) == 0 && ((1usize << (_la - 95)) & ((1usize << (T__94 - 95)) | (1usize << (Int - 95)) | (1usize << (Uint - 95)) | (1usize << (Byte - 95)) | (1usize << (Fixed - 95)) | (1usize << (Ufixed - 95)) | (1usize << (LeaveKeyword - 95)) | (1usize << (PayableKeyword - 95)) | (1usize << (GlobalKeyword - 95)) | (1usize << (ConstructorKeyword - 95)) | (1usize << (ReceiveKeyword - 95)) | (1usize << (Identifier - 95)))) != 0) {
					{
					/*InvokeRule variableDeclaration*/
					recog.base.set_state(795);
					recog.variableDeclaration()?;

					}
				}

				}
				}
				recog.base.set_state(802);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- identifierList ----------------
pub type IdentifierListContextAll<'input> = IdentifierListContext<'input>;


pub type IdentifierListContext<'input> = BaseParserRuleContext<'input,IdentifierListContextExt<'input>>;

#[derive(Clone)]
pub struct IdentifierListContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SolidityParserContext<'input> for IdentifierListContext<'input>{}

impl<'input,'a> Listenable<dyn SolidityListener<'input> + 'a> for IdentifierListContext<'input>{
	fn enter(&self,listener: &mut (dyn SolidityListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_identifierList(self);
	}
}

impl<'input> CustomRuleContext<'input> for IdentifierListContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SolidityParserContextType;
	fn get_rule_index(&self) -> usize { RULE_identifierList }
	//fn type_rule_index() -> usize where Self: Sized { RULE_identifierList }
}
antlr_rust::type_id!{IdentifierListContextExt<'a>}

impl<'input> IdentifierListContextExt<'input>{
	fn new(parent: Option<Rc<dyn SolidityParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<IdentifierListContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,IdentifierListContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait IdentifierListContextAttrs<'input>: SolidityParserContext<'input> + BorrowMut<IdentifierListContextExt<'input>>{

fn identifier_all(&self) ->  Vec<Rc<IdentifierContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn identifier(&self, i: usize) -> Option<Rc<IdentifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> IdentifierListContextAttrs<'input> for IdentifierListContext<'input>{}

impl<'input, I, H> SolidityParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn identifierList(&mut self,)
	-> Result<Rc<IdentifierListContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = IdentifierListContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 136, RULE_identifierList);
        let mut _localctx: Rc<IdentifierListContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(803);
			recog.base.match_token(T__22,&mut recog.err_handler)?;

			recog.base.set_state(810);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(88,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					{
					{
					recog.base.set_state(805);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if (((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << T__13) | (1usize << T__24) | (1usize << T__43) | (1usize << T__49) | (1usize << T__61))) != 0) || ((((_la - 95)) & !0x3f) == 0 && ((1usize << (_la - 95)) & ((1usize << (T__94 - 95)) | (1usize << (LeaveKeyword - 95)) | (1usize << (PayableKeyword - 95)) | (1usize << (GlobalKeyword - 95)) | (1usize << (ConstructorKeyword - 95)) | (1usize << (ReceiveKeyword - 95)) | (1usize << (Identifier - 95)))) != 0) {
						{
						/*InvokeRule identifier*/
						recog.base.set_state(804);
						recog.identifier()?;

						}
					}

					recog.base.set_state(807);
					recog.base.match_token(T__15,&mut recog.err_handler)?;

					}
					} 
				}
				recog.base.set_state(812);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(88,&mut recog.base)?;
			}
			recog.base.set_state(814);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if (((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << T__13) | (1usize << T__24) | (1usize << T__43) | (1usize << T__49) | (1usize << T__61))) != 0) || ((((_la - 95)) & !0x3f) == 0 && ((1usize << (_la - 95)) & ((1usize << (T__94 - 95)) | (1usize << (LeaveKeyword - 95)) | (1usize << (PayableKeyword - 95)) | (1usize << (GlobalKeyword - 95)) | (1usize << (ConstructorKeyword - 95)) | (1usize << (ReceiveKeyword - 95)) | (1usize << (Identifier - 95)))) != 0) {
				{
				/*InvokeRule identifier*/
				recog.base.set_state(813);
				recog.identifier()?;

				}
			}

			recog.base.set_state(816);
			recog.base.match_token(T__23,&mut recog.err_handler)?;

			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- elementaryTypeName ----------------
pub type ElementaryTypeNameContextAll<'input> = ElementaryTypeNameContext<'input>;


pub type ElementaryTypeNameContext<'input> = BaseParserRuleContext<'input,ElementaryTypeNameContextExt<'input>>;

#[derive(Clone)]
pub struct ElementaryTypeNameContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SolidityParserContext<'input> for ElementaryTypeNameContext<'input>{}

impl<'input,'a> Listenable<dyn SolidityListener<'input> + 'a> for ElementaryTypeNameContext<'input>{
	fn enter(&self,listener: &mut (dyn SolidityListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_elementaryTypeName(self);
	}
}

impl<'input> CustomRuleContext<'input> for ElementaryTypeNameContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SolidityParserContextType;
	fn get_rule_index(&self) -> usize { RULE_elementaryTypeName }
	//fn type_rule_index() -> usize where Self: Sized { RULE_elementaryTypeName }
}
antlr_rust::type_id!{ElementaryTypeNameContextExt<'a>}

impl<'input> ElementaryTypeNameContextExt<'input>{
	fn new(parent: Option<Rc<dyn SolidityParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ElementaryTypeNameContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ElementaryTypeNameContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ElementaryTypeNameContextAttrs<'input>: SolidityParserContext<'input> + BorrowMut<ElementaryTypeNameContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token Int
/// Returns `None` if there is no child corresponding to token Int
fn Int(&self) -> Option<Rc<TerminalNode<'input,SolidityParserContextType>>> where Self:Sized{
	self.get_token(Int, 0)
}
/// Retrieves first TerminalNode corresponding to token Uint
/// Returns `None` if there is no child corresponding to token Uint
fn Uint(&self) -> Option<Rc<TerminalNode<'input,SolidityParserContextType>>> where Self:Sized{
	self.get_token(Uint, 0)
}
/// Retrieves first TerminalNode corresponding to token Byte
/// Returns `None` if there is no child corresponding to token Byte
fn Byte(&self) -> Option<Rc<TerminalNode<'input,SolidityParserContextType>>> where Self:Sized{
	self.get_token(Byte, 0)
}
/// Retrieves first TerminalNode corresponding to token Fixed
/// Returns `None` if there is no child corresponding to token Fixed
fn Fixed(&self) -> Option<Rc<TerminalNode<'input,SolidityParserContextType>>> where Self:Sized{
	self.get_token(Fixed, 0)
}
/// Retrieves first TerminalNode corresponding to token Ufixed
/// Returns `None` if there is no child corresponding to token Ufixed
fn Ufixed(&self) -> Option<Rc<TerminalNode<'input,SolidityParserContextType>>> where Self:Sized{
	self.get_token(Ufixed, 0)
}

}

impl<'input> ElementaryTypeNameContextAttrs<'input> for ElementaryTypeNameContext<'input>{}

impl<'input, I, H> SolidityParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn elementaryTypeName(&mut self,)
	-> Result<Rc<ElementaryTypeNameContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ElementaryTypeNameContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 138, RULE_elementaryTypeName);
        let mut _localctx: Rc<ElementaryTypeNameContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(818);
			_la = recog.base.input.la(1);
			if { !(((((_la - 44)) & !0x3f) == 0 && ((1usize << (_la - 44)) & ((1usize << (T__43 - 44)) | (1usize << (T__62 - 44)) | (1usize << (T__63 - 44)) | (1usize << (T__64 - 44)) | (1usize << (T__65 - 44)) | (1usize << (Int - 44)) | (1usize << (Uint - 44)) | (1usize << (Byte - 44)) | (1usize << (Fixed - 44)) | (1usize << (Ufixed - 44)))) != 0)) } {
				recog.err_handler.recover_inline(&mut recog.base)?;

			}
			else {
				if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler);
			}
			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- expression ----------------
pub type ExpressionContextAll<'input> = ExpressionContext<'input>;


pub type ExpressionContext<'input> = BaseParserRuleContext<'input,ExpressionContextExt<'input>>;

#[derive(Clone)]
pub struct ExpressionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SolidityParserContext<'input> for ExpressionContext<'input>{}

impl<'input,'a> Listenable<dyn SolidityListener<'input> + 'a> for ExpressionContext<'input>{
	fn enter(&self,listener: &mut (dyn SolidityListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_expression(self);
	}
}

impl<'input> CustomRuleContext<'input> for ExpressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SolidityParserContextType;
	fn get_rule_index(&self) -> usize { RULE_expression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_expression }
}
antlr_rust::type_id!{ExpressionContextExt<'a>}

impl<'input> ExpressionContextExt<'input>{
	fn new(parent: Option<Rc<dyn SolidityParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ExpressionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ExpressionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ExpressionContextAttrs<'input>: SolidityParserContext<'input> + BorrowMut<ExpressionContextExt<'input>>{

fn typeName(&self) -> Option<Rc<TypeNameContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn expression_all(&self) ->  Vec<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn expression(&self, i: usize) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn primaryExpression(&self) -> Option<Rc<PrimaryExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn identifier(&self) -> Option<Rc<IdentifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn nameValueList(&self) -> Option<Rc<NameValueListContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn functionCallArguments(&self) -> Option<Rc<FunctionCallArgumentsContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> ExpressionContextAttrs<'input> for ExpressionContext<'input>{}

impl<'input, I, H> SolidityParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn  expression(&mut self,)
	-> Result<Rc<ExpressionContextAll<'input>>,ANTLRError> {
		self.expression_rec(0)
	}

	fn expression_rec(&mut self, _p: isize)
	-> Result<Rc<ExpressionContextAll<'input>>,ANTLRError> {
		let recog = self;
		let _parentctx = recog.ctx.take();
		let _parentState = recog.base.get_state();
		let mut _localctx = ExpressionContextExt::new(_parentctx.clone(), recog.base.get_state());
		recog.base.enter_recursion_rule(_localctx.clone(), 140, RULE_expression, _p);
	    let mut _localctx: Rc<ExpressionContextAll> = _localctx;
        let mut _prevctx = _localctx.clone();
		let _startState = 140;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {
			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(838);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(90,&mut recog.base)? {
				1 =>{
					{
					recog.base.set_state(821);
					recog.base.match_token(T__68,&mut recog.err_handler)?;

					/*InvokeRule typeName*/
					recog.base.set_state(822);
					recog.typeName_rec(0)?;

					}
				}
			,
				2 =>{
					{
					recog.base.set_state(823);
					recog.base.match_token(T__22,&mut recog.err_handler)?;

					/*InvokeRule expression*/
					recog.base.set_state(824);
					recog.expression_rec(0)?;

					recog.base.set_state(825);
					recog.base.match_token(T__23,&mut recog.err_handler)?;

					}
				}
			,
				3 =>{
					{
					recog.base.set_state(827);
					_la = recog.base.input.la(1);
					if { !(_la==T__66 || _la==T__67) } {
						recog.err_handler.recover_inline(&mut recog.base)?;

					}
					else {
						if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
						recog.err_handler.report_match(&mut recog.base);
						recog.base.consume(&mut recog.err_handler);
					}
					/*InvokeRule expression*/
					recog.base.set_state(828);
					recog.expression_rec(19)?;

					}
				}
			,
				4 =>{
					{
					recog.base.set_state(829);
					_la = recog.base.input.la(1);
					if { !(_la==T__29 || _la==T__30) } {
						recog.err_handler.recover_inline(&mut recog.base)?;

					}
					else {
						if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
						recog.err_handler.report_match(&mut recog.base);
						recog.base.consume(&mut recog.err_handler);
					}
					/*InvokeRule expression*/
					recog.base.set_state(830);
					recog.expression_rec(18)?;

					}
				}
			,
				5 =>{
					{
					recog.base.set_state(831);
					recog.base.match_token(T__70,&mut recog.err_handler)?;

					/*InvokeRule expression*/
					recog.base.set_state(832);
					recog.expression_rec(17)?;

					}
				}
			,
				6 =>{
					{
					recog.base.set_state(833);
					recog.base.match_token(T__71,&mut recog.err_handler)?;

					/*InvokeRule expression*/
					recog.base.set_state(834);
					recog.expression_rec(16)?;

					}
				}
			,
				7 =>{
					{
					recog.base.set_state(835);
					recog.base.match_token(T__5,&mut recog.err_handler)?;

					/*InvokeRule expression*/
					recog.base.set_state(836);
					recog.expression_rec(15)?;

					}
				}
			,
				8 =>{
					{
					/*InvokeRule primaryExpression*/
					recog.base.set_state(837);
					recog.primaryExpression()?;

					}
				}

				_ => {}
			}

			let tmp = recog.input.lt(-1).cloned();
			recog.ctx.as_ref().unwrap().set_stop(tmp);
			recog.base.set_state(914);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(94,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					recog.trigger_exit_rule_event();
					_prevctx = _localctx.clone();
					{
					recog.base.set_state(912);
					recog.err_handler.sync(&mut recog.base)?;
					match  recog.interpreter.adaptive_predict(93,&mut recog.base)? {
						1 =>{
							{
							/*recRuleAltStartAction*/
							let mut tmp = ExpressionContextExt::new(_parentctx.clone(), _parentState);
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_expression);
							_localctx = tmp;
							recog.base.set_state(840);
							if !({recog.precpred(None, 14)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 14)".to_owned()), None))?;
							}
							recog.base.set_state(841);
							recog.base.match_token(T__72,&mut recog.err_handler)?;

							/*InvokeRule expression*/
							recog.base.set_state(842);
							recog.expression_rec(14)?;

							}
						}
					,
						2 =>{
							{
							/*recRuleAltStartAction*/
							let mut tmp = ExpressionContextExt::new(_parentctx.clone(), _parentState);
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_expression);
							_localctx = tmp;
							recog.base.set_state(843);
							if !({recog.precpred(None, 13)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 13)".to_owned()), None))?;
							}
							recog.base.set_state(844);
							_la = recog.base.input.la(1);
							if { !((((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << T__2) | (1usize << T__31) | (1usize << T__32))) != 0)) } {
								recog.err_handler.recover_inline(&mut recog.base)?;

							}
							else {
								if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
								recog.err_handler.report_match(&mut recog.base);
								recog.base.consume(&mut recog.err_handler);
							}
							/*InvokeRule expression*/
							recog.base.set_state(845);
							recog.expression_rec(14)?;

							}
						}
					,
						3 =>{
							{
							/*recRuleAltStartAction*/
							let mut tmp = ExpressionContextExt::new(_parentctx.clone(), _parentState);
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_expression);
							_localctx = tmp;
							recog.base.set_state(846);
							if !({recog.precpred(None, 12)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 12)".to_owned()), None))?;
							}
							recog.base.set_state(847);
							_la = recog.base.input.la(1);
							if { !(_la==T__29 || _la==T__30) } {
								recog.err_handler.recover_inline(&mut recog.base)?;

							}
							else {
								if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
								recog.err_handler.report_match(&mut recog.base);
								recog.base.consume(&mut recog.err_handler);
							}
							/*InvokeRule expression*/
							recog.base.set_state(848);
							recog.expression_rec(13)?;

							}
						}
					,
						4 =>{
							{
							/*recRuleAltStartAction*/
							let mut tmp = ExpressionContextExt::new(_parentctx.clone(), _parentState);
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_expression);
							_localctx = tmp;
							recog.base.set_state(849);
							if !({recog.precpred(None, 11)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 11)".to_owned()), None))?;
							}
							recog.base.set_state(850);
							_la = recog.base.input.la(1);
							if { !(_la==T__73 || _la==T__74) } {
								recog.err_handler.recover_inline(&mut recog.base)?;

							}
							else {
								if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
								recog.err_handler.report_match(&mut recog.base);
								recog.base.consume(&mut recog.err_handler);
							}
							/*InvokeRule expression*/
							recog.base.set_state(851);
							recog.expression_rec(12)?;

							}
						}
					,
						5 =>{
							{
							/*recRuleAltStartAction*/
							let mut tmp = ExpressionContextExt::new(_parentctx.clone(), _parentState);
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_expression);
							_localctx = tmp;
							recog.base.set_state(852);
							if !({recog.precpred(None, 10)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 10)".to_owned()), None))?;
							}
							recog.base.set_state(853);
							recog.base.match_token(T__28,&mut recog.err_handler)?;

							/*InvokeRule expression*/
							recog.base.set_state(854);
							recog.expression_rec(11)?;

							}
						}
					,
						6 =>{
							{
							/*recRuleAltStartAction*/
							let mut tmp = ExpressionContextExt::new(_parentctx.clone(), _parentState);
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_expression);
							_localctx = tmp;
							recog.base.set_state(855);
							if !({recog.precpred(None, 9)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 9)".to_owned()), None))?;
							}
							recog.base.set_state(856);
							recog.base.match_token(T__4,&mut recog.err_handler)?;

							/*InvokeRule expression*/
							recog.base.set_state(857);
							recog.expression_rec(10)?;

							}
						}
					,
						7 =>{
							{
							/*recRuleAltStartAction*/
							let mut tmp = ExpressionContextExt::new(_parentctx.clone(), _parentState);
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_expression);
							_localctx = tmp;
							recog.base.set_state(858);
							if !({recog.precpred(None, 8)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 8)".to_owned()), None))?;
							}
							recog.base.set_state(859);
							recog.base.match_token(T__27,&mut recog.err_handler)?;

							/*InvokeRule expression*/
							recog.base.set_state(860);
							recog.expression_rec(9)?;

							}
						}
					,
						8 =>{
							{
							/*recRuleAltStartAction*/
							let mut tmp = ExpressionContextExt::new(_parentctx.clone(), _parentState);
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_expression);
							_localctx = tmp;
							recog.base.set_state(861);
							if !({recog.precpred(None, 7)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 7)".to_owned()), None))?;
							}
							recog.base.set_state(862);
							_la = recog.base.input.la(1);
							if { !((((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << T__6) | (1usize << T__7) | (1usize << T__8) | (1usize << T__9))) != 0)) } {
								recog.err_handler.recover_inline(&mut recog.base)?;

							}
							else {
								if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
								recog.err_handler.report_match(&mut recog.base);
								recog.base.consume(&mut recog.err_handler);
							}
							/*InvokeRule expression*/
							recog.base.set_state(863);
							recog.expression_rec(8)?;

							}
						}
					,
						9 =>{
							{
							/*recRuleAltStartAction*/
							let mut tmp = ExpressionContextExt::new(_parentctx.clone(), _parentState);
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_expression);
							_localctx = tmp;
							recog.base.set_state(864);
							if !({recog.precpred(None, 6)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 6)".to_owned()), None))?;
							}
							recog.base.set_state(865);
							_la = recog.base.input.la(1);
							if { !(_la==T__33 || _la==T__34) } {
								recog.err_handler.recover_inline(&mut recog.base)?;

							}
							else {
								if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
								recog.err_handler.report_match(&mut recog.base);
								recog.base.consume(&mut recog.err_handler);
							}
							/*InvokeRule expression*/
							recog.base.set_state(866);
							recog.expression_rec(7)?;

							}
						}
					,
						10 =>{
							{
							/*recRuleAltStartAction*/
							let mut tmp = ExpressionContextExt::new(_parentctx.clone(), _parentState);
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_expression);
							_localctx = tmp;
							recog.base.set_state(867);
							if !({recog.precpred(None, 5)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 5)".to_owned()), None))?;
							}
							recog.base.set_state(868);
							recog.base.match_token(T__75,&mut recog.err_handler)?;

							/*InvokeRule expression*/
							recog.base.set_state(869);
							recog.expression_rec(6)?;

							}
						}
					,
						11 =>{
							{
							/*recRuleAltStartAction*/
							let mut tmp = ExpressionContextExt::new(_parentctx.clone(), _parentState);
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_expression);
							_localctx = tmp;
							recog.base.set_state(870);
							if !({recog.precpred(None, 4)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 4)".to_owned()), None))?;
							}
							recog.base.set_state(871);
							recog.base.match_token(T__3,&mut recog.err_handler)?;

							/*InvokeRule expression*/
							recog.base.set_state(872);
							recog.expression_rec(5)?;

							}
						}
					,
						12 =>{
							{
							/*recRuleAltStartAction*/
							let mut tmp = ExpressionContextExt::new(_parentctx.clone(), _parentState);
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_expression);
							_localctx = tmp;
							recog.base.set_state(873);
							if !({recog.precpred(None, 3)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 3)".to_owned()), None))?;
							}
							recog.base.set_state(874);
							recog.base.match_token(T__76,&mut recog.err_handler)?;

							/*InvokeRule expression*/
							recog.base.set_state(875);
							recog.expression_rec(0)?;

							recog.base.set_state(876);
							recog.base.match_token(T__69,&mut recog.err_handler)?;

							/*InvokeRule expression*/
							recog.base.set_state(877);
							recog.expression_rec(3)?;

							}
						}
					,
						13 =>{
							{
							/*recRuleAltStartAction*/
							let mut tmp = ExpressionContextExt::new(_parentctx.clone(), _parentState);
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_expression);
							_localctx = tmp;
							recog.base.set_state(879);
							if !({recog.precpred(None, 2)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 2)".to_owned()), None))?;
							}
							recog.base.set_state(880);
							_la = recog.base.input.la(1);
							if { !(_la==T__10 || ((((_la - 78)) & !0x3f) == 0 && ((1usize << (_la - 78)) & ((1usize << (T__77 - 78)) | (1usize << (T__78 - 78)) | (1usize << (T__79 - 78)) | (1usize << (T__80 - 78)) | (1usize << (T__81 - 78)) | (1usize << (T__82 - 78)) | (1usize << (T__83 - 78)) | (1usize << (T__84 - 78)) | (1usize << (T__85 - 78)) | (1usize << (T__86 - 78)))) != 0)) } {
								recog.err_handler.recover_inline(&mut recog.base)?;

							}
							else {
								if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
								recog.err_handler.report_match(&mut recog.base);
								recog.base.consume(&mut recog.err_handler);
							}
							/*InvokeRule expression*/
							recog.base.set_state(881);
							recog.expression_rec(3)?;

							}
						}
					,
						14 =>{
							{
							/*recRuleAltStartAction*/
							let mut tmp = ExpressionContextExt::new(_parentctx.clone(), _parentState);
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_expression);
							_localctx = tmp;
							recog.base.set_state(882);
							if !({recog.precpred(None, 27)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 27)".to_owned()), None))?;
							}
							recog.base.set_state(883);
							_la = recog.base.input.la(1);
							if { !(_la==T__66 || _la==T__67) } {
								recog.err_handler.recover_inline(&mut recog.base)?;

							}
							else {
								if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
								recog.err_handler.report_match(&mut recog.base);
								recog.base.consume(&mut recog.err_handler);
							}
							}
						}
					,
						15 =>{
							{
							/*recRuleAltStartAction*/
							let mut tmp = ExpressionContextExt::new(_parentctx.clone(), _parentState);
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_expression);
							_localctx = tmp;
							recog.base.set_state(884);
							if !({recog.precpred(None, 25)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 25)".to_owned()), None))?;
							}
							recog.base.set_state(885);
							recog.base.match_token(T__41,&mut recog.err_handler)?;

							/*InvokeRule expression*/
							recog.base.set_state(886);
							recog.expression_rec(0)?;

							recog.base.set_state(887);
							recog.base.match_token(T__42,&mut recog.err_handler)?;

							}
						}
					,
						16 =>{
							{
							/*recRuleAltStartAction*/
							let mut tmp = ExpressionContextExt::new(_parentctx.clone(), _parentState);
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_expression);
							_localctx = tmp;
							recog.base.set_state(889);
							if !({recog.precpred(None, 24)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 24)".to_owned()), None))?;
							}
							recog.base.set_state(890);
							recog.base.match_token(T__41,&mut recog.err_handler)?;

							recog.base.set_state(892);
							recog.err_handler.sync(&mut recog.base)?;
							_la = recog.base.input.la(1);
							if ((((_la - 6)) & !0x3f) == 0 && ((1usize << (_la - 6)) & ((1usize << (T__5 - 6)) | (1usize << (T__13 - 6)) | (1usize << (T__22 - 6)) | (1usize << (T__24 - 6)) | (1usize << (T__29 - 6)) | (1usize << (T__30 - 6)) | (1usize << (T__37 - 6)) | (1usize << (T__41 - 6)) | (1usize << (T__43 - 6)) | (1usize << (T__45 - 6)) | (1usize << (T__49 - 6)) | (1usize << (T__61 - 6)) | (1usize << (T__62 - 6)) | (1usize << (T__63 - 6)) | (1usize << (T__64 - 6)) | (1usize << (T__65 - 6)) | (1usize << (T__66 - 6)) | (1usize << (T__67 - 6)) | (1usize << (T__68 - 6)))) != 0) || ((((_la - 71)) & !0x3f) == 0 && ((1usize << (_la - 71)) & ((1usize << (T__70 - 71)) | (1usize << (T__71 - 71)) | (1usize << (T__94 - 71)) | (1usize << (Int - 71)) | (1usize << (Uint - 71)) | (1usize << (Byte - 71)) | (1usize << (Fixed - 71)) | (1usize << (Ufixed - 71)) | (1usize << (BooleanLiteral - 71)) | (1usize << (DecimalNumber - 71)) | (1usize << (HexNumber - 71)) | (1usize << (HexLiteralFragment - 71)) | (1usize << (LeaveKeyword - 71)) | (1usize << (PayableKeyword - 71)) | (1usize << (TypeKeyword - 71)) | (1usize << (GlobalKeyword - 71)) | (1usize << (ConstructorKeyword - 71)) | (1usize << (ReceiveKeyword - 71)) | (1usize << (Identifier - 71)) | (1usize << (StringLiteralFragment - 71)))) != 0) {
								{
								/*InvokeRule expression*/
								recog.base.set_state(891);
								recog.expression_rec(0)?;

								}
							}

							recog.base.set_state(894);
							recog.base.match_token(T__69,&mut recog.err_handler)?;

							recog.base.set_state(896);
							recog.err_handler.sync(&mut recog.base)?;
							_la = recog.base.input.la(1);
							if ((((_la - 6)) & !0x3f) == 0 && ((1usize << (_la - 6)) & ((1usize << (T__5 - 6)) | (1usize << (T__13 - 6)) | (1usize << (T__22 - 6)) | (1usize << (T__24 - 6)) | (1usize << (T__29 - 6)) | (1usize << (T__30 - 6)) | (1usize << (T__37 - 6)) | (1usize << (T__41 - 6)) | (1usize << (T__43 - 6)) | (1usize << (T__45 - 6)) | (1usize << (T__49 - 6)) | (1usize << (T__61 - 6)) | (1usize << (T__62 - 6)) | (1usize << (T__63 - 6)) | (1usize << (T__64 - 6)) | (1usize << (T__65 - 6)) | (1usize << (T__66 - 6)) | (1usize << (T__67 - 6)) | (1usize << (T__68 - 6)))) != 0) || ((((_la - 71)) & !0x3f) == 0 && ((1usize << (_la - 71)) & ((1usize << (T__70 - 71)) | (1usize << (T__71 - 71)) | (1usize << (T__94 - 71)) | (1usize << (Int - 71)) | (1usize << (Uint - 71)) | (1usize << (Byte - 71)) | (1usize << (Fixed - 71)) | (1usize << (Ufixed - 71)) | (1usize << (BooleanLiteral - 71)) | (1usize << (DecimalNumber - 71)) | (1usize << (HexNumber - 71)) | (1usize << (HexLiteralFragment - 71)) | (1usize << (LeaveKeyword - 71)) | (1usize << (PayableKeyword - 71)) | (1usize << (TypeKeyword - 71)) | (1usize << (GlobalKeyword - 71)) | (1usize << (ConstructorKeyword - 71)) | (1usize << (ReceiveKeyword - 71)) | (1usize << (Identifier - 71)) | (1usize << (StringLiteralFragment - 71)))) != 0) {
								{
								/*InvokeRule expression*/
								recog.base.set_state(895);
								recog.expression_rec(0)?;

								}
							}

							recog.base.set_state(898);
							recog.base.match_token(T__42,&mut recog.err_handler)?;

							}
						}
					,
						17 =>{
							{
							/*recRuleAltStartAction*/
							let mut tmp = ExpressionContextExt::new(_parentctx.clone(), _parentState);
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_expression);
							_localctx = tmp;
							recog.base.set_state(899);
							if !({recog.precpred(None, 23)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 23)".to_owned()), None))?;
							}
							recog.base.set_state(900);
							recog.base.match_token(T__44,&mut recog.err_handler)?;

							/*InvokeRule identifier*/
							recog.base.set_state(901);
							recog.identifier()?;

							}
						}
					,
						18 =>{
							{
							/*recRuleAltStartAction*/
							let mut tmp = ExpressionContextExt::new(_parentctx.clone(), _parentState);
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_expression);
							_localctx = tmp;
							recog.base.set_state(902);
							if !({recog.precpred(None, 22)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 22)".to_owned()), None))?;
							}
							recog.base.set_state(903);
							recog.base.match_token(T__14,&mut recog.err_handler)?;

							/*InvokeRule nameValueList*/
							recog.base.set_state(904);
							recog.nameValueList()?;

							recog.base.set_state(905);
							recog.base.match_token(T__16,&mut recog.err_handler)?;

							}
						}
					,
						19 =>{
							{
							/*recRuleAltStartAction*/
							let mut tmp = ExpressionContextExt::new(_parentctx.clone(), _parentState);
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_expression);
							_localctx = tmp;
							recog.base.set_state(907);
							if !({recog.precpred(None, 21)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 21)".to_owned()), None))?;
							}
							recog.base.set_state(908);
							recog.base.match_token(T__22,&mut recog.err_handler)?;

							/*InvokeRule functionCallArguments*/
							recog.base.set_state(909);
							recog.functionCallArguments()?;

							recog.base.set_state(910);
							recog.base.match_token(T__23,&mut recog.err_handler)?;

							}
						}

						_ => {}
					}
					} 
				}
				recog.base.set_state(916);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(94,&mut recog.base)?;
			}
			}
		};
		match result {
		Ok(_) => {},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re)=>{
			//_localctx.exception = re;
			recog.err_handler.report_error(&mut recog.base, re);
	        recog.err_handler.recover(&mut recog.base, re)?;}
		}
		recog.base.unroll_recursion_context(_parentctx);

		Ok(_localctx)
	}
}
//------------------- primaryExpression ----------------
pub type PrimaryExpressionContextAll<'input> = PrimaryExpressionContext<'input>;


pub type PrimaryExpressionContext<'input> = BaseParserRuleContext<'input,PrimaryExpressionContextExt<'input>>;

#[derive(Clone)]
pub struct PrimaryExpressionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SolidityParserContext<'input> for PrimaryExpressionContext<'input>{}

impl<'input,'a> Listenable<dyn SolidityListener<'input> + 'a> for PrimaryExpressionContext<'input>{
	fn enter(&self,listener: &mut (dyn SolidityListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_primaryExpression(self);
	}
}

impl<'input> CustomRuleContext<'input> for PrimaryExpressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SolidityParserContextType;
	fn get_rule_index(&self) -> usize { RULE_primaryExpression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_primaryExpression }
}
antlr_rust::type_id!{PrimaryExpressionContextExt<'a>}

impl<'input> PrimaryExpressionContextExt<'input>{
	fn new(parent: Option<Rc<dyn SolidityParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<PrimaryExpressionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,PrimaryExpressionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait PrimaryExpressionContextAttrs<'input>: SolidityParserContext<'input> + BorrowMut<PrimaryExpressionContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token BooleanLiteral
/// Returns `None` if there is no child corresponding to token BooleanLiteral
fn BooleanLiteral(&self) -> Option<Rc<TerminalNode<'input,SolidityParserContextType>>> where Self:Sized{
	self.get_token(BooleanLiteral, 0)
}
fn numberLiteral(&self) -> Option<Rc<NumberLiteralContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn hexLiteral(&self) -> Option<Rc<HexLiteralContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn stringLiteral(&self) -> Option<Rc<StringLiteralContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn identifier(&self) -> Option<Rc<IdentifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token TypeKeyword
/// Returns `None` if there is no child corresponding to token TypeKeyword
fn TypeKeyword(&self) -> Option<Rc<TerminalNode<'input,SolidityParserContextType>>> where Self:Sized{
	self.get_token(TypeKeyword, 0)
}
/// Retrieves first TerminalNode corresponding to token PayableKeyword
/// Returns `None` if there is no child corresponding to token PayableKeyword
fn PayableKeyword(&self) -> Option<Rc<TerminalNode<'input,SolidityParserContextType>>> where Self:Sized{
	self.get_token(PayableKeyword, 0)
}
fn tupleExpression(&self) -> Option<Rc<TupleExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn typeName(&self) -> Option<Rc<TypeNameContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> PrimaryExpressionContextAttrs<'input> for PrimaryExpressionContext<'input>{}

impl<'input, I, H> SolidityParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{      
	pub fn primaryExpression(&mut self,)
	-> Result<Rc<PrimaryExpressionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = PrimaryExpressionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 142, RULE_primaryExpression);
        let mut _localctx: Rc<PrimaryExpressionContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			recog.base.set_state(926);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(95,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(917);
					recog.base.match_token(BooleanLiteral,&mut recog.err_handler)?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule numberLiteral*/
					recog.base.set_state(918);
					recog.numberLiteral()?;

					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					/*InvokeRule hexLiteral*/
					recog.base.set_state(919);
					recog.hexLiteral()?;

					}
				}
			,
				4 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 4);
					recog.base.enter_outer_alt(None, 4);
					{
					/*InvokeRule stringLiteral*/
					recog.base.set_state(920);
					recog.stringLiteral()?;

					}
				}
			,
				5 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 5);
					recog.base.enter_outer_alt(None, 5);
					{
					/*InvokeRule identifier*/
					recog.base.set_state(921);
					recog.identifier()?;

					}
				}
			,
				6 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 6);
					recog.base.enter_outer_alt(None, 6);
					{
					recog.base.set_state(922);
					recog.base.match_token(TypeKeyword,&mut recog.err_handler)?;

					}
				}
			,
				7 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 7);
					recog.base.enter_outer_alt(None, 7);
					{
					recog.base.set_state(923);
					recog.base.match_token(PayableKeyword,&mut recog.err_handler)?;

					}
				}
			,
				8 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 8);
					recog.base.enter_outer_alt(None, 8);
					{
					/*InvokeRule tupleExpression*/
					recog.base.set_state(924);
					recog.tupleExpression()?;

					}
				}
			,
				9 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 9);
					recog.base.enter_outer_alt(None, 9);
					{
					/*InvokeRule typeName*/
					recog.base.set_state(925);
					recog.typeName_rec(0)?;

					}
				}

				_ => {}
			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- expressionList ----------------
pub type ExpressionListContextAll<'input> = ExpressionListContext<'input>;


pub type ExpressionListContext<'input> = BaseParserRuleContext<'input,ExpressionListContextExt<'input>>;

#[derive(Clone)]
pub struct ExpressionListContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SolidityParserContext<'input> for ExpressionListContext<'input>{}

impl<'input,'a> Listenable<dyn SolidityListener<'input> + 'a> for ExpressionListContext<'input>{
	fn enter(&self,listener: &mut (dyn SolidityListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_expressionList(self);
	}
}

impl<'input> CustomRuleContext<'input> for ExpressionListContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SolidityParserContextType;
	fn get_rule_index(&self) -> usize { RULE_expressionList }
	//fn type_rule_index() -> usize where Self: Sized { RULE_expressionList }
}
antlr_rust::type_id!{ExpressionListContextExt<'a>}

impl<'input> ExpressionListContextExt<'input>{
	fn new(parent: Option<Rc<dyn SolidityParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ExpressionListContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ExpressionListContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ExpressionListContextAttrs<'input>: SolidityParserContext<'input> + BorrowMut<ExpressionListContextExt<'input>>{

fn expression_all(&self) ->  Vec<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn expression(&self, i: usize) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> ExpressionListContextAttrs<'input> for ExpressionListContext<'input>{}

impl<'input, I, H> SolidityParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn expressionList(&mut self,)
	-> Result<Rc<ExpressionListContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ExpressionListContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 144, RULE_expressionList);
        let mut _localctx: Rc<ExpressionListContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule expression*/
			recog.base.set_state(928);
			recog.expression_rec(0)?;

			recog.base.set_state(933);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==T__15 {
				{
				{
				recog.base.set_state(929);
				recog.base.match_token(T__15,&mut recog.err_handler)?;

				/*InvokeRule expression*/
				recog.base.set_state(930);
				recog.expression_rec(0)?;

				}
				}
				recog.base.set_state(935);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- nameValueList ----------------
pub type NameValueListContextAll<'input> = NameValueListContext<'input>;


pub type NameValueListContext<'input> = BaseParserRuleContext<'input,NameValueListContextExt<'input>>;

#[derive(Clone)]
pub struct NameValueListContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SolidityParserContext<'input> for NameValueListContext<'input>{}

impl<'input,'a> Listenable<dyn SolidityListener<'input> + 'a> for NameValueListContext<'input>{
	fn enter(&self,listener: &mut (dyn SolidityListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_nameValueList(self);
	}
}

impl<'input> CustomRuleContext<'input> for NameValueListContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SolidityParserContextType;
	fn get_rule_index(&self) -> usize { RULE_nameValueList }
	//fn type_rule_index() -> usize where Self: Sized { RULE_nameValueList }
}
antlr_rust::type_id!{NameValueListContextExt<'a>}

impl<'input> NameValueListContextExt<'input>{
	fn new(parent: Option<Rc<dyn SolidityParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<NameValueListContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,NameValueListContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait NameValueListContextAttrs<'input>: SolidityParserContext<'input> + BorrowMut<NameValueListContextExt<'input>>{

fn nameValue_all(&self) ->  Vec<Rc<NameValueContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn nameValue(&self, i: usize) -> Option<Rc<NameValueContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> NameValueListContextAttrs<'input> for NameValueListContext<'input>{}

impl<'input, I, H> SolidityParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn nameValueList(&mut self,)
	-> Result<Rc<NameValueListContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = NameValueListContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 146, RULE_nameValueList);
        let mut _localctx: Rc<NameValueListContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule nameValue*/
			recog.base.set_state(936);
			recog.nameValue()?;

			recog.base.set_state(941);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(97,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					{
					{
					recog.base.set_state(937);
					recog.base.match_token(T__15,&mut recog.err_handler)?;

					/*InvokeRule nameValue*/
					recog.base.set_state(938);
					recog.nameValue()?;

					}
					} 
				}
				recog.base.set_state(943);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(97,&mut recog.base)?;
			}
			recog.base.set_state(945);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==T__15 {
				{
				recog.base.set_state(944);
				recog.base.match_token(T__15,&mut recog.err_handler)?;

				}
			}

			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- nameValue ----------------
pub type NameValueContextAll<'input> = NameValueContext<'input>;


pub type NameValueContext<'input> = BaseParserRuleContext<'input,NameValueContextExt<'input>>;

#[derive(Clone)]
pub struct NameValueContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SolidityParserContext<'input> for NameValueContext<'input>{}

impl<'input,'a> Listenable<dyn SolidityListener<'input> + 'a> for NameValueContext<'input>{
	fn enter(&self,listener: &mut (dyn SolidityListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_nameValue(self);
	}
}

impl<'input> CustomRuleContext<'input> for NameValueContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SolidityParserContextType;
	fn get_rule_index(&self) -> usize { RULE_nameValue }
	//fn type_rule_index() -> usize where Self: Sized { RULE_nameValue }
}
antlr_rust::type_id!{NameValueContextExt<'a>}

impl<'input> NameValueContextExt<'input>{
	fn new(parent: Option<Rc<dyn SolidityParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<NameValueContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,NameValueContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait NameValueContextAttrs<'input>: SolidityParserContext<'input> + BorrowMut<NameValueContextExt<'input>>{

fn identifier(&self) -> Option<Rc<IdentifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> NameValueContextAttrs<'input> for NameValueContext<'input>{}

impl<'input, I, H> SolidityParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn nameValue(&mut self,)
	-> Result<Rc<NameValueContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = NameValueContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 148, RULE_nameValue);
        let mut _localctx: Rc<NameValueContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule identifier*/
			recog.base.set_state(947);
			recog.identifier()?;

			recog.base.set_state(948);
			recog.base.match_token(T__69,&mut recog.err_handler)?;

			/*InvokeRule expression*/
			recog.base.set_state(949);
			recog.expression_rec(0)?;

			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- functionCallArguments ----------------
pub type FunctionCallArgumentsContextAll<'input> = FunctionCallArgumentsContext<'input>;


pub type FunctionCallArgumentsContext<'input> = BaseParserRuleContext<'input,FunctionCallArgumentsContextExt<'input>>;

#[derive(Clone)]
pub struct FunctionCallArgumentsContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SolidityParserContext<'input> for FunctionCallArgumentsContext<'input>{}

impl<'input,'a> Listenable<dyn SolidityListener<'input> + 'a> for FunctionCallArgumentsContext<'input>{
	fn enter(&self,listener: &mut (dyn SolidityListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_functionCallArguments(self);
	}
}

impl<'input> CustomRuleContext<'input> for FunctionCallArgumentsContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SolidityParserContextType;
	fn get_rule_index(&self) -> usize { RULE_functionCallArguments }
	//fn type_rule_index() -> usize where Self: Sized { RULE_functionCallArguments }
}
antlr_rust::type_id!{FunctionCallArgumentsContextExt<'a>}

impl<'input> FunctionCallArgumentsContextExt<'input>{
	fn new(parent: Option<Rc<dyn SolidityParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<FunctionCallArgumentsContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,FunctionCallArgumentsContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait FunctionCallArgumentsContextAttrs<'input>: SolidityParserContext<'input> + BorrowMut<FunctionCallArgumentsContextExt<'input>>{

fn nameValueList(&self) -> Option<Rc<NameValueListContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn expressionList(&self) -> Option<Rc<ExpressionListContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> FunctionCallArgumentsContextAttrs<'input> for FunctionCallArgumentsContext<'input>{}

impl<'input, I, H> SolidityParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn functionCallArguments(&mut self,)
	-> Result<Rc<FunctionCallArgumentsContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = FunctionCallArgumentsContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 150, RULE_functionCallArguments);
        let mut _localctx: Rc<FunctionCallArgumentsContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			recog.base.set_state(959);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 T__14 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(951);
					recog.base.match_token(T__14,&mut recog.err_handler)?;

					recog.base.set_state(953);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if (((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << T__13) | (1usize << T__24) | (1usize << T__43) | (1usize << T__49) | (1usize << T__61))) != 0) || ((((_la - 95)) & !0x3f) == 0 && ((1usize << (_la - 95)) & ((1usize << (T__94 - 95)) | (1usize << (LeaveKeyword - 95)) | (1usize << (PayableKeyword - 95)) | (1usize << (GlobalKeyword - 95)) | (1usize << (ConstructorKeyword - 95)) | (1usize << (ReceiveKeyword - 95)) | (1usize << (Identifier - 95)))) != 0) {
						{
						/*InvokeRule nameValueList*/
						recog.base.set_state(952);
						recog.nameValueList()?;

						}
					}

					recog.base.set_state(955);
					recog.base.match_token(T__16,&mut recog.err_handler)?;

					}
				}

			 T__5 | T__13 | T__22 | T__23 | T__24 | T__29 | T__30 | T__37 | T__41 |
			 T__43 | T__45 | T__49 | T__61 | T__62 | T__63 | T__64 | T__65 | T__66 |
			 T__67 | T__68 | T__70 | T__71 | T__94 | Int | Uint | Byte | Fixed | Ufixed |
			 BooleanLiteral | DecimalNumber | HexNumber | HexLiteralFragment | LeaveKeyword |
			 PayableKeyword | TypeKeyword | GlobalKeyword | ConstructorKeyword | ReceiveKeyword |
			 Identifier | StringLiteralFragment 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(957);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if ((((_la - 6)) & !0x3f) == 0 && ((1usize << (_la - 6)) & ((1usize << (T__5 - 6)) | (1usize << (T__13 - 6)) | (1usize << (T__22 - 6)) | (1usize << (T__24 - 6)) | (1usize << (T__29 - 6)) | (1usize << (T__30 - 6)) | (1usize << (T__37 - 6)) | (1usize << (T__41 - 6)) | (1usize << (T__43 - 6)) | (1usize << (T__45 - 6)) | (1usize << (T__49 - 6)) | (1usize << (T__61 - 6)) | (1usize << (T__62 - 6)) | (1usize << (T__63 - 6)) | (1usize << (T__64 - 6)) | (1usize << (T__65 - 6)) | (1usize << (T__66 - 6)) | (1usize << (T__67 - 6)) | (1usize << (T__68 - 6)))) != 0) || ((((_la - 71)) & !0x3f) == 0 && ((1usize << (_la - 71)) & ((1usize << (T__70 - 71)) | (1usize << (T__71 - 71)) | (1usize << (T__94 - 71)) | (1usize << (Int - 71)) | (1usize << (Uint - 71)) | (1usize << (Byte - 71)) | (1usize << (Fixed - 71)) | (1usize << (Ufixed - 71)) | (1usize << (BooleanLiteral - 71)) | (1usize << (DecimalNumber - 71)) | (1usize << (HexNumber - 71)) | (1usize << (HexLiteralFragment - 71)) | (1usize << (LeaveKeyword - 71)) | (1usize << (PayableKeyword - 71)) | (1usize << (TypeKeyword - 71)) | (1usize << (GlobalKeyword - 71)) | (1usize << (ConstructorKeyword - 71)) | (1usize << (ReceiveKeyword - 71)) | (1usize << (Identifier - 71)) | (1usize << (StringLiteralFragment - 71)))) != 0) {
						{
						/*InvokeRule expressionList*/
						recog.base.set_state(956);
						recog.expressionList()?;

						}
					}

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- functionCall ----------------
pub type FunctionCallContextAll<'input> = FunctionCallContext<'input>;


pub type FunctionCallContext<'input> = BaseParserRuleContext<'input,FunctionCallContextExt<'input>>;

#[derive(Clone)]
pub struct FunctionCallContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SolidityParserContext<'input> for FunctionCallContext<'input>{}

impl<'input,'a> Listenable<dyn SolidityListener<'input> + 'a> for FunctionCallContext<'input>{
	fn enter(&self,listener: &mut (dyn SolidityListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_functionCall(self);
	}
}

impl<'input> CustomRuleContext<'input> for FunctionCallContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SolidityParserContextType;
	fn get_rule_index(&self) -> usize { RULE_functionCall }
	//fn type_rule_index() -> usize where Self: Sized { RULE_functionCall }
}
antlr_rust::type_id!{FunctionCallContextExt<'a>}

impl<'input> FunctionCallContextExt<'input>{
	fn new(parent: Option<Rc<dyn SolidityParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<FunctionCallContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,FunctionCallContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait FunctionCallContextAttrs<'input>: SolidityParserContext<'input> + BorrowMut<FunctionCallContextExt<'input>>{

fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn functionCallArguments(&self) -> Option<Rc<FunctionCallArgumentsContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> FunctionCallContextAttrs<'input> for FunctionCallContext<'input>{}

impl<'input, I, H> SolidityParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn functionCall(&mut self,)
	-> Result<Rc<FunctionCallContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = FunctionCallContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 152, RULE_functionCall);
        let mut _localctx: Rc<FunctionCallContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule expression*/
			recog.base.set_state(961);
			recog.expression_rec(0)?;

			recog.base.set_state(962);
			recog.base.match_token(T__22,&mut recog.err_handler)?;

			/*InvokeRule functionCallArguments*/
			recog.base.set_state(963);
			recog.functionCallArguments()?;

			recog.base.set_state(964);
			recog.base.match_token(T__23,&mut recog.err_handler)?;

			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- assemblyBlock ----------------
pub type AssemblyBlockContextAll<'input> = AssemblyBlockContext<'input>;


pub type AssemblyBlockContext<'input> = BaseParserRuleContext<'input,AssemblyBlockContextExt<'input>>;

#[derive(Clone)]
pub struct AssemblyBlockContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SolidityParserContext<'input> for AssemblyBlockContext<'input>{}

impl<'input,'a> Listenable<dyn SolidityListener<'input> + 'a> for AssemblyBlockContext<'input>{
	fn enter(&self,listener: &mut (dyn SolidityListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_assemblyBlock(self);
	}
}

impl<'input> CustomRuleContext<'input> for AssemblyBlockContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SolidityParserContextType;
	fn get_rule_index(&self) -> usize { RULE_assemblyBlock }
	//fn type_rule_index() -> usize where Self: Sized { RULE_assemblyBlock }
}
antlr_rust::type_id!{AssemblyBlockContextExt<'a>}

impl<'input> AssemblyBlockContextExt<'input>{
	fn new(parent: Option<Rc<dyn SolidityParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<AssemblyBlockContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,AssemblyBlockContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait AssemblyBlockContextAttrs<'input>: SolidityParserContext<'input> + BorrowMut<AssemblyBlockContextExt<'input>>{

fn assemblyItem_all(&self) ->  Vec<Rc<AssemblyItemContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn assemblyItem(&self, i: usize) -> Option<Rc<AssemblyItemContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> AssemblyBlockContextAttrs<'input> for AssemblyBlockContext<'input>{}

impl<'input, I, H> SolidityParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn assemblyBlock(&mut self,)
	-> Result<Rc<AssemblyBlockContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = AssemblyBlockContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 154, RULE_assemblyBlock);
        let mut _localctx: Rc<AssemblyBlockContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(966);
			recog.base.match_token(T__14,&mut recog.err_handler)?;

			recog.base.set_state(970);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while (((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << T__13) | (1usize << T__14) | (1usize << T__22) | (1usize << T__24) | (1usize << T__26) | (1usize << T__37) | (1usize << T__43) | (1usize << T__49) | (1usize << T__50) | (1usize << T__58) | (1usize << T__61))) != 0) || ((((_la - 66)) & !0x3f) == 0 && ((1usize << (_la - 66)) & ((1usize << (T__65 - 66)) | (1usize << (T__87 - 66)) | (1usize << (T__90 - 66)) | (1usize << (T__94 - 66)) | (1usize << (BooleanLiteral - 66)) | (1usize << (DecimalNumber - 66)) | (1usize << (HexNumber - 66)) | (1usize << (HexLiteralFragment - 66)) | (1usize << (BreakKeyword - 66)) | (1usize << (ContinueKeyword - 66)) | (1usize << (LeaveKeyword - 66)) | (1usize << (PayableKeyword - 66)) | (1usize << (GlobalKeyword - 66)) | (1usize << (ConstructorKeyword - 66)) | (1usize << (ReceiveKeyword - 66)) | (1usize << (Identifier - 66)) | (1usize << (StringLiteralFragment - 66)))) != 0) {
				{
				{
				/*InvokeRule assemblyItem*/
				recog.base.set_state(967);
				recog.assemblyItem()?;

				}
				}
				recog.base.set_state(972);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(973);
			recog.base.match_token(T__16,&mut recog.err_handler)?;

			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- assemblyItem ----------------
pub type AssemblyItemContextAll<'input> = AssemblyItemContext<'input>;


pub type AssemblyItemContext<'input> = BaseParserRuleContext<'input,AssemblyItemContextExt<'input>>;

#[derive(Clone)]
pub struct AssemblyItemContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SolidityParserContext<'input> for AssemblyItemContext<'input>{}

impl<'input,'a> Listenable<dyn SolidityListener<'input> + 'a> for AssemblyItemContext<'input>{
	fn enter(&self,listener: &mut (dyn SolidityListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_assemblyItem(self);
	}
}

impl<'input> CustomRuleContext<'input> for AssemblyItemContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SolidityParserContextType;
	fn get_rule_index(&self) -> usize { RULE_assemblyItem }
	//fn type_rule_index() -> usize where Self: Sized { RULE_assemblyItem }
}
antlr_rust::type_id!{AssemblyItemContextExt<'a>}

impl<'input> AssemblyItemContextExt<'input>{
	fn new(parent: Option<Rc<dyn SolidityParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<AssemblyItemContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,AssemblyItemContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait AssemblyItemContextAttrs<'input>: SolidityParserContext<'input> + BorrowMut<AssemblyItemContextExt<'input>>{

fn identifier(&self) -> Option<Rc<IdentifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn assemblyBlock(&self) -> Option<Rc<AssemblyBlockContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn assemblyExpression(&self) -> Option<Rc<AssemblyExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn assemblyLocalDefinition(&self) -> Option<Rc<AssemblyLocalDefinitionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn assemblyAssignment(&self) -> Option<Rc<AssemblyAssignmentContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn assemblyStackAssignment(&self) -> Option<Rc<AssemblyStackAssignmentContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn labelDefinition(&self) -> Option<Rc<LabelDefinitionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn assemblySwitch(&self) -> Option<Rc<AssemblySwitchContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn assemblyFunctionDefinition(&self) -> Option<Rc<AssemblyFunctionDefinitionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn assemblyFor(&self) -> Option<Rc<AssemblyForContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn assemblyIf(&self) -> Option<Rc<AssemblyIfContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token BreakKeyword
/// Returns `None` if there is no child corresponding to token BreakKeyword
fn BreakKeyword(&self) -> Option<Rc<TerminalNode<'input,SolidityParserContextType>>> where Self:Sized{
	self.get_token(BreakKeyword, 0)
}
/// Retrieves first TerminalNode corresponding to token ContinueKeyword
/// Returns `None` if there is no child corresponding to token ContinueKeyword
fn ContinueKeyword(&self) -> Option<Rc<TerminalNode<'input,SolidityParserContextType>>> where Self:Sized{
	self.get_token(ContinueKeyword, 0)
}
/// Retrieves first TerminalNode corresponding to token LeaveKeyword
/// Returns `None` if there is no child corresponding to token LeaveKeyword
fn LeaveKeyword(&self) -> Option<Rc<TerminalNode<'input,SolidityParserContextType>>> where Self:Sized{
	self.get_token(LeaveKeyword, 0)
}
fn numberLiteral(&self) -> Option<Rc<NumberLiteralContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn stringLiteral(&self) -> Option<Rc<StringLiteralContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn hexLiteral(&self) -> Option<Rc<HexLiteralContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> AssemblyItemContextAttrs<'input> for AssemblyItemContext<'input>{}

impl<'input, I, H> SolidityParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn assemblyItem(&mut self,)
	-> Result<Rc<AssemblyItemContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = AssemblyItemContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 156, RULE_assemblyItem);
        let mut _localctx: Rc<AssemblyItemContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			recog.base.set_state(992);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(103,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule identifier*/
					recog.base.set_state(975);
					recog.identifier()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule assemblyBlock*/
					recog.base.set_state(976);
					recog.assemblyBlock()?;

					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					/*InvokeRule assemblyExpression*/
					recog.base.set_state(977);
					recog.assemblyExpression()?;

					}
				}
			,
				4 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 4);
					recog.base.enter_outer_alt(None, 4);
					{
					/*InvokeRule assemblyLocalDefinition*/
					recog.base.set_state(978);
					recog.assemblyLocalDefinition()?;

					}
				}
			,
				5 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 5);
					recog.base.enter_outer_alt(None, 5);
					{
					/*InvokeRule assemblyAssignment*/
					recog.base.set_state(979);
					recog.assemblyAssignment()?;

					}
				}
			,
				6 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 6);
					recog.base.enter_outer_alt(None, 6);
					{
					/*InvokeRule assemblyStackAssignment*/
					recog.base.set_state(980);
					recog.assemblyStackAssignment()?;

					}
				}
			,
				7 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 7);
					recog.base.enter_outer_alt(None, 7);
					{
					/*InvokeRule labelDefinition*/
					recog.base.set_state(981);
					recog.labelDefinition()?;

					}
				}
			,
				8 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 8);
					recog.base.enter_outer_alt(None, 8);
					{
					/*InvokeRule assemblySwitch*/
					recog.base.set_state(982);
					recog.assemblySwitch()?;

					}
				}
			,
				9 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 9);
					recog.base.enter_outer_alt(None, 9);
					{
					/*InvokeRule assemblyFunctionDefinition*/
					recog.base.set_state(983);
					recog.assemblyFunctionDefinition()?;

					}
				}
			,
				10 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 10);
					recog.base.enter_outer_alt(None, 10);
					{
					/*InvokeRule assemblyFor*/
					recog.base.set_state(984);
					recog.assemblyFor()?;

					}
				}
			,
				11 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 11);
					recog.base.enter_outer_alt(None, 11);
					{
					/*InvokeRule assemblyIf*/
					recog.base.set_state(985);
					recog.assemblyIf()?;

					}
				}
			,
				12 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 12);
					recog.base.enter_outer_alt(None, 12);
					{
					recog.base.set_state(986);
					recog.base.match_token(BreakKeyword,&mut recog.err_handler)?;

					}
				}
			,
				13 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 13);
					recog.base.enter_outer_alt(None, 13);
					{
					recog.base.set_state(987);
					recog.base.match_token(ContinueKeyword,&mut recog.err_handler)?;

					}
				}
			,
				14 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 14);
					recog.base.enter_outer_alt(None, 14);
					{
					recog.base.set_state(988);
					recog.base.match_token(LeaveKeyword,&mut recog.err_handler)?;

					}
				}
			,
				15 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 15);
					recog.base.enter_outer_alt(None, 15);
					{
					/*InvokeRule numberLiteral*/
					recog.base.set_state(989);
					recog.numberLiteral()?;

					}
				}
			,
				16 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 16);
					recog.base.enter_outer_alt(None, 16);
					{
					/*InvokeRule stringLiteral*/
					recog.base.set_state(990);
					recog.stringLiteral()?;

					}
				}
			,
				17 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 17);
					recog.base.enter_outer_alt(None, 17);
					{
					/*InvokeRule hexLiteral*/
					recog.base.set_state(991);
					recog.hexLiteral()?;

					}
				}

				_ => {}
			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- assemblyExpression ----------------
pub type AssemblyExpressionContextAll<'input> = AssemblyExpressionContext<'input>;


pub type AssemblyExpressionContext<'input> = BaseParserRuleContext<'input,AssemblyExpressionContextExt<'input>>;

#[derive(Clone)]
pub struct AssemblyExpressionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SolidityParserContext<'input> for AssemblyExpressionContext<'input>{}

impl<'input,'a> Listenable<dyn SolidityListener<'input> + 'a> for AssemblyExpressionContext<'input>{
	fn enter(&self,listener: &mut (dyn SolidityListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_assemblyExpression(self);
	}
}

impl<'input> CustomRuleContext<'input> for AssemblyExpressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SolidityParserContextType;
	fn get_rule_index(&self) -> usize { RULE_assemblyExpression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_assemblyExpression }
}
antlr_rust::type_id!{AssemblyExpressionContextExt<'a>}

impl<'input> AssemblyExpressionContextExt<'input>{
	fn new(parent: Option<Rc<dyn SolidityParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<AssemblyExpressionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,AssemblyExpressionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait AssemblyExpressionContextAttrs<'input>: SolidityParserContext<'input> + BorrowMut<AssemblyExpressionContextExt<'input>>{

fn assemblyCall(&self) -> Option<Rc<AssemblyCallContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn assemblyLiteral(&self) -> Option<Rc<AssemblyLiteralContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn assemblyMember(&self) -> Option<Rc<AssemblyMemberContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> AssemblyExpressionContextAttrs<'input> for AssemblyExpressionContext<'input>{}

impl<'input, I, H> SolidityParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn assemblyExpression(&mut self,)
	-> Result<Rc<AssemblyExpressionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = AssemblyExpressionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 158, RULE_assemblyExpression);
        let mut _localctx: Rc<AssemblyExpressionContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			recog.base.set_state(997);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(104,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule assemblyCall*/
					recog.base.set_state(994);
					recog.assemblyCall()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule assemblyLiteral*/
					recog.base.set_state(995);
					recog.assemblyLiteral()?;

					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					/*InvokeRule assemblyMember*/
					recog.base.set_state(996);
					recog.assemblyMember()?;

					}
				}

				_ => {}
			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- assemblyMember ----------------
pub type AssemblyMemberContextAll<'input> = AssemblyMemberContext<'input>;


pub type AssemblyMemberContext<'input> = BaseParserRuleContext<'input,AssemblyMemberContextExt<'input>>;

#[derive(Clone)]
pub struct AssemblyMemberContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SolidityParserContext<'input> for AssemblyMemberContext<'input>{}

impl<'input,'a> Listenable<dyn SolidityListener<'input> + 'a> for AssemblyMemberContext<'input>{
	fn enter(&self,listener: &mut (dyn SolidityListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_assemblyMember(self);
	}
}

impl<'input> CustomRuleContext<'input> for AssemblyMemberContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SolidityParserContextType;
	fn get_rule_index(&self) -> usize { RULE_assemblyMember }
	//fn type_rule_index() -> usize where Self: Sized { RULE_assemblyMember }
}
antlr_rust::type_id!{AssemblyMemberContextExt<'a>}

impl<'input> AssemblyMemberContextExt<'input>{
	fn new(parent: Option<Rc<dyn SolidityParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<AssemblyMemberContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,AssemblyMemberContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait AssemblyMemberContextAttrs<'input>: SolidityParserContext<'input> + BorrowMut<AssemblyMemberContextExt<'input>>{

fn identifier_all(&self) ->  Vec<Rc<IdentifierContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn identifier(&self, i: usize) -> Option<Rc<IdentifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> AssemblyMemberContextAttrs<'input> for AssemblyMemberContext<'input>{}

impl<'input, I, H> SolidityParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn assemblyMember(&mut self,)
	-> Result<Rc<AssemblyMemberContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = AssemblyMemberContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 160, RULE_assemblyMember);
        let mut _localctx: Rc<AssemblyMemberContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule identifier*/
			recog.base.set_state(999);
			recog.identifier()?;

			recog.base.set_state(1000);
			recog.base.match_token(T__44,&mut recog.err_handler)?;

			/*InvokeRule identifier*/
			recog.base.set_state(1001);
			recog.identifier()?;

			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- assemblyCall ----------------
pub type AssemblyCallContextAll<'input> = AssemblyCallContext<'input>;


pub type AssemblyCallContext<'input> = BaseParserRuleContext<'input,AssemblyCallContextExt<'input>>;

#[derive(Clone)]
pub struct AssemblyCallContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SolidityParserContext<'input> for AssemblyCallContext<'input>{}

impl<'input,'a> Listenable<dyn SolidityListener<'input> + 'a> for AssemblyCallContext<'input>{
	fn enter(&self,listener: &mut (dyn SolidityListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_assemblyCall(self);
	}
}

impl<'input> CustomRuleContext<'input> for AssemblyCallContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SolidityParserContextType;
	fn get_rule_index(&self) -> usize { RULE_assemblyCall }
	//fn type_rule_index() -> usize where Self: Sized { RULE_assemblyCall }
}
antlr_rust::type_id!{AssemblyCallContextExt<'a>}

impl<'input> AssemblyCallContextExt<'input>{
	fn new(parent: Option<Rc<dyn SolidityParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<AssemblyCallContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,AssemblyCallContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait AssemblyCallContextAttrs<'input>: SolidityParserContext<'input> + BorrowMut<AssemblyCallContextExt<'input>>{

fn identifier(&self) -> Option<Rc<IdentifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn assemblyExpression_all(&self) ->  Vec<Rc<AssemblyExpressionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn assemblyExpression(&self, i: usize) -> Option<Rc<AssemblyExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> AssemblyCallContextAttrs<'input> for AssemblyCallContext<'input>{}

impl<'input, I, H> SolidityParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn assemblyCall(&mut self,)
	-> Result<Rc<AssemblyCallContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = AssemblyCallContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 162, RULE_assemblyCall);
        let mut _localctx: Rc<AssemblyCallContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(1007);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(105,&mut recog.base)? {
				1 =>{
					{
					recog.base.set_state(1003);
					recog.base.match_token(T__58,&mut recog.err_handler)?;

					}
				}
			,
				2 =>{
					{
					recog.base.set_state(1004);
					recog.base.match_token(T__43,&mut recog.err_handler)?;

					}
				}
			,
				3 =>{
					{
					recog.base.set_state(1005);
					recog.base.match_token(T__65,&mut recog.err_handler)?;

					}
				}
			,
				4 =>{
					{
					/*InvokeRule identifier*/
					recog.base.set_state(1006);
					recog.identifier()?;

					}
				}

				_ => {}
			}
			recog.base.set_state(1021);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(108,&mut recog.base)? {
				x if x == 1=>{
					{
					recog.base.set_state(1009);
					recog.base.match_token(T__22,&mut recog.err_handler)?;

					recog.base.set_state(1011);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if (((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << T__13) | (1usize << T__24) | (1usize << T__43) | (1usize << T__49) | (1usize << T__58) | (1usize << T__61))) != 0) || ((((_la - 66)) & !0x3f) == 0 && ((1usize << (_la - 66)) & ((1usize << (T__65 - 66)) | (1usize << (T__94 - 66)) | (1usize << (BooleanLiteral - 66)) | (1usize << (DecimalNumber - 66)) | (1usize << (HexNumber - 66)) | (1usize << (HexLiteralFragment - 66)) | (1usize << (LeaveKeyword - 66)) | (1usize << (PayableKeyword - 66)) | (1usize << (GlobalKeyword - 66)) | (1usize << (ConstructorKeyword - 66)) | (1usize << (ReceiveKeyword - 66)) | (1usize << (Identifier - 66)) | (1usize << (StringLiteralFragment - 66)))) != 0) {
						{
						/*InvokeRule assemblyExpression*/
						recog.base.set_state(1010);
						recog.assemblyExpression()?;

						}
					}

					recog.base.set_state(1017);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					while _la==T__15 {
						{
						{
						recog.base.set_state(1013);
						recog.base.match_token(T__15,&mut recog.err_handler)?;

						/*InvokeRule assemblyExpression*/
						recog.base.set_state(1014);
						recog.assemblyExpression()?;

						}
						}
						recog.base.set_state(1019);
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
					}
					recog.base.set_state(1020);
					recog.base.match_token(T__23,&mut recog.err_handler)?;

					}
				}

				_ => {}
			}
			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- assemblyLocalDefinition ----------------
pub type AssemblyLocalDefinitionContextAll<'input> = AssemblyLocalDefinitionContext<'input>;


pub type AssemblyLocalDefinitionContext<'input> = BaseParserRuleContext<'input,AssemblyLocalDefinitionContextExt<'input>>;

#[derive(Clone)]
pub struct AssemblyLocalDefinitionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SolidityParserContext<'input> for AssemblyLocalDefinitionContext<'input>{}

impl<'input,'a> Listenable<dyn SolidityListener<'input> + 'a> for AssemblyLocalDefinitionContext<'input>{
	fn enter(&self,listener: &mut (dyn SolidityListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_assemblyLocalDefinition(self);
	}
}

impl<'input> CustomRuleContext<'input> for AssemblyLocalDefinitionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SolidityParserContextType;
	fn get_rule_index(&self) -> usize { RULE_assemblyLocalDefinition }
	//fn type_rule_index() -> usize where Self: Sized { RULE_assemblyLocalDefinition }
}
antlr_rust::type_id!{AssemblyLocalDefinitionContextExt<'a>}

impl<'input> AssemblyLocalDefinitionContextExt<'input>{
	fn new(parent: Option<Rc<dyn SolidityParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<AssemblyLocalDefinitionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,AssemblyLocalDefinitionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait AssemblyLocalDefinitionContextAttrs<'input>: SolidityParserContext<'input> + BorrowMut<AssemblyLocalDefinitionContextExt<'input>>{

fn assemblyIdentifierOrList(&self) -> Option<Rc<AssemblyIdentifierOrListContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn assemblyExpression(&self) -> Option<Rc<AssemblyExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> AssemblyLocalDefinitionContextAttrs<'input> for AssemblyLocalDefinitionContext<'input>{}

impl<'input, I, H> SolidityParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn assemblyLocalDefinition(&mut self,)
	-> Result<Rc<AssemblyLocalDefinitionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = AssemblyLocalDefinitionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 164, RULE_assemblyLocalDefinition);
        let mut _localctx: Rc<AssemblyLocalDefinitionContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(1023);
			recog.base.match_token(T__87,&mut recog.err_handler)?;

			/*InvokeRule assemblyIdentifierOrList*/
			recog.base.set_state(1024);
			recog.assemblyIdentifierOrList()?;

			recog.base.set_state(1027);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==T__88 {
				{
				recog.base.set_state(1025);
				recog.base.match_token(T__88,&mut recog.err_handler)?;

				/*InvokeRule assemblyExpression*/
				recog.base.set_state(1026);
				recog.assemblyExpression()?;

				}
			}

			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- assemblyAssignment ----------------
pub type AssemblyAssignmentContextAll<'input> = AssemblyAssignmentContext<'input>;


pub type AssemblyAssignmentContext<'input> = BaseParserRuleContext<'input,AssemblyAssignmentContextExt<'input>>;

#[derive(Clone)]
pub struct AssemblyAssignmentContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SolidityParserContext<'input> for AssemblyAssignmentContext<'input>{}

impl<'input,'a> Listenable<dyn SolidityListener<'input> + 'a> for AssemblyAssignmentContext<'input>{
	fn enter(&self,listener: &mut (dyn SolidityListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_assemblyAssignment(self);
	}
}

impl<'input> CustomRuleContext<'input> for AssemblyAssignmentContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SolidityParserContextType;
	fn get_rule_index(&self) -> usize { RULE_assemblyAssignment }
	//fn type_rule_index() -> usize where Self: Sized { RULE_assemblyAssignment }
}
antlr_rust::type_id!{AssemblyAssignmentContextExt<'a>}

impl<'input> AssemblyAssignmentContextExt<'input>{
	fn new(parent: Option<Rc<dyn SolidityParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<AssemblyAssignmentContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,AssemblyAssignmentContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait AssemblyAssignmentContextAttrs<'input>: SolidityParserContext<'input> + BorrowMut<AssemblyAssignmentContextExt<'input>>{

fn assemblyIdentifierOrList(&self) -> Option<Rc<AssemblyIdentifierOrListContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn assemblyExpression(&self) -> Option<Rc<AssemblyExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> AssemblyAssignmentContextAttrs<'input> for AssemblyAssignmentContext<'input>{}

impl<'input, I, H> SolidityParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn assemblyAssignment(&mut self,)
	-> Result<Rc<AssemblyAssignmentContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = AssemblyAssignmentContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 166, RULE_assemblyAssignment);
        let mut _localctx: Rc<AssemblyAssignmentContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule assemblyIdentifierOrList*/
			recog.base.set_state(1029);
			recog.assemblyIdentifierOrList()?;

			recog.base.set_state(1030);
			recog.base.match_token(T__88,&mut recog.err_handler)?;

			/*InvokeRule assemblyExpression*/
			recog.base.set_state(1031);
			recog.assemblyExpression()?;

			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- assemblyIdentifierOrList ----------------
pub type AssemblyIdentifierOrListContextAll<'input> = AssemblyIdentifierOrListContext<'input>;


pub type AssemblyIdentifierOrListContext<'input> = BaseParserRuleContext<'input,AssemblyIdentifierOrListContextExt<'input>>;

#[derive(Clone)]
pub struct AssemblyIdentifierOrListContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SolidityParserContext<'input> for AssemblyIdentifierOrListContext<'input>{}

impl<'input,'a> Listenable<dyn SolidityListener<'input> + 'a> for AssemblyIdentifierOrListContext<'input>{
	fn enter(&self,listener: &mut (dyn SolidityListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_assemblyIdentifierOrList(self);
	}
}

impl<'input> CustomRuleContext<'input> for AssemblyIdentifierOrListContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SolidityParserContextType;
	fn get_rule_index(&self) -> usize { RULE_assemblyIdentifierOrList }
	//fn type_rule_index() -> usize where Self: Sized { RULE_assemblyIdentifierOrList }
}
antlr_rust::type_id!{AssemblyIdentifierOrListContextExt<'a>}

impl<'input> AssemblyIdentifierOrListContextExt<'input>{
	fn new(parent: Option<Rc<dyn SolidityParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<AssemblyIdentifierOrListContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,AssemblyIdentifierOrListContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait AssemblyIdentifierOrListContextAttrs<'input>: SolidityParserContext<'input> + BorrowMut<AssemblyIdentifierOrListContextExt<'input>>{

fn identifier(&self) -> Option<Rc<IdentifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn assemblyMember(&self) -> Option<Rc<AssemblyMemberContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn assemblyIdentifierList(&self) -> Option<Rc<AssemblyIdentifierListContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> AssemblyIdentifierOrListContextAttrs<'input> for AssemblyIdentifierOrListContext<'input>{}

impl<'input, I, H> SolidityParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn assemblyIdentifierOrList(&mut self,)
	-> Result<Rc<AssemblyIdentifierOrListContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = AssemblyIdentifierOrListContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 168, RULE_assemblyIdentifierOrList);
        let mut _localctx: Rc<AssemblyIdentifierOrListContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			recog.base.set_state(1040);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(110,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule identifier*/
					recog.base.set_state(1033);
					recog.identifier()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule assemblyMember*/
					recog.base.set_state(1034);
					recog.assemblyMember()?;

					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					/*InvokeRule assemblyIdentifierList*/
					recog.base.set_state(1035);
					recog.assemblyIdentifierList()?;

					}
				}
			,
				4 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 4);
					recog.base.enter_outer_alt(None, 4);
					{
					recog.base.set_state(1036);
					recog.base.match_token(T__22,&mut recog.err_handler)?;

					/*InvokeRule assemblyIdentifierList*/
					recog.base.set_state(1037);
					recog.assemblyIdentifierList()?;

					recog.base.set_state(1038);
					recog.base.match_token(T__23,&mut recog.err_handler)?;

					}
				}

				_ => {}
			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- assemblyIdentifierList ----------------
pub type AssemblyIdentifierListContextAll<'input> = AssemblyIdentifierListContext<'input>;


pub type AssemblyIdentifierListContext<'input> = BaseParserRuleContext<'input,AssemblyIdentifierListContextExt<'input>>;

#[derive(Clone)]
pub struct AssemblyIdentifierListContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SolidityParserContext<'input> for AssemblyIdentifierListContext<'input>{}

impl<'input,'a> Listenable<dyn SolidityListener<'input> + 'a> for AssemblyIdentifierListContext<'input>{
	fn enter(&self,listener: &mut (dyn SolidityListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_assemblyIdentifierList(self);
	}
}

impl<'input> CustomRuleContext<'input> for AssemblyIdentifierListContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SolidityParserContextType;
	fn get_rule_index(&self) -> usize { RULE_assemblyIdentifierList }
	//fn type_rule_index() -> usize where Self: Sized { RULE_assemblyIdentifierList }
}
antlr_rust::type_id!{AssemblyIdentifierListContextExt<'a>}

impl<'input> AssemblyIdentifierListContextExt<'input>{
	fn new(parent: Option<Rc<dyn SolidityParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<AssemblyIdentifierListContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,AssemblyIdentifierListContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait AssemblyIdentifierListContextAttrs<'input>: SolidityParserContext<'input> + BorrowMut<AssemblyIdentifierListContextExt<'input>>{

fn identifier_all(&self) ->  Vec<Rc<IdentifierContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn identifier(&self, i: usize) -> Option<Rc<IdentifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> AssemblyIdentifierListContextAttrs<'input> for AssemblyIdentifierListContext<'input>{}

impl<'input, I, H> SolidityParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn assemblyIdentifierList(&mut self,)
	-> Result<Rc<AssemblyIdentifierListContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = AssemblyIdentifierListContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 170, RULE_assemblyIdentifierList);
        let mut _localctx: Rc<AssemblyIdentifierListContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule identifier*/
			recog.base.set_state(1042);
			recog.identifier()?;

			recog.base.set_state(1047);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==T__15 {
				{
				{
				recog.base.set_state(1043);
				recog.base.match_token(T__15,&mut recog.err_handler)?;

				/*InvokeRule identifier*/
				recog.base.set_state(1044);
				recog.identifier()?;

				}
				}
				recog.base.set_state(1049);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- assemblyStackAssignment ----------------
pub type AssemblyStackAssignmentContextAll<'input> = AssemblyStackAssignmentContext<'input>;


pub type AssemblyStackAssignmentContext<'input> = BaseParserRuleContext<'input,AssemblyStackAssignmentContextExt<'input>>;

#[derive(Clone)]
pub struct AssemblyStackAssignmentContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SolidityParserContext<'input> for AssemblyStackAssignmentContext<'input>{}

impl<'input,'a> Listenable<dyn SolidityListener<'input> + 'a> for AssemblyStackAssignmentContext<'input>{
	fn enter(&self,listener: &mut (dyn SolidityListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_assemblyStackAssignment(self);
	}
}

impl<'input> CustomRuleContext<'input> for AssemblyStackAssignmentContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SolidityParserContextType;
	fn get_rule_index(&self) -> usize { RULE_assemblyStackAssignment }
	//fn type_rule_index() -> usize where Self: Sized { RULE_assemblyStackAssignment }
}
antlr_rust::type_id!{AssemblyStackAssignmentContextExt<'a>}

impl<'input> AssemblyStackAssignmentContextExt<'input>{
	fn new(parent: Option<Rc<dyn SolidityParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<AssemblyStackAssignmentContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,AssemblyStackAssignmentContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait AssemblyStackAssignmentContextAttrs<'input>: SolidityParserContext<'input> + BorrowMut<AssemblyStackAssignmentContextExt<'input>>{

fn assemblyExpression(&self) -> Option<Rc<AssemblyExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn identifier(&self) -> Option<Rc<IdentifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> AssemblyStackAssignmentContextAttrs<'input> for AssemblyStackAssignmentContext<'input>{}

impl<'input, I, H> SolidityParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn assemblyStackAssignment(&mut self,)
	-> Result<Rc<AssemblyStackAssignmentContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = AssemblyStackAssignmentContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 172, RULE_assemblyStackAssignment);
        let mut _localctx: Rc<AssemblyStackAssignmentContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule assemblyExpression*/
			recog.base.set_state(1050);
			recog.assemblyExpression()?;

			recog.base.set_state(1051);
			recog.base.match_token(T__89,&mut recog.err_handler)?;

			/*InvokeRule identifier*/
			recog.base.set_state(1052);
			recog.identifier()?;

			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- labelDefinition ----------------
pub type LabelDefinitionContextAll<'input> = LabelDefinitionContext<'input>;


pub type LabelDefinitionContext<'input> = BaseParserRuleContext<'input,LabelDefinitionContextExt<'input>>;

#[derive(Clone)]
pub struct LabelDefinitionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SolidityParserContext<'input> for LabelDefinitionContext<'input>{}

impl<'input,'a> Listenable<dyn SolidityListener<'input> + 'a> for LabelDefinitionContext<'input>{
	fn enter(&self,listener: &mut (dyn SolidityListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_labelDefinition(self);
	}
}

impl<'input> CustomRuleContext<'input> for LabelDefinitionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SolidityParserContextType;
	fn get_rule_index(&self) -> usize { RULE_labelDefinition }
	//fn type_rule_index() -> usize where Self: Sized { RULE_labelDefinition }
}
antlr_rust::type_id!{LabelDefinitionContextExt<'a>}

impl<'input> LabelDefinitionContextExt<'input>{
	fn new(parent: Option<Rc<dyn SolidityParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<LabelDefinitionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,LabelDefinitionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait LabelDefinitionContextAttrs<'input>: SolidityParserContext<'input> + BorrowMut<LabelDefinitionContextExt<'input>>{

fn identifier(&self) -> Option<Rc<IdentifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> LabelDefinitionContextAttrs<'input> for LabelDefinitionContext<'input>{}

impl<'input, I, H> SolidityParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn labelDefinition(&mut self,)
	-> Result<Rc<LabelDefinitionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = LabelDefinitionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 174, RULE_labelDefinition);
        let mut _localctx: Rc<LabelDefinitionContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule identifier*/
			recog.base.set_state(1054);
			recog.identifier()?;

			recog.base.set_state(1055);
			recog.base.match_token(T__69,&mut recog.err_handler)?;

			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- assemblySwitch ----------------
pub type AssemblySwitchContextAll<'input> = AssemblySwitchContext<'input>;


pub type AssemblySwitchContext<'input> = BaseParserRuleContext<'input,AssemblySwitchContextExt<'input>>;

#[derive(Clone)]
pub struct AssemblySwitchContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SolidityParserContext<'input> for AssemblySwitchContext<'input>{}

impl<'input,'a> Listenable<dyn SolidityListener<'input> + 'a> for AssemblySwitchContext<'input>{
	fn enter(&self,listener: &mut (dyn SolidityListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_assemblySwitch(self);
	}
}

impl<'input> CustomRuleContext<'input> for AssemblySwitchContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SolidityParserContextType;
	fn get_rule_index(&self) -> usize { RULE_assemblySwitch }
	//fn type_rule_index() -> usize where Self: Sized { RULE_assemblySwitch }
}
antlr_rust::type_id!{AssemblySwitchContextExt<'a>}

impl<'input> AssemblySwitchContextExt<'input>{
	fn new(parent: Option<Rc<dyn SolidityParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<AssemblySwitchContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,AssemblySwitchContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait AssemblySwitchContextAttrs<'input>: SolidityParserContext<'input> + BorrowMut<AssemblySwitchContextExt<'input>>{

fn assemblyExpression(&self) -> Option<Rc<AssemblyExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn assemblyCase_all(&self) ->  Vec<Rc<AssemblyCaseContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn assemblyCase(&self, i: usize) -> Option<Rc<AssemblyCaseContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> AssemblySwitchContextAttrs<'input> for AssemblySwitchContext<'input>{}

impl<'input, I, H> SolidityParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn assemblySwitch(&mut self,)
	-> Result<Rc<AssemblySwitchContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = AssemblySwitchContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 176, RULE_assemblySwitch);
        let mut _localctx: Rc<AssemblySwitchContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(1057);
			recog.base.match_token(T__90,&mut recog.err_handler)?;

			/*InvokeRule assemblyExpression*/
			recog.base.set_state(1058);
			recog.assemblyExpression()?;

			recog.base.set_state(1062);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==T__91 || _la==T__92 {
				{
				{
				/*InvokeRule assemblyCase*/
				recog.base.set_state(1059);
				recog.assemblyCase()?;

				}
				}
				recog.base.set_state(1064);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- assemblyCase ----------------
pub type AssemblyCaseContextAll<'input> = AssemblyCaseContext<'input>;


pub type AssemblyCaseContext<'input> = BaseParserRuleContext<'input,AssemblyCaseContextExt<'input>>;

#[derive(Clone)]
pub struct AssemblyCaseContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SolidityParserContext<'input> for AssemblyCaseContext<'input>{}

impl<'input,'a> Listenable<dyn SolidityListener<'input> + 'a> for AssemblyCaseContext<'input>{
	fn enter(&self,listener: &mut (dyn SolidityListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_assemblyCase(self);
	}
}

impl<'input> CustomRuleContext<'input> for AssemblyCaseContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SolidityParserContextType;
	fn get_rule_index(&self) -> usize { RULE_assemblyCase }
	//fn type_rule_index() -> usize where Self: Sized { RULE_assemblyCase }
}
antlr_rust::type_id!{AssemblyCaseContextExt<'a>}

impl<'input> AssemblyCaseContextExt<'input>{
	fn new(parent: Option<Rc<dyn SolidityParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<AssemblyCaseContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,AssemblyCaseContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait AssemblyCaseContextAttrs<'input>: SolidityParserContext<'input> + BorrowMut<AssemblyCaseContextExt<'input>>{

fn assemblyLiteral(&self) -> Option<Rc<AssemblyLiteralContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn assemblyBlock(&self) -> Option<Rc<AssemblyBlockContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> AssemblyCaseContextAttrs<'input> for AssemblyCaseContext<'input>{}

impl<'input, I, H> SolidityParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn assemblyCase(&mut self,)
	-> Result<Rc<AssemblyCaseContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = AssemblyCaseContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 178, RULE_assemblyCase);
        let mut _localctx: Rc<AssemblyCaseContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			recog.base.set_state(1071);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 T__91 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(1065);
					recog.base.match_token(T__91,&mut recog.err_handler)?;

					/*InvokeRule assemblyLiteral*/
					recog.base.set_state(1066);
					recog.assemblyLiteral()?;

					/*InvokeRule assemblyBlock*/
					recog.base.set_state(1067);
					recog.assemblyBlock()?;

					}
				}

			 T__92 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(1069);
					recog.base.match_token(T__92,&mut recog.err_handler)?;

					/*InvokeRule assemblyBlock*/
					recog.base.set_state(1070);
					recog.assemblyBlock()?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- assemblyFunctionDefinition ----------------
pub type AssemblyFunctionDefinitionContextAll<'input> = AssemblyFunctionDefinitionContext<'input>;


pub type AssemblyFunctionDefinitionContext<'input> = BaseParserRuleContext<'input,AssemblyFunctionDefinitionContextExt<'input>>;

#[derive(Clone)]
pub struct AssemblyFunctionDefinitionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SolidityParserContext<'input> for AssemblyFunctionDefinitionContext<'input>{}

impl<'input,'a> Listenable<dyn SolidityListener<'input> + 'a> for AssemblyFunctionDefinitionContext<'input>{
	fn enter(&self,listener: &mut (dyn SolidityListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_assemblyFunctionDefinition(self);
	}
}

impl<'input> CustomRuleContext<'input> for AssemblyFunctionDefinitionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SolidityParserContextType;
	fn get_rule_index(&self) -> usize { RULE_assemblyFunctionDefinition }
	//fn type_rule_index() -> usize where Self: Sized { RULE_assemblyFunctionDefinition }
}
antlr_rust::type_id!{AssemblyFunctionDefinitionContextExt<'a>}

impl<'input> AssemblyFunctionDefinitionContextExt<'input>{
	fn new(parent: Option<Rc<dyn SolidityParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<AssemblyFunctionDefinitionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,AssemblyFunctionDefinitionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait AssemblyFunctionDefinitionContextAttrs<'input>: SolidityParserContext<'input> + BorrowMut<AssemblyFunctionDefinitionContextExt<'input>>{

fn identifier(&self) -> Option<Rc<IdentifierContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn assemblyBlock(&self) -> Option<Rc<AssemblyBlockContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn assemblyIdentifierList(&self) -> Option<Rc<AssemblyIdentifierListContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn assemblyFunctionReturns(&self) -> Option<Rc<AssemblyFunctionReturnsContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> AssemblyFunctionDefinitionContextAttrs<'input> for AssemblyFunctionDefinitionContext<'input>{}

impl<'input, I, H> SolidityParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn assemblyFunctionDefinition(&mut self,)
	-> Result<Rc<AssemblyFunctionDefinitionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = AssemblyFunctionDefinitionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 180, RULE_assemblyFunctionDefinition);
        let mut _localctx: Rc<AssemblyFunctionDefinitionContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(1073);
			recog.base.match_token(T__37,&mut recog.err_handler)?;

			/*InvokeRule identifier*/
			recog.base.set_state(1074);
			recog.identifier()?;

			recog.base.set_state(1075);
			recog.base.match_token(T__22,&mut recog.err_handler)?;

			recog.base.set_state(1077);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if (((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << T__13) | (1usize << T__24) | (1usize << T__43) | (1usize << T__49) | (1usize << T__61))) != 0) || ((((_la - 95)) & !0x3f) == 0 && ((1usize << (_la - 95)) & ((1usize << (T__94 - 95)) | (1usize << (LeaveKeyword - 95)) | (1usize << (PayableKeyword - 95)) | (1usize << (GlobalKeyword - 95)) | (1usize << (ConstructorKeyword - 95)) | (1usize << (ReceiveKeyword - 95)) | (1usize << (Identifier - 95)))) != 0) {
				{
				/*InvokeRule assemblyIdentifierList*/
				recog.base.set_state(1076);
				recog.assemblyIdentifierList()?;

				}
			}

			recog.base.set_state(1079);
			recog.base.match_token(T__23,&mut recog.err_handler)?;

			recog.base.set_state(1081);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==T__93 {
				{
				/*InvokeRule assemblyFunctionReturns*/
				recog.base.set_state(1080);
				recog.assemblyFunctionReturns()?;

				}
			}

			/*InvokeRule assemblyBlock*/
			recog.base.set_state(1083);
			recog.assemblyBlock()?;

			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- assemblyFunctionReturns ----------------
pub type AssemblyFunctionReturnsContextAll<'input> = AssemblyFunctionReturnsContext<'input>;


pub type AssemblyFunctionReturnsContext<'input> = BaseParserRuleContext<'input,AssemblyFunctionReturnsContextExt<'input>>;

#[derive(Clone)]
pub struct AssemblyFunctionReturnsContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SolidityParserContext<'input> for AssemblyFunctionReturnsContext<'input>{}

impl<'input,'a> Listenable<dyn SolidityListener<'input> + 'a> for AssemblyFunctionReturnsContext<'input>{
	fn enter(&self,listener: &mut (dyn SolidityListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_assemblyFunctionReturns(self);
	}
}

impl<'input> CustomRuleContext<'input> for AssemblyFunctionReturnsContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SolidityParserContextType;
	fn get_rule_index(&self) -> usize { RULE_assemblyFunctionReturns }
	//fn type_rule_index() -> usize where Self: Sized { RULE_assemblyFunctionReturns }
}
antlr_rust::type_id!{AssemblyFunctionReturnsContextExt<'a>}

impl<'input> AssemblyFunctionReturnsContextExt<'input>{
	fn new(parent: Option<Rc<dyn SolidityParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<AssemblyFunctionReturnsContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,AssemblyFunctionReturnsContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait AssemblyFunctionReturnsContextAttrs<'input>: SolidityParserContext<'input> + BorrowMut<AssemblyFunctionReturnsContextExt<'input>>{

fn assemblyIdentifierList(&self) -> Option<Rc<AssemblyIdentifierListContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> AssemblyFunctionReturnsContextAttrs<'input> for AssemblyFunctionReturnsContext<'input>{}

impl<'input, I, H> SolidityParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn assemblyFunctionReturns(&mut self,)
	-> Result<Rc<AssemblyFunctionReturnsContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = AssemblyFunctionReturnsContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 182, RULE_assemblyFunctionReturns);
        let mut _localctx: Rc<AssemblyFunctionReturnsContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			{
			recog.base.set_state(1085);
			recog.base.match_token(T__93,&mut recog.err_handler)?;

			/*InvokeRule assemblyIdentifierList*/
			recog.base.set_state(1086);
			recog.assemblyIdentifierList()?;

			}
			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- assemblyFor ----------------
pub type AssemblyForContextAll<'input> = AssemblyForContext<'input>;


pub type AssemblyForContext<'input> = BaseParserRuleContext<'input,AssemblyForContextExt<'input>>;

#[derive(Clone)]
pub struct AssemblyForContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SolidityParserContext<'input> for AssemblyForContext<'input>{}

impl<'input,'a> Listenable<dyn SolidityListener<'input> + 'a> for AssemblyForContext<'input>{
	fn enter(&self,listener: &mut (dyn SolidityListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_assemblyFor(self);
	}
}

impl<'input> CustomRuleContext<'input> for AssemblyForContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SolidityParserContextType;
	fn get_rule_index(&self) -> usize { RULE_assemblyFor }
	//fn type_rule_index() -> usize where Self: Sized { RULE_assemblyFor }
}
antlr_rust::type_id!{AssemblyForContextExt<'a>}

impl<'input> AssemblyForContextExt<'input>{
	fn new(parent: Option<Rc<dyn SolidityParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<AssemblyForContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,AssemblyForContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait AssemblyForContextAttrs<'input>: SolidityParserContext<'input> + BorrowMut<AssemblyForContextExt<'input>>{

fn assemblyExpression_all(&self) ->  Vec<Rc<AssemblyExpressionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn assemblyExpression(&self, i: usize) -> Option<Rc<AssemblyExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn assemblyBlock_all(&self) ->  Vec<Rc<AssemblyBlockContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn assemblyBlock(&self, i: usize) -> Option<Rc<AssemblyBlockContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> AssemblyForContextAttrs<'input> for AssemblyForContext<'input>{}

impl<'input, I, H> SolidityParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn assemblyFor(&mut self,)
	-> Result<Rc<AssemblyForContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = AssemblyForContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 184, RULE_assemblyFor);
        let mut _localctx: Rc<AssemblyForContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(1088);
			recog.base.match_token(T__26,&mut recog.err_handler)?;

			recog.base.set_state(1091);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 T__14 
				=> {
					{
					/*InvokeRule assemblyBlock*/
					recog.base.set_state(1089);
					recog.assemblyBlock()?;

					}
				}

			 T__13 | T__24 | T__43 | T__49 | T__58 | T__61 | T__65 | T__94 | BooleanLiteral |
			 DecimalNumber | HexNumber | HexLiteralFragment | LeaveKeyword | PayableKeyword |
			 GlobalKeyword | ConstructorKeyword | ReceiveKeyword | Identifier | StringLiteralFragment 
				=> {
					{
					/*InvokeRule assemblyExpression*/
					recog.base.set_state(1090);
					recog.assemblyExpression()?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			/*InvokeRule assemblyExpression*/
			recog.base.set_state(1093);
			recog.assemblyExpression()?;

			recog.base.set_state(1096);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 T__14 
				=> {
					{
					/*InvokeRule assemblyBlock*/
					recog.base.set_state(1094);
					recog.assemblyBlock()?;

					}
				}

			 T__13 | T__24 | T__43 | T__49 | T__58 | T__61 | T__65 | T__94 | BooleanLiteral |
			 DecimalNumber | HexNumber | HexLiteralFragment | LeaveKeyword | PayableKeyword |
			 GlobalKeyword | ConstructorKeyword | ReceiveKeyword | Identifier | StringLiteralFragment 
				=> {
					{
					/*InvokeRule assemblyExpression*/
					recog.base.set_state(1095);
					recog.assemblyExpression()?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			/*InvokeRule assemblyBlock*/
			recog.base.set_state(1098);
			recog.assemblyBlock()?;

			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- assemblyIf ----------------
pub type AssemblyIfContextAll<'input> = AssemblyIfContext<'input>;


pub type AssemblyIfContext<'input> = BaseParserRuleContext<'input,AssemblyIfContextExt<'input>>;

#[derive(Clone)]
pub struct AssemblyIfContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SolidityParserContext<'input> for AssemblyIfContext<'input>{}

impl<'input,'a> Listenable<dyn SolidityListener<'input> + 'a> for AssemblyIfContext<'input>{
	fn enter(&self,listener: &mut (dyn SolidityListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_assemblyIf(self);
	}
}

impl<'input> CustomRuleContext<'input> for AssemblyIfContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SolidityParserContextType;
	fn get_rule_index(&self) -> usize { RULE_assemblyIf }
	//fn type_rule_index() -> usize where Self: Sized { RULE_assemblyIf }
}
antlr_rust::type_id!{AssemblyIfContextExt<'a>}

impl<'input> AssemblyIfContextExt<'input>{
	fn new(parent: Option<Rc<dyn SolidityParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<AssemblyIfContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,AssemblyIfContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait AssemblyIfContextAttrs<'input>: SolidityParserContext<'input> + BorrowMut<AssemblyIfContextExt<'input>>{

fn assemblyExpression(&self) -> Option<Rc<AssemblyExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn assemblyBlock(&self) -> Option<Rc<AssemblyBlockContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> AssemblyIfContextAttrs<'input> for AssemblyIfContext<'input>{}

impl<'input, I, H> SolidityParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn assemblyIf(&mut self,)
	-> Result<Rc<AssemblyIfContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = AssemblyIfContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 186, RULE_assemblyIf);
        let mut _localctx: Rc<AssemblyIfContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(1100);
			recog.base.match_token(T__50,&mut recog.err_handler)?;

			/*InvokeRule assemblyExpression*/
			recog.base.set_state(1101);
			recog.assemblyExpression()?;

			/*InvokeRule assemblyBlock*/
			recog.base.set_state(1102);
			recog.assemblyBlock()?;

			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- assemblyLiteral ----------------
pub type AssemblyLiteralContextAll<'input> = AssemblyLiteralContext<'input>;


pub type AssemblyLiteralContext<'input> = BaseParserRuleContext<'input,AssemblyLiteralContextExt<'input>>;

#[derive(Clone)]
pub struct AssemblyLiteralContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SolidityParserContext<'input> for AssemblyLiteralContext<'input>{}

impl<'input,'a> Listenable<dyn SolidityListener<'input> + 'a> for AssemblyLiteralContext<'input>{
	fn enter(&self,listener: &mut (dyn SolidityListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_assemblyLiteral(self);
	}
}

impl<'input> CustomRuleContext<'input> for AssemblyLiteralContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SolidityParserContextType;
	fn get_rule_index(&self) -> usize { RULE_assemblyLiteral }
	//fn type_rule_index() -> usize where Self: Sized { RULE_assemblyLiteral }
}
antlr_rust::type_id!{AssemblyLiteralContextExt<'a>}

impl<'input> AssemblyLiteralContextExt<'input>{
	fn new(parent: Option<Rc<dyn SolidityParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<AssemblyLiteralContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,AssemblyLiteralContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait AssemblyLiteralContextAttrs<'input>: SolidityParserContext<'input> + BorrowMut<AssemblyLiteralContextExt<'input>>{

fn stringLiteral(&self) -> Option<Rc<StringLiteralContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token DecimalNumber
/// Returns `None` if there is no child corresponding to token DecimalNumber
fn DecimalNumber(&self) -> Option<Rc<TerminalNode<'input,SolidityParserContextType>>> where Self:Sized{
	self.get_token(DecimalNumber, 0)
}
/// Retrieves first TerminalNode corresponding to token HexNumber
/// Returns `None` if there is no child corresponding to token HexNumber
fn HexNumber(&self) -> Option<Rc<TerminalNode<'input,SolidityParserContextType>>> where Self:Sized{
	self.get_token(HexNumber, 0)
}
fn hexLiteral(&self) -> Option<Rc<HexLiteralContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token BooleanLiteral
/// Returns `None` if there is no child corresponding to token BooleanLiteral
fn BooleanLiteral(&self) -> Option<Rc<TerminalNode<'input,SolidityParserContextType>>> where Self:Sized{
	self.get_token(BooleanLiteral, 0)
}

}

impl<'input> AssemblyLiteralContextAttrs<'input> for AssemblyLiteralContext<'input>{}

impl<'input, I, H> SolidityParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn assemblyLiteral(&mut self,)
	-> Result<Rc<AssemblyLiteralContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = AssemblyLiteralContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 188, RULE_assemblyLiteral);
        let mut _localctx: Rc<AssemblyLiteralContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			recog.base.set_state(1109);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 StringLiteralFragment 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule stringLiteral*/
					recog.base.set_state(1104);
					recog.stringLiteral()?;

					}
				}

			 DecimalNumber 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(1105);
					recog.base.match_token(DecimalNumber,&mut recog.err_handler)?;

					}
				}

			 HexNumber 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					recog.base.set_state(1106);
					recog.base.match_token(HexNumber,&mut recog.err_handler)?;

					}
				}

			 HexLiteralFragment 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 4);
					recog.base.enter_outer_alt(None, 4);
					{
					/*InvokeRule hexLiteral*/
					recog.base.set_state(1107);
					recog.hexLiteral()?;

					}
				}

			 BooleanLiteral 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 5);
					recog.base.enter_outer_alt(None, 5);
					{
					recog.base.set_state(1108);
					recog.base.match_token(BooleanLiteral,&mut recog.err_handler)?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- tupleExpression ----------------
pub type TupleExpressionContextAll<'input> = TupleExpressionContext<'input>;


pub type TupleExpressionContext<'input> = BaseParserRuleContext<'input,TupleExpressionContextExt<'input>>;

#[derive(Clone)]
pub struct TupleExpressionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SolidityParserContext<'input> for TupleExpressionContext<'input>{}

impl<'input,'a> Listenable<dyn SolidityListener<'input> + 'a> for TupleExpressionContext<'input>{
	fn enter(&self,listener: &mut (dyn SolidityListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_tupleExpression(self);
	}
}

impl<'input> CustomRuleContext<'input> for TupleExpressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SolidityParserContextType;
	fn get_rule_index(&self) -> usize { RULE_tupleExpression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_tupleExpression }
}
antlr_rust::type_id!{TupleExpressionContextExt<'a>}

impl<'input> TupleExpressionContextExt<'input>{
	fn new(parent: Option<Rc<dyn SolidityParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<TupleExpressionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,TupleExpressionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait TupleExpressionContextAttrs<'input>: SolidityParserContext<'input> + BorrowMut<TupleExpressionContextExt<'input>>{

fn expression_all(&self) ->  Vec<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn expression(&self, i: usize) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> TupleExpressionContextAttrs<'input> for TupleExpressionContext<'input>{}

impl<'input, I, H> SolidityParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn tupleExpression(&mut self,)
	-> Result<Rc<TupleExpressionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = TupleExpressionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 190, RULE_tupleExpression);
        let mut _localctx: Rc<TupleExpressionContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			recog.base.set_state(1137);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 T__22 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(1111);
					recog.base.match_token(T__22,&mut recog.err_handler)?;

					{
					recog.base.set_state(1113);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if ((((_la - 6)) & !0x3f) == 0 && ((1usize << (_la - 6)) & ((1usize << (T__5 - 6)) | (1usize << (T__13 - 6)) | (1usize << (T__22 - 6)) | (1usize << (T__24 - 6)) | (1usize << (T__29 - 6)) | (1usize << (T__30 - 6)) | (1usize << (T__37 - 6)) | (1usize << (T__41 - 6)) | (1usize << (T__43 - 6)) | (1usize << (T__45 - 6)) | (1usize << (T__49 - 6)) | (1usize << (T__61 - 6)) | (1usize << (T__62 - 6)) | (1usize << (T__63 - 6)) | (1usize << (T__64 - 6)) | (1usize << (T__65 - 6)) | (1usize << (T__66 - 6)) | (1usize << (T__67 - 6)) | (1usize << (T__68 - 6)))) != 0) || ((((_la - 71)) & !0x3f) == 0 && ((1usize << (_la - 71)) & ((1usize << (T__70 - 71)) | (1usize << (T__71 - 71)) | (1usize << (T__94 - 71)) | (1usize << (Int - 71)) | (1usize << (Uint - 71)) | (1usize << (Byte - 71)) | (1usize << (Fixed - 71)) | (1usize << (Ufixed - 71)) | (1usize << (BooleanLiteral - 71)) | (1usize << (DecimalNumber - 71)) | (1usize << (HexNumber - 71)) | (1usize << (HexLiteralFragment - 71)) | (1usize << (LeaveKeyword - 71)) | (1usize << (PayableKeyword - 71)) | (1usize << (TypeKeyword - 71)) | (1usize << (GlobalKeyword - 71)) | (1usize << (ConstructorKeyword - 71)) | (1usize << (ReceiveKeyword - 71)) | (1usize << (Identifier - 71)) | (1usize << (StringLiteralFragment - 71)))) != 0) {
						{
						/*InvokeRule expression*/
						recog.base.set_state(1112);
						recog.expression_rec(0)?;

						}
					}

					recog.base.set_state(1121);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					while _la==T__15 {
						{
						{
						recog.base.set_state(1115);
						recog.base.match_token(T__15,&mut recog.err_handler)?;

						recog.base.set_state(1117);
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
						if ((((_la - 6)) & !0x3f) == 0 && ((1usize << (_la - 6)) & ((1usize << (T__5 - 6)) | (1usize << (T__13 - 6)) | (1usize << (T__22 - 6)) | (1usize << (T__24 - 6)) | (1usize << (T__29 - 6)) | (1usize << (T__30 - 6)) | (1usize << (T__37 - 6)) | (1usize << (T__41 - 6)) | (1usize << (T__43 - 6)) | (1usize << (T__45 - 6)) | (1usize << (T__49 - 6)) | (1usize << (T__61 - 6)) | (1usize << (T__62 - 6)) | (1usize << (T__63 - 6)) | (1usize << (T__64 - 6)) | (1usize << (T__65 - 6)) | (1usize << (T__66 - 6)) | (1usize << (T__67 - 6)) | (1usize << (T__68 - 6)))) != 0) || ((((_la - 71)) & !0x3f) == 0 && ((1usize << (_la - 71)) & ((1usize << (T__70 - 71)) | (1usize << (T__71 - 71)) | (1usize << (T__94 - 71)) | (1usize << (Int - 71)) | (1usize << (Uint - 71)) | (1usize << (Byte - 71)) | (1usize << (Fixed - 71)) | (1usize << (Ufixed - 71)) | (1usize << (BooleanLiteral - 71)) | (1usize << (DecimalNumber - 71)) | (1usize << (HexNumber - 71)) | (1usize << (HexLiteralFragment - 71)) | (1usize << (LeaveKeyword - 71)) | (1usize << (PayableKeyword - 71)) | (1usize << (TypeKeyword - 71)) | (1usize << (GlobalKeyword - 71)) | (1usize << (ConstructorKeyword - 71)) | (1usize << (ReceiveKeyword - 71)) | (1usize << (Identifier - 71)) | (1usize << (StringLiteralFragment - 71)))) != 0) {
							{
							/*InvokeRule expression*/
							recog.base.set_state(1116);
							recog.expression_rec(0)?;

							}
						}

						}
						}
						recog.base.set_state(1123);
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
					}
					}
					recog.base.set_state(1124);
					recog.base.match_token(T__23,&mut recog.err_handler)?;

					}
				}

			 T__41 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(1125);
					recog.base.match_token(T__41,&mut recog.err_handler)?;

					recog.base.set_state(1134);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if ((((_la - 6)) & !0x3f) == 0 && ((1usize << (_la - 6)) & ((1usize << (T__5 - 6)) | (1usize << (T__13 - 6)) | (1usize << (T__22 - 6)) | (1usize << (T__24 - 6)) | (1usize << (T__29 - 6)) | (1usize << (T__30 - 6)) | (1usize << (T__37 - 6)) | (1usize << (T__41 - 6)) | (1usize << (T__43 - 6)) | (1usize << (T__45 - 6)) | (1usize << (T__49 - 6)) | (1usize << (T__61 - 6)) | (1usize << (T__62 - 6)) | (1usize << (T__63 - 6)) | (1usize << (T__64 - 6)) | (1usize << (T__65 - 6)) | (1usize << (T__66 - 6)) | (1usize << (T__67 - 6)) | (1usize << (T__68 - 6)))) != 0) || ((((_la - 71)) & !0x3f) == 0 && ((1usize << (_la - 71)) & ((1usize << (T__70 - 71)) | (1usize << (T__71 - 71)) | (1usize << (T__94 - 71)) | (1usize << (Int - 71)) | (1usize << (Uint - 71)) | (1usize << (Byte - 71)) | (1usize << (Fixed - 71)) | (1usize << (Ufixed - 71)) | (1usize << (BooleanLiteral - 71)) | (1usize << (DecimalNumber - 71)) | (1usize << (HexNumber - 71)) | (1usize << (HexLiteralFragment - 71)) | (1usize << (LeaveKeyword - 71)) | (1usize << (PayableKeyword - 71)) | (1usize << (TypeKeyword - 71)) | (1usize << (GlobalKeyword - 71)) | (1usize << (ConstructorKeyword - 71)) | (1usize << (ReceiveKeyword - 71)) | (1usize << (Identifier - 71)) | (1usize << (StringLiteralFragment - 71)))) != 0) {
						{
						/*InvokeRule expression*/
						recog.base.set_state(1126);
						recog.expression_rec(0)?;

						recog.base.set_state(1131);
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
						while _la==T__15 {
							{
							{
							recog.base.set_state(1127);
							recog.base.match_token(T__15,&mut recog.err_handler)?;

							/*InvokeRule expression*/
							recog.base.set_state(1128);
							recog.expression_rec(0)?;

							}
							}
							recog.base.set_state(1133);
							recog.err_handler.sync(&mut recog.base)?;
							_la = recog.base.input.la(1);
						}
						}
					}

					recog.base.set_state(1136);
					recog.base.match_token(T__42,&mut recog.err_handler)?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- numberLiteral ----------------
pub type NumberLiteralContextAll<'input> = NumberLiteralContext<'input>;


pub type NumberLiteralContext<'input> = BaseParserRuleContext<'input,NumberLiteralContextExt<'input>>;

#[derive(Clone)]
pub struct NumberLiteralContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SolidityParserContext<'input> for NumberLiteralContext<'input>{}

impl<'input,'a> Listenable<dyn SolidityListener<'input> + 'a> for NumberLiteralContext<'input>{
	fn enter(&self,listener: &mut (dyn SolidityListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_numberLiteral(self);
	}
}

impl<'input> CustomRuleContext<'input> for NumberLiteralContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SolidityParserContextType;
	fn get_rule_index(&self) -> usize { RULE_numberLiteral }
	//fn type_rule_index() -> usize where Self: Sized { RULE_numberLiteral }
}
antlr_rust::type_id!{NumberLiteralContextExt<'a>}

impl<'input> NumberLiteralContextExt<'input>{
	fn new(parent: Option<Rc<dyn SolidityParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<NumberLiteralContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,NumberLiteralContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait NumberLiteralContextAttrs<'input>: SolidityParserContext<'input> + BorrowMut<NumberLiteralContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token DecimalNumber
/// Returns `None` if there is no child corresponding to token DecimalNumber
fn DecimalNumber(&self) -> Option<Rc<TerminalNode<'input,SolidityParserContextType>>> where Self:Sized{
	self.get_token(DecimalNumber, 0)
}
/// Retrieves first TerminalNode corresponding to token HexNumber
/// Returns `None` if there is no child corresponding to token HexNumber
fn HexNumber(&self) -> Option<Rc<TerminalNode<'input,SolidityParserContextType>>> where Self:Sized{
	self.get_token(HexNumber, 0)
}
/// Retrieves first TerminalNode corresponding to token NumberUnit
/// Returns `None` if there is no child corresponding to token NumberUnit
fn NumberUnit(&self) -> Option<Rc<TerminalNode<'input,SolidityParserContextType>>> where Self:Sized{
	self.get_token(NumberUnit, 0)
}

}

impl<'input> NumberLiteralContextAttrs<'input> for NumberLiteralContext<'input>{}

impl<'input, I, H> SolidityParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn numberLiteral(&mut self,)
	-> Result<Rc<NumberLiteralContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = NumberLiteralContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 192, RULE_numberLiteral);
        let mut _localctx: Rc<NumberLiteralContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(1139);
			_la = recog.base.input.la(1);
			if { !(_la==DecimalNumber || _la==HexNumber) } {
				recog.err_handler.recover_inline(&mut recog.base)?;

			}
			else {
				if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler);
			}
			recog.base.set_state(1141);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(125,&mut recog.base)? {
				x if x == 1=>{
					{
					recog.base.set_state(1140);
					recog.base.match_token(NumberUnit,&mut recog.err_handler)?;

					}
				}

				_ => {}
			}
			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- identifier ----------------
pub type IdentifierContextAll<'input> = IdentifierContext<'input>;


pub type IdentifierContext<'input> = BaseParserRuleContext<'input,IdentifierContextExt<'input>>;

#[derive(Clone)]
pub struct IdentifierContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SolidityParserContext<'input> for IdentifierContext<'input>{}

impl<'input,'a> Listenable<dyn SolidityListener<'input> + 'a> for IdentifierContext<'input>{
	fn enter(&self,listener: &mut (dyn SolidityListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_identifier(self);
	}
}

impl<'input> CustomRuleContext<'input> for IdentifierContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SolidityParserContextType;
	fn get_rule_index(&self) -> usize { RULE_identifier }
	//fn type_rule_index() -> usize where Self: Sized { RULE_identifier }
}
antlr_rust::type_id!{IdentifierContextExt<'a>}

impl<'input> IdentifierContextExt<'input>{
	fn new(parent: Option<Rc<dyn SolidityParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<IdentifierContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,IdentifierContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait IdentifierContextAttrs<'input>: SolidityParserContext<'input> + BorrowMut<IdentifierContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token ReceiveKeyword
/// Returns `None` if there is no child corresponding to token ReceiveKeyword
fn ReceiveKeyword(&self) -> Option<Rc<TerminalNode<'input,SolidityParserContextType>>> where Self:Sized{
	self.get_token(ReceiveKeyword, 0)
}
/// Retrieves first TerminalNode corresponding to token GlobalKeyword
/// Returns `None` if there is no child corresponding to token GlobalKeyword
fn GlobalKeyword(&self) -> Option<Rc<TerminalNode<'input,SolidityParserContextType>>> where Self:Sized{
	self.get_token(GlobalKeyword, 0)
}
/// Retrieves first TerminalNode corresponding to token ConstructorKeyword
/// Returns `None` if there is no child corresponding to token ConstructorKeyword
fn ConstructorKeyword(&self) -> Option<Rc<TerminalNode<'input,SolidityParserContextType>>> where Self:Sized{
	self.get_token(ConstructorKeyword, 0)
}
/// Retrieves first TerminalNode corresponding to token PayableKeyword
/// Returns `None` if there is no child corresponding to token PayableKeyword
fn PayableKeyword(&self) -> Option<Rc<TerminalNode<'input,SolidityParserContextType>>> where Self:Sized{
	self.get_token(PayableKeyword, 0)
}
/// Retrieves first TerminalNode corresponding to token LeaveKeyword
/// Returns `None` if there is no child corresponding to token LeaveKeyword
fn LeaveKeyword(&self) -> Option<Rc<TerminalNode<'input,SolidityParserContextType>>> where Self:Sized{
	self.get_token(LeaveKeyword, 0)
}
/// Retrieves first TerminalNode corresponding to token Identifier
/// Returns `None` if there is no child corresponding to token Identifier
fn Identifier(&self) -> Option<Rc<TerminalNode<'input,SolidityParserContextType>>> where Self:Sized{
	self.get_token(Identifier, 0)
}

}

impl<'input> IdentifierContextAttrs<'input> for IdentifierContext<'input>{}

impl<'input, I, H> SolidityParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn identifier(&mut self,)
	-> Result<Rc<IdentifierContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = IdentifierContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 194, RULE_identifier);
        let mut _localctx: Rc<IdentifierContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(1143);
			_la = recog.base.input.la(1);
			if { !((((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << T__13) | (1usize << T__24) | (1usize << T__43) | (1usize << T__49) | (1usize << T__61))) != 0) || ((((_la - 95)) & !0x3f) == 0 && ((1usize << (_la - 95)) & ((1usize << (T__94 - 95)) | (1usize << (LeaveKeyword - 95)) | (1usize << (PayableKeyword - 95)) | (1usize << (GlobalKeyword - 95)) | (1usize << (ConstructorKeyword - 95)) | (1usize << (ReceiveKeyword - 95)) | (1usize << (Identifier - 95)))) != 0)) } {
				recog.err_handler.recover_inline(&mut recog.base)?;

			}
			else {
				if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler);
			}
			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- hexLiteral ----------------
pub type HexLiteralContextAll<'input> = HexLiteralContext<'input>;


pub type HexLiteralContext<'input> = BaseParserRuleContext<'input,HexLiteralContextExt<'input>>;

#[derive(Clone)]
pub struct HexLiteralContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SolidityParserContext<'input> for HexLiteralContext<'input>{}

impl<'input,'a> Listenable<dyn SolidityListener<'input> + 'a> for HexLiteralContext<'input>{
	fn enter(&self,listener: &mut (dyn SolidityListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_hexLiteral(self);
	}
}

impl<'input> CustomRuleContext<'input> for HexLiteralContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SolidityParserContextType;
	fn get_rule_index(&self) -> usize { RULE_hexLiteral }
	//fn type_rule_index() -> usize where Self: Sized { RULE_hexLiteral }
}
antlr_rust::type_id!{HexLiteralContextExt<'a>}

impl<'input> HexLiteralContextExt<'input>{
	fn new(parent: Option<Rc<dyn SolidityParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<HexLiteralContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,HexLiteralContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait HexLiteralContextAttrs<'input>: SolidityParserContext<'input> + BorrowMut<HexLiteralContextExt<'input>>{

/// Retrieves all `TerminalNode`s corresponding to token HexLiteralFragment in current rule
fn HexLiteralFragment_all(&self) -> Vec<Rc<TerminalNode<'input,SolidityParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token HexLiteralFragment, starting from 0.
/// Returns `None` if number of children corresponding to token HexLiteralFragment is less or equal than `i`.
fn HexLiteralFragment(&self, i: usize) -> Option<Rc<TerminalNode<'input,SolidityParserContextType>>> where Self:Sized{
	self.get_token(HexLiteralFragment, i)
}

}

impl<'input> HexLiteralContextAttrs<'input> for HexLiteralContext<'input>{}

impl<'input, I, H> SolidityParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn hexLiteral(&mut self,)
	-> Result<Rc<HexLiteralContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = HexLiteralContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 196, RULE_hexLiteral);
        let mut _localctx: Rc<HexLiteralContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(1146); 
			recog.err_handler.sync(&mut recog.base)?;
			_alt = 1;
			loop {
				match _alt {
				    x if x == 1=>
					{
					{
					recog.base.set_state(1145);
					recog.base.match_token(HexLiteralFragment,&mut recog.err_handler)?;

					}
					}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
				}
				recog.base.set_state(1148); 
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(126,&mut recog.base)?;
				if _alt==2 || _alt==INVALID_ALT { break }
			}
			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- overrideSpecifier ----------------
pub type OverrideSpecifierContextAll<'input> = OverrideSpecifierContext<'input>;


pub type OverrideSpecifierContext<'input> = BaseParserRuleContext<'input,OverrideSpecifierContextExt<'input>>;

#[derive(Clone)]
pub struct OverrideSpecifierContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SolidityParserContext<'input> for OverrideSpecifierContext<'input>{}

impl<'input,'a> Listenable<dyn SolidityListener<'input> + 'a> for OverrideSpecifierContext<'input>{
	fn enter(&self,listener: &mut (dyn SolidityListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_overrideSpecifier(self);
	}
}

impl<'input> CustomRuleContext<'input> for OverrideSpecifierContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SolidityParserContextType;
	fn get_rule_index(&self) -> usize { RULE_overrideSpecifier }
	//fn type_rule_index() -> usize where Self: Sized { RULE_overrideSpecifier }
}
antlr_rust::type_id!{OverrideSpecifierContextExt<'a>}

impl<'input> OverrideSpecifierContextExt<'input>{
	fn new(parent: Option<Rc<dyn SolidityParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<OverrideSpecifierContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,OverrideSpecifierContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait OverrideSpecifierContextAttrs<'input>: SolidityParserContext<'input> + BorrowMut<OverrideSpecifierContextExt<'input>>{

fn userDefinedTypeName_all(&self) ->  Vec<Rc<UserDefinedTypeNameContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn userDefinedTypeName(&self, i: usize) -> Option<Rc<UserDefinedTypeNameContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> OverrideSpecifierContextAttrs<'input> for OverrideSpecifierContext<'input>{}

impl<'input, I, H> SolidityParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn overrideSpecifier(&mut self,)
	-> Result<Rc<OverrideSpecifierContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = OverrideSpecifierContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 198, RULE_overrideSpecifier);
        let mut _localctx: Rc<OverrideSpecifierContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(1150);
			recog.base.match_token(T__95,&mut recog.err_handler)?;

			recog.base.set_state(1162);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==T__22 {
				{
				recog.base.set_state(1151);
				recog.base.match_token(T__22,&mut recog.err_handler)?;

				/*InvokeRule userDefinedTypeName*/
				recog.base.set_state(1152);
				recog.userDefinedTypeName()?;

				recog.base.set_state(1157);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				while _la==T__15 {
					{
					{
					recog.base.set_state(1153);
					recog.base.match_token(T__15,&mut recog.err_handler)?;

					/*InvokeRule userDefinedTypeName*/
					recog.base.set_state(1154);
					recog.userDefinedTypeName()?;

					}
					}
					recog.base.set_state(1159);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
				}
				recog.base.set_state(1160);
				recog.base.match_token(T__23,&mut recog.err_handler)?;

				}
			}

			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- stringLiteral ----------------
pub type StringLiteralContextAll<'input> = StringLiteralContext<'input>;


pub type StringLiteralContext<'input> = BaseParserRuleContext<'input,StringLiteralContextExt<'input>>;

#[derive(Clone)]
pub struct StringLiteralContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> SolidityParserContext<'input> for StringLiteralContext<'input>{}

impl<'input,'a> Listenable<dyn SolidityListener<'input> + 'a> for StringLiteralContext<'input>{
	fn enter(&self,listener: &mut (dyn SolidityListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_stringLiteral(self);
	}
}

impl<'input> CustomRuleContext<'input> for StringLiteralContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = SolidityParserContextType;
	fn get_rule_index(&self) -> usize { RULE_stringLiteral }
	//fn type_rule_index() -> usize where Self: Sized { RULE_stringLiteral }
}
antlr_rust::type_id!{StringLiteralContextExt<'a>}

impl<'input> StringLiteralContextExt<'input>{
	fn new(parent: Option<Rc<dyn SolidityParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<StringLiteralContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,StringLiteralContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait StringLiteralContextAttrs<'input>: SolidityParserContext<'input> + BorrowMut<StringLiteralContextExt<'input>>{

/// Retrieves all `TerminalNode`s corresponding to token StringLiteralFragment in current rule
fn StringLiteralFragment_all(&self) -> Vec<Rc<TerminalNode<'input,SolidityParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token StringLiteralFragment, starting from 0.
/// Returns `None` if number of children corresponding to token StringLiteralFragment is less or equal than `i`.
fn StringLiteralFragment(&self, i: usize) -> Option<Rc<TerminalNode<'input,SolidityParserContextType>>> where Self:Sized{
	self.get_token(StringLiteralFragment, i)
}

}

impl<'input> StringLiteralContextAttrs<'input> for StringLiteralContext<'input>{}

impl<'input, I, H> SolidityParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn stringLiteral(&mut self,)
	-> Result<Rc<StringLiteralContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = StringLiteralContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 200, RULE_stringLiteral);
        let mut _localctx: Rc<StringLiteralContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(1165); 
			recog.err_handler.sync(&mut recog.base)?;
			_alt = 1;
			loop {
				match _alt {
				    x if x == 1=>
					{
					{
					recog.base.set_state(1164);
					recog.base.match_token(StringLiteralFragment,&mut recog.err_handler)?;

					}
					}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
				}
				recog.base.set_state(1167); 
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(129,&mut recog.base)?;
				if _alt==2 || _alt==INVALID_ALT { break }
			}
			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}

lazy_static! {
    static ref _ATN: Arc<ATN> =
        Arc::new(ATNDeserializer::new(None).deserialize(_serializedATN.chars()));
    static ref _decision_to_DFA: Arc<Vec<antlr_rust::RwLock<DFA>>> = {
        let mut dfa = Vec::new();
        let size = _ATN.decision_to_state.len();
        for i in 0..size {
            dfa.push(DFA::new(
                _ATN.clone(),
                _ATN.get_decision_state(i),
                i as isize,
            ).into())
        }
        Arc::new(dfa)
    };
}



const _serializedATN:&'static str =
	"\x03\u{608b}\u{a72a}\u{8133}\u{b9ed}\u{417c}\u{3be7}\u{7786}\u{5964}\x03\
	\u{87}\u{494}\x04\x02\x09\x02\x04\x03\x09\x03\x04\x04\x09\x04\x04\x05\x09\
	\x05\x04\x06\x09\x06\x04\x07\x09\x07\x04\x08\x09\x08\x04\x09\x09\x09\x04\
	\x0a\x09\x0a\x04\x0b\x09\x0b\x04\x0c\x09\x0c\x04\x0d\x09\x0d\x04\x0e\x09\
	\x0e\x04\x0f\x09\x0f\x04\x10\x09\x10\x04\x11\x09\x11\x04\x12\x09\x12\x04\
	\x13\x09\x13\x04\x14\x09\x14\x04\x15\x09\x15\x04\x16\x09\x16\x04\x17\x09\
	\x17\x04\x18\x09\x18\x04\x19\x09\x19\x04\x1a\x09\x1a\x04\x1b\x09\x1b\x04\
	\x1c\x09\x1c\x04\x1d\x09\x1d\x04\x1e\x09\x1e\x04\x1f\x09\x1f\x04\x20\x09\
	\x20\x04\x21\x09\x21\x04\x22\x09\x22\x04\x23\x09\x23\x04\x24\x09\x24\x04\
	\x25\x09\x25\x04\x26\x09\x26\x04\x27\x09\x27\x04\x28\x09\x28\x04\x29\x09\
	\x29\x04\x2a\x09\x2a\x04\x2b\x09\x2b\x04\x2c\x09\x2c\x04\x2d\x09\x2d\x04\
	\x2e\x09\x2e\x04\x2f\x09\x2f\x04\x30\x09\x30\x04\x31\x09\x31\x04\x32\x09\
	\x32\x04\x33\x09\x33\x04\x34\x09\x34\x04\x35\x09\x35\x04\x36\x09\x36\x04\
	\x37\x09\x37\x04\x38\x09\x38\x04\x39\x09\x39\x04\x3a\x09\x3a\x04\x3b\x09\
	\x3b\x04\x3c\x09\x3c\x04\x3d\x09\x3d\x04\x3e\x09\x3e\x04\x3f\x09\x3f\x04\
	\x40\x09\x40\x04\x41\x09\x41\x04\x42\x09\x42\x04\x43\x09\x43\x04\x44\x09\
	\x44\x04\x45\x09\x45\x04\x46\x09\x46\x04\x47\x09\x47\x04\x48\x09\x48\x04\
	\x49\x09\x49\x04\x4a\x09\x4a\x04\x4b\x09\x4b\x04\x4c\x09\x4c\x04\x4d\x09\
	\x4d\x04\x4e\x09\x4e\x04\x4f\x09\x4f\x04\x50\x09\x50\x04\x51\x09\x51\x04\
	\x52\x09\x52\x04\x53\x09\x53\x04\x54\x09\x54\x04\x55\x09\x55\x04\x56\x09\
	\x56\x04\x57\x09\x57\x04\x58\x09\x58\x04\x59\x09\x59\x04\x5a\x09\x5a\x04\
	\x5b\x09\x5b\x04\x5c\x09\x5c\x04\x5d\x09\x5d\x04\x5e\x09\x5e\x04\x5f\x09\
	\x5f\x04\x60\x09\x60\x04\x61\x09\x61\x04\x62\x09\x62\x04\x63\x09\x63\x04\
	\x64\x09\x64\x04\x65\x09\x65\x04\x66\x09\x66\x03\x02\x03\x02\x03\x02\x03\
	\x02\x03\x02\x03\x02\x03\x02\x03\x02\x03\x02\x03\x02\x03\x02\x07\x02\u{d8}\
	\x0a\x02\x0c\x02\x0e\x02\u{db}\x0b\x02\x03\x02\x03\x02\x03\x03\x03\x03\x03\
	\x03\x03\x03\x03\x03\x03\x04\x03\x04\x03\x05\x03\x05\x03\x05\x05\x05\u{e9}\
	\x0a\x05\x03\x06\x03\x06\x05\x06\u{ed}\x0a\x06\x03\x06\x07\x06\u{f0}\x0a\
	\x06\x0c\x06\x0e\x06\u{f3}\x0b\x06\x03\x07\x03\x07\x03\x08\x05\x08\u{f8}\
	\x0a\x08\x03\x08\x03\x08\x05\x08\u{fc}\x0a\x08\x03\x08\x05\x08\u{ff}\x0a\
	\x08\x03\x09\x03\x09\x03\x09\x05\x09\u{104}\x0a\x09\x03\x0a\x03\x0a\x03\
	\x0a\x03\x0a\x05\x0a\u{10a}\x0a\x0a\x03\x0a\x03\x0a\x03\x0a\x03\x0a\x03\
	\x0a\x05\x0a\u{111}\x0a\x0a\x03\x0a\x03\x0a\x05\x0a\u{115}\x0a\x0a\x03\x0a\
	\x03\x0a\x03\x0a\x03\x0a\x03\x0a\x03\x0a\x03\x0a\x03\x0a\x03\x0a\x07\x0a\
	\u{120}\x0a\x0a\x0c\x0a\x0e\x0a\u{123}\x0b\x0a\x03\x0a\x03\x0a\x03\x0a\x03\
	\x0a\x03\x0a\x05\x0a\u{12a}\x0a\x0a\x03\x0b\x03\x0b\x03\x0c\x05\x0c\u{12f}\
	\x0a\x0c\x03\x0c\x03\x0c\x03\x0c\x03\x0c\x03\x0c\x03\x0c\x07\x0c\u{137}\
	\x0a\x0c\x0c\x0c\x0e\x0c\u{13a}\x0b\x0c\x05\x0c\u{13c}\x0a\x0c\x03\x0c\x03\
	\x0c\x07\x0c\u{140}\x0a\x0c\x0c\x0c\x0e\x0c\u{143}\x0b\x0c\x03\x0c\x03\x0c\
	\x03\x0d\x03\x0d\x03\x0d\x05\x0d\u{14a}\x0a\x0d\x03\x0d\x05\x0d\u{14d}\x0a\
	\x0d\x03\x0e\x03\x0e\x03\x0e\x03\x0e\x03\x0e\x03\x0e\x03\x0e\x03\x0e\x03\
	\x0e\x05\x0e\u{158}\x0a\x0e\x03\x0f\x03\x0f\x03\x0f\x03\x0f\x03\x0f\x03\
	\x0f\x03\x0f\x07\x0f\u{161}\x0a\x0f\x0c\x0f\x0e\x0f\u{164}\x0b\x0f\x03\x0f\
	\x03\x0f\x03\x0f\x05\x0f\u{169}\x0a\x0f\x03\x0f\x03\x0f\x03\x10\x03\x10\
	\x03\x10\x03\x10\x03\x10\x03\x10\x03\x10\x03\x11\x03\x11\x03\x11\x03\x11\
	\x03\x11\x03\x12\x03\x12\x03\x12\x03\x12\x03\x12\x03\x12\x03\x13\x03\x13\
	\x03\x13\x03\x13\x03\x13\x05\x13\u{184}\x0a\x13\x03\x13\x05\x13\u{187}\x0a\
	\x13\x03\x13\x03\x13\x03\x14\x03\x14\x03\x14\x03\x14\x03\x14\x07\x14\u{190}\
	\x0a\x14\x0c\x14\x0e\x14\u{193}\x0b\x14\x03\x14\x03\x14\x05\x14\u{197}\x0a\
	\x14\x03\x15\x03\x15\x03\x15\x05\x15\u{19c}\x0a\x15\x03\x16\x03\x16\x03\
	\x17\x03\x17\x03\x17\x03\x17\x03\x17\x03\x17\x03\x17\x03\x17\x07\x17\u{1a8}\
	\x0a\x17\x0c\x17\x0e\x17\u{1ab}\x0b\x17\x05\x17\u{1ad}\x0a\x17\x03\x17\x03\
	\x17\x03\x18\x03\x18\x03\x18\x05\x18\u{1b4}\x0a\x18\x03\x18\x03\x18\x07\
	\x18\u{1b8}\x0a\x18\x0c\x18\x0e\x18\u{1bb}\x0b\x18\x03\x18\x03\x18\x05\x18\
	\u{1bf}\x0a\x18\x03\x19\x03\x19\x03\x19\x05\x19\u{1c4}\x0a\x19\x03\x19\x05\
	\x19\u{1c7}\x0a\x19\x03\x1a\x03\x1a\x03\x1a\x03\x1a\x05\x1a\u{1cd}\x0a\x1a\
	\x03\x1a\x03\x1a\x05\x1a\u{1d1}\x0a\x1a\x03\x1b\x03\x1b\x05\x1b\u{1d5}\x0a\
	\x1b\x03\x1b\x03\x1b\x03\x1b\x05\x1b\u{1da}\x0a\x1b\x03\x1c\x03\x1c\x03\
	\x1c\x03\x1d\x03\x1d\x03\x1d\x03\x1d\x03\x1d\x03\x1d\x03\x1d\x03\x1d\x07\
	\x1d\u{1e7}\x0a\x1d\x0c\x1d\x0e\x1d\u{1ea}\x0b\x1d\x03\x1e\x03\x1e\x03\x1e\
	\x03\x1e\x05\x1e\u{1f0}\x0a\x1e\x03\x1e\x03\x1e\x03\x1f\x03\x1f\x03\x20\
	\x03\x20\x03\x20\x03\x20\x05\x20\u{1fa}\x0a\x20\x03\x20\x03\x20\x07\x20\
	\u{1fe}\x0a\x20\x0c\x20\x0e\x20\u{201}\x0b\x20\x03\x20\x03\x20\x03\x21\x03\
	\x21\x03\x21\x03\x21\x07\x21\u{209}\x0a\x21\x0c\x21\x0e\x21\u{20c}\x0b\x21\
	\x05\x21\u{20e}\x0a\x21\x03\x21\x03\x21\x03\x22\x03\x22\x05\x22\u{214}\x0a\
	\x22\x03\x22\x05\x22\u{217}\x0a\x22\x03\x23\x03\x23\x03\x23\x03\x23\x07\
	\x23\u{21d}\x0a\x23\x0c\x23\x0e\x23\u{220}\x0b\x23\x05\x23\u{222}\x0a\x23\
	\x03\x23\x03\x23\x03\x24\x03\x24\x05\x24\u{228}\x0a\x24\x03\x24\x05\x24\
	\u{22b}\x0a\x24\x03\x25\x03\x25\x03\x25\x03\x25\x07\x25\u{231}\x0a\x25\x0c\
	\x25\x0e\x25\u{234}\x0b\x25\x05\x25\u{236}\x0a\x25\x03\x25\x03\x25\x03\x26\
	\x03\x26\x05\x26\u{23c}\x0a\x26\x03\x27\x03\x27\x05\x27\u{240}\x0a\x27\x03\
	\x27\x03\x27\x03\x28\x03\x28\x03\x28\x03\x28\x03\x28\x03\x28\x03\x28\x05\
	\x28\u{24b}\x0a\x28\x03\x28\x03\x28\x03\x28\x05\x28\u{250}\x0a\x28\x03\x28\
	\x07\x28\u{253}\x0a\x28\x0c\x28\x0e\x28\u{256}\x0b\x28\x03\x29\x03\x29\x03\
	\x29\x07\x29\u{25b}\x0a\x29\x0c\x29\x0e\x29\u{25e}\x0b\x29\x03\x2a\x03\x2a\
	\x05\x2a\u{262}\x0a\x2a\x03\x2b\x03\x2b\x03\x2b\x03\x2b\x05\x2b\u{268}\x0a\
	\x2b\x03\x2b\x03\x2b\x03\x2b\x05\x2b\u{26d}\x0a\x2b\x03\x2b\x03\x2b\x03\
	\x2c\x03\x2c\x03\x2d\x03\x2d\x03\x2e\x03\x2e\x03\x2e\x03\x2e\x03\x2e\x07\
	\x2e\u{27a}\x0a\x2e\x0c\x2e\x0e\x2e\u{27d}\x0b\x2e\x03\x2e\x03\x2e\x05\x2e\
	\u{281}\x0a\x2e\x03\x2f\x03\x2f\x03\x30\x03\x30\x03\x31\x03\x31\x07\x31\
	\u{289}\x0a\x31\x0c\x31\x0e\x31\u{28c}\x0b\x31\x03\x31\x03\x31\x03\x32\x03\
	\x32\x03\x32\x03\x32\x03\x32\x03\x32\x03\x32\x03\x32\x03\x32\x03\x32\x03\
	\x32\x03\x32\x03\x32\x03\x32\x03\x32\x05\x32\u{29f}\x0a\x32\x03\x33\x03\
	\x33\x03\x33\x03\x34\x03\x34\x03\x34\x03\x34\x03\x34\x03\x34\x03\x34\x05\
	\x34\u{2ab}\x0a\x34\x03\x35\x03\x35\x03\x35\x05\x35\u{2b0}\x0a\x35\x03\x35\
	\x03\x35\x06\x35\u{2b4}\x0a\x35\x0d\x35\x0e\x35\u{2b5}\x03\x36\x03\x36\x05\
	\x36\u{2ba}\x0a\x36\x03\x36\x05\x36\u{2bd}\x0a\x36\x03\x36\x03\x36\x03\x37\
	\x03\x37\x03\x37\x03\x37\x03\x37\x03\x37\x03\x38\x03\x38\x05\x38\u{2c9}\
	\x0a\x38\x03\x39\x03\x39\x03\x39\x03\x3a\x03\x3a\x03\x3a\x03\x3a\x05\x3a\
	\u{2d2}\x0a\x3a\x03\x3a\x03\x3a\x05\x3a\u{2d6}\x0a\x3a\x03\x3a\x05\x3a\u{2d9}\
	\x0a\x3a\x03\x3a\x03\x3a\x03\x3a\x03\x3b\x03\x3b\x05\x3b\u{2e0}\x0a\x3b\
	\x03\x3b\x03\x3b\x03\x3b\x03\x3b\x05\x3b\u{2e6}\x0a\x3b\x03\x3b\x03\x3b\
	\x03\x3c\x03\x3c\x03\x3d\x03\x3d\x03\x3d\x03\x3d\x03\x3d\x03\x3d\x03\x3d\
	\x03\x3d\x03\x3e\x03\x3e\x03\x3e\x03\x3f\x03\x3f\x03\x3f\x03\x40\x03\x40\
	\x05\x40\u{2fc}\x0a\x40\x03\x40\x03\x40\x03\x41\x03\x41\x03\x41\x03\x42\
	\x03\x42\x03\x42\x03\x42\x03\x43\x03\x43\x03\x43\x03\x43\x03\x44\x03\x44\
	\x03\x44\x03\x44\x03\x44\x03\x44\x03\x44\x05\x44\u{312}\x0a\x44\x03\x44\
	\x03\x44\x05\x44\u{316}\x0a\x44\x03\x44\x03\x44\x03\x45\x05\x45\u{31b}\x0a\
	\x45\x03\x45\x03\x45\x05\x45\u{31f}\x0a\x45\x07\x45\u{321}\x0a\x45\x0c\x45\
	\x0e\x45\u{324}\x0b\x45\x03\x46\x03\x46\x05\x46\u{328}\x0a\x46\x03\x46\x07\
	\x46\u{32b}\x0a\x46\x0c\x46\x0e\x46\u{32e}\x0b\x46\x03\x46\x05\x46\u{331}\
	\x0a\x46\x03\x46\x03\x46\x03\x47\x03\x47\x03\x48\x03\x48\x03\x48\x03\x48\
	\x03\x48\x03\x48\x03\x48\x03\x48\x03\x48\x03\x48\x03\x48\x03\x48\x03\x48\
	\x03\x48\x03\x48\x03\x48\x03\x48\x03\x48\x05\x48\u{349}\x0a\x48\x03\x48\
	\x03\x48\x03\x48\x03\x48\x03\x48\x03\x48\x03\x48\x03\x48\x03\x48\x03\x48\
	\x03\x48\x03\x48\x03\x48\x03\x48\x03\x48\x03\x48\x03\x48\x03\x48\x03\x48\
	\x03\x48\x03\x48\x03\x48\x03\x48\x03\x48\x03\x48\x03\x48\x03\x48\x03\x48\
	\x03\x48\x03\x48\x03\x48\x03\x48\x03\x48\x03\x48\x03\x48\x03\x48\x03\x48\
	\x03\x48\x03\x48\x03\x48\x03\x48\x03\x48\x03\x48\x03\x48\x03\x48\x03\x48\
	\x03\x48\x03\x48\x03\x48\x03\x48\x03\x48\x03\x48\x05\x48\u{37f}\x0a\x48\
	\x03\x48\x03\x48\x05\x48\u{383}\x0a\x48\x03\x48\x03\x48\x03\x48\x03\x48\
	\x03\x48\x03\x48\x03\x48\x03\x48\x03\x48\x03\x48\x03\x48\x03\x48\x03\x48\
	\x03\x48\x07\x48\u{393}\x0a\x48\x0c\x48\x0e\x48\u{396}\x0b\x48\x03\x49\x03\
	\x49\x03\x49\x03\x49\x03\x49\x03\x49\x03\x49\x03\x49\x03\x49\x05\x49\u{3a1}\
	\x0a\x49\x03\x4a\x03\x4a\x03\x4a\x07\x4a\u{3a6}\x0a\x4a\x0c\x4a\x0e\x4a\
	\u{3a9}\x0b\x4a\x03\x4b\x03\x4b\x03\x4b\x07\x4b\u{3ae}\x0a\x4b\x0c\x4b\x0e\
	\x4b\u{3b1}\x0b\x4b\x03\x4b\x05\x4b\u{3b4}\x0a\x4b\x03\x4c\x03\x4c\x03\x4c\
	\x03\x4c\x03\x4d\x03\x4d\x05\x4d\u{3bc}\x0a\x4d\x03\x4d\x03\x4d\x05\x4d\
	\u{3c0}\x0a\x4d\x05\x4d\u{3c2}\x0a\x4d\x03\x4e\x03\x4e\x03\x4e\x03\x4e\x03\
	\x4e\x03\x4f\x03\x4f\x07\x4f\u{3cb}\x0a\x4f\x0c\x4f\x0e\x4f\u{3ce}\x0b\x4f\
	\x03\x4f\x03\x4f\x03\x50\x03\x50\x03\x50\x03\x50\x03\x50\x03\x50\x03\x50\
	\x03\x50\x03\x50\x03\x50\x03\x50\x03\x50\x03\x50\x03\x50\x03\x50\x03\x50\
	\x03\x50\x05\x50\u{3e3}\x0a\x50\x03\x51\x03\x51\x03\x51\x05\x51\u{3e8}\x0a\
	\x51\x03\x52\x03\x52\x03\x52\x03\x52\x03\x53\x03\x53\x03\x53\x03\x53\x05\
	\x53\u{3f2}\x0a\x53\x03\x53\x03\x53\x05\x53\u{3f6}\x0a\x53\x03\x53\x03\x53\
	\x07\x53\u{3fa}\x0a\x53\x0c\x53\x0e\x53\u{3fd}\x0b\x53\x03\x53\x05\x53\u{400}\
	\x0a\x53\x03\x54\x03\x54\x03\x54\x03\x54\x05\x54\u{406}\x0a\x54\x03\x55\
	\x03\x55\x03\x55\x03\x55\x03\x56\x03\x56\x03\x56\x03\x56\x03\x56\x03\x56\
	\x03\x56\x05\x56\u{413}\x0a\x56\x03\x57\x03\x57\x03\x57\x07\x57\u{418}\x0a\
	\x57\x0c\x57\x0e\x57\u{41b}\x0b\x57\x03\x58\x03\x58\x03\x58\x03\x58\x03\
	\x59\x03\x59\x03\x59\x03\x5a\x03\x5a\x03\x5a\x07\x5a\u{427}\x0a\x5a\x0c\
	\x5a\x0e\x5a\u{42a}\x0b\x5a\x03\x5b\x03\x5b\x03\x5b\x03\x5b\x03\x5b\x03\
	\x5b\x05\x5b\u{432}\x0a\x5b\x03\x5c\x03\x5c\x03\x5c\x03\x5c\x05\x5c\u{438}\
	\x0a\x5c\x03\x5c\x03\x5c\x05\x5c\u{43c}\x0a\x5c\x03\x5c\x03\x5c\x03\x5d\
	\x03\x5d\x03\x5d\x03\x5e\x03\x5e\x03\x5e\x05\x5e\u{446}\x0a\x5e\x03\x5e\
	\x03\x5e\x03\x5e\x05\x5e\u{44b}\x0a\x5e\x03\x5e\x03\x5e\x03\x5f\x03\x5f\
	\x03\x5f\x03\x5f\x03\x60\x03\x60\x03\x60\x03\x60\x03\x60\x05\x60\u{458}\
	\x0a\x60\x03\x61\x03\x61\x05\x61\u{45c}\x0a\x61\x03\x61\x03\x61\x05\x61\
	\u{460}\x0a\x61\x07\x61\u{462}\x0a\x61\x0c\x61\x0e\x61\u{465}\x0b\x61\x03\
	\x61\x03\x61\x03\x61\x03\x61\x03\x61\x07\x61\u{46c}\x0a\x61\x0c\x61\x0e\
	\x61\u{46f}\x0b\x61\x05\x61\u{471}\x0a\x61\x03\x61\x05\x61\u{474}\x0a\x61\
	\x03\x62\x03\x62\x05\x62\u{478}\x0a\x62\x03\x63\x03\x63\x03\x64\x06\x64\
	\u{47d}\x0a\x64\x0d\x64\x0e\x64\u{47e}\x03\x65\x03\x65\x03\x65\x03\x65\x03\
	\x65\x07\x65\u{486}\x0a\x65\x0c\x65\x0e\x65\u{489}\x0b\x65\x03\x65\x03\x65\
	\x05\x65\u{48d}\x0a\x65\x03\x66\x06\x66\u{490}\x0a\x66\x0d\x66\x0e\x66\u{491}\
	\x03\x66\x02\x04\x4e\u{8e}\x67\x02\x04\x06\x08\x0a\x0c\x0e\x10\x12\x14\x16\
	\x18\x1a\x1c\x1e\x20\x22\x24\x26\x28\x2a\x2c\x2e\x30\x32\x34\x36\x38\x3a\
	\x3c\x3e\x40\x42\x44\x46\x48\x4a\x4c\x4e\x50\x52\x54\x56\x58\x5a\x5c\x5e\
	\x60\x62\x64\x66\x68\x6a\x6c\x6e\x70\x72\x74\x76\x78\x7a\x7c\x7e\u{80}\u{82}\
	\u{84}\u{86}\u{88}\u{8a}\u{8c}\u{8e}\u{90}\u{92}\u{94}\u{96}\u{98}\u{9a}\
	\u{9c}\u{9e}\u{a0}\u{a2}\u{a4}\u{a6}\u{a8}\u{aa}\u{ac}\u{ae}\u{b0}\u{b2}\
	\u{b4}\u{b6}\u{b8}\u{ba}\u{bc}\u{be}\u{c0}\u{c2}\u{c4}\u{c6}\u{c8}\u{ca}\
	\x02\x11\x03\x02\x07\x0d\x03\x02\x15\x17\x05\x02\x05\x05\x07\x0c\x1e\x25\
	\x03\x02\x32\x34\x06\x02\x70\x70\x77\x77\x7b\x7b\x7d\x7d\x05\x02\x2e\x2e\
	\x41\x44\x63\x67\x03\x02\x45\x46\x03\x02\x20\x21\x04\x02\x05\x05\x22\x23\
	\x03\x02\x4c\x4d\x03\x02\x09\x0c\x03\x02\x24\x25\x04\x02\x0d\x0d\x50\x59\
	\x03\x02\x69\x6a\x0c\x02\x10\x10\x1b\x1b\x2e\x2e\x34\x34\x40\x40\x61\x61\
	\x73\x73\x77\x77\x7e\x7f\u{81}\u{82}\x02\u{515}\x02\u{d9}\x03\x02\x02\x02\
	\x04\u{de}\x03\x02\x02\x02\x06\u{e3}\x03\x02\x02\x02\x08\u{e8}\x03\x02\x02\
	\x02\x0a\u{ea}\x03\x02\x02\x02\x0c\u{f4}\x03\x02\x02\x02\x0e\u{fe}\x03\x02\
	\x02\x02\x10\u{100}\x03\x02\x02\x02\x12\u{129}\x03\x02\x02\x02\x14\u{12b}\
	\x03\x02\x02\x02\x16\u{12e}\x03\x02\x02\x02\x18\u{146}\x03\x02\x02\x02\x1a\
	\u{157}\x03\x02\x02\x02\x1c\u{159}\x03\x02\x02\x02\x1e\u{16c}\x03\x02\x02\
	\x02\x20\u{173}\x03\x02\x02\x02\x22\u{178}\x03\x02\x02\x02\x24\u{17e}\x03\
	\x02\x02\x02\x26\u{196}\x03\x02\x02\x02\x28\u{198}\x03\x02\x02\x02\x2a\u{19d}\
	\x03\x02\x02\x02\x2c\u{19f}\x03\x02\x02\x02\x2e\u{1b0}\x03\x02\x02\x02\x30\
	\u{1c0}\x03\x02\x02\x02\x32\u{1c8}\x03\x02\x02\x02\x34\u{1d9}\x03\x02\x02\
	\x02\x36\u{1db}\x03\x02\x02\x02\x38\u{1e8}\x03\x02\x02\x02\x3a\u{1eb}\x03\
	\x02\x02\x02\x3c\u{1f3}\x03\x02\x02\x02\x3e\u{1f5}\x03\x02\x02\x02\x40\u{204}\
	\x03\x02\x02\x02\x42\u{211}\x03\x02\x02\x02\x44\u{218}\x03\x02\x02\x02\x46\
	\u{225}\x03\x02\x02\x02\x48\u{22c}\x03\x02\x02\x02\x4a\u{239}\x03\x02\x02\
	\x02\x4c\u{23d}\x03\x02\x02\x02\x4e\u{24a}\x03\x02\x02\x02\x50\u{257}\x03\
	\x02\x02\x02\x52\u{261}\x03\x02\x02\x02\x54\u{263}\x03\x02\x02\x02\x56\u{270}\
	\x03\x02\x02\x02\x58\u{272}\x03\x02\x02\x02\x5a\u{274}\x03\x02\x02\x02\x5c\
	\u{282}\x03\x02\x02\x02\x5e\u{284}\x03\x02\x02\x02\x60\u{286}\x03\x02\x02\
	\x02\x62\u{29e}\x03\x02\x02\x02\x64\u{2a0}\x03\x02\x02\x02\x66\u{2a3}\x03\
	\x02\x02\x02\x68\u{2ac}\x03\x02\x02\x02\x6a\u{2b7}\x03\x02\x02\x02\x6c\u{2c0}\
	\x03\x02\x02\x02\x6e\u{2c8}\x03\x02\x02\x02\x70\u{2ca}\x03\x02\x02\x02\x72\
	\u{2cd}\x03\x02\x02\x02\x74\u{2dd}\x03\x02\x02\x02\x76\u{2e9}\x03\x02\x02\
	\x02\x78\u{2eb}\x03\x02\x02\x02\x7a\u{2f3}\x03\x02\x02\x02\x7c\u{2f6}\x03\
	\x02\x02\x02\x7e\u{2f9}\x03\x02\x02\x02\u{80}\u{2ff}\x03\x02\x02\x02\u{82}\
	\u{302}\x03\x02\x02\x02\u{84}\u{306}\x03\x02\x02\x02\u{86}\u{311}\x03\x02\
	\x02\x02\u{88}\u{31a}\x03\x02\x02\x02\u{8a}\u{325}\x03\x02\x02\x02\u{8c}\
	\u{334}\x03\x02\x02\x02\u{8e}\u{348}\x03\x02\x02\x02\u{90}\u{3a0}\x03\x02\
	\x02\x02\u{92}\u{3a2}\x03\x02\x02\x02\u{94}\u{3aa}\x03\x02\x02\x02\u{96}\
	\u{3b5}\x03\x02\x02\x02\u{98}\u{3c1}\x03\x02\x02\x02\u{9a}\u{3c3}\x03\x02\
	\x02\x02\u{9c}\u{3c8}\x03\x02\x02\x02\u{9e}\u{3e2}\x03\x02\x02\x02\u{a0}\
	\u{3e7}\x03\x02\x02\x02\u{a2}\u{3e9}\x03\x02\x02\x02\u{a4}\u{3f1}\x03\x02\
	\x02\x02\u{a6}\u{401}\x03\x02\x02\x02\u{a8}\u{407}\x03\x02\x02\x02\u{aa}\
	\u{412}\x03\x02\x02\x02\u{ac}\u{414}\x03\x02\x02\x02\u{ae}\u{41c}\x03\x02\
	\x02\x02\u{b0}\u{420}\x03\x02\x02\x02\u{b2}\u{423}\x03\x02\x02\x02\u{b4}\
	\u{431}\x03\x02\x02\x02\u{b6}\u{433}\x03\x02\x02\x02\u{b8}\u{43f}\x03\x02\
	\x02\x02\u{ba}\u{442}\x03\x02\x02\x02\u{bc}\u{44e}\x03\x02\x02\x02\u{be}\
	\u{457}\x03\x02\x02\x02\u{c0}\u{473}\x03\x02\x02\x02\u{c2}\u{475}\x03\x02\
	\x02\x02\u{c4}\u{479}\x03\x02\x02\x02\u{c6}\u{47c}\x03\x02\x02\x02\u{c8}\
	\u{480}\x03\x02\x02\x02\u{ca}\u{48f}\x03\x02\x02\x02\u{cc}\u{d8}\x05\x04\
	\x03\x02\u{cd}\u{d8}\x05\x12\x0a\x02\u{ce}\u{d8}\x05\x16\x0c\x02\u{cf}\u{d8}\
	\x05\x3e\x20\x02\u{d0}\u{d8}\x05\x3a\x1e\x02\u{d1}\u{d8}\x05\x2c\x17\x02\
	\u{d2}\u{d8}\x05\x32\x1a\x02\u{d3}\u{d8}\x05\x1e\x10\x02\u{d4}\u{d8}\x05\
	\x20\x11\x02\u{d5}\u{d8}\x05\x22\x12\x02\u{d6}\u{d8}\x05\x24\x13\x02\u{d7}\
	\u{cc}\x03\x02\x02\x02\u{d7}\u{cd}\x03\x02\x02\x02\u{d7}\u{ce}\x03\x02\x02\
	\x02\u{d7}\u{cf}\x03\x02\x02\x02\u{d7}\u{d0}\x03\x02\x02\x02\u{d7}\u{d1}\
	\x03\x02\x02\x02\u{d7}\u{d2}\x03\x02\x02\x02\u{d7}\u{d3}\x03\x02\x02\x02\
	\u{d7}\u{d4}\x03\x02\x02\x02\u{d7}\u{d5}\x03\x02\x02\x02\u{d7}\u{d6}\x03\
	\x02\x02\x02\u{d8}\u{db}\x03\x02\x02\x02\u{d9}\u{d7}\x03\x02\x02\x02\u{d9}\
	\u{da}\x03\x02\x02\x02\u{da}\u{dc}\x03\x02\x02\x02\u{db}\u{d9}\x03\x02\x02\
	\x02\u{dc}\u{dd}\x07\x02\x02\x03\u{dd}\x03\x03\x02\x02\x02\u{de}\u{df}\x07\
	\x03\x02\x02\u{df}\u{e0}\x05\x06\x04\x02\u{e0}\u{e1}\x05\x08\x05\x02\u{e1}\
	\u{e2}\x07\x04\x02\x02\u{e2}\x05\x03\x02\x02\x02\u{e3}\u{e4}\x05\u{c4}\x63\
	\x02\u{e4}\x07\x03\x02\x02\x02\u{e5}\u{e9}\x07\x05\x02\x02\u{e6}\u{e9}\x05\
	\x0a\x06\x02\u{e7}\u{e9}\x05\u{8e}\x48\x02\u{e8}\u{e5}\x03\x02\x02\x02\u{e8}\
	\u{e6}\x03\x02\x02\x02\u{e8}\u{e7}\x03\x02\x02\x02\u{e9}\x09\x03\x02\x02\
	\x02\u{ea}\u{f1}\x05\x0e\x08\x02\u{eb}\u{ed}\x07\x06\x02\x02\u{ec}\u{eb}\
	\x03\x02\x02\x02\u{ec}\u{ed}\x03\x02\x02\x02\u{ed}\u{ee}\x03\x02\x02\x02\
	\u{ee}\u{f0}\x05\x0e\x08\x02\u{ef}\u{ec}\x03\x02\x02\x02\u{f0}\u{f3}\x03\
	\x02\x02\x02\u{f1}\u{ef}\x03\x02\x02\x02\u{f1}\u{f2}\x03\x02\x02\x02\u{f2}\
	\x0b\x03\x02\x02\x02\u{f3}\u{f1}\x03\x02\x02\x02\u{f4}\u{f5}\x09\x02\x02\
	\x02\u{f5}\x0d\x03\x02\x02\x02\u{f6}\u{f8}\x05\x0c\x07\x02\u{f7}\u{f6}\x03\
	\x02\x02\x02\u{f7}\u{f8}\x03\x02\x02\x02\u{f8}\u{f9}\x03\x02\x02\x02\u{f9}\
	\u{ff}\x07\u{84}\x02\x02\u{fa}\u{fc}\x05\x0c\x07\x02\u{fb}\u{fa}\x03\x02\
	\x02\x02\u{fb}\u{fc}\x03\x02\x02\x02\u{fc}\u{fd}\x03\x02\x02\x02\u{fd}\u{ff}\
	\x07\x69\x02\x02\u{fe}\u{f7}\x03\x02\x02\x02\u{fe}\u{fb}\x03\x02\x02\x02\
	\u{ff}\x0f\x03\x02\x02\x02\u{100}\u{103}\x05\u{c4}\x63\x02\u{101}\u{102}\
	\x07\x0e\x02\x02\u{102}\u{104}\x05\u{c4}\x63\x02\u{103}\u{101}\x03\x02\x02\
	\x02\u{103}\u{104}\x03\x02\x02\x02\u{104}\x11\x03\x02\x02\x02\u{105}\u{106}\
	\x07\x0f\x02\x02\u{106}\u{109}\x05\x14\x0b\x02\u{107}\u{108}\x07\x0e\x02\
	\x02\u{108}\u{10a}\x05\u{c4}\x63\x02\u{109}\u{107}\x03\x02\x02\x02\u{109}\
	\u{10a}\x03\x02\x02\x02\u{10a}\u{10b}\x03\x02\x02\x02\u{10b}\u{10c}\x07\
	\x04\x02\x02\u{10c}\u{12a}\x03\x02\x02\x02\u{10d}\u{110}\x07\x0f\x02\x02\
	\u{10e}\u{111}\x07\x05\x02\x02\u{10f}\u{111}\x05\u{c4}\x63\x02\u{110}\u{10e}\
	\x03\x02\x02\x02\u{110}\u{10f}\x03\x02\x02\x02\u{111}\u{114}\x03\x02\x02\
	\x02\u{112}\u{113}\x07\x0e\x02\x02\u{113}\u{115}\x05\u{c4}\x63\x02\u{114}\
	\u{112}\x03\x02\x02\x02\u{114}\u{115}\x03\x02\x02\x02\u{115}\u{116}\x03\
	\x02\x02\x02\u{116}\u{117}\x07\x10\x02\x02\u{117}\u{118}\x05\x14\x0b\x02\
	\u{118}\u{119}\x07\x04\x02\x02\u{119}\u{12a}\x03\x02\x02\x02\u{11a}\u{11b}\
	\x07\x0f\x02\x02\u{11b}\u{11c}\x07\x11\x02\x02\u{11c}\u{121}\x05\x10\x09\
	\x02\u{11d}\u{11e}\x07\x12\x02\x02\u{11e}\u{120}\x05\x10\x09\x02\u{11f}\
	\u{11d}\x03\x02\x02\x02\u{120}\u{123}\x03\x02\x02\x02\u{121}\u{11f}\x03\
	\x02\x02\x02\u{121}\u{122}\x03\x02\x02\x02\u{122}\u{124}\x03\x02\x02\x02\
	\u{123}\u{121}\x03\x02\x02\x02\u{124}\u{125}\x07\x13\x02\x02\u{125}\u{126}\
	\x07\x10\x02\x02\u{126}\u{127}\x05\x14\x0b\x02\u{127}\u{128}\x07\x04\x02\
	\x02\u{128}\u{12a}\x03\x02\x02\x02\u{129}\u{105}\x03\x02\x02\x02\u{129}\
	\u{10d}\x03\x02\x02\x02\u{129}\u{11a}\x03\x02\x02\x02\u{12a}\x13\x03\x02\
	\x02\x02\u{12b}\u{12c}\x07\u{83}\x02\x02\u{12c}\x15\x03\x02\x02\x02\u{12d}\
	\u{12f}\x07\x14\x02\x02\u{12e}\u{12d}\x03\x02\x02\x02\u{12e}\u{12f}\x03\
	\x02\x02\x02\u{12f}\u{130}\x03\x02\x02\x02\u{130}\u{131}\x09\x03\x02\x02\
	\u{131}\u{13b}\x05\u{c4}\x63\x02\u{132}\u{133}\x07\x18\x02\x02\u{133}\u{138}\
	\x05\x18\x0d\x02\u{134}\u{135}\x07\x12\x02\x02\u{135}\u{137}\x05\x18\x0d\
	\x02\u{136}\u{134}\x03\x02\x02\x02\u{137}\u{13a}\x03\x02\x02\x02\u{138}\
	\u{136}\x03\x02\x02\x02\u{138}\u{139}\x03\x02\x02\x02\u{139}\u{13c}\x03\
	\x02\x02\x02\u{13a}\u{138}\x03\x02\x02\x02\u{13b}\u{132}\x03\x02\x02\x02\
	\u{13b}\u{13c}\x03\x02\x02\x02\u{13c}\u{13d}\x03\x02\x02\x02\u{13d}\u{141}\
	\x07\x11\x02\x02\u{13e}\u{140}\x05\x1a\x0e\x02\u{13f}\u{13e}\x03\x02\x02\
	\x02\u{140}\u{143}\x03\x02\x02\x02\u{141}\u{13f}\x03\x02\x02\x02\u{141}\
	\u{142}\x03\x02\x02\x02\u{142}\u{144}\x03\x02\x02\x02\u{143}\u{141}\x03\
	\x02\x02\x02\u{144}\u{145}\x07\x13\x02\x02\u{145}\x17\x03\x02\x02\x02\u{146}\
	\u{14c}\x05\x50\x29\x02\u{147}\u{149}\x07\x19\x02\x02\u{148}\u{14a}\x05\
	\u{92}\x4a\x02\u{149}\u{148}\x03\x02\x02\x02\u{149}\u{14a}\x03\x02\x02\x02\
	\u{14a}\u{14b}\x03\x02\x02\x02\u{14b}\u{14d}\x07\x1a\x02\x02\u{14c}\u{147}\
	\x03\x02\x02\x02\u{14c}\u{14d}\x03\x02\x02\x02\u{14d}\x19\x03\x02\x02\x02\
	\u{14e}\u{158}\x05\x1c\x0f\x02\u{14f}\u{158}\x05\x24\x13\x02\u{150}\u{158}\
	\x05\x2c\x17\x02\u{151}\u{158}\x05\x2e\x18\x02\u{152}\u{158}\x05\x32\x1a\
	\x02\u{153}\u{158}\x05\x3a\x1e\x02\u{154}\u{158}\x05\x3e\x20\x02\u{155}\
	\u{158}\x05\x20\x11\x02\u{156}\u{158}\x05\x22\x12\x02\u{157}\u{14e}\x03\
	\x02\x02\x02\u{157}\u{14f}\x03\x02\x02\x02\u{157}\u{150}\x03\x02\x02\x02\
	\u{157}\u{151}\x03\x02\x02\x02\u{157}\u{152}\x03\x02\x02\x02\u{157}\u{153}\
	\x03\x02\x02\x02\u{157}\u{154}\x03\x02\x02\x02\u{157}\u{155}\x03\x02\x02\
	\x02\u{157}\u{156}\x03\x02\x02\x02\u{158}\x1b\x03\x02\x02\x02\u{159}\u{162}\
	\x05\x4e\x28\x02\u{15a}\u{161}\x07\x79\x02\x02\u{15b}\u{161}\x07\x76\x02\
	\x02\u{15c}\u{161}\x07\x78\x02\x02\u{15d}\u{161}\x07\x70\x02\x02\u{15e}\
	\u{161}\x07\x71\x02\x02\u{15f}\u{161}\x05\u{c8}\x65\x02\u{160}\u{15a}\x03\
	\x02\x02\x02\u{160}\u{15b}\x03\x02\x02\x02\u{160}\u{15c}\x03\x02\x02\x02\
	\u{160}\u{15d}\x03\x02\x02\x02\u{160}\u{15e}\x03\x02\x02\x02\u{160}\u{15f}\
	\x03\x02\x02\x02\u{161}\u{164}\x03\x02\x02\x02\u{162}\u{160}\x03\x02\x02\
	\x02\u{162}\u{163}\x03\x02\x02\x02\u{163}\u{165}\x03\x02\x02\x02\u{164}\
	\u{162}\x03\x02\x02\x02\u{165}\u{168}\x05\u{c4}\x63\x02\u{166}\u{167}\x07\
	\x0d\x02\x02\u{167}\u{169}\x05\u{8e}\x48\x02\u{168}\u{166}\x03\x02\x02\x02\
	\u{168}\u{169}\x03\x02\x02\x02\u{169}\u{16a}\x03\x02\x02\x02\u{16a}\u{16b}\
	\x07\x04\x02\x02\u{16b}\x1d\x03\x02\x02\x02\u{16c}\u{16d}\x05\x4e\x28\x02\
	\u{16d}\u{16e}\x07\x70\x02\x02\u{16e}\u{16f}\x05\u{c4}\x63\x02\u{16f}\u{170}\
	\x07\x0d\x02\x02\u{170}\u{171}\x05\u{8e}\x48\x02\u{171}\u{172}\x07\x04\x02\
	\x02\u{172}\x1f\x03\x02\x02\x02\u{173}\u{174}\x07\x1b\x02\x02\u{174}\u{175}\
	\x05\u{c4}\x63\x02\u{175}\u{176}\x05\x40\x21\x02\u{176}\u{177}\x07\x04\x02\
	\x02\u{177}\x21\x03\x02\x02\x02\u{178}\u{179}\x07\x7c\x02\x02\u{179}\u{17a}\
	\x05\u{c4}\x63\x02\u{17a}\u{17b}\x07\x18\x02\x02\u{17b}\u{17c}\x05\u{8c}\
	\x47\x02\u{17c}\u{17d}\x07\x04\x02\x02\u{17d}\x23\x03\x02\x02\x02\u{17e}\
	\u{17f}\x07\x1c\x02\x02\u{17f}\u{180}\x05\x26\x14\x02\u{180}\u{183}\x07\
	\x1d\x02\x02\u{181}\u{184}\x07\x05\x02\x02\u{182}\u{184}\x05\x4e\x28\x02\
	\u{183}\u{181}\x03\x02\x02\x02\u{183}\u{182}\x03\x02\x02\x02\u{184}\u{186}\
	\x03\x02\x02\x02\u{185}\u{187}\x07\x7e\x02\x02\u{186}\u{185}\x03\x02\x02\
	\x02\u{186}\u{187}\x03\x02\x02\x02\u{187}\u{188}\x03\x02\x02\x02\u{188}\
	\u{189}\x07\x04\x02\x02\u{189}\x25\x03\x02\x02\x02\u{18a}\u{197}\x05\x50\
	\x29\x02\u{18b}\u{18c}\x07\x11\x02\x02\u{18c}\u{191}\x05\x28\x15\x02\u{18d}\
	\u{18e}\x07\x12\x02\x02\u{18e}\u{190}\x05\x28\x15\x02\u{18f}\u{18d}\x03\
	\x02\x02\x02\u{190}\u{193}\x03\x02\x02\x02\u{191}\u{18f}\x03\x02\x02\x02\
	\u{191}\u{192}\x03\x02\x02\x02\u{192}\u{194}\x03\x02\x02\x02\u{193}\u{191}\
	\x03\x02\x02\x02\u{194}\u{195}\x07\x13\x02\x02\u{195}\u{197}\x03\x02\x02\
	\x02\u{196}\u{18a}\x03\x02\x02\x02\u{196}\u{18b}\x03\x02\x02\x02\u{197}\
	\x27\x03\x02\x02\x02\u{198}\u{19b}\x05\x50\x29\x02\u{199}\u{19a}\x07\x0e\
	\x02\x02\u{19a}\u{19c}\x05\x2a\x16\x02\u{19b}\u{199}\x03\x02\x02\x02\u{19b}\
	\u{19c}\x03\x02\x02\x02\u{19c}\x29\x03\x02\x02\x02\u{19d}\u{19e}\x09\x04\
	\x02\x02\u{19e}\x2b\x03\x02\x02\x02\u{19f}\u{1a0}\x07\x26\x02\x02\u{1a0}\
	\u{1a1}\x05\u{c4}\x63\x02\u{1a1}\u{1ac}\x07\x11\x02\x02\u{1a2}\u{1a3}\x05\
	\x4c\x27\x02\u{1a3}\u{1a9}\x07\x04\x02\x02\u{1a4}\u{1a5}\x05\x4c\x27\x02\
	\u{1a5}\u{1a6}\x07\x04\x02\x02\u{1a6}\u{1a8}\x03\x02\x02\x02\u{1a7}\u{1a4}\
	\x03\x02\x02\x02\u{1a8}\u{1ab}\x03\x02\x02\x02\u{1a9}\u{1a7}\x03\x02\x02\
	\x02\u{1a9}\u{1aa}\x03\x02\x02\x02\u{1aa}\u{1ad}\x03\x02\x02\x02\u{1ab}\
	\u{1a9}\x03\x02\x02\x02\u{1ac}\u{1a2}\x03\x02\x02\x02\u{1ac}\u{1ad}\x03\
	\x02\x02\x02\u{1ad}\u{1ae}\x03\x02\x02\x02\u{1ae}\u{1af}\x07\x13\x02\x02\
	\u{1af}\x2d\x03\x02\x02\x02\u{1b0}\u{1b1}\x07\x27\x02\x02\u{1b1}\u{1b3}\
	\x05\u{c4}\x63\x02\u{1b2}\u{1b4}\x05\x40\x21\x02\u{1b3}\u{1b2}\x03\x02\x02\
	\x02\u{1b3}\u{1b4}\x03\x02\x02\x02\u{1b4}\u{1b9}\x03\x02\x02\x02\u{1b5}\
	\u{1b8}\x07\x7a\x02\x02\u{1b6}\u{1b8}\x05\u{c8}\x65\x02\u{1b7}\u{1b5}\x03\
	\x02\x02\x02\u{1b7}\u{1b6}\x03\x02\x02\x02\u{1b8}\u{1bb}\x03\x02\x02\x02\
	\u{1b9}\u{1b7}\x03\x02\x02\x02\u{1b9}\u{1ba}\x03\x02\x02\x02\u{1ba}\u{1be}\
	\x03\x02\x02\x02\u{1bb}\u{1b9}\x03\x02\x02\x02\u{1bc}\u{1bf}\x07\x04\x02\
	\x02\u{1bd}\u{1bf}\x05\x60\x31\x02\u{1be}\u{1bc}\x03\x02\x02\x02\u{1be}\
	\u{1bd}\x03\x02\x02\x02\u{1bf}\x2f\x03\x02\x02\x02\u{1c0}\u{1c6}\x05\u{c4}\
	\x63\x02\u{1c1}\u{1c3}\x07\x19\x02\x02\u{1c2}\u{1c4}\x05\u{92}\x4a\x02\u{1c3}\
	\u{1c2}\x03\x02\x02\x02\u{1c3}\u{1c4}\x03\x02\x02\x02\u{1c4}\u{1c5}\x03\
	\x02\x02\x02\u{1c5}\u{1c7}\x07\x1a\x02\x02\u{1c6}\u{1c1}\x03\x02\x02\x02\
	\u{1c6}\u{1c7}\x03\x02\x02\x02\u{1c7}\x31\x03\x02\x02\x02\u{1c8}\u{1c9}\
	\x05\x34\x1b\x02\u{1c9}\u{1ca}\x05\x40\x21\x02\u{1ca}\u{1cc}\x05\x38\x1d\
	\x02\u{1cb}\u{1cd}\x05\x36\x1c\x02\u{1cc}\u{1cb}\x03\x02\x02\x02\u{1cc}\
	\u{1cd}\x03\x02\x02\x02\u{1cd}\u{1d0}\x03\x02\x02\x02\u{1ce}\u{1d1}\x07\
	\x04\x02\x02\u{1cf}\u{1d1}\x05\x60\x31\x02\u{1d0}\u{1ce}\x03\x02\x02\x02\
	\u{1d0}\u{1cf}\x03\x02\x02\x02\u{1d1}\x33\x03\x02\x02\x02\u{1d2}\u{1d4}\
	\x07\x28\x02\x02\u{1d3}\u{1d5}\x05\u{c4}\x63\x02\u{1d4}\u{1d3}\x03\x02\x02\
	\x02\u{1d4}\u{1d5}\x03\x02\x02\x02\u{1d5}\u{1da}\x03\x02\x02\x02\u{1d6}\
	\u{1da}\x07\x7f\x02\x02\u{1d7}\u{1da}\x07\u{80}\x02\x02\u{1d8}\u{1da}\x07\
	\u{81}\x02\x02\u{1d9}\u{1d2}\x03\x02\x02\x02\u{1d9}\u{1d6}\x03\x02\x02\x02\
	\u{1d9}\u{1d7}\x03\x02\x02\x02\u{1d9}\u{1d8}\x03\x02\x02\x02\u{1da}\x35\
	\x03\x02\x02\x02\u{1db}\u{1dc}\x07\x29\x02\x02\u{1dc}\u{1dd}\x05\x40\x21\
	\x02\u{1dd}\x37\x03\x02\x02\x02\u{1de}\u{1e7}\x07\x74\x02\x02\u{1df}\u{1e7}\
	\x07\x79\x02\x02\u{1e0}\u{1e7}\x07\x76\x02\x02\u{1e1}\u{1e7}\x07\x78\x02\
	\x02\u{1e2}\u{1e7}\x07\x7a\x02\x02\u{1e3}\u{1e7}\x05\x5e\x30\x02\u{1e4}\
	\u{1e7}\x05\x30\x19\x02\u{1e5}\u{1e7}\x05\u{c8}\x65\x02\u{1e6}\u{1de}\x03\
	\x02\x02\x02\u{1e6}\u{1df}\x03\x02\x02\x02\u{1e6}\u{1e0}\x03\x02\x02\x02\
	\u{1e6}\u{1e1}\x03\x02\x02\x02\u{1e6}\u{1e2}\x03\x02\x02\x02\u{1e6}\u{1e3}\
	\x03\x02\x02\x02\u{1e6}\u{1e4}\x03\x02\x02\x02\u{1e6}\u{1e5}\x03\x02\x02\
	\x02\u{1e7}\u{1ea}\x03\x02\x02\x02\u{1e8}\u{1e6}\x03\x02\x02\x02\u{1e8}\
	\u{1e9}\x03\x02\x02\x02\u{1e9}\x39\x03\x02\x02\x02\u{1ea}\u{1e8}\x03\x02\
	\x02\x02\u{1eb}\u{1ec}\x07\x2a\x02\x02\u{1ec}\u{1ed}\x05\u{c4}\x63\x02\u{1ed}\
	\u{1ef}\x05\x44\x23\x02\u{1ee}\u{1f0}\x07\x6e\x02\x02\u{1ef}\u{1ee}\x03\
	\x02\x02\x02\u{1ef}\u{1f0}\x03\x02\x02\x02\u{1f0}\u{1f1}\x03\x02\x02\x02\
	\u{1f1}\u{1f2}\x07\x04\x02\x02\u{1f2}\x3b\x03\x02\x02\x02\u{1f3}\u{1f4}\
	\x05\u{c4}\x63\x02\u{1f4}\x3d\x03\x02\x02\x02\u{1f5}\u{1f6}\x07\x2b\x02\
	\x02\u{1f6}\u{1f7}\x05\u{c4}\x63\x02\u{1f7}\u{1f9}\x07\x11\x02\x02\u{1f8}\
	\u{1fa}\x05\x3c\x1f\x02\u{1f9}\u{1f8}\x03\x02\x02\x02\u{1f9}\u{1fa}\x03\
	\x02\x02\x02\u{1fa}\u{1ff}\x03\x02\x02\x02\u{1fb}\u{1fc}\x07\x12\x02\x02\
	\u{1fc}\u{1fe}\x05\x3c\x1f\x02\u{1fd}\u{1fb}\x03\x02\x02\x02\u{1fe}\u{201}\
	\x03\x02\x02\x02\u{1ff}\u{1fd}\x03\x02\x02\x02\u{1ff}\u{200}\x03\x02\x02\
	\x02\u{200}\u{202}\x03\x02\x02\x02\u{201}\u{1ff}\x03\x02\x02\x02\u{202}\
	\u{203}\x07\x13\x02\x02\u{203}\x3f\x03\x02\x02\x02\u{204}\u{20d}\x07\x19\
	\x02\x02\u{205}\u{20a}\x05\x42\x22\x02\u{206}\u{207}\x07\x12\x02\x02\u{207}\
	\u{209}\x05\x42\x22\x02\u{208}\u{206}\x03\x02\x02\x02\u{209}\u{20c}\x03\
	\x02\x02\x02\u{20a}\u{208}\x03\x02\x02\x02\u{20a}\u{20b}\x03\x02\x02\x02\
	\u{20b}\u{20e}\x03\x02\x02\x02\u{20c}\u{20a}\x03\x02\x02\x02\u{20d}\u{205}\
	\x03\x02\x02\x02\u{20d}\u{20e}\x03\x02\x02\x02\u{20e}\u{20f}\x03\x02\x02\
	\x02\u{20f}\u{210}\x07\x1a\x02\x02\u{210}\x41\x03\x02\x02\x02\u{211}\u{213}\
	\x05\x4e\x28\x02\u{212}\u{214}\x05\x5c\x2f\x02\u{213}\u{212}\x03\x02\x02\
	\x02\u{213}\u{214}\x03\x02\x02\x02\u{214}\u{216}\x03\x02\x02\x02\u{215}\
	\u{217}\x05\u{c4}\x63\x02\u{216}\u{215}\x03\x02\x02\x02\u{216}\u{217}\x03\
	\x02\x02\x02\u{217}\x43\x03\x02\x02\x02\u{218}\u{221}\x07\x19\x02\x02\u{219}\
	\u{21e}\x05\x46\x24\x02\u{21a}\u{21b}\x07\x12\x02\x02\u{21b}\u{21d}\x05\
	\x46\x24\x02\u{21c}\u{21a}\x03\x02\x02\x02\u{21d}\u{220}\x03\x02\x02\x02\
	\u{21e}\u{21c}\x03\x02\x02\x02\u{21e}\u{21f}\x03\x02\x02\x02\u{21f}\u{222}\
	\x03\x02\x02\x02\u{220}\u{21e}\x03\x02\x02\x02\u{221}\u{219}\x03\x02\x02\
	\x02\u{221}\u{222}\x03\x02\x02\x02\u{222}\u{223}\x03\x02\x02\x02\u{223}\
	\u{224}\x07\x1a\x02\x02\u{224}\x45\x03\x02\x02\x02\u{225}\u{227}\x05\x4e\
	\x28\x02\u{226}\u{228}\x07\x75\x02\x02\u{227}\u{226}\x03\x02\x02\x02\u{227}\
	\u{228}\x03\x02\x02\x02\u{228}\u{22a}\x03\x02\x02\x02\u{229}\u{22b}\x05\
	\u{c4}\x63\x02\u{22a}\u{229}\x03\x02\x02\x02\u{22a}\u{22b}\x03\x02\x02\x02\
	\u{22b}\x47\x03\x02\x02\x02\u{22c}\u{235}\x07\x19\x02\x02\u{22d}\u{232}\
	\x05\x4a\x26\x02\u{22e}\u{22f}\x07\x12\x02\x02\u{22f}\u{231}\x05\x4a\x26\
	\x02\u{230}\u{22e}\x03\x02\x02\x02\u{231}\u{234}\x03\x02\x02\x02\u{232}\
	\u{230}\x03\x02\x02\x02\u{232}\u{233}\x03\x02\x02\x02\u{233}\u{236}\x03\
	\x02\x02\x02\u{234}\u{232}\x03\x02\x02\x02\u{235}\u{22d}\x03\x02\x02\x02\
	\u{235}\u{236}\x03\x02\x02\x02\u{236}\u{237}\x03\x02\x02\x02\u{237}\u{238}\
	\x07\x1a\x02\x02\u{238}\x49\x03\x02\x02\x02\u{239}\u{23b}\x05\x4e\x28\x02\
	\u{23a}\u{23c}\x05\x5c\x2f\x02\u{23b}\u{23a}\x03\x02\x02\x02\u{23b}\u{23c}\
	\x03\x02\x02\x02\u{23c}\x4b\x03\x02\x02\x02\u{23d}\u{23f}\x05\x4e\x28\x02\
	\u{23e}\u{240}\x05\x5c\x2f\x02\u{23f}\u{23e}\x03\x02\x02\x02\u{23f}\u{240}\
	\x03\x02\x02\x02\u{240}\u{241}\x03\x02\x02\x02\u{241}\u{242}\x05\u{c4}\x63\
	\x02\u{242}\x4d\x03\x02\x02\x02\u{243}\u{244}\x08\x28\x01\x02\u{244}\u{24b}\
	\x05\u{8c}\x47\x02\u{245}\u{24b}\x05\x50\x29\x02\u{246}\u{24b}\x05\x54\x2b\
	\x02\u{247}\u{24b}\x05\x5a\x2e\x02\u{248}\u{249}\x07\x2e\x02\x02\u{249}\
	\u{24b}\x07\x77\x02\x02\u{24a}\u{243}\x03\x02\x02\x02\u{24a}\u{245}\x03\
	\x02\x02\x02\u{24a}\u{246}\x03\x02\x02\x02\u{24a}\u{247}\x03\x02\x02\x02\
	\u{24a}\u{248}\x03\x02\x02\x02\u{24b}\u{254}\x03\x02\x02\x02\u{24c}\u{24d}\
	\x0c\x05\x02\x02\u{24d}\u{24f}\x07\x2c\x02\x02\u{24e}\u{250}\x05\u{8e}\x48\
	\x02\u{24f}\u{24e}\x03\x02\x02\x02\u{24f}\u{250}\x03\x02\x02\x02\u{250}\
	\u{251}\x03\x02\x02\x02\u{251}\u{253}\x07\x2d\x02\x02\u{252}\u{24c}\x03\
	\x02\x02\x02\u{253}\u{256}\x03\x02\x02\x02\u{254}\u{252}\x03\x02\x02\x02\
	\u{254}\u{255}\x03\x02\x02\x02\u{255}\x4f\x03\x02\x02\x02\u{256}\u{254}\
	\x03\x02\x02\x02\u{257}\u{25c}\x05\u{c4}\x63\x02\u{258}\u{259}\x07\x2f\x02\
	\x02\u{259}\u{25b}\x05\u{c4}\x63\x02\u{25a}\u{258}\x03\x02\x02\x02\u{25b}\
	\u{25e}\x03\x02\x02\x02\u{25c}\u{25a}\x03\x02\x02\x02\u{25c}\u{25d}\x03\
	\x02\x02\x02\u{25d}\x51\x03\x02\x02\x02\u{25e}\u{25c}\x03\x02\x02\x02\u{25f}\
	\u{262}\x05\u{8c}\x47\x02\u{260}\u{262}\x05\x50\x29\x02\u{261}\u{25f}\x03\
	\x02\x02\x02\u{261}\u{260}\x03\x02\x02\x02\u{262}\x53\x03\x02\x02\x02\u{263}\
	\u{264}\x07\x30\x02\x02\u{264}\u{265}\x07\x19\x02\x02\u{265}\u{267}\x05\
	\x52\x2a\x02\u{266}\u{268}\x05\x56\x2c\x02\u{267}\u{266}\x03\x02\x02\x02\
	\u{267}\u{268}\x03\x02\x02\x02\u{268}\u{269}\x03\x02\x02\x02\u{269}\u{26a}\
	\x07\x31\x02\x02\u{26a}\u{26c}\x05\x4e\x28\x02\u{26b}\u{26d}\x05\x58\x2d\
	\x02\u{26c}\u{26b}\x03\x02\x02\x02\u{26c}\u{26d}\x03\x02\x02\x02\u{26d}\
	\u{26e}\x03\x02\x02\x02\u{26e}\u{26f}\x07\x1a\x02\x02\u{26f}\x55\x03\x02\
	\x02\x02\u{270}\u{271}\x05\u{c4}\x63\x02\u{271}\x57\x03\x02\x02\x02\u{272}\
	\u{273}\x05\u{c4}\x63\x02\u{273}\x59\x03\x02\x02\x02\u{274}\u{275}\x07\x28\
	\x02\x02\u{275}\u{27b}\x05\x48\x25\x02\u{276}\u{27a}\x07\x76\x02\x02\u{277}\
	\u{27a}\x07\x74\x02\x02\u{278}\u{27a}\x05\x5e\x30\x02\u{279}\u{276}\x03\
	\x02\x02\x02\u{279}\u{277}\x03\x02\x02\x02\u{279}\u{278}\x03\x02\x02\x02\
	\u{27a}\u{27d}\x03\x02\x02\x02\u{27b}\u{279}\x03\x02\x02\x02\u{27b}\u{27c}\
	\x03\x02\x02\x02\u{27c}\u{280}\x03\x02\x02\x02\u{27d}\u{27b}\x03\x02\x02\
	\x02\u{27e}\u{27f}\x07\x29\x02\x02\u{27f}\u{281}\x05\x48\x25\x02\u{280}\
	\u{27e}\x03\x02\x02\x02\u{280}\u{281}\x03\x02\x02\x02\u{281}\x5b\x03\x02\
	\x02\x02\u{282}\u{283}\x09\x05\x02\x02\u{283}\x5d\x03\x02\x02\x02\u{284}\
	\u{285}\x09\x06\x02\x02\u{285}\x5f\x03\x02\x02\x02\u{286}\u{28a}\x07\x11\
	\x02\x02\u{287}\u{289}\x05\x62\x32\x02\u{288}\u{287}\x03\x02\x02\x02\u{289}\
	\u{28c}\x03\x02\x02\x02\u{28a}\u{288}\x03\x02\x02\x02\u{28a}\u{28b}\x03\
	\x02\x02\x02\u{28b}\u{28d}\x03\x02\x02\x02\u{28c}\u{28a}\x03\x02\x02\x02\
	\u{28d}\u{28e}\x07\x13\x02\x02\u{28e}\x61\x03\x02\x02\x02\u{28f}\u{29f}\
	\x05\x66\x34\x02\u{290}\u{29f}\x05\x68\x35\x02\u{291}\u{29f}\x05\x6c\x37\
	\x02\u{292}\u{29f}\x05\x72\x3a\x02\u{293}\u{29f}\x05\x60\x31\x02\u{294}\
	\u{29f}\x05\x74\x3b\x02\u{295}\u{29f}\x05\x78\x3d\x02\u{296}\u{29f}\x05\
	\x7a\x3e\x02\u{297}\u{29f}\x05\x7c\x3f\x02\u{298}\u{29f}\x05\x7e\x40\x02\
	\u{299}\u{29f}\x05\u{80}\x41\x02\u{29a}\u{29f}\x05\u{82}\x42\x02\u{29b}\
	\u{29f}\x05\x6e\x38\x02\u{29c}\u{29f}\x05\x70\x39\x02\u{29d}\u{29f}\x05\
	\u{84}\x43\x02\u{29e}\u{28f}\x03\x02\x02\x02\u{29e}\u{290}\x03\x02\x02\x02\
	\u{29e}\u{291}\x03\x02\x02\x02\u{29e}\u{292}\x03\x02\x02\x02\u{29e}\u{293}\
	\x03\x02\x02\x02\u{29e}\u{294}\x03\x02\x02\x02\u{29e}\u{295}\x03\x02\x02\
	\x02\u{29e}\u{296}\x03\x02\x02\x02\u{29e}\u{297}\x03\x02\x02\x02\u{29e}\
	\u{298}\x03\x02\x02\x02\u{29e}\u{299}\x03\x02\x02\x02\u{29e}\u{29a}\x03\
	\x02\x02\x02\u{29e}\u{29b}\x03\x02\x02\x02\u{29e}\u{29c}\x03\x02\x02\x02\
	\u{29e}\u{29d}\x03\x02\x02\x02\u{29f}\x63\x03\x02\x02\x02\u{2a0}\u{2a1}\
	\x05\u{8e}\x48\x02\u{2a1}\u{2a2}\x07\x04\x02\x02\u{2a2}\x65\x03\x02\x02\
	\x02\u{2a3}\u{2a4}\x07\x35\x02\x02\u{2a4}\u{2a5}\x07\x19\x02\x02\u{2a5}\
	\u{2a6}\x05\u{8e}\x48\x02\u{2a6}\u{2a7}\x07\x1a\x02\x02\u{2a7}\u{2aa}\x05\
	\x62\x32\x02\u{2a8}\u{2a9}\x07\x36\x02\x02\u{2a9}\u{2ab}\x05\x62\x32\x02\
	\u{2aa}\u{2a8}\x03\x02\x02\x02\u{2aa}\u{2ab}\x03\x02\x02\x02\u{2ab}\x67\
	\x03\x02\x02\x02\u{2ac}\u{2ad}\x07\x37\x02\x02\u{2ad}\u{2af}\x05\u{8e}\x48\
	\x02\u{2ae}\u{2b0}\x05\x36\x1c\x02\u{2af}\u{2ae}\x03\x02\x02\x02\u{2af}\
	\u{2b0}\x03\x02\x02\x02\u{2b0}\u{2b1}\x03\x02\x02\x02\u{2b1}\u{2b3}\x05\
	\x60\x31\x02\u{2b2}\u{2b4}\x05\x6a\x36\x02\u{2b3}\u{2b2}\x03\x02\x02\x02\
	\u{2b4}\u{2b5}\x03\x02\x02\x02\u{2b5}\u{2b3}\x03\x02\x02\x02\u{2b5}\u{2b6}\
	\x03\x02\x02\x02\u{2b6}\x69\x03\x02\x02\x02\u{2b7}\u{2bc}\x07\x38\x02\x02\
	\u{2b8}\u{2ba}\x05\u{c4}\x63\x02\u{2b9}\u{2b8}\x03\x02\x02\x02\u{2b9}\u{2ba}\
	\x03\x02\x02\x02\u{2ba}\u{2bb}\x03\x02\x02\x02\u{2bb}\u{2bd}\x05\x40\x21\
	\x02\u{2bc}\u{2b9}\x03\x02\x02\x02\u{2bc}\u{2bd}\x03\x02\x02\x02\u{2bd}\
	\u{2be}\x03\x02\x02\x02\u{2be}\u{2bf}\x05\x60\x31\x02\u{2bf}\x6b\x03\x02\
	\x02\x02\u{2c0}\u{2c1}\x07\x39\x02\x02\u{2c1}\u{2c2}\x07\x19\x02\x02\u{2c2}\
	\u{2c3}\x05\u{8e}\x48\x02\u{2c3}\u{2c4}\x07\x1a\x02\x02\u{2c4}\u{2c5}\x05\
	\x62\x32\x02\u{2c5}\x6d\x03\x02\x02\x02\u{2c6}\u{2c9}\x05\u{86}\x44\x02\
	\u{2c7}\u{2c9}\x05\x64\x33\x02\u{2c8}\u{2c6}\x03\x02\x02\x02\u{2c8}\u{2c7}\
	\x03\x02\x02\x02\u{2c9}\x6f\x03\x02\x02\x02\u{2ca}\u{2cb}\x07\x3a\x02\x02\
	\u{2cb}\u{2cc}\x05\x60\x31\x02\u{2cc}\x71\x03\x02\x02\x02\u{2cd}\u{2ce}\
	\x07\x1d\x02\x02\u{2ce}\u{2d1}\x07\x19\x02\x02\u{2cf}\u{2d2}\x05\x6e\x38\
	\x02\u{2d0}\u{2d2}\x07\x04\x02\x02\u{2d1}\u{2cf}\x03\x02\x02\x02\u{2d1}\
	\u{2d0}\x03\x02\x02\x02\u{2d2}\u{2d5}\x03\x02\x02\x02\u{2d3}\u{2d6}\x05\
	\x64\x33\x02\u{2d4}\u{2d6}\x07\x04\x02\x02\u{2d5}\u{2d3}\x03\x02\x02\x02\
	\u{2d5}\u{2d4}\x03\x02\x02\x02\u{2d6}\u{2d8}\x03\x02\x02\x02\u{2d7}\u{2d9}\
	\x05\u{8e}\x48\x02\u{2d8}\u{2d7}\x03\x02\x02\x02\u{2d8}\u{2d9}\x03\x02\x02\
	\x02\u{2d9}\u{2da}\x03\x02\x02\x02\u{2da}\u{2db}\x07\x1a\x02\x02\u{2db}\
	\u{2dc}\x05\x62\x32\x02\u{2dc}\x73\x03\x02\x02\x02\u{2dd}\u{2df}\x07\x3b\
	\x02\x02\u{2de}\u{2e0}\x07\u{83}\x02\x02\u{2df}\u{2de}\x03\x02\x02\x02\u{2df}\
	\u{2e0}\x03\x02\x02\x02\u{2e0}\u{2e5}\x03\x02\x02\x02\u{2e1}\u{2e2}\x07\
	\x19\x02\x02\u{2e2}\u{2e3}\x05\x76\x3c\x02\u{2e3}\u{2e4}\x07\x1a\x02\x02\
	\u{2e4}\u{2e6}\x03\x02\x02\x02\u{2e5}\u{2e1}\x03\x02\x02\x02\u{2e5}\u{2e6}\
	\x03\x02\x02\x02\u{2e6}\u{2e7}\x03\x02\x02\x02\u{2e7}\u{2e8}\x05\u{9c}\x4f\
	\x02\u{2e8}\x75\x03\x02\x02\x02\u{2e9}\u{2ea}\x05\u{ca}\x66\x02\u{2ea}\x77\
	\x03\x02\x02\x02\u{2eb}\u{2ec}\x07\x3c\x02\x02\u{2ec}\u{2ed}\x05\x62\x32\
	\x02\u{2ed}\u{2ee}\x07\x39\x02\x02\u{2ee}\u{2ef}\x07\x19\x02\x02\u{2ef}\
	\u{2f0}\x05\u{8e}\x48\x02\u{2f0}\u{2f1}\x07\x1a\x02\x02\u{2f1}\u{2f2}\x07\
	\x04\x02\x02\u{2f2}\x79\x03\x02\x02\x02\u{2f3}\u{2f4}\x07\x72\x02\x02\u{2f4}\
	\u{2f5}\x07\x04\x02\x02\u{2f5}\x7b\x03\x02\x02\x02\u{2f6}\u{2f7}\x07\x6f\
	\x02\x02\u{2f7}\u{2f8}\x07\x04\x02\x02\u{2f8}\x7d\x03\x02\x02\x02\u{2f9}\
	\u{2fb}\x07\x3d\x02\x02\u{2fa}\u{2fc}\x05\u{8e}\x48\x02\u{2fb}\u{2fa}\x03\
	\x02\x02\x02\u{2fb}\u{2fc}\x03\x02\x02\x02\u{2fc}\u{2fd}\x03\x02\x02\x02\
	\u{2fd}\u{2fe}\x07\x04\x02\x02\u{2fe}\x7f\x03\x02\x02\x02\u{2ff}\u{300}\
	\x07\x3e\x02\x02\u{300}\u{301}\x07\x04\x02\x02\u{301}\u{81}\x03\x02\x02\
	\x02\u{302}\u{303}\x07\x3f\x02\x02\u{303}\u{304}\x05\u{9a}\x4e\x02\u{304}\
	\u{305}\x07\x04\x02\x02\u{305}\u{83}\x03\x02\x02\x02\u{306}\u{307}\x07\x40\
	\x02\x02\u{307}\u{308}\x05\u{9a}\x4e\x02\u{308}\u{309}\x07\x04\x02\x02\u{309}\
	\u{85}\x03\x02\x02\x02\u{30a}\u{30b}\x07\x41\x02\x02\u{30b}\u{312}\x05\u{8a}\
	\x46\x02\u{30c}\u{312}\x05\x4c\x27\x02\u{30d}\u{30e}\x07\x19\x02\x02\u{30e}\
	\u{30f}\x05\u{88}\x45\x02\u{30f}\u{310}\x07\x1a\x02\x02\u{310}\u{312}\x03\
	\x02\x02\x02\u{311}\u{30a}\x03\x02\x02\x02\u{311}\u{30c}\x03\x02\x02\x02\
	\u{311}\u{30d}\x03\x02\x02\x02\u{312}\u{315}\x03\x02\x02\x02\u{313}\u{314}\
	\x07\x0d\x02\x02\u{314}\u{316}\x05\u{8e}\x48\x02\u{315}\u{313}\x03\x02\x02\
	\x02\u{315}\u{316}\x03\x02\x02\x02\u{316}\u{317}\x03\x02\x02\x02\u{317}\
	\u{318}\x07\x04\x02\x02\u{318}\u{87}\x03\x02\x02\x02\u{319}\u{31b}\x05\x4c\
	\x27\x02\u{31a}\u{319}\x03\x02\x02\x02\u{31a}\u{31b}\x03\x02\x02\x02\u{31b}\
	\u{322}\x03\x02\x02\x02\u{31c}\u{31e}\x07\x12\x02\x02\u{31d}\u{31f}\x05\
	\x4c\x27\x02\u{31e}\u{31d}\x03\x02\x02\x02\u{31e}\u{31f}\x03\x02\x02\x02\
	\u{31f}\u{321}\x03\x02\x02\x02\u{320}\u{31c}\x03\x02\x02\x02\u{321}\u{324}\
	\x03\x02\x02\x02\u{322}\u{320}\x03\x02\x02\x02\u{322}\u{323}\x03\x02\x02\
	\x02\u{323}\u{89}\x03\x02\x02\x02\u{324}\u{322}\x03\x02\x02\x02\u{325}\u{32c}\
	\x07\x19\x02\x02\u{326}\u{328}\x05\u{c4}\x63\x02\u{327}\u{326}\x03\x02\x02\
	\x02\u{327}\u{328}\x03\x02\x02\x02\u{328}\u{329}\x03\x02\x02\x02\u{329}\
	\u{32b}\x07\x12\x02\x02\u{32a}\u{327}\x03\x02\x02\x02\u{32b}\u{32e}\x03\
	\x02\x02\x02\u{32c}\u{32a}\x03\x02\x02\x02\u{32c}\u{32d}\x03\x02\x02\x02\
	\u{32d}\u{330}\x03\x02\x02\x02\u{32e}\u{32c}\x03\x02\x02\x02\u{32f}\u{331}\
	\x05\u{c4}\x63\x02\u{330}\u{32f}\x03\x02\x02\x02\u{330}\u{331}\x03\x02\x02\
	\x02\u{331}\u{332}\x03\x02\x02\x02\u{332}\u{333}\x07\x1a\x02\x02\u{333}\
	\u{8b}\x03\x02\x02\x02\u{334}\u{335}\x09\x07\x02\x02\u{335}\u{8d}\x03\x02\
	\x02\x02\u{336}\u{337}\x08\x48\x01\x02\u{337}\u{338}\x07\x47\x02\x02\u{338}\
	\u{349}\x05\x4e\x28\x02\u{339}\u{33a}\x07\x19\x02\x02\u{33a}\u{33b}\x05\
	\u{8e}\x48\x02\u{33b}\u{33c}\x07\x1a\x02\x02\u{33c}\u{349}\x03\x02\x02\x02\
	\u{33d}\u{33e}\x09\x08\x02\x02\u{33e}\u{349}\x05\u{8e}\x48\x15\u{33f}\u{340}\
	\x09\x09\x02\x02\u{340}\u{349}\x05\u{8e}\x48\x14\u{341}\u{342}\x07\x49\x02\
	\x02\u{342}\u{349}\x05\u{8e}\x48\x13\u{343}\u{344}\x07\x4a\x02\x02\u{344}\
	\u{349}\x05\u{8e}\x48\x12\u{345}\u{346}\x07\x08\x02\x02\u{346}\u{349}\x05\
	\u{8e}\x48\x11\u{347}\u{349}\x05\u{90}\x49\x02\u{348}\u{336}\x03\x02\x02\
	\x02\u{348}\u{339}\x03\x02\x02\x02\u{348}\u{33d}\x03\x02\x02\x02\u{348}\
	\u{33f}\x03\x02\x02\x02\u{348}\u{341}\x03\x02\x02\x02\u{348}\u{343}\x03\
	\x02\x02\x02\u{348}\u{345}\x03\x02\x02\x02\u{348}\u{347}\x03\x02\x02\x02\
	\u{349}\u{394}\x03\x02\x02\x02\u{34a}\u{34b}\x0c\x10\x02\x02\u{34b}\u{34c}\
	\x07\x4b\x02\x02\u{34c}\u{393}\x05\u{8e}\x48\x10\u{34d}\u{34e}\x0c\x0f\x02\
	\x02\u{34e}\u{34f}\x09\x0a\x02\x02\u{34f}\u{393}\x05\u{8e}\x48\x10\u{350}\
	\u{351}\x0c\x0e\x02\x02\u{351}\u{352}\x09\x09\x02\x02\u{352}\u{393}\x05\
	\u{8e}\x48\x0f\u{353}\u{354}\x0c\x0d\x02\x02\u{354}\u{355}\x09\x0b\x02\x02\
	\u{355}\u{393}\x05\u{8e}\x48\x0e\u{356}\u{357}\x0c\x0c\x02\x02\u{357}\u{358}\
	\x07\x1f\x02\x02\u{358}\u{393}\x05\u{8e}\x48\x0d\u{359}\u{35a}\x0c\x0b\x02\
	\x02\u{35a}\u{35b}\x07\x07\x02\x02\u{35b}\u{393}\x05\u{8e}\x48\x0c\u{35c}\
	\u{35d}\x0c\x0a\x02\x02\u{35d}\u{35e}\x07\x1e\x02\x02\u{35e}\u{393}\x05\
	\u{8e}\x48\x0b\u{35f}\u{360}\x0c\x09\x02\x02\u{360}\u{361}\x09\x0c\x02\x02\
	\u{361}\u{393}\x05\u{8e}\x48\x0a\u{362}\u{363}\x0c\x08\x02\x02\u{363}\u{364}\
	\x09\x0d\x02\x02\u{364}\u{393}\x05\u{8e}\x48\x09\u{365}\u{366}\x0c\x07\x02\
	\x02\u{366}\u{367}\x07\x4e\x02\x02\u{367}\u{393}\x05\u{8e}\x48\x08\u{368}\
	\u{369}\x0c\x06\x02\x02\u{369}\u{36a}\x07\x06\x02\x02\u{36a}\u{393}\x05\
	\u{8e}\x48\x07\u{36b}\u{36c}\x0c\x05\x02\x02\u{36c}\u{36d}\x07\x4f\x02\x02\
	\u{36d}\u{36e}\x05\u{8e}\x48\x02\u{36e}\u{36f}\x07\x48\x02\x02\u{36f}\u{370}\
	\x05\u{8e}\x48\x05\u{370}\u{393}\x03\x02\x02\x02\u{371}\u{372}\x0c\x04\x02\
	\x02\u{372}\u{373}\x09\x0e\x02\x02\u{373}\u{393}\x05\u{8e}\x48\x05\u{374}\
	\u{375}\x0c\x1d\x02\x02\u{375}\u{393}\x09\x08\x02\x02\u{376}\u{377}\x0c\
	\x1b\x02\x02\u{377}\u{378}\x07\x2c\x02\x02\u{378}\u{379}\x05\u{8e}\x48\x02\
	\u{379}\u{37a}\x07\x2d\x02\x02\u{37a}\u{393}\x03\x02\x02\x02\u{37b}\u{37c}\
	\x0c\x1a\x02\x02\u{37c}\u{37e}\x07\x2c\x02\x02\u{37d}\u{37f}\x05\u{8e}\x48\
	\x02\u{37e}\u{37d}\x03\x02\x02\x02\u{37e}\u{37f}\x03\x02\x02\x02\u{37f}\
	\u{380}\x03\x02\x02\x02\u{380}\u{382}\x07\x48\x02\x02\u{381}\u{383}\x05\
	\u{8e}\x48\x02\u{382}\u{381}\x03\x02\x02\x02\u{382}\u{383}\x03\x02\x02\x02\
	\u{383}\u{384}\x03\x02\x02\x02\u{384}\u{393}\x07\x2d\x02\x02\u{385}\u{386}\
	\x0c\x19\x02\x02\u{386}\u{387}\x07\x2f\x02\x02\u{387}\u{393}\x05\u{c4}\x63\
	\x02\u{388}\u{389}\x0c\x18\x02\x02\u{389}\u{38a}\x07\x11\x02\x02\u{38a}\
	\u{38b}\x05\u{94}\x4b\x02\u{38b}\u{38c}\x07\x13\x02\x02\u{38c}\u{393}\x03\
	\x02\x02\x02\u{38d}\u{38e}\x0c\x17\x02\x02\u{38e}\u{38f}\x07\x19\x02\x02\
	\u{38f}\u{390}\x05\u{98}\x4d\x02\u{390}\u{391}\x07\x1a\x02\x02\u{391}\u{393}\
	\x03\x02\x02\x02\u{392}\u{34a}\x03\x02\x02\x02\u{392}\u{34d}\x03\x02\x02\
	\x02\u{392}\u{350}\x03\x02\x02\x02\u{392}\u{353}\x03\x02\x02\x02\u{392}\
	\u{356}\x03\x02\x02\x02\u{392}\u{359}\x03\x02\x02\x02\u{392}\u{35c}\x03\
	\x02\x02\x02\u{392}\u{35f}\x03\x02\x02\x02\u{392}\u{362}\x03\x02\x02\x02\
	\u{392}\u{365}\x03\x02\x02\x02\u{392}\u{368}\x03\x02\x02\x02\u{392}\u{36b}\
	\x03\x02\x02\x02\u{392}\u{371}\x03\x02\x02\x02\u{392}\u{374}\x03\x02\x02\
	\x02\u{392}\u{376}\x03\x02\x02\x02\u{392}\u{37b}\x03\x02\x02\x02\u{392}\
	\u{385}\x03\x02\x02\x02\u{392}\u{388}\x03\x02\x02\x02\u{392}\u{38d}\x03\
	\x02\x02\x02\u{393}\u{396}\x03\x02\x02\x02\u{394}\u{392}\x03\x02\x02\x02\
	\u{394}\u{395}\x03\x02\x02\x02\u{395}\u{8f}\x03\x02\x02\x02\u{396}\u{394}\
	\x03\x02\x02\x02\u{397}\u{3a1}\x07\x68\x02\x02\u{398}\u{3a1}\x05\u{c2}\x62\
	\x02\u{399}\u{3a1}\x05\u{c6}\x64\x02\u{39a}\u{3a1}\x05\u{ca}\x66\x02\u{39b}\
	\u{3a1}\x05\u{c4}\x63\x02\u{39c}\u{3a1}\x07\x7c\x02\x02\u{39d}\u{3a1}\x07\
	\x77\x02\x02\u{39e}\u{3a1}\x05\u{c0}\x61\x02\u{39f}\u{3a1}\x05\x4e\x28\x02\
	\u{3a0}\u{397}\x03\x02\x02\x02\u{3a0}\u{398}\x03\x02\x02\x02\u{3a0}\u{399}\
	\x03\x02\x02\x02\u{3a0}\u{39a}\x03\x02\x02\x02\u{3a0}\u{39b}\x03\x02\x02\
	\x02\u{3a0}\u{39c}\x03\x02\x02\x02\u{3a0}\u{39d}\x03\x02\x02\x02\u{3a0}\
	\u{39e}\x03\x02\x02\x02\u{3a0}\u{39f}\x03\x02\x02\x02\u{3a1}\u{91}\x03\x02\
	\x02\x02\u{3a2}\u{3a7}\x05\u{8e}\x48\x02\u{3a3}\u{3a4}\x07\x12\x02\x02\u{3a4}\
	\u{3a6}\x05\u{8e}\x48\x02\u{3a5}\u{3a3}\x03\x02\x02\x02\u{3a6}\u{3a9}\x03\
	\x02\x02\x02\u{3a7}\u{3a5}\x03\x02\x02\x02\u{3a7}\u{3a8}\x03\x02\x02\x02\
	\u{3a8}\u{93}\x03\x02\x02\x02\u{3a9}\u{3a7}\x03\x02\x02\x02\u{3aa}\u{3af}\
	\x05\u{96}\x4c\x02\u{3ab}\u{3ac}\x07\x12\x02\x02\u{3ac}\u{3ae}\x05\u{96}\
	\x4c\x02\u{3ad}\u{3ab}\x03\x02\x02\x02\u{3ae}\u{3b1}\x03\x02\x02\x02\u{3af}\
	\u{3ad}\x03\x02\x02\x02\u{3af}\u{3b0}\x03\x02\x02\x02\u{3b0}\u{3b3}\x03\
	\x02\x02\x02\u{3b1}\u{3af}\x03\x02\x02\x02\u{3b2}\u{3b4}\x07\x12\x02\x02\
	\u{3b3}\u{3b2}\x03\x02\x02\x02\u{3b3}\u{3b4}\x03\x02\x02\x02\u{3b4}\u{95}\
	\x03\x02\x02\x02\u{3b5}\u{3b6}\x05\u{c4}\x63\x02\u{3b6}\u{3b7}\x07\x48\x02\
	\x02\u{3b7}\u{3b8}\x05\u{8e}\x48\x02\u{3b8}\u{97}\x03\x02\x02\x02\u{3b9}\
	\u{3bb}\x07\x11\x02\x02\u{3ba}\u{3bc}\x05\u{94}\x4b\x02\u{3bb}\u{3ba}\x03\
	\x02\x02\x02\u{3bb}\u{3bc}\x03\x02\x02\x02\u{3bc}\u{3bd}\x03\x02\x02\x02\
	\u{3bd}\u{3c2}\x07\x13\x02\x02\u{3be}\u{3c0}\x05\u{92}\x4a\x02\u{3bf}\u{3be}\
	\x03\x02\x02\x02\u{3bf}\u{3c0}\x03\x02\x02\x02\u{3c0}\u{3c2}\x03\x02\x02\
	\x02\u{3c1}\u{3b9}\x03\x02\x02\x02\u{3c1}\u{3bf}\x03\x02\x02\x02\u{3c2}\
	\u{99}\x03\x02\x02\x02\u{3c3}\u{3c4}\x05\u{8e}\x48\x02\u{3c4}\u{3c5}\x07\
	\x19\x02\x02\u{3c5}\u{3c6}\x05\u{98}\x4d\x02\u{3c6}\u{3c7}\x07\x1a\x02\x02\
	\u{3c7}\u{9b}\x03\x02\x02\x02\u{3c8}\u{3cc}\x07\x11\x02\x02\u{3c9}\u{3cb}\
	\x05\u{9e}\x50\x02\u{3ca}\u{3c9}\x03\x02\x02\x02\u{3cb}\u{3ce}\x03\x02\x02\
	\x02\u{3cc}\u{3ca}\x03\x02\x02\x02\u{3cc}\u{3cd}\x03\x02\x02\x02\u{3cd}\
	\u{3cf}\x03\x02\x02\x02\u{3ce}\u{3cc}\x03\x02\x02\x02\u{3cf}\u{3d0}\x07\
	\x13\x02\x02\u{3d0}\u{9d}\x03\x02\x02\x02\u{3d1}\u{3e3}\x05\u{c4}\x63\x02\
	\u{3d2}\u{3e3}\x05\u{9c}\x4f\x02\u{3d3}\u{3e3}\x05\u{a0}\x51\x02\u{3d4}\
	\u{3e3}\x05\u{a6}\x54\x02\u{3d5}\u{3e3}\x05\u{a8}\x55\x02\u{3d6}\u{3e3}\
	\x05\u{ae}\x58\x02\u{3d7}\u{3e3}\x05\u{b0}\x59\x02\u{3d8}\u{3e3}\x05\u{b2}\
	\x5a\x02\u{3d9}\u{3e3}\x05\u{b6}\x5c\x02\u{3da}\u{3e3}\x05\u{ba}\x5e\x02\
	\u{3db}\u{3e3}\x05\u{bc}\x5f\x02\u{3dc}\u{3e3}\x07\x6f\x02\x02\u{3dd}\u{3e3}\
	\x07\x72\x02\x02\u{3de}\u{3e3}\x07\x73\x02\x02\u{3df}\u{3e3}\x05\u{c2}\x62\
	\x02\u{3e0}\u{3e3}\x05\u{ca}\x66\x02\u{3e1}\u{3e3}\x05\u{c6}\x64\x02\u{3e2}\
	\u{3d1}\x03\x02\x02\x02\u{3e2}\u{3d2}\x03\x02\x02\x02\u{3e2}\u{3d3}\x03\
	\x02\x02\x02\u{3e2}\u{3d4}\x03\x02\x02\x02\u{3e2}\u{3d5}\x03\x02\x02\x02\
	\u{3e2}\u{3d6}\x03\x02\x02\x02\u{3e2}\u{3d7}\x03\x02\x02\x02\u{3e2}\u{3d8}\
	\x03\x02\x02\x02\u{3e2}\u{3d9}\x03\x02\x02\x02\u{3e2}\u{3da}\x03\x02\x02\
	\x02\u{3e2}\u{3db}\x03\x02\x02\x02\u{3e2}\u{3dc}\x03\x02\x02\x02\u{3e2}\
	\u{3dd}\x03\x02\x02\x02\u{3e2}\u{3de}\x03\x02\x02\x02\u{3e2}\u{3df}\x03\
	\x02\x02\x02\u{3e2}\u{3e0}\x03\x02\x02\x02\u{3e2}\u{3e1}\x03\x02\x02\x02\
	\u{3e3}\u{9f}\x03\x02\x02\x02\u{3e4}\u{3e8}\x05\u{a4}\x53\x02\u{3e5}\u{3e8}\
	\x05\u{be}\x60\x02\u{3e6}\u{3e8}\x05\u{a2}\x52\x02\u{3e7}\u{3e4}\x03\x02\
	\x02\x02\u{3e7}\u{3e5}\x03\x02\x02\x02\u{3e7}\u{3e6}\x03\x02\x02\x02\u{3e8}\
	\u{a1}\x03\x02\x02\x02\u{3e9}\u{3ea}\x05\u{c4}\x63\x02\u{3ea}\u{3eb}\x07\
	\x2f\x02\x02\u{3eb}\u{3ec}\x05\u{c4}\x63\x02\u{3ec}\u{a3}\x03\x02\x02\x02\
	\u{3ed}\u{3f2}\x07\x3d\x02\x02\u{3ee}\u{3f2}\x07\x2e\x02\x02\u{3ef}\u{3f2}\
	\x07\x44\x02\x02\u{3f0}\u{3f2}\x05\u{c4}\x63\x02\u{3f1}\u{3ed}\x03\x02\x02\
	\x02\u{3f1}\u{3ee}\x03\x02\x02\x02\u{3f1}\u{3ef}\x03\x02\x02\x02\u{3f1}\
	\u{3f0}\x03\x02\x02\x02\u{3f2}\u{3ff}\x03\x02\x02\x02\u{3f3}\u{3f5}\x07\
	\x19\x02\x02\u{3f4}\u{3f6}\x05\u{a0}\x51\x02\u{3f5}\u{3f4}\x03\x02\x02\x02\
	\u{3f5}\u{3f6}\x03\x02\x02\x02\u{3f6}\u{3fb}\x03\x02\x02\x02\u{3f7}\u{3f8}\
	\x07\x12\x02\x02\u{3f8}\u{3fa}\x05\u{a0}\x51\x02\u{3f9}\u{3f7}\x03\x02\x02\
	\x02\u{3fa}\u{3fd}\x03\x02\x02\x02\u{3fb}\u{3f9}\x03\x02\x02\x02\u{3fb}\
	\u{3fc}\x03\x02\x02\x02\u{3fc}\u{3fe}\x03\x02\x02\x02\u{3fd}\u{3fb}\x03\
	\x02\x02\x02\u{3fe}\u{400}\x07\x1a\x02\x02\u{3ff}\u{3f3}\x03\x02\x02\x02\
	\u{3ff}\u{400}\x03\x02\x02\x02\u{400}\u{a5}\x03\x02\x02\x02\u{401}\u{402}\
	\x07\x5a\x02\x02\u{402}\u{405}\x05\u{aa}\x56\x02\u{403}\u{404}\x07\x5b\x02\
	\x02\u{404}\u{406}\x05\u{a0}\x51\x02\u{405}\u{403}\x03\x02\x02\x02\u{405}\
	\u{406}\x03\x02\x02\x02\u{406}\u{a7}\x03\x02\x02\x02\u{407}\u{408}\x05\u{aa}\
	\x56\x02\u{408}\u{409}\x07\x5b\x02\x02\u{409}\u{40a}\x05\u{a0}\x51\x02\u{40a}\
	\u{a9}\x03\x02\x02\x02\u{40b}\u{413}\x05\u{c4}\x63\x02\u{40c}\u{413}\x05\
	\u{a2}\x52\x02\u{40d}\u{413}\x05\u{ac}\x57\x02\u{40e}\u{40f}\x07\x19\x02\
	\x02\u{40f}\u{410}\x05\u{ac}\x57\x02\u{410}\u{411}\x07\x1a\x02\x02\u{411}\
	\u{413}\x03\x02\x02\x02\u{412}\u{40b}\x03\x02\x02\x02\u{412}\u{40c}\x03\
	\x02\x02\x02\u{412}\u{40d}\x03\x02\x02\x02\u{412}\u{40e}\x03\x02\x02\x02\
	\u{413}\u{ab}\x03\x02\x02\x02\u{414}\u{419}\x05\u{c4}\x63\x02\u{415}\u{416}\
	\x07\x12\x02\x02\u{416}\u{418}\x05\u{c4}\x63\x02\u{417}\u{415}\x03\x02\x02\
	\x02\u{418}\u{41b}\x03\x02\x02\x02\u{419}\u{417}\x03\x02\x02\x02\u{419}\
	\u{41a}\x03\x02\x02\x02\u{41a}\u{ad}\x03\x02\x02\x02\u{41b}\u{419}\x03\x02\
	\x02\x02\u{41c}\u{41d}\x05\u{a0}\x51\x02\u{41d}\u{41e}\x07\x5c\x02\x02\u{41e}\
	\u{41f}\x05\u{c4}\x63\x02\u{41f}\u{af}\x03\x02\x02\x02\u{420}\u{421}\x05\
	\u{c4}\x63\x02\u{421}\u{422}\x07\x48\x02\x02\u{422}\u{b1}\x03\x02\x02\x02\
	\u{423}\u{424}\x07\x5d\x02\x02\u{424}\u{428}\x05\u{a0}\x51\x02\u{425}\u{427}\
	\x05\u{b4}\x5b\x02\u{426}\u{425}\x03\x02\x02\x02\u{427}\u{42a}\x03\x02\x02\
	\x02\u{428}\u{426}\x03\x02\x02\x02\u{428}\u{429}\x03\x02\x02\x02\u{429}\
	\u{b3}\x03\x02\x02\x02\u{42a}\u{428}\x03\x02\x02\x02\u{42b}\u{42c}\x07\x5e\
	\x02\x02\u{42c}\u{42d}\x05\u{be}\x60\x02\u{42d}\u{42e}\x05\u{9c}\x4f\x02\
	\u{42e}\u{432}\x03\x02\x02\x02\u{42f}\u{430}\x07\x5f\x02\x02\u{430}\u{432}\
	\x05\u{9c}\x4f\x02\u{431}\u{42b}\x03\x02\x02\x02\u{431}\u{42f}\x03\x02\x02\
	\x02\u{432}\u{b5}\x03\x02\x02\x02\u{433}\u{434}\x07\x28\x02\x02\u{434}\u{435}\
	\x05\u{c4}\x63\x02\u{435}\u{437}\x07\x19\x02\x02\u{436}\u{438}\x05\u{ac}\
	\x57\x02\u{437}\u{436}\x03\x02\x02\x02\u{437}\u{438}\x03\x02\x02\x02\u{438}\
	\u{439}\x03\x02\x02\x02\u{439}\u{43b}\x07\x1a\x02\x02\u{43a}\u{43c}\x05\
	\u{b8}\x5d\x02\u{43b}\u{43a}\x03\x02\x02\x02\u{43b}\u{43c}\x03\x02\x02\x02\
	\u{43c}\u{43d}\x03\x02\x02\x02\u{43d}\u{43e}\x05\u{9c}\x4f\x02\u{43e}\u{b7}\
	\x03\x02\x02\x02\u{43f}\u{440}\x07\x60\x02\x02\u{440}\u{441}\x05\u{ac}\x57\
	\x02\u{441}\u{b9}\x03\x02\x02\x02\u{442}\u{445}\x07\x1d\x02\x02\u{443}\u{446}\
	\x05\u{9c}\x4f\x02\u{444}\u{446}\x05\u{a0}\x51\x02\u{445}\u{443}\x03\x02\
	\x02\x02\u{445}\u{444}\x03\x02\x02\x02\u{446}\u{447}\x03\x02\x02\x02\u{447}\
	\u{44a}\x05\u{a0}\x51\x02\u{448}\u{44b}\x05\u{9c}\x4f\x02\u{449}\u{44b}\
	\x05\u{a0}\x51\x02\u{44a}\u{448}\x03\x02\x02\x02\u{44a}\u{449}\x03\x02\x02\
	\x02\u{44b}\u{44c}\x03\x02\x02\x02\u{44c}\u{44d}\x05\u{9c}\x4f\x02\u{44d}\
	\u{bb}\x03\x02\x02\x02\u{44e}\u{44f}\x07\x35\x02\x02\u{44f}\u{450}\x05\u{a0}\
	\x51\x02\u{450}\u{451}\x05\u{9c}\x4f\x02\u{451}\u{bd}\x03\x02\x02\x02\u{452}\
	\u{458}\x05\u{ca}\x66\x02\u{453}\u{458}\x07\x69\x02\x02\u{454}\u{458}\x07\
	\x6a\x02\x02\u{455}\u{458}\x05\u{c6}\x64\x02\u{456}\u{458}\x07\x68\x02\x02\
	\u{457}\u{452}\x03\x02\x02\x02\u{457}\u{453}\x03\x02\x02\x02\u{457}\u{454}\
	\x03\x02\x02\x02\u{457}\u{455}\x03\x02\x02\x02\u{457}\u{456}\x03\x02\x02\
	\x02\u{458}\u{bf}\x03\x02\x02\x02\u{459}\u{45b}\x07\x19\x02\x02\u{45a}\u{45c}\
	\x05\u{8e}\x48\x02\u{45b}\u{45a}\x03\x02\x02\x02\u{45b}\u{45c}\x03\x02\x02\
	\x02\u{45c}\u{463}\x03\x02\x02\x02\u{45d}\u{45f}\x07\x12\x02\x02\u{45e}\
	\u{460}\x05\u{8e}\x48\x02\u{45f}\u{45e}\x03\x02\x02\x02\u{45f}\u{460}\x03\
	\x02\x02\x02\u{460}\u{462}\x03\x02\x02\x02\u{461}\u{45d}\x03\x02\x02\x02\
	\u{462}\u{465}\x03\x02\x02\x02\u{463}\u{461}\x03\x02\x02\x02\u{463}\u{464}\
	\x03\x02\x02\x02\u{464}\u{466}\x03\x02\x02\x02\u{465}\u{463}\x03\x02\x02\
	\x02\u{466}\u{474}\x07\x1a\x02\x02\u{467}\u{470}\x07\x2c\x02\x02\u{468}\
	\u{46d}\x05\u{8e}\x48\x02\u{469}\u{46a}\x07\x12\x02\x02\u{46a}\u{46c}\x05\
	\u{8e}\x48\x02\u{46b}\u{469}\x03\x02\x02\x02\u{46c}\u{46f}\x03\x02\x02\x02\
	\u{46d}\u{46b}\x03\x02\x02\x02\u{46d}\u{46e}\x03\x02\x02\x02\u{46e}\u{471}\
	\x03\x02\x02\x02\u{46f}\u{46d}\x03\x02\x02\x02\u{470}\u{468}\x03\x02\x02\
	\x02\u{470}\u{471}\x03\x02\x02\x02\u{471}\u{472}\x03\x02\x02\x02\u{472}\
	\u{474}\x07\x2d\x02\x02\u{473}\u{459}\x03\x02\x02\x02\u{473}\u{467}\x03\
	\x02\x02\x02\u{474}\u{c1}\x03\x02\x02\x02\u{475}\u{477}\x09\x0f\x02\x02\
	\u{476}\u{478}\x07\x6b\x02\x02\u{477}\u{476}\x03\x02\x02\x02\u{477}\u{478}\
	\x03\x02\x02\x02\u{478}\u{c3}\x03\x02\x02\x02\u{479}\u{47a}\x09\x10\x02\
	\x02\u{47a}\u{c5}\x03\x02\x02\x02\u{47b}\u{47d}\x07\x6c\x02\x02\u{47c}\u{47b}\
	\x03\x02\x02\x02\u{47d}\u{47e}\x03\x02\x02\x02\u{47e}\u{47c}\x03\x02\x02\
	\x02\u{47e}\u{47f}\x03\x02\x02\x02\u{47f}\u{c7}\x03\x02\x02\x02\u{480}\u{48c}\
	\x07\x62\x02\x02\u{481}\u{482}\x07\x19\x02\x02\u{482}\u{487}\x05\x50\x29\
	\x02\u{483}\u{484}\x07\x12\x02\x02\u{484}\u{486}\x05\x50\x29\x02\u{485}\
	\u{483}\x03\x02\x02\x02\u{486}\u{489}\x03\x02\x02\x02\u{487}\u{485}\x03\
	\x02\x02\x02\u{487}\u{488}\x03\x02\x02\x02\u{488}\u{48a}\x03\x02\x02\x02\
	\u{489}\u{487}\x03\x02\x02\x02\u{48a}\u{48b}\x07\x1a\x02\x02\u{48b}\u{48d}\
	\x03\x02\x02\x02\u{48c}\u{481}\x03\x02\x02\x02\u{48c}\u{48d}\x03\x02\x02\
	\x02\u{48d}\u{c9}\x03\x02\x02\x02\u{48e}\u{490}\x07\u{83}\x02\x02\u{48f}\
	\u{48e}\x03\x02\x02\x02\u{490}\u{491}\x03\x02\x02\x02\u{491}\u{48f}\x03\
	\x02\x02\x02\u{491}\u{492}\x03\x02\x02\x02\u{492}\u{cb}\x03\x02\x02\x02\
	\u{84}\u{d7}\u{d9}\u{e8}\u{ec}\u{f1}\u{f7}\u{fb}\u{fe}\u{103}\u{109}\u{110}\
	\u{114}\u{121}\u{129}\u{12e}\u{138}\u{13b}\u{141}\u{149}\u{14c}\u{157}\u{160}\
	\u{162}\u{168}\u{183}\u{186}\u{191}\u{196}\u{19b}\u{1a9}\u{1ac}\u{1b3}\u{1b7}\
	\u{1b9}\u{1be}\u{1c3}\u{1c6}\u{1cc}\u{1d0}\u{1d4}\u{1d9}\u{1e6}\u{1e8}\u{1ef}\
	\u{1f9}\u{1ff}\u{20a}\u{20d}\u{213}\u{216}\u{21e}\u{221}\u{227}\u{22a}\u{232}\
	\u{235}\u{23b}\u{23f}\u{24a}\u{24f}\u{254}\u{25c}\u{261}\u{267}\u{26c}\u{279}\
	\u{27b}\u{280}\u{28a}\u{29e}\u{2aa}\u{2af}\u{2b5}\u{2b9}\u{2bc}\u{2c8}\u{2d1}\
	\u{2d5}\u{2d8}\u{2df}\u{2e5}\u{2fb}\u{311}\u{315}\u{31a}\u{31e}\u{322}\u{327}\
	\u{32c}\u{330}\u{348}\u{37e}\u{382}\u{392}\u{394}\u{3a0}\u{3a7}\u{3af}\u{3b3}\
	\u{3bb}\u{3bf}\u{3c1}\u{3cc}\u{3e2}\u{3e7}\u{3f1}\u{3f5}\u{3fb}\u{3ff}\u{405}\
	\u{412}\u{419}\u{428}\u{431}\u{437}\u{43b}\u{445}\u{44a}\u{457}\u{45b}\u{45f}\
	\u{463}\u{46d}\u{470}\u{473}\u{477}\u{47e}\u{487}\u{48c}\u{491}";

