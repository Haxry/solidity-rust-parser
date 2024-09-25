use antlr_rust::tree::{ParseTreeVisitor, ParseTree};
use antlr_rust::parser_rule_context::ParserRuleContext;
use crate::antlr::gen_output::antlr::solidityparser as SP;
use crate::antlr::gen_output::antlr::solidityvisitor::*;
use crate::ast::ast_types as AST;
use crate::ast::types::ParseOptions;

pub trait WithMeta {}

pub type ASTBuilderNode =  AST::ASTNode ;

pub struct ASTBuilder<'input> {
    pub result: Option<AST::ASTNode>,
    current_contract: Option<String>,
    options: ParseOptions, // Define ParseOptions as per your requirements
}

impl<'input> ASTBuilder<'input> {
    pub fn new(options: ParseOptions) -> Self {
        ASTBuilder {
            result: None,
            current_contract: None,
            options,
        }
    }

    // Example of adding metadata to a node (like WithMeta)
    fn add_meta<T>(&mut self, node: T, ctx: &ParserRuleContext<'input>) -> T
     // Ensure T implements BaseASTNode and is clonable
    {
        let mut node_with_meta = node.clone(); // Create a mutable clone of the node

        // Add metadata based on options
        if self.options.loc {
            node_with_meta.set_loc(self.loc(ctx)); // Assuming you have a set_loc method
        }
        if self.options.range {
            node_with_meta.set_range(self.range(ctx)); // Assuming you have a set_range method
        }

        // Return the modified node with metadata
        node_with_meta
    }


    fn default_result(&self) -> AST::ASTNode {
        panic!("Unknown node");
    }

    fn aggregate_result(&self) -> AST::ASTNode {
        ASTNode {
            node_type: String::from(""), // Default empty type
        }
    }
}

impl<'input> SolidityVisitor<'input> for ASTBuilder<'input> {
    fn visit_sourceUnit(&mut self, ctx: &SP::SourceUnitContext<'input>) -> AST::ASTNode {
        let children = ctx.children();
        
        // Create the SourceUnit node
        let node = AST::SourceUnit {
            node_type: String::from("SourceUnit"),
            children: children
                .iter()
                .map(|child| self.visit(child.as_ref())) // visit each child node
                .collect(),
        };
        
        let result = self.add_meta(node, ctx);
        self.result = Some(result.clone());
        result
    }

    fn visit_contractDefinition(
        &mut self,
        ctx: &SP::ContractDefinitionContext<'input>,
    ) -> AST::ASTNode {
        let name = self.to_text(ctx.identifier());
        let kind = self.to_text(ctx.get_child(0)); // Assuming the first child is the kind

        self.current_contract = Some(name.clone());

        let node = AST::ContractDefinition {
            node_type: String::from("ContractDefinition"),
            name,
            base_contracts: ctx
                .inheritance_specifier_list()
                .iter()
                .map(|x| self.visit_inheritance_specifier(x))
                .collect(),
            sub_nodes: ctx
                .contract_part_list()
                .iter()
                .map(|x| self.visit(x))
                .collect(),
            kind,
        };

        self.add_meta(node, ctx)
    }
    
     fn visit_contract_part(&mut self, ctx: &SP::ContractPartContext<'input>) -> AST::ASTNode {
        // Visit the first child of the ContractPart context
        self.visit(ctx.get_child(0))
    }

