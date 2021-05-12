(seq
 (seq
  (call "{0}" ("" "") ["stream1"] stream1)
  (call "{0}" ("" "") ["stream2"] stream2)
  )
 (fold stream1 v1
       (seq
        (fold stream2 v2
              (seq
               (call "{1}" ("" "") [v1 v2] stream_out1[])
               (seq
                (call "{1}" ("" "") [v1 v2] stream_out2[])
                (seq

                 (next v2)
                 (seq
                  (call "{1}" ("" "") [v1 v2] stream_out3[])
                  (seq
                   (call "{1}" ("" "") [v1 v2] stream_out4[])
                   (seq
                    (call "{1}" ("" "") [v1 v2] stream_out5[])
                    (call "{1}" ("" "") [v1 v2] stream_out6[])
                    )
                   )
                  )
                 )
                )
               )
              )
        (next v1)
        )
       )
 )
