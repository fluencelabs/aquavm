(seq
 (xor
  (seq
   (call "{0}" ("" "") []) ;; initiator that should send data to stream generators
   (par
    (par
     (par
      (par
       (par
        (par
         (call "{1}" ("" "") [] $stream)
         (call "{2}" ("" "") [] $stream)
         )
        (call "{1}" ("" "") [] $stream)
        )
       (call "{3}" ("" "") [] $stream)
       )
      (call "{3}" ("error" "") [] $stream) ;; will trigger an error
      )
     (call "{3}" ("" "") [] $stream)
     )
    (call "{2}" ("" "") [] $stream)
    )
   )
  (null)
  )
 (call "{0}" ("" "") []) ;; this one is needed to check check that sliders switched correctly
 )