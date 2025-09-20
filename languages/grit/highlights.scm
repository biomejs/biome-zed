; =============================================================================
; GritQL Syntax Highlighting Rules
; =============================================================================

; TODO(daivinhtran): Use @injection to highlight our supported languages inside
; GritQL. https://zed.dev/docs/extensions/languages#code-injections

[
  (variable) 
  (underscore)
  (languageName)
] @variable

[
  (codeSnippet)
  (doubleQuoteSnippet)
] @string

[
  (intConstant)
  (signedIntConstant)
  (doubleConstant)
] @number

(booleanConstant) @boolean

(comment) @comment

[
  (predicateCall
    name: (name))
  (nodeLike
    name: (name))
  (name) 
] @function

(nodeLike
  named_args: (namedArg)) @attribute

(languageSpecificSnippet
  language: (languageName) @label)

; TODO(daivinhtran): Tweak tree-sitter-gritql to allow biome engine
["marzano"] @variable.special

[
  "bubble"
  "sequential"
  "multifile"
  "and"
  "any"
  "not"
  "maybe"
  "contains"
  "until"
  "as"
  "within"
  "after"
  "before"
  "some"
  "every"
  "limit"
  "includes"
  "like"
  "private"
  "if"
  "else"
  "where"
  "or"
  "orelse"
  "return"
] @keyword

[
  "*"
  "/"
  "%"
  "+"
  "-"
  "!"
  "="
  "+="
  ">"
  "<"
  ">="
  "<="
  "!="
  "=="
  "<:"
] @operator

[
  ";"
  "."
  ","
  ":"
] @punctuation.delimiter

[
  "("
  ")"
  "["
  "]"
  "{"
  "}"
] @punctuation.bracket

[
  (regex)
  (snippetRegex)
] @string.regex
