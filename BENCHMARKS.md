# Performance benchmarks

The CodSpeed workflow runs CPU simulation and memory profiling on pull requests and pushes to `main`. Both instruments use the same benchmark cases.

Run `just bench-test` to validate every workload once without collecting timings. Use `nix develop -c just bench-test` if local build tools are missing.

With `cargo-codspeed` installed, run `just bench-build` and `just bench-run` to check the instrumented benchmarks. Local runs do not produce CodSpeed performance measurements without the CodSpeed runner.

## Workloads

- Query extraction: parse a page number and repeated tags, including a percent-encoded value, into typed parameters.

Each workload checks the parsed values before measurement and rejects parse errors while measuring.

## Interpreting results

Use CPU profiles to locate work and memory profiles to assess allocation changes.

CPU simulation does not measure time spent in system calls. These workloads do not establish network latency, concurrent client throughput, or backpressure behavior. Use a representative walltime workload on a dedicated runner when those properties are the optimization target.
