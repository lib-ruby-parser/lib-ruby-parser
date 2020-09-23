%expect 0

%define api.parser.struct {Parser}
%define api.location.type {Loc}
%define api.value.type { Value }

%define parse.error custom
%define parse.trace


%code use {
  // all use goes here
}

%code parser_struct_fields {
  result: Option<String>
}

%code {
  // code
}


/* Bison Declarations */
%token
    BANG   "!"
    PLUS   "+"
    MINUS  "-"
    STAR   "*"
    SLASH  "/"
    CARET  "^"
    LPAREN "("
    RPAREN ")"
    EQUAL  "="
    EOL    _("end of line")
  <String>
    NUM    _("number")
%type <Expr> exp input line

%nonassoc "="       /* comparison            */
%left "-" "+"
%left "*" "/"
%precedence NEG     /* negation--unary minus */
%right "^"          /* exponentiation        */

/* Grammar follows */
%%
program:
  input { self.result = Some($<Expr>1.clone()); }
;

input:
  line
| input line
;

line:
  EOL                { $$ = Value::Expr("EOL".to_owned()) }
| exp EOL            { println!("{:#?}", $exp); }
| error EOL          { println!("err recoery"); $$ = Value::Expr("EOL".to_owned()) }
;

exp:
  NUM                { $$ = Value::Expr($<Token>1.1.clone()) }
| exp "=" exp {
      if $1 != $3 {
          self.yyerror(&@$, &format!("calc: error: {:#?} != {:#?}", $1, $3));
      }
  }
| exp "+" exp        { $$ = Value::Expr(format!("{:#?} + {:#?}", $<Expr>1, $<Expr>3)); }
| exp "-" exp        { $$ = Value::Expr(format!("{:#?} - {:#?}", $<Expr>1, $<Expr>3)); }
| exp "*" exp        { $$ = Value::Expr(format!("{:#?} * {:#?}", $<Expr>1, $<Expr>3)); }
| exp "/" exp        { $$ = Value::Expr(format!("{:#?}/+ {:#?}", $<Expr>1, $<Expr>3)); }
| "-" exp  %prec NEG { $$ = Value::Expr(format!("-{:#?}", $<Expr>2)); }
| exp "^" exp        { $$ = Value::Expr(format!("{:#?} ^ {:#?}", $<Expr>1, $<Expr>3)); }
| "(" exp ")"        { $$ = Value::Expr(format!("({:#?})", $<Expr>2)); }
| "(" error ")"      { $$ = Value::Expr(format!("(err)")); }
| "!"                { return Self::YYERROR; }
| "-" error          { return Self::YYERROR; }
;

%%

#[derive(Clone, PartialEq)]
pub enum Value {
    None,
    Token(Token),
    Expr(String)
}

impl Value {
    pub fn from_token(token: Token) -> Self {
        Self::Token(token)
    }
}

impl std::fmt::Debug for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { //'
        match self {
            Value::None => f.write_str("<No Token>"),
            Value::Token((token_type, token_value, loc)) => {
              f.write_fmt(format_args!("Token({}, {:?}, {:?})", token_type, token_value, loc))
            },
            Value::Expr(expr) => f.write_fmt(format_args!("Expr({})", expr))
        }
    }
}

impl Parser {
  #[allow(dead_code)]
  pub fn new(tokens: Vec<Token>) -> Self {
      let mut parser = Self::build();
      parser.yylexer.tokens = tokens;
      parser.result = None;
      parser
  }

  #[allow(dead_code)]
  pub fn do_parse(mut self) -> Option<String> {
      self.parse();
      self.result
  }
}

#[derive(Default)]
pub struct Lexer {
    tokens: Vec<Token>
}

impl Lexer {
    pub fn yylex(&mut self) -> Token {
        self.tokens.remove(0)
    }

    fn report_syntax_error(&self, ctx: &Context) {
        eprintln!("{:#?}", ctx)
    }

    fn yyerror(&mut self, loc: &Loc, msg: &str) {
        eprintln!("{:#?} {:#?}", loc, msg)
    }
}
