
# The Penmark Pencil Puzzling Language

Penmark is a declarative programming language and visaul programming
envirionment for rapidly creating enjoyable pencil puzzles, like Sudoku and it's
variants. Penmark is inspired by and draws from Excel, Prolog, SVG, Rust and
Elixir. The reference implementation and editing environment is written in
Elixir. Penmark currently is intrepreted and has static types.

```
sudoku = new Puzzle()

sudoku.grid
1 2 3 | . . . | . . . 
1 2 3 | . . . | . . . 
1 2 3 | . . . | . . . 
- - - + - - - + - - - 
1 2 3 | . . . | . . . 
1 2 3 | . . . | . . . 
1 2 3 | . . . | . . . 
- - - + - - - + - - - 
1 2 3 | . . . | . . . 
1 2 3 | . . . | . . . 
1 2 3 | . . . | . . . 

def rule classic_sudoku(g:Grid):
    each c in g.columns: all_distinct(c) end
    each r in g.rows: all_distinct(r) end 
    each x in 1..3, y in 1..3: 
        all_distinct(g[square({1+3x,1+3y},3)])
    end
    each c in g.cells: 
        c in 1..9
    end 
end

def rule knights_move(o:Cell)
    . x . x . 
    x . . . x 
    . . o . .
    x . . . x 
    . x . x .
end 

def rule knights_move_part_2(o:Cell)
    . y . y . 
    y . . . y 
    . . x . .
    y . . . y 
    . y . y .
    x != y+1
end 

sudoku.rules += classic_sudoku
sudoku.rules += knights_move

sudoku.solve() # naive backtracking
=> solved_sudoku
sudoku.human_solve() # solve using built in human like techinques
=> Explanation, .solution, .steps
sudoku.grade() # using the human solve 
=> Grade, .estimate 300, .diffuclty hard
```

# Motivation 

CTC miracle sudoku 
variants 
always done killer sudoku 
cross sudoku 

Wanted to start creating my own 
Started on a visual editor 
Found it to be easy enough to do 

But lacking 

Building from bottom up feels like it misses what could be made. 

# Types

## Rational numbers
```
1
100
1.5
-1
3/2
type(1)
# Number
```

## String
```
"An example of a description of a rule or title."
"""
Multiline strings are cool and useful. 
"""
"There probably should be interpolation #{if x==y then "yes" else "no"}"
```
Any valid utf8 string.

## List<Type>
```
[]
[1,2,3]
```
1 indexed lists (maybe?)

## Sets<Type>
```
#{}
#{1,2,3}
```

## Symbols
```
x
y 
anti_knight
puzzle
```

## Comments 
```
# Blah
x.x # blah blah 
# Blah
```

## Coordinates
```
A1
r1c1
{1,1}
{"A",1}
{0.5,0.5} # only wholes and halves are all allowed in coordinates for now. This lefts you specify borders vs middle for svg. Maybe include thirds later on too, who knows. 
```

## Coordinate Range
A list of coordinates, not necessarily contiguous or rectangular.
```
A1:A3 # A1, A2, A3
A1:Z5 # All the cells in a rectangle with A1 and Z5 as opposing corners
r1c1:r9c9 # standard sudoku grid 
square(r1c1,9) # same thing, just calling a function to create it and do the math

# Defining grid for Samuri Sudoku https://www.samurai-sudoku.com/ 
upper_left = square(r1c1,9)
middle = square(r7c7,9)
upper_right = rect(r13c13,9,-9) # Negatives mean go up and left, positive numbers mean go down and right
lower_left = rect(r13c7,-9,9) # this is when a visual programming environment would be handy, I don't enjoy doing all this math in my head
lower_right = rect(r)

samuri_sudoku_grid = upper_left + middle + upper_right+ lower_left+ lower_right
```

## Grids
```
.x.
.o.
...

1 2 3 _ _ _ 1 3 4
4 . 6 _ _ _ 2 3 4
7 8 9 _ _ _ . . .
```

