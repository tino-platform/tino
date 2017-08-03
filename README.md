# TINO (Is Not OTP)

Tino is basically a clone of OTP, the platform that Erlang is centered around.
The main component of Tino is Truss, a stack-based virtual machine.

OTP : BEAM :: TINO : TRUSS

Tino is a recursive acronym for "Tino is not OTP".  Truss is another acronym
for (the) Tino Runtime Utility for Scalable Systems.

The best part about all this is that it's super fast and 100% safe because
we're using Rust, a systems programming language built around safety and
correctness.

## Scalable?

Yes!  While Truss has built-in garbage collection, it also operates on the
actor model.  This means that garbage collection domains are separated
between actors (called processes in OTP land, so also called processes in Truss
land).  This means that if one process has paused its execution to do garbage
collection, it doesn't pause everything else in the system.  Another benefit
of using processes instead of other forms of multithreading is that our primary
form of communication between threads is with message passing, and with no
mutable shared memory.  That means that it's *much easier* to spread the VM
across multiple physical machines.  Truss does not have support for this yet,
but it's a goal in the future.

## Features

### Truss

(Not in order of importance.)

- [x] Object, class framework

- [ ] Garbage collector

- [x] First-class function support

- [ ] Generic type support

- [ ] Basic VM functionality

- [ ] Multithreaded scheduler

- [ ] Bytecode format specification

- [ ] Bytecode loader

- [ ] Streaming opcode support

- [ ] Immutable shared memory

- [ ] Native function execution (NIFs)

- [ ] IO framework (via NIFs)

### Tino (apart from Truss)

- [ ] Verbose bytecode assembler

- [ ] C-like language targeting Truss

- [ ] LFT: Lisp-Flavored Tino

- [ ] CLI tools for managing distributed nodes

- [ ] VisualVM but for TRUSS

- [ ] WebSocket-based server to serve code execution requests in light clients

- [ ] Broswer extension to support aforementioned server
