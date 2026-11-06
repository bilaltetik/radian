use crate::lexer::{core::tokenize, types::*};
use crate::parser::{core::parse, types::*};

// ---------- yardımcılar ----------

fn parse_src(src: &str) -> AstNode {
    parse(tokenize(src).unwrap())
}

// Ağacı S-expression olarak yazar: yaprak -> "x", düğüm -> "(op arg arg)"
fn sexp(n: &AstNode) -> String {
    if n.args.is_empty() {
        n.token.lexeme.clone()
    } else {
        let inner: Vec<String> = n.args.iter().map(sexp).collect();
        format!("({} {})", n.token.lexeme, inner.join(" "))
    }
}

// Program düğümünün çocuklarını (statement'ları) boşlukla birleştirir.
fn tree(src: &str) -> String {
    let program = parse_src(src);
    let parts: Vec<String> = program.args.iter().map(sexp).collect();
    parts.join(" ")
}

fn count_nodes(n: &AstNode) -> usize {
    1 + n.args.iter().map(count_nodes).sum::<usize>()
}

// ---------- ağaç şekli ----------

#[test]
fn single_declaration() {
    assert_eq!(tree("var x = 5;"), "(; (var (= x 5)))");
}

#[test]
fn two_statements() {
    assert_eq!(
        tree("var x = 5;var y = 0-5+x;"),
        "(; (var (= x 5))) (; (var (= y (+ (- 0 5) x))))"
    );
}

#[test]
fn program_and_statement_kinds() {
    let program = parse_src("var x = 5;");
    assert_eq!(program.kind, AstKind::Program);
    assert_eq!(program.args.len(), 1);
    assert_eq!(program.args[0].kind, AstKind::Statment);
}

// ---------- tekli / ikili ayrımı ----------

#[test]
fn binary_minus_after_operand() {
    assert_eq!(tree("var y = 0-5;"), "(; (var (= y (- 0 5))))");
}

#[test]
fn unary_minus_after_operator() {
    assert_eq!(tree("var y = -5;"), "(; (var (= y (- 5))))");
}

#[test]
fn unary_then_binary() {
    // 5'in kaybolduğu hata: '+' tekli sanılıyordu
    assert_eq!(tree("var y = -5+x;"), "(; (var (= y (+ (- 5) x))))");
}

#[test]
fn binary_then_unary() {
    assert_eq!(tree("var y = 5 - -3;"), "(; (var (= y (- 5 (- 3)))))");
}

#[test]
fn unary_and_binary_mixed() {
    assert_eq!(
        tree("var y = -5 - -3 + 2;"),
        "(; (var (= y (+ (- (- 5) (- 3)) 2))))"
    );
}

#[test]
fn unary_plus() {
    assert_eq!(tree("var y = +x;"), "(; (var (= y (+ x))))");
}

// ---------- öncelik ve ilişkilendirme ----------

#[test]
fn assignment_is_lower_than_sum() {
    assert_eq!(tree("var y = 1+2;"), "(; (var (= y (+ 1 2))))");
}

#[test]
fn sum_is_left_associative() {
    assert_eq!(tree("var y = 1-2-3;"), "(; (var (= y (- (- 1 2) 3))))");
    assert_eq!(tree("var y = 1+2-3;"), "(; (var (= y (- (+ 1 2) 3))))");
}

#[test]
fn assignment_is_right_associative() {
    assert_eq!(tree("var y = x = 5;"), "(; (var (= y (= x 5))))");
}

#[test]
fn type_as_binds_tighter_than_sum() {
    assert_eq!(tree("var y = 1 + 2 As T;"), "(; (var (= y (+ 1 (As 2 T)))))");
}

#[test]
fn unary_binds_tighter_than_type_as() {
    assert_eq!(tree("var y = -x As T;"), "(; (var (= y (As (- x) T))))");
}

// ---------- hiçbir token kaybolmamalı ----------

