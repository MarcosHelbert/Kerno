#[derive(Debug, Clone)]
pub enum Type {
    Int,
    Float,
    Decimal,
    String,
    Bool,
    Void,

    List(Box<Type>),
    Array(Box<Type>), // remove tamanho fixo, ou implementa em outro lugar
    Map(Box<Type>, Box<Type>),
    Set(Box<Type>),
    Tuple(Vec<Type>),
    Queue(Box<Type>),
    Stack(Box<Type>),
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum Expression {
    IntegerLiteral(i64),
    FloatLiteral(f64),
    DecimalLiteral {
        integer_part: i64,
        fractional_part: i64,
    },
    StringLiteral(String),
    BooleanLiteral(bool),
    NullLiteral,
    Identifier(String),

    BinaryOperator {
        left: Box<Expression>,
        operator: BinaryOperator,
        right: Box<Expression>,
    },

    Call {
        callee: String,
        arguments: Vec<CallArgument>,
    },

    ListLiteral(Vec<Expression>),
    ArrayLiteral(Vec<Expression>),
    MapLiteral(Vec<(Expression, Expression)>),
    SetLiteral(Vec<Expression>),
    TupleLiteral(Vec<Expression>),

    // TODO: Pode adicionar depois lambda, closures, etc.
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum CallArgument {
    Positional(Expression),
    Named {
        name: String,
        value: Expression,
    },
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Module,
    Equal,
    NotEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    And,
    Or,

    PlusEqual,
    MinusEqual,
    StarEqual,
    SlashEqual,
    PercentEqual,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum Statement {
    Function(FunctionDeclaration),
    Let {
        name: String,
        is_mutable: bool,
        variable_type: Option<Type>,
        value: Expression,
    },
    Return(Option<Expression>),
    ExpressionStmt(Expression),

    For {
        initializer: Option<Box<Statement>>,
        condition: Option<Expression>,
        increment: Option<Expression>,
        body: Vec<Statement>,
    },
    While {
        condition: Expression,
        body: Vec<Statement>,
    },
    If {
        condition: Expression,
        then_branch: Vec<Statement>,
        else_branch: Option<Vec<Statement>>,
    },
    Break,
    Continue,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct FunctionDeclaration {
    pub name: String,
    pub is_async: bool,
    pub parameters: Vec<Parameter>,
    pub return_type: Option<Type>,
    pub body: Vec<Statement>,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Parameter {
    pub name: String,
    pub parameter_type: Type,
    pub optional: bool,
    pub default_value: Option<Expression>,
}
