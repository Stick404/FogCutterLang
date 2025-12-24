# Languages to use
Choosing a language to use is a multi-faceted problem that requires extensive cost-benefit analysis.
Since the used language must be well suited for both a compiler and a VM system, it must have/be:
- Speedy, since this will be running every few milliseconds and should not take longer than that.
- Light weight, since this will be running *with* Minecraft, we dont want it to take more CPU/RAM than Minecraft
- Small file sizes, to make sure the compiled Jar is not measured in gigabytes. Preferably under 30 mBs.

So far the languages that have been tested have matched these rough guidelines.
However, each has their upsides and downsides, which are as follows:
- C/C++
  - Positives:
    - Extremely low level
    - Light weight
    - Smaller file sizes
  - Downsides:
    - Difficulties in compiling from Windows
    - Barebones compared to other languages explored
    - Difficulties in cross compiling and using (Windows made) `.dll`s
- D(lang)
  - Positives:
    - Higher level programing
    - Large Standard Library
    - Low level workings
  - Downsides:
    - Difficulties in cross compiling and using (Windows made) `.dll`s
    - Bundles the STD library in binaries
- Zig
  - Positives:
    - Nice to work with
    - Preformat
    - Low level workings
  - Downsides:
    - Difficulties in cross compiling and using (Windows made) `.dll`s
    - Difficult to install on some Linux Systems
- Go:
  - Positives:
    - Nice to work with
    - Basic C ABI systems
    - Easy to cross compile and has proven-working Shared Libraries
  - Downsides:
    - Slow compared to other langs here
    - Larger binaries
    - Slow compile times

There are a few languages that show promise, but are untested:
- Rust
  - Not a massive fan of Rust, but it might be well suited
- Plain C (no C++)
  - Basic enough to get a large enough performance boost (~~Just a translator to bytecode~~)
- Pony
  - A smaller language that has strong native parallelization (not sure how helpful this would be during a VM/Compiler)


## Conclusion:
Currently, as I am stuck with using Windows for the next Week, Go is likely the best to use.
However, Zig or Dlang would be the best language to work with for this project;
since they are the most familiar languages out of the list, they have low level systems workings with higher level feeling.
The biggest downside of both Zig and Dlang is that they are unable to be compiled on Windows11, or at least my device.
However, the bulk of development will be done on a Linux system, and anyone wishing to contribute will most likely use Linux as well.