mod ast;
mod evaluators;

use ast::ArithCmpOp::*;
use ast::ArithExpr::*;
use ast::BinArithOp::*;
use ast::BinLogicOp::*;
use ast::BoolExpr::*;
use ast::Expr::*;
use ast::Value::*;
use evaluators::*;

fn main() {
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_units() {
    eval_arith_expr(BinArithExpr{left: Box::new(IntLit(4)), right: Box::new(IntLit(2)), op: AddOp});
    eval_arith_expr(BinArithExpr{left: Box::new(IntLit(4)), right: Box::new(IntLit(2)), op: SubOp});
    eval_arith_expr(BinArithExpr{left: Box::new(IntLit(4)), right: Box::new(IntLit(2)), op: MulOp});
    eval_arith_expr(BinArithExpr{left: Box::new(IntLit(4)), right: Box::new(IntLit(2)), op: IntDivOp});
    
    eval_bool_expr(ArithCmpExpr{left: Box::new(IntLit(4)), right: Box::new(IntLit(2)), op: LtOp});
    eval_bool_expr(ArithCmpExpr{left: Box::new(IntLit(4)), right: Box::new(IntLit(2)), op: LteOp});
    eval_bool_expr(ArithCmpExpr{left: Box::new(IntLit(4)), right: Box::new(IntLit(2)), op: GtOp});
    eval_bool_expr(ArithCmpExpr{left: Box::new(IntLit(4)), right: Box::new(IntLit(2)), op: GteOp});
    eval_bool_expr(ArithCmpExpr{left: Box::new(IntLit(4)), right: Box::new(IntLit(2)), op: ArithEqOp});
    eval_bool_expr(ArithCmpExpr{left: Box::new(IntLit(4)), right: Box::new(IntLit(2)), op: ArithNeqOp});

    eval_bool_expr(BinBoolExpr{left: Box::new(BoolLit(true)), right: Box::new(BoolLit(false)), op: AndOp});
    eval_bool_expr(BinBoolExpr{left: Box::new(BoolLit(true)), right: Box::new(BoolLit(false)), op: OrOp});
    eval_bool_expr(BinBoolExpr{left: Box::new(BoolLit(true)), right: Box::new(BoolLit(false)), op: BoolEqOp});
    eval_bool_expr(BinBoolExpr{left: Box::new(BoolLit(true)), right: Box::new(BoolLit(false)), op: BoolNeqOp});

    eval_bool_expr(NotExpr(Box::new(BoolLit(true))));
    eval(ArithExpr(BinArithExpr{left: Box::new(IntLit(4)), right: Box::new(IntLit(2)), op: AddOp}));
    eval(BoolExpr(BinBoolExpr{left: Box::new(BoolLit(true)), right: Box::new(BoolLit(false)), op: AndOp}));
    }


    #[test]
    fn test_sample() {
        let expr = BoolExpr(BoolLit(true));
        let answer = BoolValue(true);   

        assert_eq!(eval(expr), answer);  // eval(BoolExpr(BoolLit(true))) == BoolValue(true)
    }

    #[test]
    fn test_others() {
        main();
        println!("{:?}", BoolValue(true));
    }
}