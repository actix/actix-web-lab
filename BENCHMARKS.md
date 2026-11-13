# Performance benchmarks

The CodSpeed workflow runs CPU simulation and memory profiling on pull requests and pushes to `main`. Both instruments use the same benchmark cases.

Run `just bench-test` to validate every workload once without collecting timings. Use `nix develop -c just bench-test` if local build tools are missing.

With `cargo-codspeed` installed, run `just bench-build` and `just bench-run` to check the instrumented benchmarks. Local runs do not produce CodSpeed performance measurements without the CodSpeed runner.

## Workloads

- Actix SSE body: direct streams, prefilled channels, streams that insert `Pending` polls, and initial retry events. Cases vary message size and batch size.

Each workload checks expected events or bytes before measurement. Body benchmarks also reject errors while consuming the measured output.

Message size arguments specify payload bytes. Cases include 32 B, 1 KiB, and 64 KiB, plus 1,000 B and 60,000 B. Batch counts include one message, 16 messages, and 128 messages.

Metadata adds more framing bytes in the Actix SSE cases. Use memory profiles to see which cases change allocation counts or sizes.

Actix SSE body cases exclude input and channel setup. They measure body polling and encoding without collecting the response into another buffer.

## Interpreting results

Use CPU profiles to locate work and memory profiles to assess allocation changes. Compare all affected cases: an improvement for large messages can increase the cost of small messages.

CPU simulation does not measure time spent in system calls. These workloads do not establish network latency, concurrent client throughput, or backpressure behavior. Use a representative walltime workload on a dedicated runner when those properties are the optimization target.
