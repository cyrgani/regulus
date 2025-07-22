## Documentation

### Syntax
* comments start with `#` and continue until the end of the line
* everything else is either a function call, an atom or a variable
* function call syntax: `FUNCTION_NAME(ARG_1, ARG_2, ...)`
  * functions are commonly called `FUNCTION_NAME(ARGC)`, e.g. `+(2)` or `print(_)` (variable number of arguments allowed) 
* variable: any identifier that does not contain forbidden characters (including, but not limited to `(`, `)`, `#`, `,`)
* atom: one of:
  * integer: `1`, `-5`, ...
  * string: `"hello, world"`, ...
  * null: `null`
  * bool: `true`, `false`
  * (other non-literal atoms: lists, objects, function pointers)
* all meaningful operations and statements are function calls
* every program is implicitly wrapped in `_(` and `)` for convenience
* every program except for the STL itself automatically imports `__builtin_prelude`

### Builtins
* are present in every program without having to import them
* fundamental language features like `=(2)` or `def(_)` are among them
* see `src/builtins` for a list of them

#### Globals
* constructable with `global(1)`, example `global(x)`
* can be read and written from anywhere
* value is shared between all imported modules and function scopes, unlike regular local variables with `=(2)`
* there is no language mechanism to prevent name collisions at the moment
  * naming conventions are required (see below)

### Standard library (STL)
* still extremely minimal
* STL modules can be imported with `import(1)`, example `import(range)`

### Error handling
* any fallible operation can `raise` an `Exception`
* exceptions bubble up unless stopped with `catch`
* note: rust panics in Regulus outside of the `State` API are generally bugs and should be exceptions instead

### Naming conventions
* identifiers starting with `__stl` are reserved (this is not enforced though) for internal use in the STL, manipulating them is expected to cause panics or crashes
* identifiers starting with `__builtin` are similarly reserved for internal APIs and may be added, changed or removed at any time
