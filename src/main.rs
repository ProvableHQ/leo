extern crate pest;
#[macro_use]
extern crate pest_derive;

extern crate from_pest;
#[macro_use]
extern crate pest_ast;

#[macro_use]
extern crate lazy_static;

use pest::Parser;
use std::fs;

#[derive(Parser)]
#[grammar = "language.pest"]
pub struct LanguageParser;

mod ast {
    use from_pest::ConversionError;
    use from_pest::FromPest;
    use from_pest::Void;
    use pest::iterators::{Pair, Pairs};
    use pest::prec_climber::{Assoc, Operator, PrecClimber};
    use pest::Span;
    use pest_ast::FromPest;
    use super::Rule;

    fn span_into_string(span: Span) -> String {
        span.as_str().to_string()
    }

    lazy_static! {
        static ref PRECEDENCE_CLIMBER: PrecClimber<Rule> = precedence_climber();
    }

    fn precedence_climber() -> PrecClimber<Rule> {
        PrecClimber::new(vec![
            Operator::new(Rule::operation_or, Assoc::Left),
            Operator::new(Rule::operation_and, Assoc::Left),
            Operator::new(Rule::operation_eq, Assoc::Left)
                | Operator::new(Rule::operation_neq, Assoc::Left),
            Operator::new(Rule::operation_geq, Assoc::Left)
                | Operator::new(Rule::operation_gt, Assoc::Left)
                | Operator::new(Rule::operation_leq, Assoc::Left)
                | Operator::new(Rule::operation_lt, Assoc::Left),
            Operator::new(Rule::operation_add, Assoc::Left)
                | Operator::new(Rule::operation_sub, Assoc::Left),
            Operator::new(Rule::operation_mul, Assoc::Left)
                | Operator::new(Rule::operation_div, Assoc::Left),
            Operator::new(Rule::operation_pow, Assoc::Left),
        ])
    }

    fn parse_term(pair: Pair<Rule>) -> Box<Expression> {
        Box::new(match pair.as_rule() {
            Rule::expression_term => {
                let clone = pair.clone();
                let next = clone.into_inner().next().unwrap();
                match next.as_rule() {
                    Rule::expression_primitive => {
                        let next = next.into_inner().next().unwrap();
                        match next.as_rule() {
                            Rule::value => Expression::Value(
                                Value::from_pest(&mut pair.into_inner().next().unwrap().into_inner()).unwrap()
                            ),
                            Rule::variable => Expression::Variable(
                                Variable::from_pest(&mut pair.into_inner().next().unwrap().into_inner()).unwrap(),
                            ),
                            rule => unreachable!("`expression_primitive` should contain one of [`value`, `variable`], found {:#?}", rule)
                        }
                    }
                    Rule::expression_not => {
                        let span = next.as_span();
                        let mut inner = next.into_inner();
                        let operation = match inner.next().unwrap().as_rule() {
                            Rule::operation_pre_not => Not::from_pest(&mut pair.into_inner().next().unwrap().into_inner()).unwrap(),
                            rule => unreachable!("`expression_not` should yield `operation_pre_not`, found {:#?}", rule)
                        };
                        let expression = parse_term(inner.next().unwrap());
                        Expression::Not(NotExpression { operation, expression, span })
                    },
                    Rule::expression => Expression::from_pest(&mut pair.into_inner()).unwrap(), // Parenthesis case

                    // Rule::expression_increment => {
                    //     let span = next.as_span();
                    //     let mut inner = next.into_inner();
                    //     let expression = parse_expression_term(inner.next().unwrap());
                    //     let operation = match inner.next().unwrap().as_rule() {
                    //         Rule::operation_post_increment => Increment::from_pest(&mut pair.into_inner().next().unwrap().into_inner()).unwrap(),
                    //         rule => unreachable!("`expression_increment` should yield `operation_post_increment`, found {:#?}", rule)
                    //     };
                    //     Expression::Increment(IncrementExpression { operation, expression, span })
                    // },
                    // Rule::expression_decrement => {
                    //     let span = next.as_span();
                    //     let mut inner = next.into_inner();
                    //     let expression = parse_expression_term(inner.next().unwrap());
                    //     let operation = match inner.next().unwrap().as_rule() {
                    //         Rule::operation_post_decrement => Decrement::from_pest(&mut pair.into_inner().next().unwrap().into_inner()).unwrap(),
                    //         rule => unreachable!("`expression_decrement` should yield `operation_post_decrement`, found {:#?}", rule)
                    //     };
                    //     Expression::Decrement(DecrementExpression { operation, expression, span })
                    // },

                    rule => unreachable!("`term` should contain one of ['value', 'variable', 'expression', 'expression_not', 'expression_increment', 'expression_decrement'], found {:#?}", rule)
                }
            }
            rule => unreachable!("`parse_expression_term` should be invoked on `Rule::expression_term`, found {:#?}", rule),
        })
    }

