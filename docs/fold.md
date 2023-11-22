## AIR: Fold instruction
Here is the `fold` syntax.

`(fold <iterable> <iterator> <instruction> [<last_instruction>])`

Iterable can be: a stream, map, canonicalized stream, canonicalized map or scalar array.

If `last_instruction` is not set, then it's assumed:
 - `null` in case of `(fold (seq ...`
 - `never` in case of `(fold (par ...`

Here is a simple example when `fold` loops over a scalar to populate map using `iterator` as key-value pair value:
```clojure
(fold iterable iterator
	(seq
	  (ap ("key" iterator) %map)
	  (next iterator)
	)
	;; here is an implicit (null)
)
```

This example demonstrates the case when `next` is not the last instruction. This feature is supported with `fold`` over scalar only.
```clojure
(fold iterable iterator
	(seq
	  (next iterator)
	  (ap ("key" iterator) %map)
	)
	;; here is an implicit (null)
)
```

Fold body can have no `next`. The body will be executed only once in this case.

```clojure
(fold iterable iterator
	  (ap ("key" iterator) %map)
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
There are some subtle details about fold processing over a number of generations, namely:
- runs of previous generations do not affect next generations in any way
- if previous generation doesn't call `next` AquaVM executes `fold` for the next generation.

There is a property of `fold` completeness. The invariant for this property is as follows, if fold finishes a run for at least one generation the fold is marked as complete. This property belongs to AquaVM execution engine and it is not explicitly sent out in a particle.
Please note that fold body that triggers a node change events, e.g. `call`,`canon` destined for a node other than the current, forces the executiont to migrate to the destined node.
There is an exception for this rule. Consider the example:

```clojure
(seq
	(call "local_node" ("m" "returns_array") [] $stream)
	(fold $stream iter
		(seq
			(ap 42 $stream)
			(seq
				(call "remote_node" ("m" "f") [42] scalar)
				(next iter)
			)
		)
	)
)
```

with the example above there will be multiple iterations b/c `$stream` is populated with a new generation every `(ap 42 $stream)` call. AquaVM runs the part that preceds the `call` at `remote_node` for every generation added. The migration triggered by the `call` does not stop the runs of that fold body part that preceds the `call`.