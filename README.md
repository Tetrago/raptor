# raptor

## Syntax

```
(* Classes *)
    digit = "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9"
    alpha = "A" | "B" | "C" | "D" | "E" | "F" | "G" | "H" | "I" | "J" | "K"
          | "L" | "M" | "N" | "O" | "P" | "Q" | "R" | "S" | "T" | "U" | "V"
          | "W" | "X" | "Y" | "Z" | "a" | "b" | "c" | "d" | "e" | "f" | "g"
          | "h" | "i" | "j" | "k" | "l" | "m" | "n" | "o" | "p" | "q" | "r"
          | "s" | "t" | "u" | "v" | "w" | "x" | "y" | "z"

(* Tokens *)
    (* Comment *)
    singleline_comment = "//", { ANY }, ("\n" | EOF);
    multiline_comment  = "/*", { ANY }, "*/";
    comment            = singleline_comment | multiline_comment;

    (* Literal *)
    decimal = { digit }, ".", digit, { digit };

    hexadecimal_integer = "0x", digit, { digit };
    binary_integer      = "0b", digit, { digit };
    octal_integer       = "0o", digit, { digit };

    integer = (digit, { digit }) | hexadecimal_integer | binary_integer
            | octal_integer;

    literal = decimal | integer;

    (* Identifier *)
    backtick_identifier = "`", ANY, { ANY }, "`";
    alpha_identifier    = alpha, { alpha | digit | "_" };
    identifier          = alpha_identifier | backtick_identifier;

    (* Separator *)
    separator = "[" | "]" | "(" | ")" | "{" | "}" | ";" | ",";

    (* Whitespace *)
    whitespace = "\t" | "\r" | "\n" | " ", { "\t" | "\r" | "\n" | " " };

    (* Operator *)
    operator = ANY - digit - alpha - "_" - whitespace - separator
             , { ANY - digit - alpha - "_" - whitespace - separator };

(* Items *)
    (* Expression *)
    invocation = expression, "(", [ expression, { ",", expression } ], ")";
    mono_operation = operator, expression;
    binary_operation  = expression, (operator | ","), expression;
    group      = "(", expression, ")";

    expression = identifier | literal | binary_operation | group;

    (* Statement *)
    empty  = ";";
    block  = "{", { statement }, "}";
    let    = "let", identifier, "=", expression, ";";
    if     = "if", "(", expression, ")", statement;
    while  = "while", "(", expression, ")", statement;
    until  = "until", "(", expression, ")", statement;
    loop   = "loop", statement;
    return = "return", expression, ";";
    break  = "break", ";";
    for    = "for", "(", statement, expression, ";", statement, ")", statement;
    do     = "do", statement, "while", "(", expression, ")", ";";

    statement = empty | let | if | while | for | do | until | block;

    (* Structure *)
    structure = "struct", identifier, "{", { identifier, ":", identifier, "," }
              , "}";

    (* Function *)
    parameter = identifier, ":", identifier;
    function  = "fn", identifier, "(", [ parameter, { ",", parameter } ], ")"
              , statement;

    (* File *)
    file = { structure | function }
```
