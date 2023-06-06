use super::*;

#[derive(Default)]
pub struct AstBuilder {
    id: i64,
    scope: i64,
}

impl AstBuilder {
    pub fn next_node_id(&mut self) -> i64 {
        let result = self.id;
        self.id += 1;
        result
    }

    pub fn next_scope(&mut self) -> i64 {
        let result = self.scope;
        self.scope += 1;
        result
    }

    pub fn loc_to_src(&self, loc: &solang_parser::pt::Loc) -> String {
        let solang_parser::pt::Loc::File(_, start, end) = loc else { return "0:0:0".to_string() };
        format!("{}:{}:0", *start, *end - *start)
    }

    pub fn build_source_unit(&mut self, input: &solang_parser::pt::SourceUnit) -> SourceUnit {
        let source_unit_id = self.next_node_id();
        let source_unit_scope = self.next_scope();

        let mut result = SourceUnit {
            license: None,
            nodes: vec![],
            exported_symbols: None,
            absolute_path: None,
            id: source_unit_id,
            source: None,
        };

        for part in input.0.iter() {
            match part {
                solang_parser::pt::SourceUnitPart::PragmaDirective(loc, identifier, literal) => {
                    let Some(identifier) = identifier else { todo!() };
                    let Some(literal) = literal else { todo!() };
                    result.nodes.push(SourceUnitNode::PragmaDirective(self.build_pragma_directive(loc, identifier, literal)));
                }

                solang_parser::pt::SourceUnitPart::ImportDirective(input) => {
                    result.nodes.push(SourceUnitNode::ImportDirective(self.build_import_directive(source_unit_scope, input)));
                }

                solang_parser::pt::SourceUnitPart::ContractDefinition(input) => {
                    result.nodes.push(SourceUnitNode::ContractDefinition(self.build_contract_definition(source_unit_scope, input)));
                }

                solang_parser::pt::SourceUnitPart::EnumDefinition(input) => {
                    result.nodes.push(SourceUnitNode::EnumDefinition(self.build_enum_definition(input)));
                }

                solang_parser::pt::SourceUnitPart::StructDefinition(input) => {
                    result.nodes.push(SourceUnitNode::StructDefinition(self.build_struct_definition(source_unit_scope, input)));
                }

                solang_parser::pt::SourceUnitPart::EventDefinition(input) => {
                    todo!()
                }

                solang_parser::pt::SourceUnitPart::ErrorDefinition(input) => {
                    result.nodes.push(SourceUnitNode::ErrorDefinition(self.build_error_definition(input)));
                }

                solang_parser::pt::SourceUnitPart::FunctionDefinition(input) => {
                    todo!()
                }

                solang_parser::pt::SourceUnitPart::VariableDefinition(input) => {
                    result.nodes.push(SourceUnitNode::VariableDeclaration(self.build_variable_declaration(source_unit_scope, input)));
                }

                solang_parser::pt::SourceUnitPart::TypeDefinition(input) => {
                    result.nodes.push(SourceUnitNode::UserDefinedValueTypeDefinition(self.build_user_defined_value_type_definition(input)));
                }

                solang_parser::pt::SourceUnitPart::Annotation(annotation) => {
                    todo!()
                }

                solang_parser::pt::SourceUnitPart::Using(using) => {
                    todo!()
                }

                solang_parser::pt::SourceUnitPart::StraySemicolon(_) => {}
            }
        }

        result
    }

    pub fn build_pragma_directive(
        &mut self,
        loc: &solang_parser::pt::Loc,
        identifier: &solang_parser::pt::Identifier,
        literal: &solang_parser::pt::StringLiteral,
    ) -> PragmaDirective {
        //
        // TODO: check identifier and split literal into expected parts
        //
        PragmaDirective {
            literals: vec![
                identifier.name.clone(),
                literal.string.clone()
            ],
            src: self.loc_to_src(loc),
            id: self.next_node_id(),
        }
    }

    pub fn build_import_directive(&mut self, scope: i64, input: &solang_parser::pt::Import) -> ImportDirective {
        match input {
            solang_parser::pt::Import::Plain(file, loc) => {
                ImportDirective {
                    file: file.string.clone(),
                    source_unit: -1, // TODO: use imported `source_unit.id`
                    scope,
                    absolute_path: Some(file.string.clone()),
                    unit_alias: String::new(),
                    name_location: Some("-1:-1:-1".to_string()), // TODO: is this important?
                    symbol_aliases: vec![],
                    src: self.loc_to_src(loc),
                    id: self.next_node_id(),
                }
            }

            solang_parser::pt::Import::GlobalSymbol(_, _, _) => todo!(),
            solang_parser::pt::Import::Rename(_, _, _) => todo!(),
        }
    }

