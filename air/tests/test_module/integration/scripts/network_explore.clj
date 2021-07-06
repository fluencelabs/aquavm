(seq
 (seq
  (call "client_id" ("" "") ["relay"] relay)
  (call "client_id" ("" "") ["client"] client)
  )
 (seq
  (call relay ("dht" "neighborhood") [relay] neighs_top)
  (seq
   (fold neighs_top n
         (seq
          (call n ("dht" "neighborhood") [n] $neighs_inner)
          (next n)
          )
         )
   (fold $neighs_inner ns
         (seq
          (fold ns n
                (seq
                 (call n ("op" "identify") [] $services)
                 (next n)
                 )
                )
          (next ns)
          )
         )
   )
  )
 )