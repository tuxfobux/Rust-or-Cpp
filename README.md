# Rust-or-Cpp
Little research for school to compare the efficiency of Rust and C++. Coded three C++ programs that used different algorithms to generate relatively bad sudokus. After that implemented the same in Rust and compiled everything using the maximum optimization settings. Used `psrecord` to generate graphs of each programs memory as well as CPU usage.

## Results

<p align="center">
  <b>C++ vs Rust first algorithm</b>
</p>
<p align="center">
  <img src="graphs/cpp1.png" width="440" height="330">
  <img src="graphs/rust1.png" width="440" height="330">
</p>
<p>
  Rust consumes 1MB less memory and executes the program 38% quicker.
</p>

<p align="center">
  <b>C++ vs Rust second algorithm</b>
</p>
<p align="center">
  <img src="graphs/cpp2.png" width="440" height="330">
  <img src="graphs/rust2.png" width="440" height="330">
</p>
<p>
  Execution speed is around the same but C++ user more than double the amount of memory than Rust.
</p>

<p align="center">
  <b>C++ vs Rust third algorithm</b>
</p>
<p align="center">
  <img src="graphs/cpp3.png" width="440" height="330">
  <img src="graphs/rust3.png" width="440" height="330">
</p>
<p>
  Similarly to previous algo, the speed is equal but Rust consumes two times less memory.
</p>

<p>
  To conclude, it is clear that Rust dominates C++ in these charts and by quite a margin. Therefore Rust is considered to be the more efficent one in this experiment but to be clear it doesn't show us much in general. It is by no means an easy task to name a certain language more efficent than the other due to their complexity and potential performance differences in specific areas. 
</p>
