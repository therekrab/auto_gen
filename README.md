# `auto_gen`

`auto-gen` is a tool for developing and modifying PathPlanner autonomous routines through the command line. It uses a powerful but simple language that allows users to modify autons easily.

## Examples

In PathPlanner, let's consider the following sequence:

<img width="496" height="526" alt="image" src="https://github.com/user-attachments/assets/2276b3ba-6ea7-4e9f-b437-13530e8ff978" />

Here, we run this sequence:

1. Firstly, we run `A` by itself.
2. Secnodly, we run `B` and `C` at the same time
3. Finally, we run `D`

We never have to open PathPlanner to edit or create this auton - not anymore. Instead, open a new file, say `auton.txt` and put the following contents in it:

```text
A + B & C + D
```

Let's break down the syntax here.

Firstly, note that we use the name of the named command without any special surrounding characters.
Even if the command name has a space in it, there are no symbols around the command name.
If all we had written was `"hello world"` instead of the real contents, this would be parsed as a command with the name `"hello world"` (including the quotes).
**Note**: This also will break the super-simple JSON builder I made.

Secondly, we seem to be adding and multiplying command names.
What's up with this?
Well, in the `auto_gen` language, the plus sign (`+`) indicates sequencing commands together, and the ampersand (`&`) runs them in parallel.
The parallel operator (`&`) has precidence, which is why the expression is equivalent to `A + (B & C) + D`.
That's right - this language supports grouping with parenthesis to make sure command flow is as you would expect.
Nesting these is also allowed.

You can use these operators as many times as you would like without grouping, however; `A + B + C` is just how three commands get put into a sequential group - `(A + B) + C` is unnecessary.

## Syntax

### Combination Symbols

The following symbols are currently supported in the `auto_gen` language, ordered from highest precedence to least:

| Symbol | Meaning | Example |
| -- | -- | -- |
| `*` | Race group | `A * B` runs A and B in parallel until any finish |
| `?` | Deadline group | `A ? B` runs A and B in parallel until A finishes |
| `&` | Parallel group | `A & B` runs A and B in parallel. |
| `+` | Sequential group | `A + B` runs A to completion, then B. |

More combinations (race groups, deadline groups) are coming and will be supported soon.

### New lines

You can increase organization by using new lines to help see the control flow.
At the end, the commands represented by each of the lines are grouped together into a sequential group.
This means that each line must be an indiviaully valid command sequence in the language.

The following examples are illegal:

```text
A +
B & C
```

The first line uses the `+` operator but doesn't supply a second operand.

```text
A + B + (C
& D & F)
```

This has two problems - firstly, the opening paren on the first line isn't closed on the same line - making it an invalid line.
Secondly, the second line begins with a `&` operator but no previous command name to use; again, invalid.

Proper use of newlines can be used to group related logic together:

```text
Intake & Drive to piece
Drive to score
Score
```

The above will spit out the exact same auton as `Intake + Drive to piece + Drive to score + Score` but is easier to read.

### Comments

Another bonus here is the ability to leave commends in the text file.
Use the `#` symbol, like in Python, to indicate a comment.
Everything after that is disregarded by `auto_gen`.
Example:

```text
# Intake sequence:
Align Intake + Intake # this is a valid comment, too.
```

Comments can really help organize and explain autonomous logic.

## Exporting a command

PathPlanner saves its commands to `src/main/deploy/pathplanner/autos/` and the commands are stored using JSON.
`auto_gen` also exports its autos to JSON, but it is up to the user to make sure that they get sent to the correct directory.

To transpile an auto from `auto_gen` syntax to PathPlanner JSON, it's as easy as this:

```sh
$ auto_gen auto.txt
```

This prints out to `stdout` the valid JSON corresponding to the auto in the file provided. To save it to a file, you can do something like this:

```sh
$ auto_gen auto.txt > src/main/deploy/pathplanner/autos/Auto.auto
```

Ensure that the filename ends with `.auto`, because that's what PathPlanner expects to see.

## Command auto-grouping

To avoid nesting commands deeper than they should be, `auto_gen` employs an auto-grouping algorithm for nested command groups.

For example, consider a group like this:

```text
A + (B + (C + D))
```

`auto_gen` doesn't need to create the following command tree:

```text
[sequential]:
  - A
  - [sequential]:
    - B
    - [sequential]:
      - C
      - D
```

Instead, `auto_gen` simply notices the grouped commands and flattens the group to be this:

```text
[sequential]
  - A
  - B
  - C
  - D
```

Autopilot also groups parallel groups inside parallel groups.
