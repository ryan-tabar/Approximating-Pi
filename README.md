# Approximating-Pi
Approximating pi using several monte carlo methods. (First time using Rust)

## How to run
  1. Make sure Rust is installed from https://www.rust-lang.org/tools/install
  2. Clone the repository
  3. Go to the directory of the cloned repository
  4. Type: cargo run
  5. Output should be displayed in terminal
  
## Motivation
I wanted to see if there was a language that was as powerful as C++ but handles memory more safely and had abstractions that didn't compromise performance. 
That's where I turned to Rust. I wanted my first project in Rust to be something basic because this language is new to me and has a lot of 
new paradigms to programming such as its ownership feature.

## What is this project about?
It approximates pi using three Monte Carlo methods. Monte Carlo methods are computational algorithms that rely on repeated random sampling to obtain numerical results.

The first method makes use of a circle inside a square with sides equal to the diameter of the circle. The ratio between the area of the circle and the area of the square is pi / 4.
By applying a random set of points to the square. One can approximate pi by the ratio of points landed inside the circle to the total number of points.

The second method is known as Buffon's needle. Take a set of parallel lines and drop needles on it.
pi is approximatly equal to (2 * n * l / x * t). Where n = number of times droped, l = length of needle, t = distance between lines, and x = number of needles crossed a line.

The final method uses averages distances of walks. Start a walk at position 0 and flip a coin. If heads, move in a positive position else, move in a negative position.
Do this steps number of times. Calculate the absolute distance from the origin and sum it cumulatively. Do this walk number of times. 
Average the number of absolute distances. pi is approximatly equal to 2 * steps / average_distance^2.

## Possible improvements
- Add some visualisations (possibly using SDL2 bindings for Rust)
