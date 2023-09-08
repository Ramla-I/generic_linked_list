# generic_linked_list

The generic linked list verifies and can be used with a concrete type.
When it is used with a generic type, it gives the error message "[Prusti: invalid specification] return type of pure function does not implement Copy".

```with_concrete_type``` is the module the verifies.
```with_generic_type``` is nearly identical except it tries to use the linked list with a generic type ```Range<U>```.