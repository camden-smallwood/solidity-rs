use std::collections::HashMap;

use solidity::ast::{ArrayTypeName, ContractKind, ElementaryTypeName, Literal, NodeID, SourceUnit, TypeName, UserDefinedTypeName};

use super::AstVisitor;

#[derive(Debug)]
struct StorageSlot {
    member_sizes: Vec<usize>
}

#[derive(Debug)]
pub struct TightVariablePackingVisitor {
    storage_slots: HashMap<NodeID, Vec<StorageSlot>>
}

impl Default for TightVariablePackingVisitor {
    fn default() -> Self {
        Self {
            storage_slots: HashMap::new()
        }
    }
}

fn type_name_size(source_units: &[SourceUnit], type_name: &TypeName) -> std::io::Result<usize> {
    Ok(match type_name {
        TypeName::ElementaryTypeName(ElementaryTypeName { name, .. }) => match name.as_str() {
            "bool" => 1,
    
            "address" => 20,
    
            type_name if type_name.starts_with("uint") => {
                let size_in_bits = match type_name.trim_start_matches("uint") {
                    "" => 256,
                    s => s.parse().map_err(|err| {
                        std::io::Error::new(
                            std::io::ErrorKind::InvalidData,
                            format!("unhandled {} type: {}", type_name, err)
                        )
                    })?
                };
    
                size_in_bits / 8
            }
    
            type_name if type_name.starts_with("int") => {
                let size_in_bits = match type_name.trim_start_matches("int") {
                    "" => 256,
                    s => s.parse().map_err(|err| {
                        std::io::Error::new(
                            std::io::ErrorKind::InvalidData,
                            format!("unhandled {} type: {}", type_name, err)
                        )
                    })?
                };
                
                size_in_bits / 8
            }
    
            type_name if type_name.starts_with("bytes") => {
                let size_in_bytes: usize = match type_name.trim_start_matches("bytes") {
                    "" => 32,
                    s => s.parse().map_err(|err| {
                        std::io::Error::new(
                            std::io::ErrorKind::InvalidData,
                            format!("unhandled {} type: {}", type_name, err)
                        )
                    })?
                };
                
                size_in_bytes
            }
    
            type_name => {
                println!("WARNING: unhandled type name: {:?}", type_name);
    
                32 // TODO: handle this correctly...
            }
        }

        TypeName::UserDefinedTypeName(UserDefinedTypeName { referenced_declaration, .. }) => {
            let id = referenced_declaration.clone();

            for source_unit in source_units.iter() {
                if let Some(_) = source_unit.enum_definition(id) {
                    return Ok(1)
                } else if let Some(struct_definition) = source_unit.struct_definition(id) {
                    let mut size = 0;

                    for member in struct_definition.members.iter() {
                        size += type_name_size(source_units, member.type_name.as_ref().unwrap())?;
                    }

                    return Ok(size)
                } else {
                    for contract_definition in source_unit.contract_definitions() {
                        if let Some(_) = contract_definition.enum_definition(id) {
                            return Ok(1)
                        } else if let Some(struct_definition) = contract_definition.struct_definition(id) {
                            let mut size = 0;
        
                            for member in struct_definition.members.iter() {
                                size += type_name_size(source_units, member.type_name.as_ref().unwrap())?;
                            }
        
                            return Ok(size)
                        }
                    }
                }
            }

            return Err(std::io::Error::new(std::io::ErrorKind::NotFound, format!("User-defined type not found: {}", type_name)))
        }

        TypeName::ArrayTypeName(ArrayTypeName { base_type, length: Some(Literal { value, .. }), .. }) => {
            let value = value.as_ref().unwrap();

            if let Ok(length) = if value.starts_with("0x") {
                i64::from_str_radix(value.trim_start_matches("0x"), 16)
            } else {
                value.parse()
            } {
                return Ok(type_name_size(source_units, base_type.as_ref())? * length as usize)
            }

            return Err(std::io::Error::new(std::io::ErrorKind::NotFound, format!("Unhandled array type: {}", type_name)))
        }

        TypeName::ArrayTypeName(ArrayTypeName { length: None, .. }) => 32,
        TypeName::FunctionTypeName(_) => 32,
        TypeName::Mapping(_) => 32,
        TypeName::String(_) => 32,
    })
}

