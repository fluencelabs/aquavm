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

## Aquamarine Intermediate Representation (AIR): IR For P2P Systems

AIR scripts control the Fluence peer-to-peer network, its peers and, through [Marine adapter services]() even resources on other (p2p) betworks, such as IPFS and Filecoin.

Mike: would be good to maybe briefly explain the design choices for Air, e.., S-epr as opposed to byte code, etc.

### What is AIR?

- S-expression-based low-level language
- Consists of twelve (12) instructions with more instructions to come
- Semantics are inspired by [π-calculus](https://en.wikipedia.org/wiki/%CE%A0-calculus), [λ-calculus](https://en.wikipedia.org/wiki/Lambda_calculus) and [category theory](https://en.wikipedia.org/wiki/Category_theory)
- Syntax is inspired by [Wasm Text Format](https://developer.mozilla.org/en-US/docs/WebAssembly/Understanding_the_text_format) (WAT) and [Lisp](https://en.wikipedia.org/wiki/Lisp_(programming_language))

### AIR: Instructions

Mike: could use some of the calculus references and explainers we had a year ago or so in readme

#### call

Mike: wouldn't it be better to have a generic function specification before the example?

```wasm
(call <peer_id> (<service namespace> <service function>) [key value] <output>)
```


```wasm
(call "peer_id" ("dht" "put") [key value] result)
```

- moves execution to the peer specified, e.g., `"peer_id"`
- the peer is expected to host the specified Wasm service, e.g., `"dht"`
- the `service` is expected to contain the specified function, e.g.,  `"put"`
- the argument list `[key value]` is given to the function and may be empty 
- the result of the function execution is saved and returned by it's output name, e.g., `result`

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
