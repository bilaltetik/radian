use crate::{lexer::types::*, parser::{ast::slot_of, types::*, ast::marker_of}};
use miette::{Diagnostic, NamedSource, SourceSpan,Report};
use thiserror::Error;

#[derive(Error, Debug, Diagnostic)]
#[error("[Parser] Syntax error")]
#[diagnostic(code(parser::invalid_op))]
struct ParserError {
    #[help]
    help: String,
    #[source_code]
    src: NamedSource<String>,
    #[label("Cannot find valid operator")]
    span: SourceSpan,
}

//3a.
fn resolve_op_cmd(args: &[AstNode], counter: usize) -> Option<OpCmd> {

    if !matches!(args[counter].kind, AstKind::Operand) {
        return None;
    }

    //3aI
    let op = Operation::from_token_kind(args[counter].token.kind.clone()).0;
    let mut op_variants = Vec::<OpCmd>::from(op);
    
    op_variants.sort_by_key(|o| {
        // Tip belirterek Rust'a hangi From'u kullanacağını söylüyoruz
        let (l, _): (Vec<Slot>, Vec<Slot>) = (*o).into();
        std::cmp::Reverse(l.len())
    });

    //3aII
    for op in op_variants {
        let (left, right): (Vec<Slot>, Vec<Slot>) = op.into();
        // YENİ: İkinci tabloyu (Markers) çekiyoruz
        let (left_markers, right_markers): (Vec<Markers>, Vec<Markers>) = op.into();

        if counter < left.len() || counter + right.len() >= args.len() {
            continue;
        }

        let mut left_ok = true;
        for slot_c in 0..left.len() {
            let arg_c = counter - left.len() + slot_c;
            let allowed_slot = left[slot_c];
            let arg_kind = &args[arg_c].kind;
            let arg_slot = slot_of(arg_kind);

            // 1. Sorgu: Semantic Slot uyuyor mu?
            if !allowed_slot.contains(arg_slot) {
                left_ok = false;
                break;
            }

            // 2. Sorgu: Eğer Slot::Marker bekleniyorsa, türü uyuyor mu?
            if allowed_slot.contains(Slot::Marker) {
                let expected_marker = left_markers[slot_c];
                let actual_marker = marker_of(arg_kind);
                
                if !expected_marker.contains(actual_marker) {
                    left_ok = false;
                    break;
                }
            }
        }

        let mut right_ok = true;
        for slot_c in 0..right.len() {
            let arg_c = counter + 1 + slot_c;
            let allowed_slot = right[slot_c];
            let arg_kind = &args[arg_c].kind;
            let arg_slot = slot_of(arg_kind);

            // 1. Sorgu: Semantic Slot uyuyor mu?
            if !allowed_slot.contains(arg_slot) {
                right_ok = false;
                break;
            }

            // 2. Sorgu: Eğer Slot::Marker bekleniyorsa, türü uyuyor mu?
            if allowed_slot.contains(Slot::Marker) {
                let expected_marker = right_markers[slot_c];
                let actual_marker = marker_of(arg_kind);
                
                if !expected_marker.contains(actual_marker) {
                    right_ok = false;
                    break;
                }
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


fn explain_op_failure(args: &[AstNode], pos: usize) -> String {
    let token = &args[pos].token;
    let mut msg = format!(
        "\nGeçerli Operator bulunamadı.\nHata veren token: {:?} (Span: {:?}) | counter={}\n\nOlası Adaylar ve Hata Sebepleri:\n",
        token.lexeme, token.span, pos
    );

    let oper = Operation::from_token_kind(token.kind.clone()).0;
    let op_variants = Vec::<OpCmd>::from(oper);

    if op_variants.is_empty() {
        msg.push_str("  -> Bu token için tanımlanmış hiçbir OpCmd adayı yok.\n");
    }

    for op in op_variants {
        let (left, right): (Vec<Slot>, Vec<Slot>) = op.into();
        msg.push_str(&format!("- OpCmd::{:?} (Beklenen Sol: {:?}, Beklenen Sağ: {:?})\n", op, left, right));

        if pos < left.len() || pos + right.len() >= args.len() {
            msg.push_str("  -> Eksik Argüman (Syntax Hatası): Operatörün solunda veya sağında yeterli ifade bulunamadı.\n");
            continue;
        }

        let mut ok = true;
        for slot_c in 0..left.len() {
            let arg_c = pos - left.len() + slot_c;
            let arg_slot = slot_of(&args[arg_c].kind);
            if !left[slot_c].contains(arg_slot) {
                msg.push_str(&format!("  -> Sol taraf başarısız (index {}): Beklenen {:?}, Bulunan {:?} (Token: {:?})\n", arg_c, left[slot_c], arg_slot, args[arg_c].token.lexeme));
                ok = false; break;
            }
        }
        if !ok { continue; }

        for slot_c in 0..right.len() {
            let arg_c = pos + 1 + slot_c;
            let arg_slot = slot_of(&args[arg_c].kind);
            if !right[slot_c].contains(arg_slot) {
                msg.push_str(&format!("  -> Sağ taraf başarısız (index {}): Beklenen {:?}, Bulunan {:?} (Token: {:?})\n", arg_c, right[slot_c], arg_slot, args[arg_c].token.lexeme));
                
                //ok = false;
                break;
            }
        }
    }
    msg
}
//3. aşama
fn reduce_or_defer(program: &mut AstNode, counter: &mut usize) {
    let self_pos = *counter; // kendi pozisyonunu sakla


    let op = resolve_op_cmd(&program.args, self_pos).unwrap_or_else(|| {
        let token = &program.args[self_pos].token;
        let err = ParserError {
            help: explain_op_failure(&program.args, self_pos),
            src: NamedSource::new("girdi", token.lexeme.clone()), 
            span: (token.span.col as usize, token.lexeme.len()).into(),
        };
        
        eprintln!("{:?}", Report::new(err)); 
        std::process::exit(1); // Panic yerine sessizce hata koduyla çık
    });

    let (next_op, inx) = get_next_op(&program.args, self_pos + 1);

    

    let should_reduce_self =  next_op == OpCmd::None
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

                // YENİ: Comma flatten — sol zaten Tuple ise birleştir
        if op == OpCmd::ExpGroup {
            let left_is_tuple = matches!(
                extracted.first().map(|n| &n.kind),
                Some(AstKind::Expression(ExpressionKind::Group))
            );
            if left_is_tuple {
                let mut tuple = extracted.remove(0);
                tuple.args.extend(extracted);      // sağdakini tuple'a yedir
                tuple.token = operator.token;      // span'i güncelle
                program.args.insert(start, tuple);
                *counter = start;
                return;
            }
        }

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
