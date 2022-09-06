# CellTail

[Try the interactive online demo](https://mousetail.github.io/celltail)

Cell tail is a 2d automata based programming language.

At the start of the program, one cell is created for every element of the input. Then a rule is applied to every cell until no more rules are possible or every cell is deleted.

A program is made up of rules, rules look like this:

    1,5,12:2,8,14

Read this like

> If the message from the left is 1, the same is 5, and the right is 12, then send 2 to the left, 8 down, and 14 to the right.

Of course, you can make these expressions much more complex, like this:

    (N,b),"hello",c:c+x,"world",b

This can be read like:

> If the value from the left is tuple starting with null, the value from the top is the string "hello", and the value from the right is some value C:
> Let x be the first element of the list and b be the rest.
> Let c be the value from the right
> Add c and x then pass the result to the left, "world" down, and b to the right.

A program is made up of any number of these rules.

# Types

## None

`None`, represents no value. Can be appreviated as `N`. Also considered a empty list. Applying any operator to None yields the second operand.

## Tuple

A tuple is any number of types treated as a single value, like this: `(a,b,c)`. Applying any operator to a tuple will apply the operator to it's last value.

## Lists

Lists are actually just tuples. The first element of the tuple is the first element of the list, the second element is the "rest" of the list. For convenience you can define lists
like `[1, 5, 12]` but it will be equivelent to `(1, (5, (12, None)))`. A empty list is the same as the the value `None`. Writing strings like `"hello"` are considered lists of numbers, so hello would be equivelent to `(104, (101, (108, (108, (111, N)))))'

You can use the `+` operator to concatenate 2 lists.

## Numbers

A number represents any integer. There are no floating point numbers. All the basic operators are available, including `+`, `-`, `*`, `/`, `^`.

When writing matching expressions for numbers, you can use `5..` for example to match nubmers over 5, of `8..10` for numbers 8 and 9.

# Patterns

Each rule starts with a matching expression then a `:` then the resulting value. The matching expression will be a 3 tuple containing the element from the left, center, and right, then returns a 3 tuple for values passed in each direction.

The first matching rule from top to bottom is always evaluated.

Rules can contain expressions themselves, for example a rule like this:

```
a,a+1,a+2:0,1,2
```
Would match 3 increasing numbers. However, the variable must appear in the "raw" form to the far left. So this would be invalid:
```
a+1,a+2,a+3:0,2,2; # Variable never defined raw
a+3,a+1,a:0,1,3; # Variable is used in a expression before being defined raw.
```

## Ranges

You can use the `..` operator to check if a value is in a range, for example:

```
7..12
```

Means that the value must be between 8 and 11. Ranges are always exclusive on both sides. If you want a inclusive range you can use `7|7..12` for example.

Numbers compare lower than literals which compare lower than tuples. So to check if a value is a number you can use:

```
N..()
```

Or the more common case where you just want to check not null:

```
N..
```

Tuples are compared lexicographically.

## Combining Operators

The `&` and `|` operators can be used to combine different patterns. For example, this rule:

```
(N,_,_)|(_,_,N):N,N,N
```
Will match values bounded on either side by NULL.

The `&` operator is useful for binding variables as well as checking some condition. For example, this checks if a value falls into a range and binds it to a:

```
..7&a
```

(This works similarly to the `@` operator in rust except there is no limit to the number of variables you can bind)

All options for the `|` operator must bind the same variables. Otherwise some variables could be un

# Modifiers

Special attribibutes can be set to modify how the program works:

## Input Mode

```
Input = Input Numbers; # Take a list of comma seperated numbers from STDIN
Input = Input Characters; # Take characters as input from STDIN, each byte will become a number of its'byte value
Input = CMD Numbers; # Take a list of comma seperated numbers as a single command line argument
Input = CMD Characters # Take a string from command line arguments as input, with each byte becoming one number
Input = 5,12,-5 # take no input, initialize with the values 5,12,-5
```

You may also abbreviate each to only it's first character.

## Output Mode

There are 2 available output modes:

```
Output = Characters # Attemptt to convert the output to character values, substituting ? for any numbers out of range
Output = Numbers # Output as , seperated numbers
```

# Debug Mode

There are 2 values:

```
Debug = False # Do not print intermediate states
Debug = True # Print intermediate states
```

# Functions

Functions allow you to resuse expressions. They are also the only way to create something akin to a IF statement inside of a rule. A function is defined with the `fn` keyword:

```
fn bob x: x+1
(z, bob z, bob (bob z)): z, bob z, z
```

Every function takes exactly 1 argument. However, that argument may itself be a tuple containing multiple arguments.

You can define a function with the same name and they work like patterns: The first one that matches will be called.

```
fn div x,0: 1
fn div x,y: x/y

a,b,(c,d): a,div(b,c),N
a,b,c: a,div(b,0),N
```

Functions may not call other functions, except built in functions when they are implemented.

If you call a function but no pattern matches, a warning is printed and `Null` is returned.

# Example Programs

## Hello World

Takes P as input

```
I='p';
N,80,N:N,N,(104, (101, (108, (108, (111, (32, (119, (111, (114, (108, (100, N)))))))))));
(p,q),N,N:N,p,q;
```

Alternatively:

```
I='p';
N,80,N:N,N,"hello world";
(p,q),N,N:N,p,q;
```

Alternatively
```
I="Hello world";
```

## Countdown

Takes a letter as input, counts down to A

```
'A',N,N:N,N,N;
N,L,N:N,N,L;
A,N,N:N,A,A-1;
```

Counts down from the given letter down to A.

## Primes

```
I=-1; # Start with the special value -1
D=false; # Debug = False
O=N; # Output as numbers
N,-1,N :                        N,(1,1,1),N; # Initial value: 0, 0, 0
# Recursing base case to prevent infinite loop
174, N,N:                        N,N,N;

# number, factor, modulo

# Found a prime, number equals factor
A, (number, number, modulo), N:    N, number, number + 1;

# Modulo is 0, so it's not a prime
A, (number, factor, 0), N:      N, (number + 1, 2), N;

# Did not find a prime or 0 factor
A, (number, factor), N:         N, (number, factor, number%factor), N;
A, (number, factor, modulo), N: N, (number, factor+1, number%(factor+1)), N;
# First Step
number, N, N:                   N, (number, 1, number), N;
```

## Sorting a list

```
I=9,1,3,2,1,5,13,883,7,-1,14,8,999,15,4,17;
O=N;
D=T;

N,u,N:u,u,u;
(N,N,_)|(_,N,N):N,N,N;
u,v&..u,w&..v&N..:w,(v,v),u;
_,(v,v),_:v,v,v;
u,..u,_:u,u,u;
_,a,..a&c&N..:c,c,c;
```
