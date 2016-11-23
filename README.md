# OGLdev Tutorials in Rust

[OGLdev](http://ogldev.atspace.co.uk/) is a very nice serise of tutorials that introduces basic knowledge for writing a OpenGL program. The original tutorials are written in C. However, I want to reproduce the same results in [Rust](https://www.rust-lang.org/) for fun and learning. So that is why I created this repository.

I used [glium](https://github.com/tomaka/glium) to use OpenGL in Rust. It is quite different from the one used by the original tutorials. You may see lots of difference (including variable names, function calls... etc.) between those tutorials and my implementations. Just stay calm. Take some time to read carefully. You will find that they are basically the same. At least they try to archive the same goals.

The [documents](http://tomaka.github.io/glium/glium/index.html) of glium will be very useful when you dig my code.

## The Goal of Each Tutorial

1. [To Create A Window](src/bin/tutorial_01.rs)
2. [To Draw A Dot](src/bin/tutorial_02.rs)
3. [To Draw A Triangle](src/bin/tutorial_03.rs)
4. [To Use Shader Programs](src/bin/tutorial_04.rs)
  - This one does not have too much difference from the 2nd and 3rd tutorial since I have used shader programs in those tutorials.
5. [To Use Unifrom Variables](src/bin/tutorial_05.rs)
6. [To Perform Translation Transformation](src/bin/tutorial_06.rs)
7. [To Perform Rotation Transformation](src/bin/tutorial_07.rs)
8. [To Perform Scaling Transformation](src/bin/tutorial_08.rs)
9. [To Interpolate Color inside a Triangle](src/bin/tutorial_09.rs)

## How to Run It ?

Build them:

```
> cargo build
```

Run the `n`-th tutorial:

```
> target\debug\tutorial_n
```
