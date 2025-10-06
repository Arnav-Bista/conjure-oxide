use tree_morph::prelude::*;
use uniplate::Uniplate;

/// Initialize tracing subscriber for tests to see trace output
fn init_tracing() {
    use std::sync::Once;
    static INIT: Once = Once::new();

    INIT.call_once(|| {
        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::TRACE)
            .with_test_writer()
            .init();
    });
}

#[derive(Debug, Clone, PartialEq, Eq, Uniplate)]
#[uniplate()]
enum Expr {
    A,
    B,
    C,
    D,
}

fn rule_b_to_c(_: &mut Commands<Expr, ()>, expr: &Expr, _: &()) -> Option<Expr> {
    if let Expr::B = expr {
        return Some(Expr::C);
    }
    None
}

#[test]
fn closure_rules() {
    init_tracing();
    let expr = Expr::A;

    let engine = EngineBuilder::new()
        .add_rule(
            // Same as macro expansion
            (|_, t, _| match t {
                Expr::A => Some(Expr::B),
                _ => None,
            }) as RuleFn<_, _>,
        )
        .add_rule_group(rule_fns![
            |_, t, _| match t {
                Expr::C => Some(Expr::D),
                _ => None,
            },
            rule_b_to_c,
        ])
        .build();

    let (result, _) = engine.morph(expr, ());

    assert_eq!(result, Expr::D);
}
