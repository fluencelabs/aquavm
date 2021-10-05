## AIR trace handler

This crate contains implementation of the CRDT-based merging data algorithm. It exposes the `TraceHandler` struct that based on the visitor pattern and has public methods that should be called in certain places of AIR instructions execution. Internally `TraceHandler` contains several FSM and each of such public methods does state transitioning of one or more these FSMs. Below are state transition sequences for all instructions that caller must follow.

### Ap instruction

Expected sequence of `TraceHandler` calls for the `ap` instruction:
```
meet_ap_start
    -> meet_ap_end
```

### Call instruction

Expected sequence of `TraceHandler` calls for the `call` instruction:
```
meet_call_start
    -> meet_call_end
```

### Par instruction

Expected sequence of `TraceHandler` calls for the `par` instruction: 
```
meet_par_start
    -> meet_par_subtree_end(..., SubtreeType::Left)
    -> meet_par_subtree_end(..., SubtreeType::Right)
```

### Fold instruction

Expected sequence of `TraceHandler` calls for the `fold` instruction:
```
meet_fold_start.1 ->
    meet_generation_start.N ->
        meet_next.M ->
        meet_prev.M ->
    meet_generation_end.N ->
meet_fold_end.1
```
where .T means that this function should be called exactly T times.
