type Be = Box<Expression>;

enum Expression {
    Add(Be, Be),
    Sub(Be, Be),
    Mul(Be, Be),
    Div(Be, Be),
    Exp(Be, Be),
    // No `not` in Syntax, use - instead, as it looks more like the Logical Negation ¬
    Neg(Be),
    Value(Value),
}

enum Value {
    Float(f64),
    Integer(f64),
    // TODO...
}
