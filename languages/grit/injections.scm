; By default, highlight all snippetContent nodes as JavaScript
((snippetContent) @injection.content
  (#set! injection.language "javascript"))


; The rule below only matches the top-level code snippet before "where"
; Currently, tree-sitter doesn't have a way to bind the target language
; to all snippetContent nodes
; Upstream changes to both tree-sitter and tree-sitter-gritql
; are needed in order to fully support all CSS code snippets

; Highlight snippetContent as CSS if it's specified
(source_file
  language: (langdecl
    name: (languageName) @injection.language)
  pattern: (patternWhere
    pattern: (codeSnippet
      source: (backtickSnippet
	      content: (snippetContent) @injection.content))))
