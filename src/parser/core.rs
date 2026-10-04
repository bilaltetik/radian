use crate::{lexer::types::*, parser::core::{ ExpressionKind::Identifier, OpCmd::{TypeAs}} };

bitflags::bitflags! {
#[derive(Debug, Clone, Copy, PartialEq, Eq)]

    pub struct Slot: u8 {
        const EXPR  = 1 << 0;  // değer üreten herşey: Atom, Infix, Prefix, Postfix
        const STMT  = 1 << 1;  // Statement
        const OP    = 1 << 2;  // çıplak operatör (örn: blok içinde pattern)
        const NONE  = 0b1000;  // boş / yok
    }
}

impl From<OpCmd> for (Vec<Slot>, Vec<Slot>) {
    fn from(value: OpCmd) -> Self {
        match value {
            // Binary
            OpCmd::PlusSum | OpCmd::MinusSub | OpCmd::ValueAs | OpCmd::TypeAs =>
            { (vec![Slot::EXPR], vec![Slot::EXPR]) },

            // Unary
            OpCmd::PlusSign | OpCmd::MinusSign |
            OpCmd::Create => 
            { (vec![], vec![Slot::EXPR]) },

            // Statement Terminator (Noktalı Virgül)
            OpCmd::FinishStatment => 
            { (vec![Slot::EXPR | Slot::STMT], vec![]) },

            OpCmd::None => { (vec![], vec![]) }
        }
    }
}

pub enum Operation{
    Sym(SymbolDomain),
    Key(KeywordDomain),
    Invalid,
}

impl From<SymbolDomain> for Operation {
    fn from(op: SymbolDomain) -> Operation{
        Operation::Sym(op)
    }
}

impl From<KeywordDomain> for Operation {
    fn from(op: KeywordDomain) -> Operation{
        Operation::Key(op)
    }
}

impl Operation {


