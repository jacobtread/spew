enum Operator {
  Equals,

}

enum AST {
    Variable {
        name: String,
        modifiers: Vec<Modifier>,
        value: Box<AST>,
    },
    Operation {
        left: Box<AST>,
        operator: Operator,
        right: Box<AST>
    }
    
}
