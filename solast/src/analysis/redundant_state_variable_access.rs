use super::AstVisitor;

pub struct RedundantStateVariableAccessVisitor;

impl AstVisitor for RedundantStateVariableAccessVisitor {
    //
    // TODO: check if a state variable is accessed multiple times in a block without any changes being made
    //
}
