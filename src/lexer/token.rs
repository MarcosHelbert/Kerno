use logos::Logos;

#[derive(Logos, Debug, PartialEq, Clone)]
pub enum Token {
    // ─── Palavras-chave ───────────────────────────────────────
    #[token("function")] Function,
    #[token("async")] Async,
    #[token("const")] Const,
    #[token("var")] Var,
    #[token("return")] Return,
    #[token("import")] Import,

    // ─── Controle de fluxo ────────────────────────────────────
    #[token("for")] For,
    #[token("while")] While,
    #[token("if")] If,
    #[token("else")] Else,
    #[token("break")] Break,
    #[token("continue")] Continue,

    // ─── Tipos primitivos ─────────────────────────────────────
    #[token("int")] IntegerType,
    #[token("string")] StringType,
    #[token("bool")] BooleanType,
    #[token("float")] FloatType,
    #[token("decimal")] DecimalType,
    #[token("void")] VoidType,
    #[token("null")] NullLiteral,

    // ─── Tipos compostos ──────────────────────────────────────
    #[token("list")] ListType,
    #[token("array")] ArrayType,
    #[token("map")] MapType,
    #[token("set")] SetType,
    #[token("tuple")] TupleType,
    #[token("queue")] QueueType,
    #[token("stack")] StackType,

    // ─── Operadores compostos ─────────────────────────────────
    #[token("==")] EqualEqual,
    #[token("!=")] BangEqual,
    #[token(">=")] GreaterEqual,
    #[token("<=")] LessEqual,
    #[token("&&")] AndAnd,
    #[token("||")] OrOr,
    #[token("+=")] PlusEqual,
    #[token("-=")] MinusEqual,
    #[token("*=")] StarEqual,
    #[token("/=")] SlashEqual,
    #[token("%=")] PercentEqual,
    #[token("**")] StarStar,
    #[token("++")] PlusPlus,
    #[token("--")] MinusMinus,
    #[token("&=")] AndEqual,
    #[token("|=")] OrEqual,
    #[token("^=")] CaretEqual,
    #[token(">>=")] RShiftEqual,
    #[token("<<=")] LShiftEqual,

    // ─── Operadores simples ──────────────────────────────────
    #[token("=")] Equals,
    #[token("!")] Bang,
    #[token("?")] QuestionMark,
    #[token("+")] Plus,
    #[token("-")] Minus,
    #[token("*")] Star,
    #[token("/")] Slash,
    #[token("%")] Percent,
    #[token(">")] Greater,
    #[token("<")] Less,
    #[token("&")] Ampersand,
    #[token("|")] Pipe,
    #[token("^")] Caret,
    #[token(">>")] RShift,
    #[token("<<")] LShift,

    // ─── Símbolos ─────────────────────────────────────────────
    #[token("(")] LParen,
    #[token(")")] RParen,
    #[token("{")] LBrace,
    #[token("}")] RBrace,
    #[token(":")] Colon,
    #[token(";")] Semicolon,
    #[token(",")] Comma,
    #[token(".")] Dot,
    #[token("@")] At,

    // ─── Símbolos para tipos genéricos e arrays ──────────────
    #[token("[")] LBracket,
    #[token("]")] RBracket,

    // ─── Literais e Identificadores ───────────────────────────
    #[regex("true|false")] BooleanLiteral,
    #[regex(r"[0-9]+\.[0-9]+m")] DecimalLiteral,
    #[regex(r"[0-9]+\.[0-9]+")] FloatLiteral,
    #[regex(r"[0-9]+")] IntLiteral,
    #[regex(r#""([^"\\]|\\.)*""#)] StringLiteral,
    #[regex("[a-zA-Z_][a-zA-Z0-9_]*")] Ident,

    // ─── Ignorados ────────────────────────────────────────────
    #[regex(r"[ \t\n\f]+")] Whitespace,
    #[regex(r"//[^\n]*")] Comment,
    #[regex(r"/\*([^*]|\*[^/])*\*/")] BlockComment,
}
