(echo_off) @keyword

(comment) @comment

(label) @label

(set_keyword) @keyword
(variable_name) @variable
(set_option) @constant
(assignment_literal) @string
(arithmetic_expression) @string

(if_stmt) @keyword
(for_stmt) @keyword
(goto_stmt) @keyword
(call_stmt) @keyword
(setlocal_stmt) @keyword
(endlocal_stmt) @keyword
(exit_stmt) @keyword

(comparison_op) @operator
(redirect_op) @operator
(fd_redirect) @operator

(command_name) @function

((variable_reference) @variable.special
  (#match? @variable.special "(?i)^[%!](CD|ERRORLEVEL|DATE|TIME|RANDOM|CMDCMDLINE|CMDEXTVERSION|HIGHESTNUMANODENUMBER|__APPDIR__|__CD__)[%!]$"))

(variable_reference) @variable

(for_set_literal) @string

(for_variable) @variable

(for_options) @constant

(string) @string

(integer) @number

(command_option) @constant

(argument_value) @string

(redirect_target) @string.special

[
  "("
  ")"
] @punctuation.bracket

[
  "&"
  "&&"
  "|"
  "||"
] @operator