// Her token (Eof hariç) ağaçta tam bir düğüm olmalı.
#[test]
fn every_token_becomes_exactly_one_node() {
    let sources = [
        "var x = 5;",
        "var x = 5;var y = 0-5+x;",
        "var x = 5;var y = -5+x;",
        "var y = 1 - -3;",
        "var y = -5 - -3 + 2;",
        "var y = x = 5;",
        "var y = 1 + 2 As T;",
        "var y = -x As T;",
    ];
    for src in sources {
        let tokens = tokenize(src).unwrap();
        let expected = tokens.len() - 1; // Eof düğüm olmaz
        let program = parse(tokens);
        let actual: usize = program.args.iter().map(count_nodes).sum();
        assert_eq!(actual, expected, "token kaybı/fazlası: {src}");
    }
}

// ---------- operatör tabloları ----------

#[test]
fn precedence_ordering() {
    // ; < var < = < (+ -) < As < tekli işaret
    assert!(OpCmd::FinishStatment.precedence() < OpCmd::Create.precedence());
    assert!(OpCmd::Create.precedence() < OpCmd::ValueAs.precedence());
    assert!(OpCmd::ValueAs.precedence() < OpCmd::PlusSum.precedence());
    assert_eq!(OpCmd::PlusSum.precedence(), OpCmd::MinusSub.precedence());
    assert!(OpCmd::PlusSum.precedence() < OpCmd::TypeAs.precedence());
    assert!(OpCmd::TypeAs.precedence() < OpCmd::PlusSign.precedence());
    assert_eq!(OpCmd::PlusSign.precedence(), OpCmd::MinusSign.precedence());
}

#[test]
fn associativity() {
    assert_eq!(OpCmd::ValueAs.assoc(), Assoc::Right);
    assert_eq!(OpCmd::PlusSum.assoc(), Assoc::Left);
    assert_eq!(OpCmd::MinusSub.assoc(), Assoc::Left);
    assert_eq!(OpCmd::PlusSign.assoc(), Assoc::Right);
    assert_eq!(OpCmd::MinusSign.assoc(), Assoc::Right);
}

#[test]
fn slot_shapes() {
    let (l, r): (Vec<Slot>, Vec<Slot>) = OpCmd::PlusSum.into();
    assert_eq!((l, r), (vec![Slot::EXPR], vec![Slot::EXPR])); // ikili

    let (l, r): (Vec<Slot>, Vec<Slot>) = OpCmd::MinusSign.into();
    assert_eq!((l, r), (vec![], vec![Slot::EXPR])); // tekli (önek)

    let (l, r): (Vec<Slot>, Vec<Slot>) = OpCmd::FinishStatment.into();
    assert_eq!((l, r), (vec![Slot::EXPR | Slot::STMT], vec![])); // sonek
}

#[test]
fn symbol_to_op_candidates() {
    let minus = Vec::<OpCmd>::from(SymbolDomain::Minus);
    assert!(minus.contains(&OpCmd::MinusSign) && minus.contains(&OpCmd::MinusSub));

    let plus = Vec::<OpCmd>::from(SymbolDomain::Plus);
    assert!(plus.contains(&OpCmd::PlusSign) && plus.contains(&OpCmd::PlusSum));

    assert_eq!(Vec::<OpCmd>::from(SymbolDomain::Eq), vec![OpCmd::ValueAs]);
    assert_eq!(Vec::<OpCmd>::from(SymbolDomain::Semicolon), vec![OpCmd::FinishStatment]);
}

#[test]
fn keyword_to_op_candidates() {
    assert_eq!(Vec::<OpCmd>::from(KeywordDomain::Var), vec![OpCmd::Create]);
    assert_eq!(Vec::<OpCmd>::from(KeywordDomain::As), vec![OpCmd::TypeAs]);
}

// ---------- bilinen eksikler ----------

#[test]
#[ignore = "boş girdide parse() tokens.last().unwrap() ile panic yapıyor"]
fn empty_input_gives_empty_program() {
    let program = parse(tokenize("").unwrap());
    assert_eq!(program.kind, AstKind::Program);
    assert!(program.args.is_empty());
}
