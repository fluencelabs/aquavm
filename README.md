[![crates.io version](https://img.shields.io/crates/v/air-interpreter-wasm?style=flat-square)](https://crates.io/crates/air-interpreter-wasm)
[![npm version](https://img.shields.io/npm/v/@fluencelabs/avm)](https://www.npmjs.com/package/@fluencelabs/avm)

# AquaVM

AquaVM executes compiled [Aqua](https://github.com/fluencelabs/aqua), i.e., Aqua Intermediate Representation (AIR) scripts, and plays an integral part in the implementation of the Fluence peer-to-peer compute protocol. In this capacity, AquaVM allows expressing network choreography in scripts and composing distributed, peer-to-peer hosted services. Moreover, AquaVM plays a significant role in facilitating *function addressability* in the Fluence network.

Since AquaVM compiles to Wasm, it can run both clients, such as browsers and nodejs apps, and servers. See Figure 1.

**Figure 1: Stylized AquaVM And Air Model**

<img alt="AquaVM & AIR model" src="images/aquavm_air_model.png" />

## AquaVM: Interpreter Execution Model

AquaVM's execution model facilitates Fluence protocol's data push model implemented as a *particle*, i.e., a smart packet comprised of data, AIR, and some metadata. In this context, AquaVM can be viewed as a pure state transition function that facilitates particle updates, which includes state management of particle data by taking previous and current state to produce a new state and an updated list of peers and call requests, in the remaining AIR workflow. In addition to local service call execution, AquaVM handles requests from remote peers, e.g. as part of a parallel execution block, to call local services and the future response. See Figure 2.

**Figure 2: AquaVM Interpreter Execution Model**

<img alt="interpreter execution model" src="images/interpreter_execution_model.png"/>

In summary, the AquaVM execution model allows (async) parallel service execution on one or multiple peers.

## Aquamarine Intermediate Representation: IR for p2p systems

### AIR: What is it?

- S-expression-based low-level language
- Controls Fluence network and its peers
- Consists of 12 instructions (more instructions to come)
- Semantic inspired by pi-calculus, lambda-calculus and theory of category
- Syntax inspired by WAT (Wasm Text Format) and Lisp

### AIR: Instructions
#### call

```wasm
(call "peer_id" ("dht" "put") [key value] result)
```

- moves execution to a peer, specified by location (`"peer_id"` in the example)
- peer is expected to have the specified Wasm service (`"dht"`)
- the `service` must have specified function (`"put"`) available to be called
- argument list (`[key value]`) will be given to the function
- result of the function execution is saved and available under output name (`result`)

#### seq

```wasm
(seq
    (call "node_id" ("dht" "get") [key] value)
    (call "storage_id" ("SQLite" "put") [key value] store_result)
)
```

- `seq` takes two instructions
- executes them sequentially: second instruction will be executed iff first one finished successfully

#### par

```wasm
(par
    (call "client_a_id" ("chat" "display") [msg])
    (call "client_b_id" ("chat" "display") [msg])
)
```

- `par` takes two instructions
- executes them in parallel: the second instruction will be executed independently of the completion of the first one

#### ap

```wasm
(seq
    (call "peer_id" ("user-list" "get_users") [] users)
    (ap users.$.[0].peer_id user_0)
)
```

- `ap` takes two values
- applies lambda to first and saves the result in second

#### match/mismath

```wasm
(seq
    (call "peer_id" ("user-list" "get_users") [] users)
    (mismatch users.$.length 0
        (ap users.$.[0].peer_id user_0)
    )
)
```

- `match`/`mismatch` takes two variables and an instruction
- executes the instruction iff variables are equal/notequal

#### fold

```wasm
(fold users user
    (seq
        (call user.$.peer_id ("chat" "display") [msg])
        (next user)
    )
)
```

- `fold` is a form of a fixed-point combinator
- takes an array or an iterable variable and an instruction
- iterates through the iterable (`users`), assigning each element to the iterator (`user`) 
- on each iteration instruction (`(seq ...)`) is executed
- instruction can read the iterator
- `next` triggers next iteration

#### xor

```wasm
(xor
    (call "client_a_id" ("chat" "display") [msg])
    (call "client_b_id" ("chat" "display") [msg])
)
```

- `xor` takes two instructions
- second one is executed iff the first one failed

#### null

```wasm
(null)
```

- `null` takes no arguments
- does nothing, useful for code generation

### AIR: values
#### Scalars

- scalars are fully consistent - have the same value on each peer during a script execution
- could be an argument of any instruction
- JSON-based (fold could iterate only over array-based value)

#### Streams

- streams are CRDT-like (locally-consistent) - have deterministic execution wrt one peer
- versioned
- could be used only by call and fold instructions (more instructions for streams to come)
- could be turned to scalar (canonicalized)
