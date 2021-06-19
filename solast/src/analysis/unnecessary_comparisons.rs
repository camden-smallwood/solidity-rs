use super::AstVisitor;

pub struct UnnecessaryComparisonsVisitor;

impl AstVisitor for UnnecessaryComparisonsVisitor {
    //
    // TODO:
    // * uint >= 0
    // * uint8 < 256
    // * etc
}
