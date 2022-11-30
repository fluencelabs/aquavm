(seq
    (seq
        (call "{0}" ("" "") ["stream_1"] stream_peers_1)
        (call "{0}" ("" "") ["stream_2"] stream_peers_2))
    (seq
        (par
            (fold stream_peers_1 v1
                (par
                    (seq
                        (call v1 ("" "") [v1] $stream_1)
                        (call v1 ("" "") [v1] $stream_1))
                    (next v1)))
            (fold stream_peers_2 v2
                (par
                    (seq
                        (call v2 ("" "") [v2] $stream_2)
                        (call v2 ("" "") [v2] $stream_2))
                    (next v2))))
        (fold $stream_1 v1
            (seq
                (fold $stream_2 v2
                    (seq
                        (par
                            (call "{1}" ("" "") [v1 v2])
                            (next v2))
                        (call "{1}" ("" "") [v1 v2])))
                (next v1)))))
