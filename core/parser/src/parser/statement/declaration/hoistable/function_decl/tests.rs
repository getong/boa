use crate::parser::tests::check_script_parser;
use boa_ast::{
    function::{FormalParameterList, FunctionBody, FunctionDeclaration},
    Declaration,
};
use boa_interner::Interner;
use boa_macros::utf16;

const PSEUDO_LINEAR_POS: boa_ast::LinearPosition = boa_ast::LinearPosition::new(0);
const EMPTY_LINEAR_SPAN: boa_ast::LinearSpan =
    boa_ast::LinearSpan::new(PSEUDO_LINEAR_POS, PSEUDO_LINEAR_POS);

/// Function declaration parsing.
#[test]
fn function_declaration() {
    let interner = &mut Interner::default();
    check_script_parser(
        "function hello() {}",
        vec![Declaration::FunctionDeclaration(FunctionDeclaration::new(
            interner
                .get_or_intern_static("hello", utf16!("hello"))
                .into(),
            FormalParameterList::default(),
            FunctionBody::default(),
            EMPTY_LINEAR_SPAN,
        ))
        .into()],
        interner,
    );
}

/// Function declaration parsing with keywords.
#[test]
fn function_declaration_keywords() {
    macro_rules! genast {
        ($keyword:literal, $interner:expr) => {
            vec![Declaration::FunctionDeclaration(FunctionDeclaration::new(
                $interner
                    .get_or_intern_static($keyword, utf16!($keyword))
                    .into(),
                FormalParameterList::default(),
                FunctionBody::default(),
                EMPTY_LINEAR_SPAN,
            ))
            .into()]
        };
    }

    let interner = &mut Interner::default();
    let ast = genast!("yield", interner);
    check_script_parser("function yield() {}", ast, interner);

    let interner = &mut Interner::default();
    let ast = genast!("await", interner);
    check_script_parser("function await() {}", ast, interner);

    let interner = &mut Interner::default();
    let ast = genast!("as", interner);
    check_script_parser("function as() {}", ast, interner);

    let interner = &mut Interner::default();
    let ast = genast!("async", interner);
    check_script_parser("function async() {}", ast, interner);

    let interner = &mut Interner::default();
    let ast = genast!("from", interner);
    check_script_parser("function from() {}", ast, interner);

    let interner = &mut Interner::default();
    let ast = genast!("get", interner);
    check_script_parser("function get() {}", ast, interner);

    let interner = &mut Interner::default();
    let ast = genast!("meta", interner);
    check_script_parser("function meta() {}", ast, interner);

    let interner = &mut Interner::default();
    let ast = genast!("of", interner);
    check_script_parser("function of() {}", ast, interner);

    let interner = &mut Interner::default();
    let ast = genast!("set", interner);
    check_script_parser("function set() {}", ast, interner);

    let interner = &mut Interner::default();
    let ast = genast!("target", interner);
    check_script_parser("function target() {}", ast, interner);
}
