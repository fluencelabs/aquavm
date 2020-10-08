# Aquamarine

Aquamarine is a distributed choreography language &amp; platform

## AIR

The current version supports the following instructions:
- call
- par
- seq
- fold
- next
- null

## Examples

```lisp
(seq (
    (call (%current_peer_id1% (local_service_id local_fn_name) () result_name_1))
    (call (remote_peer_id (service_id fn_name) () result_name_2))
)),
```

This instruction sequence contains two call instructions in the sequential order:
1. call a function with `local_fn_name` name of a local service with `local_service_id` id and bind result to `result_name`
2. call a remote peer with `remote_peer_id` id

```lisp
(fold (Iterable i
    (seq (
        (call (%current_peer_id% (local_service_id local_fn_name) (i) acc[]))
        (next i)
    )
)))
```

This example is an analog of left fold. It iterates over `Iterable` and on each iteration calls `local_service_id` and puts result to `acc`.
