

For original `Error` struct:

```rust
#[derive(Diagnostic)]
#[diag(parse_unexpected_default_value_for_lifetime_in_generic_parameters)]
pub(crate) struct UnexpectedDefaultValueForLifetimeInGenericParameters {
    #[primary_span]
    #[label]
    pub span: Span,
}
```

and `Fluent` file:

```
parse_unexpected_default_value_for_lifetime_in_generic_parameters = unexpected default lifetime parameter
    .label = lifetime parameters cannot have default values

```

change to:

```rust
#[derive(Diagnostic)]
#[diag(msg = "unexpected default lifetime parameter", label="lifetime parameters cannot have default values")]
pub(crate) struct UnexpectedDefaultValueForLifetimeInGenericParameters {
    #[primary_span]
    #[label]
    pub span: Span,
}
```



