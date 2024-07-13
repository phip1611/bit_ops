# Contributing

## General Guidelines

- The Function API is the baseline
- Each function must be `const` if possible
- The BitOps trait functions should have the same order as the functions of
  the Functions API
- The BitOps implementors should use the same ordering for the methods as the
  Trait
- The Trait API implementations should forward to the Function API to prevent
  duplication.

## Unit Tests

- The base for all unit tests is the Function API module.
- Don't repeat yourself. If a test works for u64, we don't necessarily also have
  to test it with all other types, as the underlying macro produces similar code
  anyway. Selected samples for tests of other modules/other types are okay.
- Each function should have an Example, which is also a doc test.
