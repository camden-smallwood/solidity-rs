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
                    name_location: Some("-1:-1:-1".to_string()), // TODO
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
        let contract_scope = self.next_scope();

        ContractDefinition {
            name: input.name.as_ref().map(|x| x.name.clone()).unwrap(),
            name_location: Some(self.loc_to_src(&input.name.as_ref().map(|x| x.loc).unwrap())),
            documentation: None,
            kind: match input.ty {
                solang_parser::pt::ContractTy::Abstract(_) => ContractKind::Contract,
                solang_parser::pt::ContractTy::Contract(_) => ContractKind::Contract,
                solang_parser::pt::ContractTy::Interface(_) => ContractKind::Interface,
                solang_parser::pt::ContractTy::Library(_) => ContractKind::Library,
            },
            is_abstract: Some(matches!(input.ty, solang_parser::pt::ContractTy::Abstract(_))),
            base_contracts: input.base.iter().map(|base| InheritanceSpecifier {
                base_name: IdentifierPath {
                    name: base.name.identifiers.iter().map(|x| x.name.clone()).collect::<Vec<_>>().join("."),
                    referenced_declaration: None,
                    src: self.loc_to_src(&base.name.loc),
                    id: self.next_node_id(),
                },
                arguments: base.args.as_ref().map(|args| args.iter().map(|x| self.build_expression(x)).collect()),
                src: self.loc_to_src(&base.loc),
                id: self.next_node_id(),
            }).collect(),
            contract_dependencies: vec![], // TODO
            used_events: None, // TODO
            used_errors: None, // TODO
            nodes: input.parts.iter()
                .map(|part| {
                    match part {
                        solang_parser::pt::ContractPart::StructDefinition(x) => Some(ContractDefinitionNode::StructDefinition(self.build_struct_definition(contract_scope, x))),
                        solang_parser::pt::ContractPart::EventDefinition(x) => Some(ContractDefinitionNode::EventDefinition(self.build_event_definition(x))),
                        solang_parser::pt::ContractPart::EnumDefinition(x) => Some(ContractDefinitionNode::EnumDefinition(self.build_enum_definition(x))),
                        solang_parser::pt::ContractPart::ErrorDefinition(x) => Some(ContractDefinitionNode::ErrorDefinition(self.build_error_definition(x))),
                        solang_parser::pt::ContractPart::VariableDefinition(x) => Some(ContractDefinitionNode::VariableDeclaration(self.build_variable_declaration(contract_scope, x))),
                        solang_parser::pt::ContractPart::FunctionDefinition(x) => Some(ContractDefinitionNode::FunctionDefinition(self.build_function_definition(contract_scope, x))),
                        solang_parser::pt::ContractPart::TypeDefinition(x) => Some(ContractDefinitionNode::UserDefinedValueTypeDefinition(self.build_user_defined_value_type_definition(x))),
                        solang_parser::pt::ContractPart::Annotation(_) => None,
                        solang_parser::pt::ContractPart::Using(x) => Some(ContractDefinitionNode::UsingForDirective(self.build_using_for_directive(x))),
                        solang_parser::pt::ContractPart::StraySemicolon(_) => None,
                    }
                })
                .filter(|x| x.is_some())
                .map(|x| x.unwrap())
                .collect(),
            scope,
            fully_implemented: None, // TODO
            linearized_base_contracts: None, // TODO
            internal_function_ids: None, // TODO
            src: self.loc_to_src(&input.loc),
            id: self.next_node_id(),
        }
    }

    pub fn build_enum_definition(&mut self, input: &solang_parser::pt::EnumDefinition) -> EnumDefinition {
        EnumDefinition {
            name: input.name.as_ref().map(|x| x.name.clone()).unwrap(),
            name_location: input.name.as_ref().map(|x| self.loc_to_src(&x.loc)),
            members: input.values.iter().map(|value| {
                EnumValue {
                    name: value.as_ref().map(|x| x.name.clone()).unwrap(),
                    name_location: None, // TODO
                    src: value.as_ref().map(|x| self.loc_to_src(&x.loc)).unwrap(),
                    id: self.next_node_id(),
                }
            }).collect(),
            canonical_name: None, // TODO
            src: self.loc_to_src(&input.loc),
            id: self.next_node_id(),
        }
    }

    pub fn build_struct_definition(&mut self, scope: i64, input: &solang_parser::pt::StructDefinition) -> StructDefinition {
        StructDefinition {
            name: input.name.as_ref().map(|x| x.name.clone()).unwrap(),
            name_location: input.name.as_ref().map(|x| self.loc_to_src(&x.loc)),
            visibility: Visibility::Public,
            members: input.fields.iter()
                .map(|field| {
                    VariableDeclaration {
                        base_functions: None,
                        constant: false,
                        documentation: None,
                        function_selector: None,
                        indexed: None,
                        mutability: None, // TODO
                        name: field.name.as_ref().map(|x| x.name.clone()).unwrap(),
                        name_location: field.name.as_ref().map(|x| self.loc_to_src(&x.loc)),
                        overrides: None,
                        scope,
                        state_variable: false,
                        storage_location: StorageLocation::Default,
                        type_descriptions: TypeDescriptions {
                            type_identifier: None, // TODO
                            type_string: None, // TODO
                        },
                        type_name: Some(self.build_type_name(&field.ty)),
                        value: None,
                        visibility: Visibility::Public,
                        src: self.loc_to_src(&field.loc),
                        id: self.next_node_id(),
                    }
                })
                .collect(),
            scope,
            canonical_name: None, // TODO
            src: self.loc_to_src(&input.loc),
            id: self.next_node_id(),
        }
    }

    pub fn build_event_definition(&mut self, input: &solang_parser::pt::EventDefinition) -> EventDefinition {
        let event_scope = self.next_scope();

        EventDefinition {
            anonymous: input.anonymous,
            documentation: None,
            name: input.name.as_ref().map(|x| x.name.clone()).unwrap(),
            name_location: input.name.as_ref().map(|x| self.loc_to_src(&x.loc)),
            parameters: ParameterList {
                parameters: input.fields.iter()
                    .map(|field| {
                        VariableDeclaration {
                            base_functions: None,
                            constant: false,
                            documentation: None,
                            function_selector: None,
                            indexed: Some(field.indexed),
                            mutability: None,
                            name: field.name.as_ref().map(|x| x.name.clone()).unwrap(),
                            name_location: field.name.as_ref().map(|x| self.loc_to_src(&x.loc)),
                            overrides: None,
                            scope: event_scope,
                            state_variable: false,
                            storage_location: StorageLocation::Default,
                            type_descriptions: TypeDescriptions {
                                type_identifier: None,
                                type_string: None,
                            },
                            type_name: Some(self.build_type_name(&field.ty)),
                            value: None,
                            visibility: Visibility::Public,
                            src: self.loc_to_src(&field.loc),
                            id: self.next_node_id(),
                        }
                    })
                    .collect(),
                src: self.loc_to_src(&input.loc),
                id: self.next_node_id(),
            },
            src: self.loc_to_src(&input.loc),
            id: self.next_node_id(),
        }
    }

    pub fn build_error_definition(&mut self, input: &solang_parser::pt::ErrorDefinition) -> ErrorDefinition {
        let error_scope = self.next_scope();

        ErrorDefinition {
            documentation: None,
            name: input.name.as_ref().map(|x| x.name.clone()).unwrap(),
            name_location: input.name.as_ref().map(|x| self.loc_to_src(&x.loc)),
            parameters: ParameterList {
                parameters: input.fields.iter()
                    .map(|field| {
                        VariableDeclaration {
                            base_functions: None,
                            constant: false,
                            documentation: None,
                            function_selector: None,
                            indexed: None,
                            mutability: None,
                            name: field.name.as_ref().map(|x| x.name.clone()).unwrap(),
                            name_location: field.name.as_ref().map(|x| self.loc_to_src(&x.loc)),
                            overrides: None,
                            scope: error_scope,
                            state_variable: false,
                            storage_location: StorageLocation::Default,
                            type_descriptions: TypeDescriptions {
                                type_identifier: None,
                                type_string: None,
                            },
                            type_name: Some(self.build_type_name(&field.ty)),
                            value: None,
                            visibility: Visibility::Public,
                            src: self.loc_to_src(&field.loc),
                            id: self.next_node_id(),
                        }
                    })
                    .collect(),
                src: self.loc_to_src(&input.loc),
                id: self.next_node_id(),
            },
            src: self.loc_to_src(&input.loc),
            id: self.next_node_id(),
        }
    }

    pub fn build_function_definition(&mut self, scope: i64, input: &solang_parser::pt::FunctionDefinition) -> FunctionDefinition {
        let mut visibility = Visibility::Internal;
        let mut state_mutability = StateMutability::NonPayable;
        let mut is_virtual = None;
        let mut overrides = None;
        let mut modifiers = vec![];

        for attr in input.attributes.iter() {
            match attr {
                solang_parser::pt::FunctionAttribute::Visibility(x) => match x {
                    solang_parser::pt::Visibility::External(_) => visibility = Visibility::External,
                    solang_parser::pt::Visibility::Public(_) => visibility = Visibility::Public,
                    solang_parser::pt::Visibility::Internal(_) => visibility = Visibility::Internal,
                    solang_parser::pt::Visibility::Private(_) => visibility = Visibility::Private,
                },

                solang_parser::pt::FunctionAttribute::Mutability(x) => match x {
                    solang_parser::pt::Mutability::Pure(_) => state_mutability = StateMutability::Pure,
                    solang_parser::pt::Mutability::View(_) => state_mutability = StateMutability::View,
                    solang_parser::pt::Mutability::Constant(_) => panic!("Invalid function state mutability: Constant"),
                    solang_parser::pt::Mutability::Payable(_) => state_mutability = StateMutability::Payable,
                },

                solang_parser::pt::FunctionAttribute::Virtual(_) => is_virtual = Some(true),

                solang_parser::pt::FunctionAttribute::Immutable(_) => panic!("Invalid function attribute: Immutable"),

                solang_parser::pt::FunctionAttribute::Override(loc, x) => overrides = Some(OverrideSpecifier {
                    overrides: x.iter()
                        .map(|x| {
                            IdentifierPath {
                                name: x.identifiers.iter().map(|x| x.name.clone()).collect::<Vec<_>>().join("."), // TODO
                                referenced_declaration: None, // TODO
                                src: self.loc_to_src(&x.loc),
                                id: self.next_node_id(),
                            }
                        })
                        .collect(),
                    src: self.loc_to_src(loc),
                    id: self.next_node_id(),
                }),

                solang_parser::pt::FunctionAttribute::BaseOrModifier(loc, x) => modifiers.push(ModifierInvocation {
                    arguments: x.args.as_ref()
                        .map(|args| {
                            args.iter()
                                .map(|arg| self.build_expression(arg))
                                .collect()
                        }),
                    modifier_name: IdentifierPath {
                        name: x.name.identifiers.iter().map(|x| x.name.clone()).collect::<Vec<_>>().join("."), // TODO
                        referenced_declaration: None, // TODO
                        src: self.loc_to_src(&x.name.loc),
                        id: self.next_node_id(),
                    },
                    src: self.loc_to_src(loc),
                    id: self.next_node_id(),
                    kind: None, // TODO
                }),

                solang_parser::pt::FunctionAttribute::Error(_) => {}
            }
        }

        let function_scope = self.next_scope();

        FunctionDefinition {
            base_functions: None, // TODO
            body: input.body.as_ref()
                .map(|body| {
                    match body {
                        solang_parser::pt::Statement::Block { loc, statements, .. } => Block {
                            statements: statements.iter()
                                .map(|stmt| self.build_statement(stmt))
                                .collect(),
                            src: self.loc_to_src(loc),
                            id: self.next_node_id(),
                        },
                        stmt => panic!("Invalid function body statement: {stmt:?}"),
                    }
                }),
            documentation: None, // TODO
            function_selector: None, // TODO
            implemented: input.body.is_some(), // TODO: is this correct?
            kind: match input.ty {
                solang_parser::pt::FunctionTy::Constructor => FunctionKind::Constructor,
                solang_parser::pt::FunctionTy::Function => FunctionKind::Function,
                solang_parser::pt::FunctionTy::Fallback => FunctionKind::Fallback,
                solang_parser::pt::FunctionTy::Receive => FunctionKind::Receive,
                solang_parser::pt::FunctionTy::Modifier => panic!("Invalid function kind: Modifier"), // TODO: handle ahead of time?
            },
            modifiers,
            name: input.name.as_ref().map(|x| x.name.clone()).unwrap_or_else(String::new),
            name_location: input.name.as_ref().map(|x| self.loc_to_src(&x.loc)),
            overrides,
            parameters: ParameterList {
                parameters: input.params.iter()
                    .map(|(loc, parameter)| {
                        VariableDeclaration {
                            base_functions: None,
                            constant: false,
                            documentation: None,
                            function_selector: None,
                            indexed: None,
                            mutability: None,
                            name: parameter.as_ref().map(|x| x.name.as_ref().map(|x| x.name.clone()).unwrap()).unwrap(),
                            name_location: parameter.as_ref().map(|x| x.name.as_ref().map(|x| self.loc_to_src(&x.loc))).unwrap(),
                            overrides: None,
                            scope: function_scope,
                            state_variable: false,
                            storage_location: parameter.as_ref()
                                .map(|x| {
                                    x.storage.as_ref()
                                        .map(|x| match x {
                                            solang_parser::pt::StorageLocation::Memory(_) => StorageLocation::Memory,
                                            solang_parser::pt::StorageLocation::Storage(_) => StorageLocation::Storage,
                                            solang_parser::pt::StorageLocation::Calldata(_) => StorageLocation::Calldata,
                                        })
                                        .unwrap_or_else(|| StorageLocation::Default)
                                })
                                .unwrap(),
                            type_descriptions: TypeDescriptions {
                                type_identifier: None, // TODO
                                type_string: None, // TODO
                            },
                            type_name: Some(parameter.as_ref().map(|x| self.build_type_name(&x.ty)).unwrap()),
                            value: None,
                            visibility,
                            src: self.loc_to_src(loc),
                            id: self.next_node_id(),
                        }
                    })
                    .collect(),
                src: self.loc_to_src(&input.loc),
                id: self.next_node_id(),
            },
            return_parameters: ParameterList {
                parameters: input.returns.iter()
                    .map(|(loc, parameter)| {
                        VariableDeclaration {
                            base_functions: None,
                            constant: false,
                            documentation: None,
                            function_selector: None,
                            indexed: None,
                            mutability: None,
                            name: parameter.as_ref().map(|x| x.name.as_ref().map(|x| x.name.clone()).unwrap()).unwrap(),
                            name_location: parameter.as_ref().map(|x| x.name.as_ref().map(|x| self.loc_to_src(&x.loc))).unwrap(),
                            overrides: None,
                            scope: function_scope,
                            state_variable: false,
                            storage_location: parameter.as_ref()
                                .map(|x| {
                                    x.storage.as_ref()
                                        .map(|x| match x {
                                            solang_parser::pt::StorageLocation::Memory(_) => StorageLocation::Memory,
                                            solang_parser::pt::StorageLocation::Storage(_) => StorageLocation::Storage,
                                            solang_parser::pt::StorageLocation::Calldata(_) => StorageLocation::Calldata,
                                        })
                                        .unwrap_or_else(|| StorageLocation::Default)
                                })
                                .unwrap(),
                            type_descriptions: TypeDescriptions {
                                type_identifier: None, // TODO
                                type_string: None, // TODO
                            },
                            type_name: Some(parameter.as_ref().map(|x| self.build_type_name(&x.ty)).unwrap()),
                            value: None,
                            visibility,
                            src: self.loc_to_src(loc),
                            id: self.next_node_id(),
                        }
                    })
                    .collect(),
                src: self.loc_to_src(&input.loc),
                id: self.next_node_id(),
            },
            scope,
            state_mutability,
            super_function: None, // TODO
            is_virtual,
            visibility,
            src: self.loc_to_src(&input.loc),
            id: self.next_node_id(),
        }
    }

    pub fn build_variable_declaration(&mut self, scope: i64, input: &solang_parser::pt::VariableDefinition) -> VariableDeclaration {
        let mut visibility = Visibility::Public;
        let mut mutability = None;
        let mut constant = false;
        let mut overrides = None;

        for attr in input.attrs.iter() {
            match attr {
                solang_parser::pt::VariableAttribute::Visibility(x) => match x {
                    solang_parser::pt::Visibility::External(_) => visibility = Visibility::External,
                    solang_parser::pt::Visibility::Public(_) => visibility = Visibility::Public,
                    solang_parser::pt::Visibility::Internal(_) => visibility = Visibility::Internal,
                    solang_parser::pt::Visibility::Private(_) => visibility = Visibility::Private,
                },

                solang_parser::pt::VariableAttribute::Constant(_) => constant = true,

                solang_parser::pt::VariableAttribute::Immutable(_) => mutability = Some(Mutability::Immutable),

                solang_parser::pt::VariableAttribute::Override(loc, x) => overrides = Some(OverrideSpecifier {
                    overrides: x.iter()
                        .map(|x| {
                            IdentifierPath {
                                name: x.identifiers.iter().map(|x| x.name.clone()).collect::<Vec<_>>().join("."), // TODO
                                referenced_declaration: None, // TODO
                                src: self.loc_to_src(&x.loc),
                                id: self.next_node_id(),
                            }
                        })
                        .collect(),
                    src: self.loc_to_src(loc),
                    id: self.next_node_id(),
                }),
            }
        }

        VariableDeclaration {
            base_functions: None,
            constant,
            documentation: None,
            function_selector: None,
            indexed: None,
            mutability,
            name: input.name.as_ref().map(|x| x.name.clone()).unwrap(),
            name_location: input.name.as_ref().map(|x| self.loc_to_src(&x.loc)),
            overrides,
            scope,
            state_variable: false, // TODO: is this in the type expression?
            storage_location: StorageLocation::Default, // TODO: is this in the type expression?
            type_descriptions: TypeDescriptions {
                type_identifier: None, // TODO
                type_string: None, // TODO
            },
            type_name: Some(self.build_type_name(&input.ty)),
            value: input.initializer.as_ref().map(|x| self.build_expression(x)),
            visibility,
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

    pub fn build_using_for_directive(&mut self, input: &solang_parser::pt::Using) -> UsingForDirective {
        UsingForDirective {
            library_name: todo!(),
            type_name: todo!(),
            src: self.loc_to_src(&input.loc),
            id: self.next_node_id(),
        }
    }

    pub fn build_type_name(&mut self, input: &solang_parser::pt::Expression) -> TypeName {
        match input {
            solang_parser::pt::Expression::Type(_loc, ty) => match ty {
                solang_parser::pt::Type::Address => TypeName::ElementaryTypeName(ElementaryTypeName {
                    state_mutability: None, // TODO
                    name: "address".to_string(),
                    type_descriptions: TypeDescriptions {
                        type_identifier: None, // TODO
                        type_string: None, // TODO
                    },
                }),
    
                solang_parser::pt::Type::AddressPayable => TypeName::ElementaryTypeName(ElementaryTypeName {
                    state_mutability: None, // TODO
                    name: "address payable".to_string(),
                    type_descriptions: TypeDescriptions {
                        type_identifier: None, // TODO
                        type_string: None, // TODO
                    },
                }),
    
                solang_parser::pt::Type::Payable => TypeName::ElementaryTypeName(ElementaryTypeName {
                    state_mutability: None, // TODO
                    name: "payable".to_string(),
                    type_descriptions: TypeDescriptions {
                        type_identifier: None, // TODO
                        type_string: None, // TODO
                    },
                }),
    
                solang_parser::pt::Type::Bool => TypeName::ElementaryTypeName(ElementaryTypeName {
                    state_mutability: None, // TODO
                    name: "bool".to_string(),
                    type_descriptions: TypeDescriptions {
                        type_identifier: None, // TODO
                        type_string: None, // TODO
                    },
                }),
    
                solang_parser::pt::Type::String => TypeName::ElementaryTypeName(ElementaryTypeName {
                    state_mutability: None, // TODO
                    name: "string".to_string(),
                    type_descriptions: TypeDescriptions {
                        type_identifier: None, // TODO
                        type_string: None, // TODO
                    },
                }),
    
                solang_parser::pt::Type::Int(bits) => TypeName::ElementaryTypeName(ElementaryTypeName {
                    state_mutability: None, // TODO
                    name: format!("int{bits}"),
                    type_descriptions: TypeDescriptions {
                        type_identifier: None, // TODO
                        type_string: None, // TODO
                    },
                }),
    
                solang_parser::pt::Type::Uint(bits) => TypeName::ElementaryTypeName(ElementaryTypeName {
                    state_mutability: None, // TODO
                    name: format!("uint{bits}"),
                    type_descriptions: TypeDescriptions {
                        type_identifier: None, // TODO
                        type_string: None, // TODO
                    },
                }),
    
                solang_parser::pt::Type::Bytes(bytes) => TypeName::ElementaryTypeName(ElementaryTypeName {
                    state_mutability: None, // TODO
                    name: format!("bytes{bytes}"),
                    type_descriptions: TypeDescriptions {
                        type_identifier: None, // TODO
                        type_string: None, // TODO
                    },
                }),
    
                solang_parser::pt::Type::Rational => todo!(),
    
                solang_parser::pt::Type::DynamicBytes => TypeName::ElementaryTypeName(ElementaryTypeName {
                    state_mutability: None, // TODO
                    name: "bytes".to_string(),
                    type_descriptions: TypeDescriptions {
                        type_identifier: None, // TODO
                        type_string: None, // TODO
                    },
                }),
    
                solang_parser::pt::Type::Mapping { loc, key, key_name, value, value_name } => TypeName::Mapping(Mapping {
                    key_type: Box::new(self.build_type_name(key.as_ref())),
                    value_type: Box::new(self.build_type_name(value.as_ref())),
                    type_descriptions: TypeDescriptions {
                        type_identifier: None, // TODO
                        type_string: None, // TODO
                    },
                }),
    
                solang_parser::pt::Type::Function { params, attributes, returns } => {
                    let mut visibility = Visibility::Internal;
                    let mut state_mutability = StateMutability::NonPayable;
                    let mut is_virtual = None;
                    let mut overrides = None;
                    let mut modifiers = vec![];
            
                    for attr in attributes.iter() {
                        match attr {
                            solang_parser::pt::FunctionAttribute::Visibility(x) => match x {
                                solang_parser::pt::Visibility::External(_) => visibility = Visibility::External,
                                solang_parser::pt::Visibility::Public(_) => visibility = Visibility::Public,
                                solang_parser::pt::Visibility::Internal(_) => visibility = Visibility::Internal,
                                solang_parser::pt::Visibility::Private(_) => visibility = Visibility::Private,
                            },
            
                            solang_parser::pt::FunctionAttribute::Mutability(x) => match x {
                                solang_parser::pt::Mutability::Pure(_) => state_mutability = StateMutability::Pure,
                                solang_parser::pt::Mutability::View(_) => state_mutability = StateMutability::View,
                                solang_parser::pt::Mutability::Constant(_) => panic!("Invalid function state mutability: Constant"),
                                solang_parser::pt::Mutability::Payable(_) => state_mutability = StateMutability::Payable,
                            },
            
                            solang_parser::pt::FunctionAttribute::Virtual(_) => is_virtual = Some(true),
            
                            solang_parser::pt::FunctionAttribute::Immutable(_) => panic!("Invalid function attribute: Immutable"),
            
                            solang_parser::pt::FunctionAttribute::Override(loc, x) => overrides = Some(OverrideSpecifier {
                                overrides: x.iter()
                                    .map(|x| {
                                        IdentifierPath {
                                            name: x.identifiers.iter().map(|x| x.name.clone()).collect::<Vec<_>>().join("."), // TODO
                                            referenced_declaration: None, // TODO
                                            src: self.loc_to_src(&x.loc),
                                            id: self.next_node_id(),
                                        }
                                    })
                                    .collect(),
                                src: self.loc_to_src(loc),
                                id: self.next_node_id(),
                            }),
            
                            solang_parser::pt::FunctionAttribute::BaseOrModifier(loc, x) => modifiers.push(ModifierInvocation {
                                arguments: x.args.as_ref()
                                    .map(|args| {
                                        args.iter()
                                            .map(|arg| self.build_expression(arg))
                                            .collect()
                                    }),
                                modifier_name: IdentifierPath {
                                    name: x.name.identifiers.iter().map(|x| x.name.clone()).collect::<Vec<_>>().join("."), // TODO
                                    referenced_declaration: None, // TODO
                                    src: self.loc_to_src(&x.name.loc),
                                    id: self.next_node_id(),
                                },
                                src: self.loc_to_src(loc),
                                id: self.next_node_id(),
                                kind: None, // TODO
                            }),
            
                            solang_parser::pt::FunctionAttribute::Error(_) => {}
                        }
                    }
    
                    TypeName::FunctionTypeName(FunctionTypeName {
                        visibility,
                        state_mutability,
                        parameter_types: ParameterList {
                            parameters: params.iter()
                                .map(|(loc, parameter)| {
                                    VariableDeclaration {
                                        base_functions: None,
                                        constant: false,
                                        documentation: None,
                                        function_selector: None,
                                        indexed: None,
                                        mutability: None,
                                        name: parameter.as_ref().map(|x| x.name.as_ref().map(|x| x.name.clone()).unwrap()).unwrap(),
                                        name_location: parameter.as_ref().map(|x| x.name.as_ref().map(|x| self.loc_to_src(&x.loc))).unwrap(),
                                        overrides: None,
                                        scope: -1, // TODO
                                        state_variable: false,
                                        storage_location: parameter.as_ref()
                                            .map(|x| {
                                                x.storage.as_ref()
                                                    .map(|x| match x {
                                                        solang_parser::pt::StorageLocation::Memory(_) => StorageLocation::Memory,
                                                        solang_parser::pt::StorageLocation::Storage(_) => StorageLocation::Storage,
                                                        solang_parser::pt::StorageLocation::Calldata(_) => StorageLocation::Calldata,
                                                    })
                                                    .unwrap_or_else(|| StorageLocation::Default)
                                            })
                                            .unwrap(),
                                        type_descriptions: TypeDescriptions {
                                            type_identifier: None, // TODO
                                            type_string: None, // TODO
                                        },
                                        type_name: Some(parameter.as_ref().map(|x| self.build_type_name(&x.ty)).unwrap()),
                                        value: None,
                                        visibility,
                                        src: self.loc_to_src(loc),
                                        id: self.next_node_id(),
                                    }
                                })
                                .collect(),
                            src: "-1:-1:-1".to_string(), // TODO
                            id: self.next_node_id(),
                        },
                        return_parameter_types: ParameterList {
                            parameters: returns.as_ref()
                                .map(|(returns, _attrs)| {
                                    returns.iter()
                                        .map(|(loc, parameter)| {
                                            VariableDeclaration {
                                                base_functions: None,
                                                constant: false,
                                                documentation: None,
                                                function_selector: None,
                                                indexed: None,
                                                mutability: None,
                                                name: parameter.as_ref().map(|x| x.name.as_ref().map(|x| x.name.clone()).unwrap()).unwrap(),
                                                name_location: parameter.as_ref().map(|x| x.name.as_ref().map(|x| self.loc_to_src(&x.loc))).unwrap(),
                                                overrides: None,
                                                scope: -1, // TODO
                                                state_variable: false,
                                                storage_location: parameter.as_ref()
                                                    .map(|x| {
                                                        x.storage.as_ref()
                                                            .map(|x| match x {
                                                                solang_parser::pt::StorageLocation::Memory(_) => StorageLocation::Memory,
                                                                solang_parser::pt::StorageLocation::Storage(_) => StorageLocation::Storage,
                                                                solang_parser::pt::StorageLocation::Calldata(_) => StorageLocation::Calldata,
                                                            })
                                                            .unwrap_or_else(|| StorageLocation::Default)
                                                    })
                                                    .unwrap(),
                                                type_descriptions: TypeDescriptions {
                                                    type_identifier: None, // TODO
                                                    type_string: None, // TODO
                                                },
                                                type_name: Some(parameter.as_ref().map(|x| self.build_type_name(&x.ty)).unwrap()),
                                                value: None,
                                                visibility,
                                                src: self.loc_to_src(loc),
                                                id: self.next_node_id(),
                                            }
                                        })
                                        .collect()
                                })
                                .unwrap_or_else(|| vec![]),
                            src: "-1:-1:-1".to_string(), // TODO
                            id: self.next_node_id(),
                        },
                        type_descriptions: TypeDescriptions {
                            type_identifier: None, // TODO
                            type_string: None, // TODO
                        },
                    })
                }
            },

            solang_parser::pt::Expression::ArraySubscript(_loc, ty, len) => TypeName::ArrayTypeName(ArrayTypeName {
                base_type: Box::new(self.build_type_name(ty)),
                length: len.as_ref().map(|x| self.build_literal(x)),
                type_descriptions: TypeDescriptions {
                    type_identifier: None, // TODO
                    type_string: None, // TODO
                },
            }),

            _ => panic!("Unhandled type name expression: {input:#?}"),
        }
    }

    pub fn build_statement(&mut self, input: &solang_parser::pt::Statement) -> Statement {
        todo!()
    }

    pub fn build_literal(&mut self, input: &solang_parser::pt::Expression) -> Literal {
        todo!()
    }

    pub fn build_expression(&mut self, input: &solang_parser::pt::Expression) -> Expression {
        match input {
            solang_parser::pt::Expression::PostIncrement(_, _) => todo!(),
            solang_parser::pt::Expression::PostDecrement(_, _) => todo!(),
            solang_parser::pt::Expression::New(_, _) => todo!(),
            solang_parser::pt::Expression::ArraySubscript(_, _, _) => todo!(),
            solang_parser::pt::Expression::ArraySlice(_, _, _, _) => todo!(),
            solang_parser::pt::Expression::Parenthesis(_, _) => todo!(),
            solang_parser::pt::Expression::MemberAccess(_, _, _) => todo!(),
            solang_parser::pt::Expression::FunctionCall(_, _, _) => todo!(),
            solang_parser::pt::Expression::FunctionCallBlock(_, _, _) => todo!(),
            solang_parser::pt::Expression::NamedFunctionCall(_, _, _) => todo!(),
            solang_parser::pt::Expression::Not(_, _) => todo!(),
            solang_parser::pt::Expression::BitwiseNot(_, _) => todo!(),
            solang_parser::pt::Expression::Delete(_, _) => todo!(),
            solang_parser::pt::Expression::PreIncrement(_, _) => todo!(),
            solang_parser::pt::Expression::PreDecrement(_, _) => todo!(),
            solang_parser::pt::Expression::UnaryPlus(_, _) => todo!(),
            solang_parser::pt::Expression::Negate(_, _) => todo!(),
            solang_parser::pt::Expression::Power(_, _, _) => todo!(),
            solang_parser::pt::Expression::Multiply(_, _, _) => todo!(),
            solang_parser::pt::Expression::Divide(_, _, _) => todo!(),
            solang_parser::pt::Expression::Modulo(_, _, _) => todo!(),
            solang_parser::pt::Expression::Add(_, _, _) => todo!(),
            solang_parser::pt::Expression::Subtract(_, _, _) => todo!(),
            solang_parser::pt::Expression::ShiftLeft(_, _, _) => todo!(),
            solang_parser::pt::Expression::ShiftRight(_, _, _) => todo!(),
            solang_parser::pt::Expression::BitwiseAnd(_, _, _) => todo!(),
            solang_parser::pt::Expression::BitwiseXor(_, _, _) => todo!(),
            solang_parser::pt::Expression::BitwiseOr(_, _, _) => todo!(),
            solang_parser::pt::Expression::Less(_, _, _) => todo!(),
            solang_parser::pt::Expression::More(_, _, _) => todo!(),
            solang_parser::pt::Expression::LessEqual(_, _, _) => todo!(),
            solang_parser::pt::Expression::MoreEqual(_, _, _) => todo!(),
            solang_parser::pt::Expression::Equal(_, _, _) => todo!(),
            solang_parser::pt::Expression::NotEqual(_, _, _) => todo!(),
            solang_parser::pt::Expression::And(_, _, _) => todo!(),
            solang_parser::pt::Expression::Or(_, _, _) => todo!(),
            solang_parser::pt::Expression::ConditionalOperator(_, _, _, _) => todo!(),
            solang_parser::pt::Expression::Assign(_, _, _) => todo!(),
            solang_parser::pt::Expression::AssignOr(_, _, _) => todo!(),
            solang_parser::pt::Expression::AssignAnd(_, _, _) => todo!(),
            solang_parser::pt::Expression::AssignXor(_, _, _) => todo!(),
            solang_parser::pt::Expression::AssignShiftLeft(_, _, _) => todo!(),
            solang_parser::pt::Expression::AssignShiftRight(_, _, _) => todo!(),
            solang_parser::pt::Expression::AssignAdd(_, _, _) => todo!(),
            solang_parser::pt::Expression::AssignSubtract(_, _, _) => todo!(),
            solang_parser::pt::Expression::AssignMultiply(_, _, _) => todo!(),
            solang_parser::pt::Expression::AssignDivide(_, _, _) => todo!(),
            solang_parser::pt::Expression::AssignModulo(_, _, _) => todo!(),
            solang_parser::pt::Expression::BoolLiteral(_, _) => todo!(),
            solang_parser::pt::Expression::NumberLiteral(_, _, _, _) => todo!(),
            solang_parser::pt::Expression::RationalNumberLiteral(_, _, _, _, _) => todo!(),
            solang_parser::pt::Expression::HexNumberLiteral(_, _, _) => todo!(),
            solang_parser::pt::Expression::StringLiteral(_) => todo!(),
            solang_parser::pt::Expression::Type(_, _) => todo!(),
            solang_parser::pt::Expression::HexLiteral(_) => todo!(),
            solang_parser::pt::Expression::AddressLiteral(_, _) => todo!(),
            solang_parser::pt::Expression::Variable(_) => todo!(),
            solang_parser::pt::Expression::List(_, _) => todo!(),
            solang_parser::pt::Expression::ArrayLiteral(_, _) => todo!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ast_builder() {
        let src = std::fs::read_to_string("/Users/camden/Source/solidity-test/contracts/Blah.sol").unwrap();
        let (input, _comments) = solang_parser::parse(src.as_str(), 0).unwrap();
        
        let mut builder = AstBuilder::default();
        let source_unit = builder.build_source_unit(&input);

        println!("{:#?}", source_unit);
    }
}
