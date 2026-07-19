# Intel 8080 Emulator

A cycle-aware Intel 8080 emulator written in Rust capable of running the original **Space Invaders** arcade ROM.

This project was built from scratch as a way to understand how CPUs actually work, including instruction decoding, registers, memory, interrupts, bitwise operations, and hardware emulation.

<div align="center">
  <img width="442" height="474" alt="image" src="https://github.com/user-attachments/assets/d69ebaa7-0e0b-4828-a6f1-a204cd02c468" />
</div>

---

## Running

```bash
cargo run --release
```

# About the experience

# What Was Easier Than Expected

Surprisingly, some parts looked intimidating before I started but ended up being fairly straightforward. I thought the most difficult part would be the video; 
before this project I had never handled video in any project, but it was by far the easiest part. The thing that could be tricky, that the Space Invaders video 
was flipped, is very well documented, and many other people have already handled it and left instructions and warnings about it. Another thing that could be really 
tricky for me, but is not related to the project itself, is that I had never touched any Rust code. It was my first contact with Rust; before it, I only used C. Like everyone 
says, Rust is basically C on steroids, so it was very easy to understand, and Rust proved to be a great choice for the project all along.

---

# What Was Hard

**Debugging**

Most bugs were not obvious. A single incorrect flag or register update could break execution hundreds of instructions later.

I literally spent over a week trying to figure out why the game crashed when it started. In the beginning, I decided not to use cpudiag (a very bad decision) 
and just stuck with running Space Invaders. In the end, I found out I had inverted JP with JM (and calls and returns), I misunderstood the sign flag, and inverted 
the condition. This nearly made me quit the project, and when I found out it was just a !, it was about 1 AM. That day, I slept well after playing about 2 or 3 rounds 
with my own Space Invaders machine.

---

# Tips for Anyone Building an Emulator

- Start with a CPU diagnostic ROM before trying to run games; I didn't, and I regret it. I know it is not enough, but in my case, it would have helped me a lot to find the sign flag error.
- Implement instructions incrementally.
- Write helper functions; many of the instructions do the exact same thing, so you don't need to write thousands of lines.
- Keep registers and memory access isolated.
- Add lots of logging while debugging.
- Don't underestimate interrupt timing.

---

# Biggest Takeaways

Building an emulator changed the way I think about computers. High-level languages hide an incredible amount of complexity. After implementing an 8-bit CPU from scratch, assembly language, 
bitwise operations, and processor architecture became much easier to understand. This project also taught me Rust, which I fell in love with, and I am already thinking of building another emulator (maybe NES).

---

# Future Improvements

To be honest, i think i will never come back to this project, but if i do, i will add this improvements in order

- High score save to Space Invaders (I started writing some code to do it; I think this would be very easy at this point, but I don't have more time available at the moment, unfortunately).
- Save states.
- Additional Intel 8080 games.

---

# Resources

- [cbeust emulator](https://github.com/cbeust/space-invade.rs) When i had the idea, this project made me use Rust; I really liked the way the code looks and wanted to do my own.
- [Intel 8080 Programmer's Manual](https://www.nj7p.info/Manuals/PDFs/Intel/9800153B.pdf)
- [Intel 8080 Opcodes Table](https://pastraiser.com/cpu/i8080/i8080_opcodes.html) This was really helpful
- [Intel 8080 Opcodes](https://gist.github.com/joefg/634fa4a1046516d785c9)
- [Computer Archeology - Space invaders code](https://computerarcheology.com/Arcade/SpaceInvaders/Code.html#09E8) I didn't use it much, but when I started my battle against the game crashes, it was helpful to compare my debug (what opcode made the game crash) with the code itself.

---
