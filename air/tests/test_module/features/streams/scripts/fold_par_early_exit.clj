(seq
    (seq
        (seq
            (seq
                (seq
                    (call "{0}" ("" "") ["stream_1"] stream_1_ingredients)
                    (call "{0}" ("" "") ["stream_2"] stream_2_ingredients))
                (call "{0}" ("" "") ["stream_3"] stream_3_ingredients))
            (call "{0}" ("" "") ["stream_4"] stream_4_ingredients))
        (seq
            (seq
                (seq
                    (fold stream_1_ingredients v1
                        (seq
                            (call "{1}" ("" "") [v1] $stream_1)
                            (next v1)))
                    (fold stream_2_ingredients v2
                        (seq
                            (call "{1}" ("" "") [v2] $stream_2)
                            (next v2))))
            (fold stream_3_ingredients v3
                (seq
                    (call "{1}" ("" "") [v3] $stream_3)
                    (next v3))))
    (fold stream_4_ingredients v4
        (seq
            (call "{1}" ("" "") [v4] $stream_4)
            (next v4)))))
    (par
        (xor
            (fold $stream_1 v1
                (par
                    (fold $stream_2 v2
                        (par
                            (par
                                (fold $stream_3 v3
                                    (par
                                        (fold $stream_4 v4
                                            (par
                                                (call "{2}" ("" "") [])
                                                (next v4)))
                                       (next v3)))
                                (call "{3}" ("error" "") [])) ; will trigger an error
                            (next v2)))
                    (next v1)))
            (call "{4}" ("" "") [%last_error%]))
        (call "{5}" ("" "") [])))
