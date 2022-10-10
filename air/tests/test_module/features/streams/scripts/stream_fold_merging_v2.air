(seq
    (seq
        (call "{0}" ("" "") []) ;; initiator that should send data to stream generators
        (par
            (par
                (par
                    (par
                        (par
                            (par
                                (call "{1}" ("" "") [] $stream)
                                (call "{2}" ("" "") [] $stream))
                            (call "{1}" ("" "") [] $stream))
                        (call "{3}" ("" "") [] $stream))
                    (call "{3}" ("" "") [] $stream))
                (call "{1}" ("" "") [] $stream))
            (call "{2}" ("" "") [] $stream)))
    (fold $stream v
        (par
            (seq
                (next v)
                (call "{4}" ("" "") [v 1]))
            (call "{4}" ("" "") [v]))))
