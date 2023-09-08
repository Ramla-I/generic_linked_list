# generic_linked_list
In this branch, the same code verifies with the newer version of a linked list (which uses the Option type) but fails with the older version of the linked list which used the Link enum. It fails with an unexpected error.

In the main.rs file, uncomment the module you want to verify: with_option verifies but without_option does not.

Interestingly though, both versions of the linked list verify by themselves, the without_option version only fails when we try to use it in the can_create_new function.