    fn binary_expression<'ast>(
        lhs: Box<Expression<'ast>>,
        pair: Pair<'ast, Rule>,
        rhs: Box<Expression<'ast>>,
    ) -> Box<Expression<'ast>> {
        let (start, _) = lhs.span().clone().split();
        let (_, end) = rhs.span().clone().split();
        let span = start.span(&end);

        Box::new(match pair.as_rule() {
            Rule::operation_or => Expression::binary(BinaryOperator::Or, lhs, rhs, span),
            Rule::operation_and => Expression::binary(BinaryOperator::And, lhs, rhs, span),
            Rule::operation_eq => Expression::binary(BinaryOperator::Eq, lhs, rhs, span),
            Rule::operation_neq => Expression::binary(BinaryOperator::Neq, lhs, rhs, span),
            Rule::operation_geq => Expression::binary(BinaryOperator::Geq, lhs, rhs, span),
            Rule::operation_gt => Expression::binary(BinaryOperator::Gt, lhs, rhs, span),
            Rule::operation_leq => Expression::binary(BinaryOperator::Leq, lhs, rhs, span),
            Rule::operation_lt => Expression::binary(BinaryOperator::Lt, lhs, rhs, span),
            Rule::operation_add => Expression::binary(BinaryOperator::Add, lhs, rhs, span),
            Rule::operation_sub => Expression::binary(BinaryOperator::Sub, lhs, rhs, span),
            Rule::operation_mul => Expression::binary(BinaryOperator::Mul, lhs, rhs, span),
            Rule::operation_div => Expression::binary(BinaryOperator::Div, lhs, rhs, span),
            Rule::operation_pow => Expression::binary(BinaryOperator::Pow, lhs, rhs, span),
            _ => unreachable!(),
        })
    }

    #[derive(Clone, Debug, FromPest, PartialEq)]
    #[pest_ast(rule(Rule::file))]
    pub struct File<'ast> {
        pub statement: Vec<Statement<'ast>>,
        pub eoi: EOI,
        #[pest_ast(outer())]
        pub span: Span<'ast>,
    }

    // Visibility

    #[derive(Clone, Debug, FromPest, PartialEq)]
    #[pest_ast(rule(Rule::visibility_public))]
    pub struct Public {}

    #[derive(Clone, Debug, FromPest, PartialEq)]
    #[pest_ast(rule(Rule::visibility_private))]
    pub struct Private {}

    #[derive(Clone, Debug, FromPest, PartialEq)]
    #[pest_ast(rule(Rule::visibility))]
    pub enum Visibility {
        Public(Public),
        Private(Private),
    }

    // Unary Operations

    #[derive(Clone, Debug, FromPest, PartialEq)]
    #[pest_ast(rule(Rule::operation_pre_not))]
    pub struct Not<'ast> {
        #[pest_ast(outer())]
        pub span: Span<'ast>,
    }

    #[derive(Clone, Debug, FromPest, PartialEq)]
    #[pest_ast(rule(Rule::operation_post_increment))]
    pub struct Increment<'ast> {
        #[pest_ast(outer())]
        pub span: Span<'ast>,
    }

    #[derive(Clone, Debug, FromPest, PartialEq)]
    #[pest_ast(rule(Rule::operation_post_decrement))]
    pub struct Decrement<'ast> {
        #[pest_ast(outer())]
        pub span: Span<'ast>,
    }

    // Binary Operations

    #[derive(Debug, PartialEq, Clone)]
    pub enum BinaryOperator {
        Or,
        And,
        Eq,
        Neq,
        Geq,
        Gt,
        Leq,
        Lt,
        Add,
        Sub,
        Mul,
        Div,
        Pow,
    }

    // Values

    #[derive(Clone, Debug, FromPest, PartialEq)]
    #[pest_ast(rule(Rule::value_boolean))]
    pub struct Boolean<'ast> {
        #[pest_ast(outer(with(span_into_string)))]
        pub value: String,
        #[pest_ast(outer())]
        pub span: Span<'ast>,
    }

    #[derive(Clone, Debug, FromPest, PartialEq)]
    #[pest_ast(rule(Rule::value_field))]
    pub struct Field<'ast> {
        #[pest_ast(outer(with(span_into_string)))]
        pub value: String,
        #[pest_ast(outer())]
        pub span: Span<'ast>,
    }

    #[derive(Clone, Debug, FromPest, PartialEq)]
    #[pest_ast(rule(Rule::value))]
    pub enum Value<'ast> {
        Boolean(Boolean<'ast>),
        Field(Field<'ast>),
    }

    impl<'ast> Value<'ast> {
        pub fn span(&self) -> &Span<'ast> {
            match self {
                Value::Boolean(value) => &value.span,
                Value::Field(value) => &value.span,
            }
        }
    }

    // Variables

    #[derive(Clone, Debug, FromPest, PartialEq)]
    #[pest_ast(rule(Rule::variable))]
    pub struct Variable<'ast> {
        #[pest_ast(outer(with(span_into_string)))]
        pub value: String,
        #[pest_ast(outer())]
        pub span: Span<'ast>,
    }

    // Expressions

    #[derive(Clone, Debug, FromPest, PartialEq)]
    #[pest_ast(rule(Rule::expression_not))]
    pub struct NotExpression<'ast> {
        pub operation: Not<'ast>,
        pub expression: Box<Expression<'ast>>,
        #[pest_ast(outer())]
        pub span: Span<'ast>,
    }

    // #[derive(Clone, Debug, FromPest, PartialEq)]
    // #[pest_ast(rule(Rule::expression_increment))]
    // pub struct IncrementExpression<'ast> {
    //     pub expression: Box<Expression<'ast>>,
    //     pub operation: Increment<'ast>,
    //     #[pest_ast(outer())]
    //     pub span: Span<'ast>,
    // }
    //
    // #[derive(Clone, Debug, FromPest, PartialEq)]
    // #[pest_ast(rule(Rule::expression_decrement))]
    // pub struct DecrementExpression<'ast> {
    //     pub expression: Box<Expression<'ast>>,
    //     pub operation: Decrement<'ast>,
    //     #[pest_ast(outer())]
    //     pub span: Span<'ast>,
    // }

    #[derive(Clone, Debug, PartialEq)]
    pub struct BinaryExpression<'ast> {
        pub operation: BinaryOperator,
        pub left: Box<Expression<'ast>>,
        pub right: Box<Expression<'ast>>,
        pub span: Span<'ast>,
    }

    #[derive(Clone, Debug, PartialEq)]
    pub enum Expression<'ast> {
        Value(Value<'ast>),
        Variable(Variable<'ast>),
        Not(NotExpression<'ast>),
        Binary(BinaryExpression<'ast>),

        // Increment(IncrementExpression<'ast>),
        // Decrement(DecrementExpression<'ast>),
    }

    impl<'ast> Expression<'ast> {
        pub fn binary(
            operation: BinaryOperator,
            left: Box<Expression<'ast>>,
            right: Box<Expression<'ast>>,
            span: Span<'ast>,
        ) -> Self {
            Expression::Binary(BinaryExpression { operation, left, right, span })
        }

        pub fn span(&self) -> &Span<'ast> {
            match self {
                Expression::Value(expression) => &expression.span(),
                Expression::Variable(expression) => &expression.span,
                Expression::Not(expression) => &expression.span,
                Expression::Binary(expression) => &expression.span,

                // Expression::Increment(expression) => &expression.span,
                // Expression::Decrement(expression) => &expression.span,
            }
        }
    }

    impl<'ast> FromPest<'ast> for Expression<'ast> {
        type Rule = Rule;
        type FatalError = Void;

        fn from_pest(pest: &mut Pairs<'ast, Rule>) -> Result<Self, ConversionError<Void>> {
            let mut clone = pest.clone();
            let pair = clone.next().ok_or(::from_pest::ConversionError::NoMatch)?;
            match pair.as_rule() {
                Rule::expression => {
                    // Transfer iterated state to pest.
                    *pest = clone;
                    Ok(*PRECEDENCE_CLIMBER.climb(pair.into_inner(), parse_term, binary_expression))
                }
                _ => Err(ConversionError::NoMatch),
            }
        }
    }

    // Statements

    #[derive(Debug, FromPest, PartialEq, Clone)]
    #[pest_ast(rule(Rule::statement_assign))]
    pub struct AssignStatement<'ast> {
        pub variable: Variable<'ast>,
        pub expression: Expression<'ast>,
        #[pest_ast(outer())]
        pub span: Span<'ast>,
    }

    #[derive(Debug, FromPest, PartialEq, Clone)]
    #[pest_ast(rule(Rule::statement_return))]
    pub struct ReturnStatement<'ast> {
        pub expressions: Vec<Expression<'ast>>,
        #[pest_ast(outer())]
        pub span: Span<'ast>,
    }

    #[derive(Clone, Debug, FromPest, PartialEq)]
    #[pest_ast(rule(Rule::statement))]
    pub enum Statement<'ast> {
        Assign(AssignStatement<'ast>),
        Return(ReturnStatement<'ast>),
    }

    // Utilities

    #[derive(Debug, FromPest, PartialEq, Clone)]
    #[pest_ast(rule(Rule::EOI))]
    pub struct EOI;
}

