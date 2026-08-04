# Luhn checksum validate/generate

```
src/lib.rs
```

Usage is covered by the test sitting next to the source, so if you want to see it in action, that's the first place to look.

This package validates and generates Luhn check digits for things like card numbers and ID strings. It's pure Go with zero dependencies, so there's no external service to stand up and no vendor code to audit. You just import it and call the functions.

The implementation stays entirely on the standard library, which keeps the deployment story simple: build the binary, run it, and you're done. No background processes to monitor, no capacity to plan around for a checksum utility. That's the kind of thing I'd rather not carry an on-call burden for, and with this you don't have to.

```go
// Example usage, assuming the API from the test:
//   valid, err := luhn.Validate("79927398713")
//   check, err := luhn.Generate("7992739871")
```

If you're weighing whether to roll your own versus pulling in a managed option, the math here is pretty one-sided. A self-hosted checksum function costs you a few dozen lines of code and nothing at runtime. A managed service for the same thing would add latency, a network dependency, and a vendor lock-in that buys you nothing for a problem this small. The standard library approach wins on every axis that matters for SLOs: fewer moving parts, fewer failure modes, and no third-party availability to factor into your error budget.

The code is deliberately small and reviewable. There's no magic, no hidden state, and nothing that's going to surprise you during a capacity review. It either returns a valid check digit or it doesn't, and the test suite pins down the edge cases so you can trust the behavior without having to trace through it yourself.