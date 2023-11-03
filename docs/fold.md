## AIR: Fold instruction
Here is the `fold` syntax.

`(fold <iterable> <iterator> <instruction> [<last_instruction>])`

Iterable can be: a stream, map, canonicalized stream, canonicalized map or scalar array.

If `last_instruction` is not set, then it's assumed:
 - `null` in case of `(fold (seq ...`
 - `never` in case of `(fold (par ...`

```clojure
(fold iterable iterator
	(seq
	  (ap ("key" iterator) %map)
	  (next iterator)
	)
	;; here is an implicit (null)
)
```

Let's consider the fold's operational semantics if iterable is the following scalar array `[1 2 3]`. Note that the recursion produced by `next` expressed in recursion of AquaVM execution engine function calls. Putting this differently the depth of the recursion depends on the size of the iterable.

```clojure
(fold iterable iterator
	(seq
	  (ap ("key" iterator) %map)
	  	(seq ;; next
		  (ap ("key" iterator) %map)
			(seq ;; next
				(ap ("key" iterator) %map)
				;; an implicit (null)
			)
		)
	)
)
```

There is so-called recursive fold which populates iterable stream `fold` is iterating over.

```clojure
(fold $iterable iterator ; data is a stream in the context
	(seq
	  (ap 42 $iterable)
	  (next iterator)
	)
	;; here is an implicit (null)
)
```


```clojure
(fold $iterable iterator ; data is a stream in the context
	(seq
	  (ap 42 $iterable)
		(seq ;; next
		  (ap 42 $iterable)
			(seq ;; next
				(ap 42 $iterable)
					...
			)
		)
	)
	;; here is an implicit (null)
)
```

A stream internally is a vector of vectors where inner vectors are called generations. In the previous case ap adds a new generation into `fold`.

Here is a high level overview how `fold` handles multiple generations of an iterable.

```clojure
fold
	run fold body for generation 1 values
		run last instruction
	run fold body for generation 2 values
		run last instruction
	...
	run fold body for generation N values
		run last instruction
```

There is a property of `fold` completeness. The invariant for this property is as follows, if fold finishes a run for at least one generation the fold is marked as complete. This property belongs to AquaVM execution engine and it is not explicitly sent out in a particle.
Please note that fold body that contains topology change events, e.g. `call`,`canon` destined for a node other than the current triggers that topology change.