# Build + test examples
test-examples: build-all test-all

# Build all example programs (runs cargo build-sbf in each example directory)
build-all:
	#!/usr/bin/env bash
	set -euo pipefail
	for e in examples/*/; do
	  e=${e%/}
	  e=${e##*/}
	  echo "[build] examples/$e"
	  if [ -d "examples/$e/programs" ]; then
	    for p in examples/$e/programs/*; do
	      if [ -f "$p/Cargo.toml" ]; then
	        echo "  -> building $p"
	        (cd "$p" && cargo build-sbf)
	      fi
	    done
	  else
	    (cd examples/"$e" && cargo build-sbf)
	  fi
	done

# Test of all example Solana programs in the `examples/` directory.
test-all:
	#!/usr/bin/env bash
	set -euo pipefail
	for e in examples/*/; do
	  e=${e%/}
	  e=${e##*/}
	  if [ -f "examples/$e/Cargo.toml" ]; then
	    echo "[test] examples/$e"
	    (cd examples/"$e" && cargo test)
	  fi
	done

# Build a single example: `just build <example>`
build example:
	#!/usr/bin/env bash
	set -euo pipefail
	if [ -d "examples/{{example}}/programs" ]; then
	  for p in examples/{{example}}/programs/*; do
	    if [ -f "$p/Cargo.toml" ]; then
	      echo "  -> building $p"
	      (cd "$p" && cargo build-sbf)
	    fi
	  done
	else
	  cd examples/{{example}} && cargo build-sbf
	fi

# Test a single example: `just test <example>`
test example:
	cd examples/{{example}} && cargo test

# Show the example list
list-examples:
	@ls -d examples/*/ | xargs -n1 basename