    fn visit_stateVariableDeclaration(
        &mut self,
        ctx: &SP::StateVariableDeclarationContext<'input>,
    ) -> AST::StateVariableDeclaration {
        // Visit the type name
        let type_name = self.visit_type_name(ctx.type_name());
    
        // Get the identifier and its text
        let iden = ctx.identifier();
        let name = self.to_text(iden);
    
        // Initialize expression
        let expression: Option<AST::Expression> = ctx.expression()
            .map(|expr| self.visit_expression(expr));
    
        // Determine visibility
        let visibility = if !ctx.internal_keyword_list().is_empty() {
            AST::VariableDeclarationVisibility::Internal
        } else if !ctx.public_keyword_list().is_empty() {
            AST::VariableDeclarationVisibility::Public
        } else if !ctx.private_keyword_list().is_empty() {
            AST::VariableDeclarationVisibility::Private
        } else {
            AST::VariableDeclarationVisibility::Default
        };
    
        // Check if declared constant
        let is_declared_const = !ctx.constant_keyword_list().is_empty();
    
        // Handle override specifiers
        let override_specifier = if ctx.override_specifier_list().is_empty() {
            None
        } else {
            Some(ctx.override_specifier_list()[0]
                .user_defined_type_name_list()
                .iter()
                .map(|x| self.visit_user_defined_type_name(x))
                .collect())
        };
    
        // Check if declared immutable
        let is_immutable = !ctx.immutable_keyword_list().is_empty();
    
        // Create the variable declaration
        let decl = AST::StateVariableDeclarationVariable {
            node_type: String::from("VariableDeclaration"),
            type_name,
            name,
            identifier: self.visit_identifier(iden),
            expression,
            visibility,
            is_state_var: true,
            is_declared_const,
            is_indexed: false,
            is_immutable,
            override_specifier,
            storage_location: None,
        };
    
        // Create the state variable declaration node
        let node = AST::StateVariableDeclaration {
            node_type: String::from("StateVariableDeclaration"),
            variables: vec![self.add_meta(decl, ctx)],
            initial_value: expression,
        };
    
        self.add_meta(node, ctx)
    }



}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::antlr::gen_output::antlr::solidityparser::SolidityParser;
    use crate::antlr::gen_output::antlr::solidityvisitor::SolidityVisitor;
    use antlr_rust::parser_rule_context::ParserRuleContext;

    // Mock context for testing
    struct MockSourceUnitContext {
        children: Vec<Box<dyn ParseTree>>,
    }

    impl ParserRuleContext for MockSourceUnitContext {
        fn children(&self) -> Vec<Box<dyn ParseTree>> {
            self.children.clone()
        }
    }

    #[test]
    pub fn test_visit_source_unit() {
        // Set up the options for ASTBuilder
        let options = ParseOptions {
            loc: true,
            range: false,
            // Add other necessary fields here
        };
        
        let mut builder = ASTBuilder::new(options);

        // Create mock children for the source unit context
        let mock_children: Vec<Box<dyn ParseTree>> = vec![
            // Add mock child parse trees here
        ];

        let ctx = MockSourceUnitContext { children: mock_children };

        // Call the method
        let result = builder.visit_sourceUnit(&ctx);

        // Perform assertions on the result
        assert_eq!(result.node_type, "SourceUnit");
        assert!(result.children.len() == mock_children.len()); // Ensure the number of children matches
        // Add assertions for other fields if available
        // For example, if you have loc or range:
        if options.loc {
            assert!(result.loc.is_some());
        }
        if options.range {
            assert!(result.range.is_some());
        }
    }

    #[test]
   pub fn test_visit_state_variable_declaration() {
        // Similar setup for testing state variable declaration
        let options = ParseOptions {
            loc: true,
            range: true,
        };

        let mut builder = ASTBuilder::new(options);
        
        let mock_children: Vec<Box<dyn ParseTree>> = vec![
            // Add mock child parse trees for StateVariableDeclaration
        ];

        let ctx = MockStateVariableDeclarationContext { children: mock_children };

        let result = builder.visit_stateVariableDeclaration(&ctx);

        assert_eq!(result.node_type, "StateVariableDeclaration");
        
        // Check properties of the state variable declaration
        assert!(result.variables.len() == 1); // Ensure there's one variable
        let variable = &result.variables[0];

        assert_eq!(variable.node_type, "VariableDeclaration");
        assert!(!variable.name.is_empty()); // Check that the name is not empty
        assert_eq!(variable.visibility, AST::VariableDeclarationVisibility::Default); // Adjust as needed based on your mock
        assert!(variable.is_state_var); // Check that it's a state variable
        assert_eq!(variable.is_declared_const, false); // Change as necessary based on mock context
        assert_eq!(variable.is_immutable, false); // Change as necessary based on mock context

        // Additional assertions based on what fields you have in the struct
        // e.g., check if expression is None or has a value
        assert!(variable.expression.is_none()); // Adjust based on mock setup
    }
}
