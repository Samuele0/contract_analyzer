use crate::evm_types::{StackValue, StackValue::*};
use ethereum_types::U256;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use z3_sys::*;

impl StackValue {
    fn simplify_sys(&self) {
        unsafe {
            let config = Z3_mk_config();
            let context = Z3_mk_context(config);
            let mut vars = Vec::new();
            let original_ast = self.sv_to_z3(context, &mut vars);
            let simplified = Z3_simplify(context, original_ast);
            let new_sv = Self::form_z3(simplified, context, &mut vars);
            println!("new SV: {:?}", new_sv);
        }
    }
    unsafe fn sv_to_z3(&self, ctx: Z3_context, symbol_map: &mut Vec<StackValue>) -> Z3_ast {
        match self {
            ActualValue(v) => {
                let c_string = CString::new(v.to_string()).unwrap();
                let sort = Z3_mk_bv_sort(ctx, 256);
                let ast = Z3_mk_numeral(ctx, c_string.as_ptr(), sort);
                println!("Value: {:?}", Z3_ast_to_string(ctx, ast));
                ast
            }
            And(a, b) => {
                let s1 = a.sv_to_z3(ctx, symbol_map);
                let s2 = b.sv_to_z3(ctx, symbol_map);
                Z3_mk_bvand(ctx, s1, s2)
            }
            sv => {
                symbol_map.push(sv.clone());
                let symb = Z3_mk_int_symbol(ctx, (symbol_map.len() - 1) as i32);
                let sort = Z3_mk_bv_sort(ctx, 256);

                Z3_mk_const(ctx, symb, sort)
            }
        }
    }

    unsafe fn form_z3(ast: Z3_ast, ctx: Z3_context, symbol_map: &Vec<StackValue>) -> StackValue {
        if Z3_is_app(ctx, ast) {
            let app = Z3_to_app(ctx, ast);
            let argn = Z3_get_app_num_args(ctx, app);
            println!("args:{}", argn);
            let mut buffer = Vec::with_capacity(argn as usize);
            for i in 0..argn {
                buffer.push(Self::form_z3(Z3_get_app_arg(ctx, app, i), ctx, symbol_map));
            }
            let dec = Z3_get_app_decl(ctx, app);
            let kind = Z3_get_decl_kind(ctx, dec);
            println!("name: {:?}", kind);
            let stri = Z3_ast_to_string(ctx, ast);
            let stri_rs = CStr::from_ptr(stri).to_str().unwrap();
            println!("to_string: {}", stri_rs);

            if let DeclKind::UNINTERPRETED = kind {
                let symbol = Z3_get_decl_name(ctx, dec);
                let sv = Z3_get_symbol_int(ctx, symbol);
                println!("Found variable {}; map: {:?}", sv, symbol_map);
                return symbol_map[sv as usize].clone();
            } else if let DeclKind::BNUM = kind {
                println!("Parsing Number");
                let number = parse_z3_num(stri_rs);
                println!("Number: {}", number);
                return ActualValue(number);
            } else {
                return make_sv(kind, buffer);
            }
        }
        Unknown
    }
}
fn make_sv(kind: DeclKind, buffer: Vec<StackValue>) -> StackValue {
    match kind {
        DeclKind::BAND => And(Box::from(buffer[0].clone()), Box::from(buffer[1].clone())),
        _ => Unknown,
    }
}
fn parse_z3_num(num: &str) -> U256 {
    let mut chars = num.chars();
    chars.next().expect("At least 2 chars");
    let base = chars.next().expect("At least 2 chars");
    match base {
        'x' => {
            let mut buffer = U256::from(0);
            for digit in chars {
                buffer *= 16;
                buffer += U256::from(u32::from_str_radix(&digit.to_string(), 16).unwrap());
            }
            return buffer;
        }
        'b' => {
            let mut buffer = U256::from(0);
            for digit in chars {
                buffer *= 2;
                buffer += U256::from(u32::from_str_radix(&digit.to_string(), 2).unwrap());
            }
            return buffer;
        }
        _ => panic!("Base not implemented"),
    };
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
        value.simplify_sys();
    }
}
