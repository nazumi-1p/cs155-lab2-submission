/*use ArithCmpOp::*;
use ArithExpr::*;
use BinArithOp::*;
use BinLogicOp::*;
use BoolExpr::*;
use Expr::*;
use Value::*;*/

pub enum Expr {
    ArithExpr(ArithExpr),
    BoolExpr(BoolExpr),
}

pub enum ArithExpr {
    BinArithExpr {
        left: Box<ArithExpr>,
        right: Box<ArithExpr>,
        op: BinArithOp,
    },
    IntLit(i64),
}

pub enum BoolExpr {
    ArithCmpExpr {
        left: Box<ArithExpr>,
        right: Box<ArithExpr>,
        op: ArithCmpOp,
    },
    BinBoolExpr {
        left: Box<BoolExpr>,
        right: Box<BoolExpr>,
        op: BinLogicOp,
    },
    NotExpr(Box<BoolExpr>),
    BoolLit(bool),
}

pub enum BinArithOp {
    AddOp,
    SubOp,
    MulOp,
    IntDivOp,
}

pub enum ArithCmpOp {
    LtOp,
    LteOp,
    GtOp,
    GteOp,
    ArithEqOp,
    ArithNeqOp,
}

pub enum BinLogicOp {
    AndOp,
    OrOp,
    BoolEqOp,
    BoolNeqOp,
}

#[derive(Debug, PartialEq)]
pub enum Value {
    BoolValue(bool),
    IntValue(i64),
}