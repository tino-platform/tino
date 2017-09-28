# Truss Bytecode Specification

# Warning: This is all outdated now.  See `src/capnp/` for more information.

Well shit, this is going to be hard.  It wouldn't be so hard if we didn't want
to support generic types, but we do because we're not Go.

## .truss Format Pseudo-BNF

Truss file format isn't a perfect context-free grammar, but it can be thought
of a bit like one.  It's actually a context-sensitive grammar if I am right, but
with some trickery and some "not giving a shit", we can use actually use it like
a context-free grammar.

It's a bit annoying because we can't just take opcodes and pull
them directly into memory and expect the VM to execute them, there's a small
step of translating (more akin to assembling) them into actual `Instruction`
values that the VM can execute, because muh memory safety.  This might be
changed if someone makes an unsafe VM in C/C++/Assembler that we can do fancy
JIT things with.

Instead of spaces you use null bytes.  Only ever use 1 or 0 null bytes between
tokens.

The magic number is `TRUSSBIN`, which can fit in a 64-bit word.  If you don't
have 64-bit words on your platform then get a better computer.

```

<U8> : an 8-bit unsigned number

<U32> : a big-endian 32-bit unsigned number

<U64> : a big-endian 64-bit unsigned number

<Identifier> : [a-zA-Z_$][a-zA-Z0-9_$]* (ascii)

<Symbol> : [a-z]+ (ascii)

<truss> ::= 'TRUSSBIN' <Identifier (blob name)> <U32 (entry count)> <trussEntry>{entry count}

<trussEntry> ::= 'clss' <classDef> | 'func' <funcDef> | 'cmmt' <U32 (length)> <U8>{length}

<classDef> ::= <fqClassName> ('(' <fqClassName (parent class)> ')')? <U32 (flags)> <U32 (field count)> <fieldDef>{field count}

<fqClassName> ::= <moduleName> ',' <className>

<moduleName> ::= <Symbol> ('.' <Symbol>+)*

<className> ::= <Identifier> <genericDecl>?

<genericDecl> ::= '<' <genericDef> (',' <genericDecl>)* '>'

<genericDef> ::= 'type' <Symbol>

<fieldDef> ::= 'f' <Identifier (name)> <U32 (flags)> <fieldType>

<fieldType> ::= <fqClassName> | <Symbol>

<funcDef> ::= <fqFuncName> <U32 (flags)> <U32 (arg count)> <fieldType>{arg count} <returnType> <U32 (opcode count)> <opcode>{opcode count}

<fqFuncName> ::= <moduleName> ',' <Identifier>

<returnType> ::= /* TODO */

<opcode> ::= /* TODO */

```
