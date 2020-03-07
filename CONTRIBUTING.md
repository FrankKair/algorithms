# Contributing guidelines

When submitting pull requests in a particular programming language, this document is a reference.
This pattern / framework was adopted to facilitate comparisons between solutions.

### Pull Request / Files

Pull requests should be named with the language and Euler problem, like: "C++ 036", "Rust 006".

You should add the programming language tag as well.

Files should be named following the pattern p001.cpp, p035.rs, p110.swift, etc.

### Bash

```bash
function() {
    # Body
}

solve() {
    # Call other functions
}

solve
```

### C

```c
#include <stdio.h>

int function(int x) {
    // Body
}

int solve() {
    // Call other functions
}

int main() {
    printf('%d\n', solve());
    return(0);
}
```

### C++

```cpp
#include <iostream>
using namespace std;

int function(int x) {
    // Body
}

int solve() {
    // Call other functions
}

int main() {
    int result = solve();
    cout << result << endl;
}
```

### Clojure

```clojure
(defn function1 [x]
  ; function body
)

(defn solve [] (function1 a))

(prn (solve))
```

### Crystal

```crystal
def function
  # Body
end

def solve
  # Call other functions
end

puts solve
```

### D

```d
import std.stdio;

int function(int x) {
    // Body
}

int solve() {
    // Call other functions
}

void main() {
    int result = solve();
    writeln(result);
}
```

### Elixir

```elixir
defmodule Problem001 do
  defp function(x) do
    # Body
  end

  def solve do
    # Call other functions
  end
end

IO.puts Problem001.solve
```

### Erlang

```erlang
-module(p001).
-export([solve/0]).

function(X) -> % Body.

solve() ->
  % Call other functions
```

### F#

```fsharp

let f x = // Body

let solve = // Call other functions

printfn "%i" solve // For printing integers

```

### Futhark

```futhark
let function(n: i32): i32 = -- Body

let solve(): i32 = -- Call other functions

let main(): i32 = solve()
```

### Go

Run `go fmt` before commit.

```go
package main

import "fmt"

func function(x int) int {
  // Body
}

func solve() int {
  // Call other functions
}

func main() {
  result := solve()
  fmt.Println(result)
}
```

### Groovy

```groovy
def function = {
  // Body
}

def solve = {
  // Call other functions
}

def result = solve()
println(result)
```


### Haskell

```haskell
function1 :: a -> b
function1 x = -- Body

function2 :: b -> c
function2 y = -- Body

main :: IO
main = print $ -- Call other functions
```
Note: type signatures are mandatory to all top-level functions.

### Java

```java
public final class p001 {
    private static int function(int x) {
        // Body
    }

    private static int solve() {
        // Call other functions
    }

    public static void main(String[] args) {
        int result = solve();
        System.out.println(result);
    }
}
```

### JavaScript

```javascript
function func(x) {
  // Body
}

function solve() {
  // Call other functions
}

const result = solve()
console.log(result)
```

### Julia

```julia
function func(x)
    # Body
end

function solve()
    # Body
end

result = solve()

println(result)
```

### Lua

```lua
function func(x)
  -- Body
end

function solve()
  -- Call other functions
end

local result = solve()
print(result)
```

### Objective-C

```objective-c
#import <Foundation/Foundation.h>

@interface Problem001 : NSObject
- (int) solve;
@end

@implementation Problem001
- (int) solve {
  ...
}
@end

int main() {
  @autoreleasepool {
    Problem001 *p = [[Problem001 alloc] init];
    int result = [p solve];
    NSLog(@"%d", result);
  }
}
```

### OCaml

Use [dune](https://github.com/ocaml/dune) to build OCaml programs with the following:

```
(executable
 (name      p___)
 (libraries base core_kernel stdio))
```

```ocaml
Open Base
Open Stdio

let solve =
  ...

let () =
  printf "%d\n" solve
```

### Perl

```perl
use strict;

sub function(x) {
    # Body
}

sub solve {
    # Call other functions
}

my $result = solve;
print($result);
```

### Python

```python
def function(x):
    # Body


def solve():
    # Call other functions


if __name__ == '__main__':
    result = solve()
    print(result)
```

### Rust

```rust
fn function(x: i32) -> i32 {
    // Body
}

fn solve() -> i32 {
    // Call other functions
}

fn main() {
    let result = solve();
    println!("{}", result);
}
```

### Ruby

Use the gem `rubocop` (`$ gem install rubocop`).
Run `$ rubocop --auto-correct` before commit.

```ruby
def function
  # Body
end

def solve
  # Call other functions
end

puts solve
```

### Scala

```scala
object Main {
  def function(x: Int): Int = {
    // Body
  }

  def solve(): Int = {
    // Call other functions
  }

  def main(args: Array[String]): Unit = {
    val result = solve()
    println(result)
  }
}

```

### Scheme
```scheme
(define func)

(define solve
  ; Use other functions
)

(display solve)
```

### Swift

```swift
func function(x: Int) -> Int {
    // Body
}

func solve() -> Int {
    // Call other functions
}

let result = solve()
print(result)
```

### TypeScript

```typescript
function func(x) {
  // Body
}

function solve(): number {
  // Call other functions
}

const result = solve()
console.log(result)
```

### Zig

You should probably use `zig fmt file.zig` to format your file

```zig
const std = @import("std");
const warn = std.debug.warn;

pub fn main() void {
    // Solution logic
    warn("{}\n", result);
}

```
