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
ndef

struct Example(
example: number,
text: string
) {
test: string

    fun my_function(key: str, my_func: str) -> string {
        
    }

    static fun static_fun(key: str) -> string? {
        if key == "test" {
            return "Wow thats right"
        } else {
            return null
        }
    }

    fun <A, B> generic_fun(key: A, value: B) {
        let variable = "Hello World"
        let
    }

}

# Type

type name = other

# Imports

import * from "other"
import * as other from "other"
import {testFunction, Other as Test} from "other"

fun myFunction(text: string, hello: number) {

}

