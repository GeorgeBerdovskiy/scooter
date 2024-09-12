
```
program ::= item-fn { item-fn }

item-fn ::= "fn" ident "(" ")" -> ty "{" block "}"

     ty ::= "i32"

  block ::= stmt ";" { stmt ";" }

  stmt  ::= local | expr

   expr ::= call-fn | term { "+" term }

  local ::= "let" ident ":" ty "=" expr

call-fn ::= ident "(" ")"

   term ::= factor { "*" factor }

 factor ::= ident | lit-int | "(" expr ")"

  ident ::= (letter | "_") { letter | digit }

 letter ::= "a" | "b" | ... | "z" | "A" | "B" | ... | "Z"

  digit ::= "0" | "1" | "2" | ... | "9"
```