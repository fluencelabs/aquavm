(xor
 (seq
  (seq
   (seq
    (seq
     (seq
      (seq
       (call %init_peer_id% ("getDataSrv" "-relay-") [] -relay-)
       (call %init_peer_id% ("getDataSrv" "tolerance") [] tolerance)
       )
      (call %init_peer_id% ("getDataSrv" "threshold") [] threshold)
      )
     (call %init_peer_id% ("getDataSrv" "node") [] node)
     )
    (call %init_peer_id% ("getDataSrv" "oracle_service_id") [] oracle_service_id)
    )
   (new $res
    (seq
     (seq
      (seq
       (call -relay- ("op" "noop") [])
       (xor
        (seq
         (call node ("op" "string_to_b58") [node] k)
         (par
          (seq
           (call node ("kad" "neighborhood") [k [] []] nodes)
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
      (par
       (call %init_peer_id% ("op" "identity") [$res.$.[19]!])
       (call %init_peer_id% ("peer" "timeout") [1000 "timeout"])
       )
      )
     (call %init_peer_id% ("op" "identity") [$res] res-fix)
     )
    )
   )
  (xor
   (call %init_peer_id% ("callbackSrv" "response") [res-fix nodes])
   (call %init_peer_id% ("errorHandlingSrv" "error") [%last_error% 3])
   )
  )
 (call %init_peer_id% ("errorHandlingSrv" "error") [%last_error% 4])
 )
