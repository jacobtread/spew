# Comments

// This is a comment

/*

Multiline comment
Test this is lots of
lines yes test

*/

# Types

str
num
obj
bool
null
any
ndef
void

trait 

struct Test {
  name: str
  value: num
}

impl Test {
  
}

struct Other {
  b: str
}

type MyType = Test & Other

# Type

type name = other

# Imports

import * from "other"
import * as other from "other"
import {testFunction, Other as Test} from "other"