impl AstVisitor for TightVariablePackingVisitor {
    fn visit_struct_definition<'a>(&mut self, context: &mut super::StructDefinitionContext<'a>) -> std::io::Result<()> {
        if self.storage_slots.contains_key(&context.struct_definition.id) {
            return Ok(())
        }

        self.storage_slots.insert(context.struct_definition.id, vec![]);

        let storage_slots = self.storage_slots.get_mut(&context.struct_definition.id).unwrap();

        let mut current_slot = None;

        for member in context.struct_definition.members.iter() {
            if current_slot.is_none() {
                current_slot = Some(StorageSlot { member_sizes: vec![] });
            }

            let current_slot_size: usize = current_slot.as_ref().unwrap().member_sizes.iter().sum();

            let member_type_name_size = match type_name_size(context.source_units, member.type_name.as_ref().unwrap()) {
                Ok(x) => x,
                Err(_) => continue,
            };

            if current_slot_size + member_type_name_size > 32 {
                storage_slots.push(current_slot.unwrap());
                current_slot = Some(StorageSlot { member_sizes: vec![] });
            }

            current_slot.as_mut().unwrap().member_sizes.push(member_type_name_size);
        }

        if let Some(false) = current_slot.as_ref().map(|slot| slot.member_sizes.is_empty()) {
            storage_slots.push(current_slot.unwrap());
        }

        let mut has_loose_variable_packing = false;

        for slot in storage_slots.split_last().map(|x| x.1).unwrap_or(&[]) {
            if slot.member_sizes.iter().sum::<usize>() != 32 {
                has_loose_variable_packing = true;
                break;
            }
        }

        //
        // TODO: try to see if looseness can be made tight
        // if no optimizations can be made, set has_loose_variable_packing to false
        //

        if has_loose_variable_packing {
            println!("\tStruct {} has loose variable packing", context.struct_definition.name);
        }

        Ok(())
    }

    fn visit_contract_definition<'a>(&mut self, context: &mut super::ContractDefinitionContext<'a>) -> std::io::Result<()> {
        if let ContractKind::Interface | ContractKind::Library = context.contract_definition.kind {
            return Ok(())
        }
        
        if self.storage_slots.contains_key(&context.contract_definition.id) {
            return Ok(())
        }

        self.storage_slots.insert(context.contract_definition.id, vec![]);

        let storage_slots = self.storage_slots.get_mut(&context.contract_definition.id).unwrap();

        let mut current_slot = None;

        for member in context.contract_definition.variable_declarations() {
            if current_slot.is_none() {
                current_slot = Some(StorageSlot { member_sizes: vec![] });
            }

            let current_slot_size: usize = current_slot.as_ref().unwrap().member_sizes.iter().sum();

            let member_type_name_size = match type_name_size(context.source_units, member.type_name.as_ref().unwrap()) {
                Ok(x) => x,
                Err(_) => continue,
            };

            if current_slot_size + member_type_name_size > 32 {
                storage_slots.push(current_slot.unwrap());
                current_slot = Some(StorageSlot { member_sizes: vec![] });
            }

            current_slot.as_mut().unwrap().member_sizes.push(member_type_name_size);
        }

        if let Some(false) = current_slot.as_ref().map(|slot| slot.member_sizes.is_empty()) {
            storage_slots.push(current_slot.unwrap());
        }

        let mut has_loose_variable_packing = false;

        for slot in storage_slots.split_last().map(|x| x.1).unwrap_or(&[]) {
            if slot.member_sizes.iter().sum::<usize>() != 32 {
                has_loose_variable_packing = true;
                break;
            }
        }

        //
        // TODO: try to see if looseness can be made tight
        // if no optimizations can be made, set has_loose_variable_packing to false
        //

        if has_loose_variable_packing {
            println!("\t{:?} {} has loose variable packing", context.contract_definition.kind, context.contract_definition.name);
        }

        Ok(())
    }
}
