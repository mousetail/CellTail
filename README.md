# CellTail

[Try the interactive online demo](https://mousetail.github.io/CellTail)

Cell tail is a 2d automata based programming language.

At the start of the program, one cell is created for every element of the input. Then a rule is applied to every cell until no more rules are possible or every cell is deleted.

A program is made up of rules, rules look like this:

    1,5,12:2,8,14

Read this like

> If the message from the left in the previous generation is 1, the same cell in the previous generation is 5, and the right in the previous generation is 12, then send 2 to the left, 8 down, and 14 to the right.

Of course, you can make these expressions much more complex, like this:

    (N,b),"hello",c:c+x,"world",b

This can be read like:

> If the value from the left of the current cell in the previous generation is tuple with where the left value is NULL and the right is the the value B, the value from the top is the string "hello", and the value from the right is some value C:
> Let c be the value from the right
> Add c and x then pass the result to the left, "world" down, and b to the right.

A program is made up of any number of these rules.

# Types

## None

`None`, represents no value. Can be appreviated as `N`. Also considered a empty list. Applying any operator to `None` yields the second operand.

## Tuple

A tuple is any number of types treated as a single value, like this: `(a,b,c)`. Applying any operator to a tuple will apply the operator to it's last value.

## Lists

Lists are actually just tuples. The first element of the tuple is the first element of the list, the second element is the "rest" of the list. For convenience you can define lists
like `[1, 5, 12]` but it will be equivelent to `(1, (5, (12, None)))`. A empty list is the same as the the value `None`. Writing strings like `"hello"` are considered lists of numbers, so hello would be equivelent to `(104, (101, (108, (108, (111, N)))))`

You can use the `+` operator to concatenate 2 lists.

## Numbers

A number represents any integer. There are no floating point numbers. All the basic operators are available, including `+`, `-`, `*`, `/`, and the bitwise XOR operator `^`.

When writing matching expressions for numbers, you can use `5..` for example to match numbers over 5, of `8..10` for numbers 8 and 9.

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

Null is considered the smallest, followed by all numbers, followed by tuples. So to check if a value is a number you can use:

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

All options for the `|` operator must bind the same variables. Otherwise some variables could be unbound.

# Modifiers

Special attributes can be set to modify how the program works:

## Input Mode

```
Input = Input Numbers; # Take a list of comma seperated numbers from STDIN
Input = Input Characters; # Take characters as input from STDIN, each byte will become a number of its'byte value
Input = CMD Numbers; # Take a list of comma seperated numbers as a single command line argument
Input = CMD Characters; # Take a string from command line arguments as input, with each byte becoming one number
Input = 5,12,-5; # take no input, initialize with the values 5,12,-5
```

You may also abbreviate each to only it's first character.

## Output Mode

There are 2 available output modes:

```
Output = Characters; # Attemptt to convert the output to character values, substituting ? for any numbers out of range
Output = Numbers; # Output as , seperated numbers
```

## Debug Mode

There are 2 values:

```
Debug = False; # Do not print intermediate states
Debug = True; # Print intermediate states
```

## Max iterations

You can use the `M` operator to set a limit on the maximum number of iterations. This is especially helpful on the web version which can't easily be killed.

```
Max=5; #Limit to 5 iterations
```

# Functions

Functions allow you to resuse expressions. They are also the only way to create something akin to a IF statement inside of a rule. A function is defined with the `fn` keyword:

```
fn bob x: x+1;
(z, bob z, bob (bob z)): z, bob z, z;
```

Every function takes exactly 1 argument. However, that argument may itself be a tuple containing multiple arguments.

You can define a function with the same name and they work like patterns: The first one that matches will be called.

```
fn div x,0: 1;
fn div x,y: x/y;

a,b,(c,d): a,div(b,c),N;
a,b,c: a,div(b,0),N;
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

Alternatively:

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

## Sorting a list, but also randomly replace some elements with others in specific situations

```
# The numbers to sort, can be mofifed to take them from input instead
I=999,9 ,1 ,3 ,2 ,1 ,5 ,1 3,8 83,7 ,- 1,1 4,8 ,9 99,1 5,4 ,1 7; 
O=N;    
D=T;    

N,   item & N..(),  N:       (),  (item, 0),     (); #  First frame, indicate to neighbors existence 
(),  (item, 0),     () | N:  (),  (item, 0),     (); #  If in the middle, do nothing                 
N,   (item, 0),     ():      (),  (item, 1, 5),  1; #   If on the left edge, start indicating index  

index,  (item, 0),  ():  (item, 1),  (item, 1, index-1),  index+4; #  Indicate the next index opposite polarity 
index,  (item, 0),  N:   (item, 1),  (item, 1, index),    N; #        End of the line                           

(prev_item, 2),  (item & prev_item.., 1 | 3, index),  (N, 2) | N | ():  (N, 2),  (item, 2, index-1),       (item, 2); #       Don't swap left 
(prev_item, 2),  (item, 1 | 3, index),                (N, 2) | N | ():  (N, 2),  (prev_item, 2, index-1),  (prev_item, 2); #  Swap left       
N,               (item, 1 | 3, index),                (N, 2) | N | ():  (N, 2),  (item, 2, index-1),       (item, 2); #       Left edge       

(N, 1) | N,  (item, 2, index),         (next_item & item.., 1):  (item, 1),       (item, 3, index-1),       (N, 1); #  Don't swap right                                                          
(N, 1) | N,  (item, 2, index),         (next_item, 1):           (next_item, 1),  (next_item, 3, index-1),  (N, 1); #  Swap right                                                                
(N, 1) | N,  (item, 2, index & -1..),  N | ():                   (item, 1),       (item, 3, index-1),       (N, 1); #  Right Edge                                                                
(N, 1) | N,  (item, 2 | 1 | 3, 0),     N | (_, -1):              (item, -1),      (item, -1),               N; #       Exit condition: If we get a kill signal exit. If the timer runs out exit. 
```
