#[derive(Debug, PartialEq, Eq)]
pub enum TokenType {
    // single character
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    LeftBrace,
    RightBrace,
    Plus,
    Minus,
    Slash,
    BackSlash,
    Star,
    Modulus,
    Pipe,
    Ampersand,
    Tilde,
    Greater,
    Lesser,
    Comma,
    Bang,
    Dollar,
    Assign,
    Dot,
    Colon,
    Semicolon,
    QuestionMark,
    At,
    Hash,
    Quote,
    DoubleQuote,
    BackTick,
    Caret,
    Underscore,
    NewLine,
    Space,

    // Double character
    Equals,
    BangEquals,
    GreaterEquals,
    LesserEquals,
    PlusEquals,
    MinusEquals,
    StarEquals,
    SlashEquals,
    ModEquals,
    AndEquals,
    OrEquals,
    XorEquals,
    NorEquals,
    Increment,
    Decrement,
    RightShift,
    LeftShift,
    LeftArrow,
    RightArrow,

    // Literals
    Identifier,
    String,
    Int,
    Float,

    // Registers
    Register,
    RegisterNumber,
    LowByte,
    HighByte,
    Word,
    DoubleWord,
    QuadWord,

    // Keywords
    Ret,
    Inc,
    Ent,
    Txt,
    Arch,
    Def,
    Con,

    EOF
}
