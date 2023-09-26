## AIR: Instructions

### call

```wasm
(call <peer_id> (<service name> <service function>) [<arguments list>] <output name>)
```

- moves execution to the `peer_id` specified
- the peer is expected to host Wasm service with the specified `service name`
- the `service function` is expected to contain the specified function
- the `arguments list` is given to the function and may be empty
- the result of the function execution is saved and returned by its `output name`

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
(ap (<key> <value>) %map)
```

- puts `literal` into `dst_variable`
- or applies `lambda` to `src_variable` and saves the result in `dst_variable`
- or inserts the `key` into the `%map` with the `value`. `Key` and `value` might be literal | scalar | scalar with a lens.

Example:

```wasm
(seq
    (call "peer_id" ("user-list" "get_users") [] users)
    (ap users.$.[0].peer_id user_0)
)
```

### canon

```wasm
(canon "peer_id" <$stream> <#$canon_stream>)
(canon "peer_id" <%map> <#%canon_map>)
```

- `peer_id` runs the `canon` fixing `$stream` or `%map` contents as they were at the moment of the first canonicalization
- all future `canon` runs will produce `#%canon_stream` | `#%canon_map`  with the same contents as after the first run at `peer_id`

Examples:

```wasm
(seq
    (ap user $users)
    (canon "peer_id" $stream #$canon_stream)
)
```

```wasm
(seq
    (ap (kvpair.$.username_key kvpair.$.username_value) %map)
    (canon "peer_id" %map #%canon_map)
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
- iterates over the `iterable`, assigning each element to the `iterator`
- on each iteration `instruction` is executed
- `next` triggers next iteration

Examples:

```wasm
(fold users user
    (seq
        (call user.$.peer_id ("chat" "display") [msg])
        (next user)
    )
)
```

```wasm
(fold %users_map user_pass_kvpair
    (seq
        (call peer_id ("chat" "auth") [user_pass_kvpair.$.key user_pass_kvpair.$.value])
        (next user_pass_pair)
    )
)
```

```wasm
(seq
    (ap "users" users_key_name)
    (fold #%canon_map.$.[users_key_name] user
        (seq
            (call peer_id ("chat" "display") [user msg])
            (next user)
        )
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
- can be a target for a lens(previously known as lambda paths)

### Streams and stream-based Maps

- streams and maps are CRDT-like (locally-consistent) - have deterministic execution within one peer
- versioned
- can be used only by call and fold instructions (more instructions for streams to come)
- can be turned to scalar (canonicalized)

### Canonicalized streams and stream-based maps

- contains an array of elements that was in a stream or a map at the moment of canonicalization
- canonicalized streams or maps are immutable and fully consistent as scalars
- has the same algebra as a stream for `match`/`mismatch` and `call` argument
- has the same algebra as a scalar for `new`
- has mixed behaviour for with other instructions
- can be a target for a lens(previously known as lambda paths)
- maps has an additional index access operation that returns a canon stream.
- maps index access syntax leverages lens syntax, e.g. `#%canon_map.$.key_name`
