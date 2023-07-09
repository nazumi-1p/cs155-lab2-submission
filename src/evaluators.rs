use crate::ast;

use ast::Expr;
use ast::ArithExpr;
use ast::BoolExpr;
use ast::Value;

use ast::ArithCmpOp::*;
use ast::ArithExpr::*;
use ast::BinArithOp::*;
use ast::BinLogicOp::*;
use ast::BoolExpr::*;
use ast::Expr::*;
use ast::Value::*;

/* REQUIRED FUNCTIONS */
pub fn eval(expr: Expr) -> Value {
   match expr {
        ArithExpr(arith_expr)   =>  IntValue(eval_arith_expr(arith_expr)),
        BoolExpr(bool_expr)     =>  BoolValue(eval_bool_expr(bool_expr)),
   }
}

pub fn eval_arith_expr(arith_expr: ArithExpr) -> i64 {
    match arith_expr{
        IntLit(num) => num,
        BinArithExpr{left, right, op}   =>  {
            match op{
                AddOp => eval_arith_expr(*left) + eval_arith_expr(*right),
                SubOp => eval_arith_expr(*left) - eval_arith_expr(*right),
                MulOp => eval_arith_expr(*left) * eval_arith_expr(*right),
                IntDivOp => eval_arith_expr(*left) / eval_arith_expr(*right),
            }
        }
    }
}

pub fn eval_bool_expr(bool_expr: BoolExpr) -> bool {
    match bool_expr{
        ArithCmpExpr{left, right, op} => {
            match op{
                LtOp        => eval_arith_expr(*left) < eval_arith_expr(*right),
                LteOp       => eval_arith_expr(*left) <= eval_arith_expr(*right),
                GtOp        => eval_arith_expr(*left) > eval_arith_expr(*right),
                GteOp       => eval_arith_expr(*left) >= eval_arith_expr(*right),
                ArithEqOp   => eval_arith_expr(*left) == eval_arith_expr(*right),
                ArithNeqOp  => eval_arith_expr(*left) != eval_arith_expr(*right),
            }
        },
        BinBoolExpr{left, right, op} => {
            match op{
                AndOp       => eval_bool_expr(*left) && eval_bool_expr(*right),
                OrOp        => eval_bool_expr(*left) || eval_bool_expr(*right),
                BoolEqOp    => eval_bool_expr(*left) == eval_bool_expr(*right),
                BoolNeqOp   => eval_bool_expr(*left) != eval_bool_expr(*right),
            }
        },
        NotExpr(expr) => !eval_bool_expr(*expr),
        BoolLit(bool_val) => bool_val,
    }
}