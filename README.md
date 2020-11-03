# Aquamarine

Aquamarine is a distributed choreography platform, controlled by AIR language
![Aquamarine schema](images/aquamarine.png)

## AIR: Aquamarine Intermediate Representation
### What is it?

- S-expression-based low-level language
- Controls Fluence network and its peers
- Inspired by WAT (WebAssembly Text Format)
- Meant to be a compile target
- Development meant to happen in a higher-level language
- Syntax is in flux, will change

### Structure
![AIR structure scheme](images/air_structure_data.png)

### Instructions
#### seq: sequential
![seq example](images/seq.png)
- `seq` takes two instructions
- executes them sequentially

#### par: parallel
![par example](images/par.png)
- `par` takes two instructions
- executes them in parallel

#### fold: iteration
![fold example](images/fold.png)
1. Gather chat members by calling chat.members
2. Iterate through elements in members array, m = element
3. Each m is an object, represented as array; [0] is the first field
4. (next m) triggers next iteration
