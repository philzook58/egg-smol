use crate::ast::{Expr, Rewrite};
use crate::HashMap;
use crate::Symbol;
use std::iter::zip;
fn expr_match<'a>(pat: &Expr, e: &'a Expr, env: &mut HashMap<Symbol, &'a Expr>) -> bool {
    match (pat, e) {
        (Expr::Lit(l), Expr::Lit(l2)) if l == l2 => true,
        (Expr::Var(x), _) => None == env.insert(*x, e),
        (Expr::Call(f, args), Expr::Call(f1, args1)) if f == f1 && args.len() == args1.len() => {
            for (pat, e) in zip(args, args1) {
                if !expr_match(pat, e, env) {
                    return false;
                }
            }
            true
        }
        _ => false,
    }
}
fn expr_rewrite(r: Rewrite, e: &Expr) -> Option<Expr> {
    None
}

fn macro_expand(rules: Vec<Rewrite>, e: Expr) -> Expr {
    for rule in rules {
        if let Some(e1) = expr_rewrite(rule, &e) {
            let e = e1;
        }
    }
    e
}
