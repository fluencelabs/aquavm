## AIR: Instructions

### call

```wasm
(call <peer_id> (<service name> <service function>) [<arguments list>] <output name>)
```

- moves execution to the `peer_id` specified
- the peer is expected to host Wasm service with the specified `service name`
- the `service function` is expected to contain the specified function
- the `arguments list` is given to the function and may be empty 
- the result of the function execution is saved and returned by it's `output name`

Example:
```wasm
(call "peer_id" ("dht" "put") [key value] result)
```

### seq

```wasm
(seq <left_instruction> <right_instruction>)
```

- executes instructions sequentially: `right_instruction` will be executed iff  `left_instruction` finished successfully

### par

```wasm
(par <left_instruction> <right_instruction>)
```

- executes instructions in parallel: `right_instruction` will be executed independently of the completion of `left_instruction`

### ap

```wasm
(ap <literal> <dst_variable>)
(ap <src_variable>.$.<lambda> <dst_variable>)
```

- puts `literal` into `dst_variable`
- or applies `lambda` to `src_variable` and saves the result in `dst_variable`

Example:

```wasm
(seq
    (call "peer_id" ("user-list" "get_users") [] users)
    (ap users.$.[0].peer_id user_0)
)
```

### canon

```wasm
(canon "peer_id" <$stream> <#canon_stream>)
```

- executes on peer_id, takes $stream as it is on the moment of first canonicalization
- every next execution #canon_stream will be the same — as first seen by peer_id

Example:

```wasm
(seq
    (ap user $users)
    (canon "peer_id" $stream #canon_stream)
)
```

### match/mismatch

```wasm
(match <variable> <variable> <instruction>)
(mismatch <variable> <variable> <instruction>)
```

- executes the instruction iff variables are equal/notequal

Example:
```wasm
(seq
    (call "peer_id" ("user-list" "get_users") [] users)
    (mismatch users.$.length 0
        (ap users.$.[0].peer_id user_0)
    )
)
```

### fold/next

```wasm
(fold <iterable> <iterator> <instruction>)
```

- is a form of a fixed-point combinator
- iterates through the `iterable`, assigning each element to the `iterator` 
- on each iteration `instruction` is executed
- `next` triggers next iteration
  
Example:

```wasm
(fold users user
    (seq
        (call user.$.peer_id ("chat" "display") [msg])
        (next user)
    )
)
```

### xor

```wasm
(xor <left_instruction> <right_instruction>)
```

- `right_instruction` is executed iff `left_instruction` failed

### new

```wasm
(new <variable>)
```

- creates a new scoped variable with the provided name (it's similar to \mu operator from pi-calculus that creates an anonymous channel)

### fail

```wasm
(fail <variable>)
(fail <error code> <error message>)
```

- throws an exception with provided `error code` and `error message` or construct it from a provided `variable`]

Example
```wasm
(fail 1337 "error message")
```

### never

```wasm
(never)
```

- marks a subgraph as incomplete, useful for code generation

### null

```wasm
(null)
```

- does nothing, useful for code generation


## AIR: values

### Scalars

- scalars are fully consistent - have the same value on each peer during a script execution
- could be an argument of any instruction
- JSON-based (fold could iterate only over array-based value)

### Streams

- streams are CRDT-like (locally-consistent) - have deterministic execution wrt one peer
- versioned
- can be used only by call and fold instructions (more instructions for streams to come)
- can be turned to scalar (canonicalized)

### Canonicalized streams

- contains an array of elements that was in a stream at the moment of canonicalization
- canonicalized streams are imutable and fully consistent as scalars
- has the same algebra as a stream for `match`/`mismatch` and `call` argument
- has the same algebra as a scalar for `new`
- has mixed behaviour for with other instructions

