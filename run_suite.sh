#!/bin/bash
set -exo pipefail

cargo run --bin autoschematic-testbench -- run --sequence sequences/unbundle.ron
cargo run --bin autoschematic-testbench -- run --sequence sequences/import.ron
cargo run --bin autoschematic-testbench -- run --sequence sequences/plan.ron
cargo run --bin autoschematic-testbench -- run --sequence sequences/apply.ron