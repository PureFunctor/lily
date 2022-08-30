use smol_str::SmolStr;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct LesserPattern {
    pub begin: usize,
    pub end: usize,
    pub kind: LesserPatternK,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum LesserPatternK {
    Null,
    Variable(SmolStr),
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct GreaterPattern {
    pub begin: usize,
    pub end: usize,
    pub kind: GreaterPatternK,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum GreaterPatternK {
    Application(Vec<GreaterPattern>),
    BinaryOperator(Box<GreaterPattern>, SmolStr, Box<GreaterPattern>),
    Constructor(SmolStr),
    Integer(SmolStr),
    Null,
    Parenthesized(Box<GreaterPattern>),
    Variable(SmolStr),
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Expression {
    pub begin: usize,
    pub end: usize,
    pub kind: ExpressionK,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum ExpressionK {
    Application(Vec<Expression>),
    BinaryOperator(Box<Expression>, SmolStr, Box<Expression>),
    Constructor(SmolStr),
    Integer(SmolStr),
    Float(SmolStr),
    Parenthesized(Box<Expression>),
    Variable(SmolStr),
}