fn main() {
    use crate::from_pest::FromPest;
    use snarkos_gadgets::curves::edwards_bls12::FqGadget;
    use snarkos_models::gadgets::{r1cs::{ConstraintSystem, TestConstraintSystem, Fr}, utilities::{alloc::AllocGadget, boolean::Boolean}};

    let unparsed_file = fs::read_to_string("simple.program").expect("cannot read file");
    let mut file = LanguageParser::parse(Rule::file, &unparsed_file).expect("unsuccessful parse");
    let syntax_tree = ast::File::from_pest(&mut file).expect("infallible");

    for statement in syntax_tree.statement {
        match statement {
            ast::Statement::Assign(statement) => {
                println!("{:#?}", statement);
            },
            ast::Statement::Return(statement) => {

            }
        }
    }

    let mut cs = TestConstraintSystem::<Fr>::new();

    Boolean::alloc(cs.ns(|| format!("boolean")), || Ok(true));


    println!("\n\n number of constraints for input: {}", cs.num_constraints());


    // for token in file.into_inner() {
    //     match token.as_rule() {
    //         Rule::statement => println!("{:?}", token.into_inner()),
    //         Rule::EOI => println!("END"),
    //         _ => println!("{:?}", token.into_inner()),
    //     }
    //     // println!("{:?}", token);
    // }


    // let mut field_sum: f64 = 0.0;
    // let mut record_count: u64 = 0;
    //
    // for record in file.into_inner() {
    //     match record.as_rule() {
    //         Rule::record => {
    //             record_count += 1;
    //
    //             for field in record.into_inner() {
    //                 field_sum += field.as_str().parse::<f64>().unwrap();
    //             }
    //         }
    //         Rule::EOI => (),
    //         _ => unreachable!(),
    //     }
    // }

    // println!("Sum of fields: {}", field_sum);
    // println!("Number of records: {}", record_count);

    // let successful_parse = LanguageParser::parse(Rule::value, "-273");
    // println!("{:?}", successful_parse);

    // let unsuccessful_parse = CSVParser::parse(Rule::field, "this is not a number");
    // println!("{:?}", unsuccessful_parse);
}