    pub fn build_contract_definition(&mut self, scope: i64, input: &solang_parser::pt::ContractDefinition) -> ContractDefinition {
        ContractDefinition {
            name: todo!(),
            name_location: todo!(),
            documentation: todo!(),
            kind: todo!(),
            is_abstract: todo!(),
            base_contracts: todo!(),
            contract_dependencies: todo!(),
            used_events: todo!(),
            used_errors: todo!(),
            nodes: todo!(),
            scope,
            fully_implemented: todo!(),
            linearized_base_contracts: todo!(),
            internal_function_ids: todo!(),
            src: self.loc_to_src(&input.loc),
            id: self.next_node_id(),
        }
    }

    pub fn build_enum_definition(&mut self, input: &solang_parser::pt::EnumDefinition) -> EnumDefinition {
        EnumDefinition {
            name: todo!(),
            name_location: todo!(),
            members: todo!(),
            canonical_name: todo!(),
            src: self.loc_to_src(&input.loc),
            id: self.next_node_id(),
        }
    }

    pub fn build_struct_definition(&mut self, scope: i64, input: &solang_parser::pt::StructDefinition) -> StructDefinition {
        StructDefinition {
            name: todo!(),
            name_location: todo!(),
            visibility: todo!(),
            members: todo!(),
            scope,
            canonical_name: todo!(),
            src: self.loc_to_src(&input.loc),
            id: self.next_node_id(),
        }
    }

    pub fn build_event_definition(&mut self, input: &solang_parser::pt::EventDefinition) -> EventDefinition {
        EventDefinition {
            anonymous: todo!(),
            documentation: todo!(),
            name: todo!(),
            name_location: todo!(),
            parameters: todo!(),
            src: self.loc_to_src(&input.loc),
            id: self.next_node_id(),
        }
    }

    pub fn build_error_definition(&mut self, input: &solang_parser::pt::ErrorDefinition) -> ErrorDefinition {
        ErrorDefinition {
            documentation: todo!(),
            name: todo!(),
            name_location: todo!(),
            parameters: todo!(),
            src: self.loc_to_src(&input.loc),
            id: self.next_node_id(),
        }
    }

    pub fn build_function_definition(&mut self, scope: i64, input: &solang_parser::pt::FunctionDefinition) -> FunctionDefinition {
        FunctionDefinition {
            base_functions: todo!(),
            body: todo!(),
            documentation: todo!(),
            function_selector: todo!(),
            implemented: todo!(),
            kind: todo!(),
            modifiers: todo!(),
            name: todo!(),
            name_location: todo!(),
            overrides: todo!(),
            parameters: todo!(),
            return_parameters: todo!(),
            scope,
            state_mutability: todo!(),
            super_function: todo!(),
            r#virtual: todo!(),
            visibility: todo!(),
            src: self.loc_to_src(&input.loc),
            id: self.next_node_id(),
        }
    }

    pub fn build_variable_declaration(&mut self, scope: i64, input: &solang_parser::pt::VariableDefinition) -> VariableDeclaration {
        VariableDeclaration {
            base_functions: todo!(),
            constant: todo!(),
            documentation: todo!(),
            function_selector: todo!(),
            indexed: todo!(),
            mutability: todo!(),
            name: todo!(),
            name_location: todo!(),
            overrides: todo!(),
            scope,
            state_variable: todo!(),
            storage_location: todo!(),
            type_descriptions: todo!(),
            type_name: todo!(),
            value: todo!(),
            visibility: todo!(),
            src: self.loc_to_src(&input.loc),
            id: self.next_node_id(),
        }
    }

    pub fn build_user_defined_value_type_definition(&mut self, input: &solang_parser::pt::TypeDefinition) -> UserDefinedValueTypeDefinition {
        UserDefinedValueTypeDefinition {
            underlying_type: todo!(),
            name: todo!(),
            name_location: todo!(),
            canonical_name: todo!(),
            src: self.loc_to_src(&input.loc),
            id: self.next_node_id(),
        }
    }
}
