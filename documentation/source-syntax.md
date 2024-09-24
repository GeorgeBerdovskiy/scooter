
```
     program ::= item-fn { item-fn }

     item-fn ::= "fn" ident "(" fn-params ")" -> ty block

 item-struct ::= "struct" ident fields

      fields ::= fields-named | fields-pos

fields-named ::= "{" { field-named "," } "}"

 field-named ::= ident ":" ty

  fields-pos ::= "(" {field-pos ","} ")"

   field-pos ::= ty

  fn-params ::= { fn-param "," }

   fn-param ::= ident ":" ty

         ty ::= "i32"

      block ::= "{" stmt ";" { stmt ";" } "}"

      stmt  ::= local | expr

       expr ::= call-fn | term { "+" term }

      local ::= "let" ident ":" ty "=" expr

    call-fn ::= ident "(" args ")"

       args ::= { expr "," }

       term ::= factor { "*" factor }

     factor ::= ident | lit-int | "(" expr ")"

      ident ::= (letter | "_") { letter | digit }

     letter ::= "a" | "b" | ... | "z" | "A" | "B" | ... | "Z"

      digit ::= "0" | "1" | "2" | ... | "9"
```