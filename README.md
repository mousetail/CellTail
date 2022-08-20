# CellTail
Cell tail is a 2d automata based programming language.

At the start of the program, one cell is created for every element of the input. Then a rule is applied to every cell until no more rules are possible or every cell is deleted.

A program is made up of rules, rules look like this:

    1,5,12:2,8,14

Read this like

> If the message from the left is 1, the same is 5, and the right is 12, then send 2 to the left, 8 down, and 14 to the right.

Of course, you can make these expressions much more complex, like this:

    [x,b],"hello",c:c+x,"world",b

This can be read like:

> If the value from the left is a array, the value from the top is the string "hello", and the value from the right is some value C:
> Let x be the first element of the list and b be the rest.
> Let c be the value from the right
> Add c and x then pass the result to the left, "world" down, and b to the right.

A program is made up of any number of these rules.

# Types

## None

`None`, represents no value. Can be appreviated as `N`. Also considered a empty list.

## Tuple

A tuple is any number of types treated as a single value, like this: `(a,b,c)`. You can get a single value from a tuple using `t.0` but more comonly you would unpack it first.

## Lists

Lists are actually just tuples. The first element of the tuple is the first element of the list, the second element is the "rest" of the list. For convenience you can define lists
like `[1, 5, 12]` but it will be equivelent to `(1, (5, (12, None)))`. A empty list is the same as the the value `None`. Writing strings like `"hello"` are considered lists of numbers, so hello would be equivelent to `(104, (101, (108, (108, (111, N)))))'

You can use the `++` operator to concatenate 2 lists. You can use the `$` operator to get the last value of a list. You can also index a list u using `[]`.

## Numbers

A number represents any integer. There are no floating point numbers. All the basic operators are available, including `+`, `-`, `*`, `/`, `&`, `|`, `^`, `**`.

When writing matching expressions for numbers, you can use `5..` for example to match nubmers over 5, of `8..10` for numbers 8 and 9.

# Rules

Each rule starts with a matching expression then a `:` then the resulting value. The matching expression will be a 3 tuple containing the element from the left, center, and right, then returns a 3 tuple for values passed in each direction.

The first matching rule from top to bottom is always evaluated.
