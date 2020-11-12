# Aquamarine
Aquamarine is a distributed choreography platform, controlled by AIR language.

<p>
<p align="center" width="100%">
<img alt="aquamarine scheme" src="images/interpreter.png" width="621"/>
</p>

<br/>
<br/>

## Fluence stack

Fluence [nodes](https://github.com/fluencelabs/fluence) use aquamarine to coordinate requests between different services run by [FCE](https://github.com/fluencelabs/fce):

<br/>
<p align="center" width="100%">
<img alt="aquamarine scheme" align="center" src="images/stack.png" width="663"/>
</p>
<br/>

## Aquamarine Intermediate Representation

### AIR: What is it?

- S-expression-based low-level language
- Controls Fluence network and its peers
- Inspired by WAT (WebAssembly Text Format)
- Meant to be a compile target
- Development meant to happen in a higher-level language
- Syntax is in flux, will change

Scripts written in AIR look like this:

<img alt="fold example" src="images/fold_example.png" width="100%"/>

1. Gather chat members by calling chat.members
2. Iterate through elements in members array, m = element
3. Each m is an object, represented as array; [0] is the first field
4. `(next m)` triggers next iteration

### AIR: Instructions
#### call: execution
<img alt="call structure" src="images/call_data.png" width="670"/>

- `call` commands the execution
- moves execution to a peer, specified by `location`
- peer is expected to have specified WASM `service`
- the `service` must have specified `function` available to be called
- `argument list` is given to the `function`
- result of the `function` is saved and available under `output name`
- example call could be thought of as `data.result = dht.put(key, value)`

#### seq: sequential
<img alt="seq structure" src="images/seq.png" width="586"/>

- `seq` takes two instructions
- executes them sequentially

#### par: parallel
<img alt="par structure" src="images/par.png" width="536"/>

- `par` takes two instructions
- executes them in parallel

#### fold: iteration
<img alt="fold structre" src="images/fold.png" width="536"/>

- `fold` takes an array, a variable and an instruction
- iterates through the array, assigning each element to the variable
- on each iteration instruction is executed
- instruction can read the variable
- `next` triggers next iteration

#### xor: branching & error handling
<img alt="xor structure" src="images/xor.png" width="577"/>

- `xor` takes two instructions
- iff first instruction fails, second one is executed

#### null
<img alt="null structure" src="images/null.png" width="577"/>


- `null` takes no arguments
- does nothing, useful for code generation

