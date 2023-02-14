#![feature(associated_type_bounds)]

use std::mem::ManuallyDrop;

struct S1 { f: dyn Iterator<Item: Copy> }
//~^ ERROR associated type bounds are only allowed in where clauses and function signatures
//~| ERROR the value of the associated type `Item` (from trait `Iterator`) must be specified
struct S2 { f: Box<dyn Iterator<Item: Copy>> }
//~^ ERROR associated type bounds are only allowed in where clauses and function signatures
//~| ERROR the value of the associated type `Item` (from trait `Iterator`) must be specified
struct S3 { f: dyn Iterator<Item: 'static> }
//~^ ERROR associated type bounds are only allowed in where clauses and function signatures
//~| ERROR the value of the associated type `Item` (from trait `Iterator`) must be specified

enum E1 { V(dyn Iterator<Item: Copy>) }
//~^ ERROR associated type bounds are only allowed in where clauses and function signatures
//~| ERROR the value of the associated type `Item` (from trait `Iterator`) must be specified
enum E2 { V(Box<dyn Iterator<Item: Copy>>) }
//~^ ERROR associated type bounds are only allowed in where clauses and function signatures
//~| ERROR the value of the associated type `Item` (from trait `Iterator`) must be specified
enum E3 { V(dyn Iterator<Item: 'static>) }
//~^ ERROR associated type bounds are only allowed in where clauses and function signatures
//~| ERROR the value of the associated type `Item` (from trait `Iterator`) must be specified

union U1 { f: ManuallyDrop<dyn Iterator<Item: Copy>> }
//~^ ERROR associated type bounds are only allowed in where clauses and function signatures
//~| ERROR the value of the associated type `Item` (from trait `Iterator`) must be specified
union U2 { f: ManuallyDrop<Box<dyn Iterator<Item: Copy>>> }
//~^ ERROR associated type bounds are only allowed in where clauses and function signatures
//~| ERROR the value of the associated type `Item` (from trait `Iterator`) must be specified
union U3 { f: ManuallyDrop<dyn Iterator<Item: 'static>> }
//~^ ERROR associated type bounds are only allowed in where clauses and function signatures
//~| ERROR the value of the associated type `Item` (from trait `Iterator`) must be specified

fn main() {}
