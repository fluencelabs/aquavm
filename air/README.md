# AIR

### General properties of the interpreter

This crates defines the core of the AIR interpreter. This interpreter has only one export function called `invoke` and no import functions. The export function has the following signature 
```rust
pub fn executed_air(
    /// AIR script to execute.
    air: String,
    
    /// Previous data that should be equal to the last returned by the interpreter. 
    prev_data: Vec<u8>,
    
    /// So-called current data that was sent with a particle from the interpreter on some other peer.
    data: Vec<u8>,
    
    /// Running parameters that includes different settings.
    params: RunParameters,
    
    /// Results of calling services.
    call_results: Vec<u8>,
) -> InterpreterOutcome {...}

pub struct InterpreterOutcome {
    /// A return code, where 0 means success.
    pub ret_code: i32,

    /// Contains error message if ret_code != 0.
    pub error_message: String,

    /// Contains so-called new data that should be preserved in an executor of this interpreter
    /// regardless of ret_code value.
    pub data: Vec<u8>,

    /// Public keys of peers that should receive data.
    pub next_peer_pks: Vec<String>,

    /// Collected parameters of all met call instructions that could be executed on a current peer.
    pub call_requests: Vec<u8>,
}
```

Let's consider the interpreter with respect to data first, because previous, current and resulted data are the most interesting parts of arguments and the outcome. Assuming `X` is a set of all possible values that data could have, we'll denote `executed_air` export function as `f: X * X -> X`. It could be seen that with respect to data `f` forms a magma. 

Even more, `f` is a free idempotent non-commutative monoid, because:
1. `f` is associative: `forall a, b, c from X: f(f(a,b), c) = f(a, f(b,c))`
2. `f` has a neutral element: `exists e, forall a from X: f(e, a) = f(a, e) = a`, where `e` is a data with an empty trace
3. `f` is a non-commutative function: `exists a, b from X: f(a, b) != f(b, a)`
4. `X` could be constructed from a four based elements that form the `ExecutedState` enum (that why this monoid is free)
5. `f` satisfies these idempotence properties:
   1. `forall x from X: f(x, x) = x`
   2. `forall a, b from X: f(a, b) = c, f(c, b) = c, f(c, a) = c`

### Interaction with the interpreter

The interpreter allows host (either node or browser) to call service asynchronously, by collecting all arguments and other stuff from `call` instructions that could be called during the execution and providing them in `InterpreterOutcome`. A host should then execute them at any time and call back the interpreter providing executed service results in the `call_results` argument.

A scheme of interacting with the interpreter should look as follows:
1. For each new `current_data` received from a network, a host should call the interpreter with corresponding `prev_data` and `current_data` and empty `call_results`. `prev_data` here is last `new_data` returned from the interpreter.

2. Having obtained a result of the interpreter, there could be non-empty `next_peer_ids` and non-empty `call_requests` in `InterpreterOutcome`:
   1. re `next_peer_pks`: it's a host duty to decide should it send particle after each interpreter call or after the whole execution.
   2. re `call_requests`: `call_requests` is a `HashMap<u32, CallRequestParams>` and it's important for host to keep that correspondence between `u32` and call `CallRequestParams`, because they should be used when results are passed back on step 3. 
   
3. If `call_requests` was non-empty on step 2, a host must call the interpreter again with supplied call results (`HashMap<u32, CallServiceResult>`). Note, that current_data here shouldn't be supplied (actually, it could be supplied because of `f` is idempotent, but it's unnecessary and slow down a bit an interpreter execution). But please note that host still must preserve `new_data` after each execution step. It's important, because `f` is non-commutative and the interpreter save an additional info in `data` expecting to see the resulted data back as `prev_data` on the next launch. Then this flow should be repeated starting from point 2.

4. If `call_requests` was empty, the whole execution is completed, `new_data` must be preserved and particle send for all `new_peer_pks` as usual.

An example of interaction could be found in (tests)[https://github.com/fluencelabs/aquavm/blob/async/crates/test-utils/src/test_runner.rs]
