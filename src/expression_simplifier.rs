use crate::evm_types::StackValue;
use std::collections::HashMap;
//use ethereum_types::U256;
use z3::ast::{Ast, Bool, Int};
use z3::{Config, Context};

enum Z3Tree<'a> {
    Int(Int<'a>),
    Bool(Bool<'a>),
}
impl<'a> Z3Tree<'a> {
    fn expect_int(self) -> Int<'a> {
        if let Z3Tree::Int(i) = self {
            return i;
        }
        panic!()
    }
}
impl StackValue {
    pub fn simplify(&self) -> StackValue {
        let ctx = Context::new(&Config::new());
        let mut sv_ctx = HashMap::new();
        let mut counter = 0;
        let z3t = self.to_z3_tree(&ctx, &mut sv_ctx, &mut counter);
        println!("Simplified: {}", z3t.expect_int().simplify());
        StackValue::CallValue
    }
    fn to_z3_tree<'a>(
        &self,
        ctx: &'a Context,
        sv_ctx: &mut HashMap<String, StackValue>,
        counter: &mut u32,
    ) -> Z3Tree<'a> {
        match self {
            StackValue::ActualValue(x) => {
                Z3Tree::Int(Int::from_str(ctx, &x.to_string()[..]).unwrap())
            }
            StackValue::Add(a, b) => {
                let i1 = a.to_z3_tree(ctx, sv_ctx, counter).expect_int();
                let i2 = b.to_z3_tree(ctx, sv_ctx, counter).expect_int();
                Z3Tree::Int(Int::add(ctx, &vec![&i1, &i2]))
            }
            StackValue::Mul(a, b) => {
                let i1 = a.to_z3_tree(ctx, sv_ctx, counter).expect_int();
                let i2 = b.to_z3_tree(ctx, sv_ctx, counter).expect_int();
                Z3Tree::Int(Int::mul(ctx, &vec![&i1, &i2]))
            }
            StackValue::Sub(a, b) => {
                let i1 = a.to_z3_tree(ctx, sv_ctx, counter).expect_int();
                let i2 = b.to_z3_tree(ctx, sv_ctx, counter).expect_int();
                Z3Tree::Int(Int::sub(ctx, &vec![&i1, &i2]))
            }
            StackValue::Div(a, b) => {
                let i1 = a.to_z3_tree(ctx, sv_ctx, counter).expect_int();
                let i2 = b.to_z3_tree(ctx, sv_ctx, counter).expect_int();
                Z3Tree::Int(i1.div(&i2))
            }
            StackValue::Mod(a, b) => {
                let i1 = a.to_z3_tree(ctx, sv_ctx, counter).expect_int();
                let i2 = b.to_z3_tree(ctx, sv_ctx, counter).expect_int();
                Z3Tree::Int(i1.modulo(&i2))
            }
            StackValue::Exp(a, b) => {
                let i1 = a.to_z3_tree(ctx, sv_ctx, counter).expect_int();
                let i2 = b.to_z3_tree(ctx, sv_ctx, counter).expect_int();
                Z3Tree::Int(i1.power(&i2))
            }
            StackValue::LT(a, b) => {
                let i1 = a.to_z3_tree(ctx, sv_ctx, counter).expect_int();
                let i2 = b.to_z3_tree(ctx, sv_ctx, counter).expect_int();
                Z3Tree::Bool(i1.lt(&i2))
            }
            StackValue::GT(a, b) => {
                let i1 = a.to_z3_tree(ctx, sv_ctx, counter).expect_int();
                let i2 = b.to_z3_tree(ctx, sv_ctx, counter).expect_int();
                Z3Tree::Bool(i1.gt(&i2))
            }
            StackValue::EQ(a, b) => {
                let i1 = a.to_z3_tree(ctx, sv_ctx, counter).expect_int();
                let i2 = b.to_z3_tree(ctx, sv_ctx, counter).expect_int();
                Z3Tree::Bool(i1._eq(&i2))
            }
            _ => {
                let name = format!("x{}", *counter);
                *counter += 1;
                sv_ctx.insert(name.clone(), self.clone());
                Z3Tree::Int(Int::new_const(ctx, name))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use z3::ast::{Ast, Int};
    use z3::{Config, Context};
    #[test]
    fn name() {
        let ctx = Context::new(&Config::new());
        let x = Int::new_const(&ctx, "x");
        let y = Int::from_i64(&ctx, 25);
        let z = Int::from_i64(&ctx, 10);
        let tree = Int::add(&ctx, &vec![&x, &y, &z][..]);
        println!("{}", tree.simplify())
    }
}