These are useful for specifying clues as well as patterns for rule sets.

## Variable
```
...
.x.
.y.
x != y+1
```
Used in patterns to specify rulesets. 


## Svg
```
<svg> <rect x=10 y=20 /> </svg>
```
No tuples, floats

Within SVG, the length of a cell is about 20 pixels and within SVG that will be
considered 1. 0 is the left of the cell, 0.5 middle and 1 the right wall. 

# Visual editor 

There should be a visual interface available at all times when editing Penmark
scripts. Working with grids and placement of cells is tough if you cannot see
how they are interacting together and looking. Moreover, one should be able to
click and edit the grid and have that be reflected in the textual
representation. 

Editor should have sections that:
* Show existing grid layout and let one modify it.
* Show all the rules and visuals for how they work and interact with one another
  (like knight's move, non consectutive, right o matic).
* Allow for editing of clues given in the puzzle grid. 
* Allow for editing of constraints on the puzzle (like thermo's, windoku, sums,
  sandwich, palindromes, renbans, etc.).
* Editor is constantly solving the puzzle as you go, showing both the number of solutions
  and, if there is a unique solution to the puzzle, the likely steps a human
  would take and the approximate difficuty overall of the puzzle. 
* If there's no unique solution, show all the candidates that are left in each cell.

# Reserved keywords

```
for #or each?
def
rule
technique
module
```

# Expressions

Infix notation

assignment with =

# built in functions
```
+ - %  ==
all_distinct
sum
gcd
even?
odd?
every?
some?
not
square 
rect 
line
```

# Built in Objects

Puzzle
.cells Grid
.rules List<Rules>
.constraints?
.tokens Set<Numbers> or Set<Strings>

Grid, 
.cells, Coordinate -> Cell, list of lists, matrix? 
.style, for the g containing the whole thing

Cell
.x
.y
.value
.style, styling of the rect that automatically gets created

Modules
anything defed is expressible

Rules
.condition
.svg

Technique
.description
.conditions
.implictions
.difficulty

Function
.call
.apply ?

# Expressions/Syntaix

each x in list

# stdlib 
```
groups
cages
non_consective
```

# Examples
``` 
rule anti_knight
. X . X .
X . . . X
. . O . .
X . . . X
. X . X .
end

rule anti_knight_plus_one
. y . y .
y . . . y
. . x . .
y . . . y
. y . y .
y != x + 1 mod 9
end

rule anti_successor
"The successor of a cell cannot be within a knights move of that cell"
y y y
y x .
y y y
y != x + 1
end

rule 159
cell
if cell.column = 1 or cell.column = 5 or cell.column = 9 then
grid[x.value][x.row] = cell.column
end
end

grid
...
...
...

... ...
... ...
... ...

... ... ...
... ... ...
... ... ...

    ... ...
    ... ...
    ... ...

        ...
        ...
        ...

end

tokens = :numbers
clues
A1 = 9
```

tuples
(,) -> eval to the right thing

A1:I9


```
rule thermo(list:[])
for i in 1..len(list):
list[i] < list[i+1]
end

rule cage(list:[],sum:n)
sum(list) = n
end

rule windoku(list:[],sum:n)
all_distinct(list)
end

rule row(list:[])
set(list) = :tokens
end

thermo(A1, A2, A3, A4)
```

modules imports

std.lib

modules are objects

typed args for functions

+= std.lib.

can do math on the coords, + -

recreate all of the sudokus from cross + a

# Notes 

borders
candidates
"outside" above a cell
solution
drag and drop?
snap to grid
fine to have cells overlapping when adding them together 
unique on the set


relative layouts

. . .

1 2 3 4 5 6 7 8 9

x x x
x o x
x x x

. x .
x o x
. x .

for x and x+1 mod % 9 only
. . .
. x x+1
. . .

eight queens

all filled
exactly one

or

c c c
c o c
c c c

x . x
. o .

what of the shapes

!=

where

how they get highlighted
Syntactic whitespace?

when if for patterns 