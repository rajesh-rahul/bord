# Abbreivations
`KW` - Keyword
`L_*` - Left
`R_*` -  Right

`Bin*` - Binary
`p` - Parser
`r` - Recovery Set(used in parsing code)
`tk` - Token
`m` - Marker (used in parsing code)
`op` - Operator
`CHEV` - Chevron - things like `<` or `>`
`LT` - Less than
`GT` - Greater than
`LTE` - Less than or Equals
`GTE` - Greater than or Equals
`lit` - literal used in parsing context, refers to things like a literal number or string: `112`, `'hello'`
`param` - parameter
`lhs` - Left hand side
`rhs` - Right hand side
- Token are an UPPER_SNAKE_CASE
- TreeNode are TableCase
- qualified: Refers to fully qualified paths of columns/ table names, like schema.table.column or schema.table

# Reading Comments
Some comments may have 'Section [SECTION_NUMBER]`. This is a way to precisely attach a comment to some lines of code. Sometimes, its not clear if a particular comment applies only to line/block immediately below it or spans further. Sections have the following rules
- Section must be used inside functions
- A particular section's scope start immediately after the Section comment ends
    - The scope of a section terminates when there is a new section or the function its defined in ends.
- Sections may have sub sections and they work similar to top level sections

