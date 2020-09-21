/*



null
out:  a!b . P
inp:  a(b). P    <- barrier
par:  P | Q
new:  nx . P        <- restriction
rep:  !P
sum:  P + Q

(seq --- arrow
  (proc -> new x)
  (proc that uses x)
)

Scope:
a <- A
a -> B

(seq
  (par
    x <- P
    y <- Q
  )
  (call fn [x y] z)
)

(seq
  (call fn1 [] x)
  (call fn2 [x] y)
)

(seq P Q) <- scope
(par P Q) <- independent
(xor P Q) <- lazy

-- fire and forget (any)
(seq
  (par
    x <- P
    y <- Q
  )
  (call fn [] z)
)

-- join
(seq
  (par
    x <- P
    y <- Q
  )
  (call noop [x y])
)

-- any (fastest)
(seq
  (par
    x[] <- P
    x[] <- Q
  )
  (call fn [x.0])
)

-- any (fastest) -- does not work
(seq
    (seq
      (par
        x <- P
        y <- Q
      )
      (xor
        (call fn [x] z)
        (call fn [y] z)
      )
    )
    (call fn11 [z])
)

ITERATORS

(seq
  (fold Iterable i
    (par
      (call fn [i] acc[])
      (next i)
    )
  )
  (match acc.length 3
    (call fnAgg [acc] y)
  )
)




    (par
      (call fn [i.0] acc[])
            (par
                (call fn [i.1] acc[])
                (par
                  (call fn [i.2 acc[])
                              (fold Iterable[3,..] i
                                (par
                                  (call fn [i] acc[])
                                  (next i)
                                )
                              )
                )
            )
    )


(seq
  (fold Iterable i
    (seq
      (call fn [i acc] acc[])
      (next i)
    )
  )
  (call fnAgg [acc] y)
)

(seq
  (fold Iterable i
    (xor
      (call fn [i] res)
      (next i)
    )
  )
  (call fnAgg [res] y)
)

 */


/*

Addressing:

To address a code we need to declare:
(peer_pk, srv_id, fn_name)

(call PEER_PART FN_PART [args] res_name)

(current)
(pk $pk)
(pk $pk $srv_id)
PEER_PART: resolves to (peer_pk) \/ (peer_pk, pk_srv_id)

(fn $name)
(fn $name $srv_id)
FN_PART: resolves to (fn_name) \/ (fn_srv_id, fn_name)

Call resolves to:
(peer_pk, fn_srv_id || pk_srv_id, fn_name)
If !fn_srv_id && !pk_srv_id <- error


(call (current) (fn "resolve" "by_pk") [pk])
 */