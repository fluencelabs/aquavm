## AIR: Fold instruction
Here is the `fold` syntax.

`(fold <iterable> <iterator> <instruction> [<last_instruction>])`

`last_instruction` is:
- an implicit `null` for scalar-based iterables
- an implicit `never` for stream-based iterables

```clojure
(fold data value
	(seq
	  (ap ("somesome-lengthy-and-scary-key" value) %map)
	  (next value)
	)
	;; here is an implicit (null)
)
```

Here is an operational semantics of fold is if data is `[1 2 3]`. Please note that the recursion produced by `next` expressed in recursion of AquaVM execution engine function calls. Putting this differently the depth of the recursion depends on the size of the iterable.

```clojure
(fold data value
	(seq
	  (ap ("somesome-lengthy-and-scary-key" value) %map)
	  	(seq ;; next
		  (ap ("somesome-lengthy-and-scary-key" value) %map)
			(seq ;; next
				(ap ("somesome-lengthy-and-scary-key" value) %map)
				;; an implicit (null)
			)
		)
	)
)
```

There is so-called recursive fold which populates iterable stream `fold` is iterating over.

```clojure
(fold $data value ; data is a stream in the context
	(seq
	  (ap 42 $data)
	  (next value)
	)
	;; here is an implicit (null)
)
```


```clojure
(fold $data value ; data is a stream in the context
	(seq
	  (ap 42 $data)
		(seq ;; next
		  (ap 42 $data)
			(seq ;; next
				(ap 42 $data)
					...
			)
		)
	)
	;; here is an implicit (null)
)
```

Fold internally is a vector of vectors where inner vectors are called generations. In the previous case ap adds a new generation into `fold`.

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