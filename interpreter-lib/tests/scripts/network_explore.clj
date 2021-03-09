(seq
 (seq
  (call "12D3KooWEDU1WwGtvHUKpGCaMjhcLWyCUq3MQiRKZBLLFcBVVMck" ("" "") ["relay"] relay)
  (call "12D3KooWEDU1WwGtvHUKpGCaMjhcLWyCUq3MQiRKZBLLFcBVVMck" ("" "") ["client"] client)
  )
 (seq
  (call relay ("dht" "neighborhood") [relay] neighs_top)
  (seq
   (fold neighs_top n
         (seq
          (call n ("dht" "neighborhood") [n] neighs_inner[])
          (next n)
          )
         )
   (fold neighs_inner ns
         (seq
          (fold ns n
                (seq
                 (call n ("op" "identify") [] services[])
                 (next n)
                 )
                )
          (next ns)
          )
         )
   )
  )
 )