(xor
 (seq
  (seq
   (call %init_peer_id% ("getDataSrv" "-relay-") [] -relay-)
   (new $res
    (seq
     (seq
      (call -relay- ("op" "noop") [])
      (xor
       (seq
        (call "12D3KooWSD5PToNiLQwKDXsu8JSysCwUt8BVUJEqCHcDe7P5h45e" ("op" "string_to_b58") ["12D3KooWSD5PToNiLQwKDXsu8JSysCwUt8BVUJEqCHcDe7P5h45e"] k)
        (par
         (seq
          (call "12D3KooWSD5PToNiLQwKDXsu8JSysCwUt8BVUJEqCHcDe7P5h45e" ("kad" "neighborhood") [k [] []] nodes)
          (call %init_peer_id% ("op" "noop") [])
          )
         (seq
          (fold nodes n
           (par
            (seq
             (xor
              (call n ("peer" "timestamp_ms") [] $res)
              (seq
               (call -relay- ("op" "noop") [])
               (call %init_peer_id% ("errorHandlingSrv" "error") [%last_error% 1])
               )
              )
              (call %init_peer_id% ("op" "noop") [])
              )
            (next n)
            )
           )
          (call %init_peer_id% ("op" "noop") [])
          )
         )
        )
       (call %init_peer_id% ("errorHandlingSrv" "error") [%last_error% 2])
       )
      )
     (seq
      (canon %init_peer_id% $res #res)
      (seq
       (par
        (call %init_peer_id% ("op" "identity") [#res.$.[19]!])
        (call %init_peer_id% ("peer" "timeout") [1000 "timeout"])
        )
       (call %init_peer_id% ("op" "identity") [#res] res-fix)
       )
      )
     )
    )
   )
  (call %init_peer_id% ("--after-callback-srv-service--" "print-and-stop") [res-fix nodes])
  )
 (call %init_peer_id% ("errorHandlingSrv" "error") [%last_error% 3])
 )