    fn from_token_kind(kind: TokenKind) -> (Operation, bool){
        match kind {
            TokenKind::TokenWord(WordKind::Keyword(keyword)) => {
                (Operation::Key(keyword), true)
            }
            TokenKind::TokenSymbol(symbol) => (Operation::Sym(symbol), true),
            _ => (Operation::Invalid, false),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Assoc{
    Left,
    Right,
    None
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OpCmd{
    PlusSum,
    PlusSign,
    MinusSub,
    MinusSign,
    ValueAs,
    TypeAs,
    Create,
    FinishStatment,
    None,
}

impl From<SymbolDomain> for Vec<OpCmd>{
    fn from(op: SymbolDomain) -> Vec<OpCmd>{
        match op {
            SymbolDomain::Plus => {vec![OpCmd::PlusSign,OpCmd::PlusSum]}
            SymbolDomain::Minus => {vec![OpCmd::MinusSign,OpCmd::MinusSub]}
            SymbolDomain::Eq => {vec![OpCmd::ValueAs]}
            SymbolDomain::Semicolon => {vec![OpCmd::FinishStatment]}
        }
    }
}

impl From<KeywordDomain> for Vec<OpCmd>{
    fn from(op: KeywordDomain) -> Vec<OpCmd>{
        match op {
            KeywordDomain::As => {vec![TypeAs]},
            KeywordDomain::Var => {vec![OpCmd::Create]}
        }
    }
}

impl From<Operation> for Vec<OpCmd>{
    fn from(op: Operation) -> Vec<OpCmd>{
        match op {
            Operation::Sym(sym) => Vec::<OpCmd>::from(sym),
            Operation::Key(key) => Vec::<OpCmd>::from(key),
            Operation::Invalid => vec![],
        }
    }
}

impl OpCmd {
    pub fn precedence(self) -> u8 {
        match self {
            OpCmd::PlusSum   => 10,
            OpCmd::MinusSub  => 10,

            OpCmd::PlusSign  => 12,
            OpCmd::MinusSign => 12,

            OpCmd::ValueAs   => 12,
            OpCmd::TypeAs    => 12,

            OpCmd::Create    => 1,
            OpCmd::FinishStatment => 0,

            OpCmd::None      => 255,
        }
    }

    pub fn assoc(self) -> Assoc {
        match self {
            OpCmd::PlusSum   => Assoc::Left,
            OpCmd::MinusSub  => Assoc::Left,

            OpCmd::PlusSign  => Assoc::Right,
            OpCmd::MinusSign => Assoc::Right,

            OpCmd::ValueAs   => Assoc::Left,
            OpCmd::TypeAs    => Assoc::Left,

            OpCmd::Create    => Assoc::Left,

            OpCmd::FinishStatment => Assoc::Left,

            OpCmd::None      => Assoc::None,

        }
    }

    pub fn result_kind(self) -> Option<AstKind> {
        match self {
            //Expr
            OpCmd::PlusSum | OpCmd::MinusSub | OpCmd::PlusSign | OpCmd::MinusSign
            => {
                Some(AstKind::Expression(ExpressionKind::Complex))
            }
            //Identifier
            OpCmd::TypeAs | OpCmd::ValueAs | OpCmd::Create
            => {
                Some(AstKind::Expression(Identifier))
            }
            //Statment
            OpCmd::FinishStatment => {
                Some(AstKind::Statment)
            }
            //
            OpCmd::None => {Option::None} //standart durumda Program işlenmez
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExpressionKind{
    Value,
    Identifier,
    Block,
    Complex
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AstKind{
    Program,
    Statment,
    Expression(ExpressionKind),
    Operand
}

fn slot_of(kind: &AstKind) -> Slot {
    match kind {
        AstKind::Expression(_) => Slot::EXPR,
        AstKind::Statment      => Slot::STMT,
        AstKind::Operand       => Slot::OP,
        AstKind::Program       => Slot::NONE,
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AstNode{
    token:Token,
    kind:AstKind,
    args: Vec<AstNode>,
}

impl AstNode {
    pub fn print_tree(&self, depth: usize) {
        let tabs = "\t".repeat(depth);
        
        if self.args.is_empty() {
            println!("{}AST({:?}, {:?}) {{}}", tabs, self.kind, self.token.lexeme);
        } else {
            println!("{}AST({:?}, {:?}) {{", tabs, self.kind, self.token.lexeme);
            for arg in &self.args {
                arg.print_tree(depth + 1);
            }
            println!("{}}}", tabs);
        }
    }

    pub fn program(tokens: Vec<Token>) -> AstNode {
        // Vektörün boş olma durumunu yönetmek için unwrap yerine pattern matching/if let kullanmanız önerilir.
        // Clone veya Copy ile referans bağımlılığını koparıyoruz.
        let last_token = tokens.last().unwrap().clone(); 
        
        let mut args: Vec<AstNode> = Vec::new();
        
        // Değişken ismini 't' yaptık ki dışarıdakini ezmesin
        for t in tokens {
            match t.kind {
                TokenKind::TokenNumber(_) | TokenKind::TokenLiteral(_) => {
                    args.push(AstNode { token: t, kind: AstKind::Expression(ExpressionKind::Value), args: Vec::new() });
                }
                TokenKind::TokenWord(WordKind::Identifier) => {
                    args.push(AstNode { token: t, kind: AstKind::Expression(ExpressionKind::Identifier), args: Vec::new() });
                }
                TokenKind::TokenSymbol(_) | TokenKind::TokenWord(WordKind::Keyword(_)) => {
                    args.push(AstNode { token: t, kind: AstKind::Operand, args: Vec::new() });
                }
                TokenKind::Eof => {
                    break;
                }
            }
        }

        AstNode { token: last_token, kind: AstKind::Program, args }
    }
}


//3a.
fn resolve_op_cmd(args: &[AstNode], counter: usize) -> Option<OpCmd> {
    //3aI
    let op = Operation::from_token_kind(args[counter].token.kind.clone()).0;
    let op_variants = Vec::<OpCmd>::from(op);

    //3aII
    for op in op_variants {
        let (left, right): (Vec<Slot>, Vec<Slot>) = op.into();

        if counter < left.len() || counter + right.len() >= args.len() {
            continue;
        }

        let mut left_ok = true;
        for slot_c in 0..left.len() {
            let arg_c = counter - left.len() + slot_c;
            let allowed = left[slot_c];
            let arg_slot = slot_of(&args[arg_c].kind);
            if !allowed.contains(arg_slot) {
                left_ok = false;
                break;
            }
        }

        let mut right_ok = true;
        for slot_c in 0..right.len() {
            let arg_c = counter + 1 + slot_c;
            let allowed = right[slot_c];
            let arg_slot = slot_of(&args[arg_c].kind);
            if !allowed.contains(arg_slot) {
                right_ok = false;
                break;
            }
        }

        if left_ok && right_ok {
            return Some(op);
        }
    }

    None
}


// 1. ilk aşamada tokenları astNode ile wrapla
// bütün nodelar yan yana

// 2. (a) operatörleri tara (b) diğer şeyleri atla

/*3. adıma denk gelince:

  3a. Hangi OpCmd? (I.adayları bul, II.argüman/slot kontrolüyle daralt)
      → "bu token burada ne anlama geliyor" sorusu, henüz öncelikle ilgisi yok

  3b. Kimin sırası? ((I)sağdaki komşuyla (II)öncelik+assoc kıyası, recursive karar)
      → "ben mi işlenmeliyim yoksa sağdaki mi önce" sorusu, 3a'nın sonucunu kullanıyor

  3c. İşle (argümanları topla, tek node'a indir)
      → sadece 3b "ben işlenmeliyim" dediğinde çalışan asıl eylemyse önce sağdakini işlemek için kendini tekrar çağır
// eğer eşitse Assoc bakılır */

/*
4. adım işleme içeriği

    4a. (II)hariç((I)program[counter-left:counter+right], program[counter]), (III)taşı -> program[counter].args içine (kopyalama, bizzat taşı artık program içinde olmasınlar)
    4b. program[counter].kind'ı operatörün çıktı tipine göre ver.
    bunu yazarken çıktı mapini daha tasarlamadım
    4c. güvenli çık

 */

//3. aşama
fn reduce_or_defer(program: &mut AstNode, counter: &mut usize) {
    let self_pos = *counter; // kendi pozisyonunu sakla

    let op = resolve_op_cmd(&program.args, self_pos)
        .unwrap_or_else(|| panic!("geçerli OpCmd bulunamadı, counter={} | Token={:?}", self_pos, program.args[self_pos]));

    let (next_op, inx) = get_next_op(&program.args, self_pos + 1);

    let should_reduce_self = next_op == OpCmd::None
        || next_op.precedence() < op.precedence()
        || (next_op.precedence() == op.precedence() && op.assoc() == Assoc::Left);

    if should_reduce_self {
        // 3c -> 4. adım
        let (left, right): (Vec<Slot>, Vec<Slot>) = op.into();
        
        let start = *counter - left.len();
        let end = *counter + right.len() + 1;
        //4a(I)
        let mut extracted: Vec<_> = program.args.drain(start..end).collect();
        //4a(II)
        let mut operator = extracted.remove(left.len());
        //4a(III)
        operator.args = extracted;
        program.args.insert(start, operator);

        //4b
        let kind = op.result_kind();
        if kind.is_none() {
            panic!("AstNode kind None olmamalı");
        }
        program.args[start].kind = kind.unwrap();

        //4c
        *counter = start;
    } else {
        *counter = inx;
        reduce_or_defer(program, counter);  // sağdakini (ve ötesini) tam çöz

        *counter = self_pos;                // kendi pozisyonuna AYNEN dön
        reduce_or_defer(program, counter);  // şimdi kendini tekrar dene
    }
}


fn get_next_op(args: &[AstNode], index: usize) -> (OpCmd,usize){
    let mut res = OpCmd::None;
    let mut inx: usize = 0;
    for i in index..args.len() {
        let op = resolve_op_cmd(args, i);
        if !op.is_none() {
            res = op.unwrap();
            inx = i;
            break;
        }
    }
    (res,inx)
}

pub fn parse(tokens: Vec<Token>) -> AstNode{
    //1. aşama
    let mut program = AstNode::program(tokens);

    let mut counter: usize = 0;
    while counter < program.args.len() {
        
        match program.args[counter].kind {
            AstKind::Operand => {
                //3 aşama
                reduce_or_defer(&mut program, &mut counter);

            }

            AstKind::Statment | AstKind::Expression(ExpressionKind::Block) => {
                // bi ihtimal blok ise scope kontrolü
            }

            _ => {
                // atla
            }
        }

        counter+=1;
    }

    program
}