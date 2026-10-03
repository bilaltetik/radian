use crate::lexer::types::{KeywordDomain, SymbolDomain, WordKind::Keyword};




enum Operation{
    Sym(SymbolDomain),
    Key(KeywordDomain),
}

impl From<KeywordDomain> for Operation {
    fn from(o: KeywordDomain) -> Self {
        Operation::Key(o)
    }
}

impl From<SymbolDomain> for Operation {
    fn from(o: SymbolDomain) -> Self {
        Operation::Sym(o)
    }
}

impl Operation{
    fn arg_count (self) -> (u8,u8){
        match self {
            //Binary
            Sym(SymbolDomain::Plus) 
            
            => {
                (1,1)
            }
            //Postfix

            //Prefix

            //bool & block

            //other
            _ => (0,0)
        }
    }
}