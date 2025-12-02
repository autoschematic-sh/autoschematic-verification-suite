#!/bin/bash
set -exo pipefail

export AUTOSCHEMATIC_NO_SANDBOX=true
cargo run --bin autoschematic-testbench -- run --sequence sequences/unbundle.ron
cargo run --bin autoschematic-testbench -- run --sequence sequences/import.ron
cargo run --bin autoschematic-testbench -- run --sequence sequences/plan.ron
cargo run --bin autoschematic-testbench -- run --sequence sequences/apply.ron
cargo run --bin autoschematic-testbench -- run --sequence sequences/task_exec.ron