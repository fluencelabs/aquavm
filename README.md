# Aquamarine

Aquamarine is a distributed choreography language &amp; platform

## Examples

```lisp
((call (%current% (local_service_id local_fn_name) () result_name)) (call (remote_peer_id (service_id fn_name) () g)))
```
This instruction sequence contains two call instructions:
1. call a function with `local_fn_name` name of a local service with `local_service_id` id and bind result to `result_name`
2. call a remote peer with `remote_peer_id` id
