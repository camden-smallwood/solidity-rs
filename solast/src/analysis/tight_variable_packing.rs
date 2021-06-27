use std::collections::HashMap;

use solidity::ast::NodeID;

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

impl AstVisitor for TightVariablePackingVisitor {
    fn visit_struct_definition<'a>(&mut self, context: &mut super::StructDefinitionContext<'a>) -> std::io::Result<()> {
        if self.storage_slots.contains_key(&context.struct_definition.id) {
            return Ok(())
        }

        let mut storage_slots: Vec<StorageSlot> = vec![];
        let mut current_slot: Option<StorageSlot> = None;

        for member in context.struct_definition.members.iter() {
            if current_slot.is_none() {
                current_slot = Some(StorageSlot { member_sizes: vec![] });
            }

            let current_slot_size: usize = current_slot.as_ref().unwrap().member_sizes.iter().sum();

            let size_in_bytes: usize = match member.type_descriptions.type_string.as_ref().map(String::as_str) {
                Some("bool") => 1,

                Some("address") => 20,

                Some(type_name) if type_name.starts_with("uint") => {
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

                Some(type_name) if type_name.starts_with("int") => {
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

                Some(type_name) if type_name.starts_with("bytes") => {
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
            };

            if current_slot_size + size_in_bytes > 32 {
                storage_slots.push(current_slot.unwrap());
                current_slot = Some(StorageSlot { member_sizes: vec![] });
            }

            current_slot.as_mut().unwrap().member_sizes.push(size_in_bytes);
        }

        if let Some(false) = current_slot.as_ref().map(|slot| slot.member_sizes.is_empty()) {
            storage_slots.push(current_slot.unwrap());
        }

        let mut loose = false;

        for slot in storage_slots.split_last().map(|x| x.1).unwrap_or(&[]) {
            if slot.member_sizes.iter().sum::<usize>() != 32 {
                loose = true;
                break;
            }
        }

        if loose {
            println!("\tStruct {} has loose variable packing", context.struct_definition.name);
        }

        self.storage_slots.insert(context.struct_definition.id, storage_slots);

        Ok(())
    }
}
