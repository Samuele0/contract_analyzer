use crate::evm_types::StackValue;
use std::collections::HashMap;
//use ethereum_types::U256;
use z3::ast::{Ast, Bool, Int, BV};
use z3::{Config, Context,FuncDecl};

enum Z3Tree<'a> {
    Int(BV<'a>),
    Bool(Bool<'a>),
}
impl<'a> Z3Tree<'a> {
    fn expect_int(self) -> BV<'a> {
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
        let z3t = self
            .to_z3_tree(&ctx, &mut sv_ctx, &mut counter)
            .expect_int();
        println!("tree: {}", z3t);

        let simplified = z3t.simplify();
        println!("Simplified: {}", simplified );
        let sys_tree= simplified.get_z3_ast();
        let sys_context= sys_tree
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
                let val = Int::from_str(ctx, &x.to_string()[..]).unwrap().to_ast(256);
                println!("ActualValue: {}", val);
                Z3Tree::Int(val)
            }
            StackValue::Add(a, b) => {
                let i1 = a.to_z3_tree(ctx, sv_ctx, counter).expect_int();
                let i2 = b.to_z3_tree(ctx, sv_ctx, counter).expect_int();
                let val = i1.bvadd(&i2);
                println!("Add: {}", val);
                Z3Tree::Int(val)
            }
            StackValue::Mul(a, b) => {
                let i1 = a.to_z3_tree(ctx, sv_ctx, counter).expect_int();
                let i2 = b.to_z3_tree(ctx, sv_ctx, counter).expect_int();
                Z3Tree::Int(i1.bvmul(&i2))
            }
            StackValue::Sub(a, b) => {
                let i1 = a.to_z3_tree(ctx, sv_ctx, counter).expect_int();
                let i2 = b.to_z3_tree(ctx, sv_ctx, counter).expect_int();
                Z3Tree::Int(i1.bvsub(&i2))
            }
            StackValue::Div(a, b) => {
                let i1 = a.to_z3_tree(ctx, sv_ctx, counter).expect_int();
                let i2 = b.to_z3_tree(ctx, sv_ctx, counter).expect_int();
                Z3Tree::Int(i1.bvudiv(&i2))
            }
            StackValue::Mod(a, b) => {
                let i1 = a.to_z3_tree(ctx, sv_ctx, counter).expect_int();
                let i2 = b.to_z3_tree(ctx, sv_ctx, counter).expect_int();
                Z3Tree::Int(i1.bvurem(&i2))
            }
            StackValue::Exp(a, b) => {
                let i1 = a
                    .to_z3_tree(ctx, sv_ctx, counter)
                    .expect_int()
                    .to_int(false);
                let i2 = b
                    .to_z3_tree(ctx, sv_ctx, counter)
                    .expect_int()
                    .to_int(false);
                Z3Tree::Int(i1.power(&i2).to_ast(256))
            }
            StackValue::LT(a, b) => {
                let i1 = a.to_z3_tree(ctx, sv_ctx, counter).expect_int();
                let i2 = b.to_z3_tree(ctx, sv_ctx, counter).expect_int();
                Z3Tree::Bool(i1.bvult(&i2))
            }
            StackValue::GT(a, b) => {
                let i1 = a.to_z3_tree(ctx, sv_ctx, counter).expect_int();
                let i2 = b.to_z3_tree(ctx, sv_ctx, counter).expect_int();
                Z3Tree::Bool(i1.bvugt(&i2))
            }
            StackValue::EQ(a, b) => {
                let i1 = a.to_z3_tree(ctx, sv_ctx, counter).expect_int();
                let i2 = b.to_z3_tree(ctx, sv_ctx, counter).expect_int();
                Z3Tree::Bool(i1._eq(&i2))
            }
            StackValue::Shr(a, b) => {
                let i1 = a.to_z3_tree(ctx, sv_ctx, counter).expect_int();
                let i2 = b.to_z3_tree(ctx, sv_ctx, counter).expect_int();
                Z3Tree::Int(i1.bvlshr(&i2))
            }
            StackValue::ShL(a, b) => {
                let i1 = a.to_z3_tree(ctx, sv_ctx, counter).expect_int();
                let i2 = b.to_z3_tree(ctx, sv_ctx, counter).expect_int();
                Z3Tree::Int(i1.bvshl(&i2))
            }
            StackValue::And(a,b)=>{
                let i1 = a.to_z3_tree(ctx, sv_ctx, counter).expect_int();
                let i2 = b.to_z3_tree(ctx, sv_ctx, counter).expect_int();
                let val = i1.bvand(&i2);
                println!("AND: {}",val);
                Z3Tree::Int(val)
            }
            _ => {
                let name = format!("x{}", *counter);
                *counter += 1;
                sv_ctx.insert(name.clone(), self.clone());
                Z3Tree::Int(BV::new_const(ctx, name, 256))
            }
        }
    }

    fn parse_z3_tree(tree: &str) {}
}

#[cfg(test)]
mod tests {
    use crate::evm_types::StackValue;
    use ethereum_types::U256;
    use z3::ast::{Ast, Int};
    use z3::{Config, Context};
    #[test]
    fn test1() {
        let ctx = Context::new(&Config::new());
        let x = Int::new_const(&ctx, "x");
        let y = Int::from_i64(&ctx, 25);
        let z = Int::from_i64(&ctx, 10);
        let tree = Int::add(&ctx, &vec![&x, &y, &z][..]);
        println!("{}", tree.simplify())
    }
    #[test]
    fn sv_test() {
        let value = StackValue::And(
            Box::from(StackValue::ActualValue(U256::from(256))),
            Box::from(StackValue::And(
                Box::from(StackValue::ActualValue(U256::from(256))),
                Box::from(StackValue::CallValue),
            )),
        );
        value.simplify();
    }
}
