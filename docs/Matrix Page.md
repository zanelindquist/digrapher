# Matrix Calculator UI

This feature will add the basic necessities for a functioning matrix calculator. It will let users do operations like multiplication, addition, subtraction, and boolean operations.

## Types

| Name | Type | Purpose | Members |
| ---- | ---- | ------- | -------- |
| MatrixEquation | `struct` | Provide the base for this feature | `raw_text: String`, `edit_raw_text()`, `terms: Vec<Term>`, `parse_raw()`, `evaluate()`, `draw(&CanvasPosition)`|
| Term | `enum` | Enum for a term that is either a vector or a scalar | `Matrix()`, `Scalar()` | 
| Matrix | `struct` | Store vector values and mathematical operations involving scalaras and other matrices | `determinate()`, `cross_product(&Matrix)`, `mulitply_v(&Matrix)`, `add(&Matrix)`, `subtract(&Matrix)`, `multiply_s(&Scalar)`
| Scalar | `struct` | Hold operations for scalars | *Include same operations as Matrix contains depending on communicability*
| MatrixEquationResult | `type` | Result of processing text in the hopes of producing a matix equation | `Result<MatrixEquation, ParsingError>` |
| ParsingError | `struct` | Record and communicate to the user what errors occurred while inputting a relation. This is with formatting, and is different from a MathError | e.x. `Parenthesis not closed` |
| MathError | `struct` | This is an errror that will be thrown when the MatrixEquation evaluation engine runs into an illegal mathematical statement | e.x. `Matrix dimension mismatch`, `Division by zero`, `Invalid operation`
| EvaluationTerm | `type` | Used in the evaluation algorithm to manage the stack. | (`i32`, `String`)

## Components
<!-- <img src="../assets/doc_images/matrix_calculator/ui_layout.png" width=400/> -->

| Name | Purpose | Major Variables | Props | Children |
| ---- | ------- | --------- | ----- | -------- |
| `<MatrixPage/>` | Serve as the joint between the input and the display. Allows each part to pass information back and forth. | `matrix_equation: MatrixEquationResult`, `object_selection: ObjectSelection`| None | `<EquasionEditor/>`, `<Canvas/>`|
| `<EquasionEditor/>` | Display specific editors based on the `object_selection` and pass changes up to the `<MatixPage/>` to communicate to the canvas. Minimize the display time and instead let the editing mostly be handled by the canvas | None | `matrix_equation`, `object_selection` | `matrix_editor` `formula_editor` |
| `<MatrixEditor/>` | Allow the user to create and edit matrices in an erganomic fashion | `current_position: (i32, i32)` | `matrix_equation` | None
| `<FormulaEditor/>` | Allow the user to directly edit the syntax of the relation OR access operators | `is_editing_raw: bool` | `matrix_equation`, `object_selection` | None

## Pipeline

| Step | Operation | Output | Description | 
| ---- | --------- | ------ | ----------- |
| User presses add matrix button in `<FormulaEditor/>` toolbar | `matrix_equation.edit_raw_text()` | `matrix_equation` | The toolbar is used to add a new vector at the insertion point |
| `matrix_equation` updates | `matrix_equation.parse_raw()` | MatrixEquationResult | The expression is parsed into its term components, or a Parsing Error or Math error is returned |
| `matrix_equation` updates | `matrix_equation.draw()` | `Html` | The raw math equation text is parsed into terms and operations that are displayed on the canvas |
| User moves mouse on canvas | `Term.is_hovered(x, y)` | ObjectSelection | Checks if each term is being hovered, if so, set it as the hovered term on the ObjectSelection |
| User clicks down | | ObjectSelection | Set the selected term in the `object_selection` variable, and let the `<EquasionEditor/>` handle which pane pops up


# Matrix Calculator Parsing Engine

The parsing engine will try to use normal math symbols as normally as possible. However, some special syntax will be needed for various purposes, namely inputting and displaying matrices.

## Syntax
| Syntax | Description |
| ------ | ----------- |
| `\matrix{(rows, cols), (a, b, c, d...)}` | Define a matrix with rows, columns, and values. Throws a parsing error on failure |

### Chart Key
| Shorthand | Description |
| --------- | ----------- |
| `s` | Scalar |
| `m` | Matrix |

### Operators

Operations will be used to identify and invoke the class-specific methods while evaluating an equation

| Name | Operands | Description |
| ---- | -------- | ------- |
| `+` | (s + s), (m + m), (m + s) | Adds two terms. Or add a scalar to every cell |
| `-` | (s - s), (m - m), (m - s) | Subtract two terms. Or subtract a scalar from every cell |
| `/` | (s / s) | Divide two scalars. |
| `*` | (s * s), (m * m), (m * s), (s * m)? | Scalar or vector multiplication |
| `^` | (s ^ s), (m ^ s) | Exponents |
| `x` | (m x m) | Take the cross product of two matrices |
| `⊙` (or `o`) | (m ⊙ m) | Boolean multiplication |
| `.` | (m . m) | Calculate the dot product of two matrices |
| `v` | (m v m) | Logical and |
| `∧` | (m ∧ m) | Logical or |
| `trans(m)` | (trans(m)) | Calculate the transpose of a matrix |
| `det(m)` | (det(m)) | Calculate the determinate of a matrix |

## Algorithm

### Parsing into term objects
This parsing engine will used to produce terms in the correct order for the `MatrixEquation.terms` variable. This will just be done through finding individual terms and converting them to objects. Only parsing errors will be produced, this function will not check for mathematical errors. The `terms` member will only be used for display only, it should not be counted as a reliable source for sequentially evaluating an equation from left to right, since PEMDAS may not apply

### Evaluating the equation
#### Order of operations
When it is time to evalute the equation, the algorithm will iteratively perform PEMDAS
- `P` : Everything in parenthesis will be iteratively evaluated first. This includes the contents of function parenthesis
- `E` : Exponents and unary operators will be applied. Subsequently, functions will also be evaluated in this step
- `M` : Every type of multiplication will be performed
- `D` : Division will occur
- `A` : All types of division will occur
- `S` : Subtraction will occur
- The resulting value must be either a scalar or a matrix, otherwise a `MathError` will be returned.

#### Algorithm
This algorithm will use a stack to keep track of its operations and iteratively evalute the expression.
```python
while stack.len() > 0 {
    evalute = stack.pop()

}
```

1. Search the raw text input for scalars and `\matrix{...}` declarations. Replace them with an id in the string, and store them in a `HashMap<TermId, Term>`
2. Declare a stack of `Vec<EvaluationTerm>` with the raw equasion as the only element
3. From left to right, identify parenthesis. Anything in parenthesis, add it's contents into the stack