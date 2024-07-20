### Designing a Generic Keybind System for a Text Editor

#### Keybind Types

Keybinds are categorized into types that describe their roles. These types include:

- **Count**: Is A Prefix that determines how often a command should be executed. Comes with limitations of which commands are valid (`2dk` is valid, where `2da`( isnt)
- **WriteOperation**: Operations that modify the buffer (e.g., delete).
- **ReadOperation**: Operations that read from the buffer without modifying it.
- **Select**: Operations that select a portion of the text.
- **Modifier**: Modifiers that alter how the operations are applied (e.g., around, inside).
- **ExactMatch**: Characters used to find specific text objects (e.g., `(` for selecting text within parentheses).

#### Command Buffer

In Normal mode, all inputs are collected into a command buffer. The buffer is analyzed to determine if the sequence of keybinds forms a valid command.

#### Command Interpretation

1. **Initial Input**:
    - The first input is expected to be a **WriteOperation**. This indicates an operation that will modify the buffer (e.g., `d` for delete).

2. **Modifier or Selector**:
    - After an operation, a **Modifier** or a specific selector is required. This provides context for how the operation should be applied.

3. **ExactMatch**:
    - An **ExactMatch** keybind (e.g., `(`) indicates the text object to be acted upon.

#### Example: `da(` Command

1. **Input**: `d` (WriteOperation - Delete)
    - Valid initial input; expect a modifier next.
    - A Modifier has to be followed by an ExactMatch
2. **Input**: `a` (Modifier - Around)
    - Valid modifier; expect a character that specifies the text object.
    - A Modifier has to be followed by an ExactMatch
3. **Input**: `(` (ExactMatch)
    - Specifies the text object to be deleted (text around parentheses).
    - An ExactMatch is always the end of a keybind sequence

#### Execution

Upon recognizing a valid sequence, the command is executed in reverse:

1. **Identify Text Object**:
    - Find the opening `(` to the left of the cursor.
    - Find the closing `)` to the right of the cursor.
2. **Apply Modifier**:
    - If the modifier specifies to include the parentheses, adjust the range accordingly.
3. **Perform Operation**:
    - Delete the identified text range, including the specified characters if applicable.
