PR reviewing conventions
----------------------

In the AquaVM repo the following conventions for PR reviewing is used:  
- **Concept ACK** - agree with the idea and overall concept, but haven't reviewed the code changes or tested them,
- **utACK (untested ACK)** - reviewed and agree with the code changes but haven't actually tested them,
- **Tested ACK** - reviewed the code changes and have verified the functionality or a bug fix, 
- **NACK** - disagree with the code changes/concept, should be accompanied by an explanation.

Coding style
----------------------

__Import scheme__  
We follow this import scheme:
 - imports from local project (crate::/super::)
 - newline
 - other imports from non-std
 - newline
 - imports from std

If there are not so many imports newlines could be omitted.

Example:
```rust
use super::ExecutionCtx;
use super::ExecutionResult;

use air_parser::ast;
use air_parser::ast::Fail;

use std::rc::Rc;
```

__Doc comment style__  
```rust
/// This is a doc comment with a dot at the end.
```
