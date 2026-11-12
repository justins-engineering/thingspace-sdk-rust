#!/usr/bin/env bash
# Live Verizon calls are left out on purpose: five failed logins lock the ThingSpace account.
# shellcheck disable=SC2329 # gates are called by name, as "gate_$g"
# Runs the gates CI runs, in CI's order. "./test.sh help" lists them.
set -euo pipefail
cd "$(dirname "${BASH_SOURCE[0]}")"

SKIP="${SKIP:-}"
GATES="fmt build clippy test doc"

# Every command names its target: each feature ships for one, and an explicit --target keeps
# .cargo/config.toml's rustflags off build scripts and proc macros.
HOST="$(rustc -vV | sed -n 's/^host: //p')"
WASM=wasm32-unknown-unknown
WORKER=(--target "$WASM")
WEB=(--target "$WASM" --no-default-features --features wasm)
REQWEST=(--target "$HOST" --no-default-features --features reqwest)

usage() {
  cat <<EOF
usage: ./test.sh [gate...]   no gate runs them all; SKIP="gate..." sets gates aside visibly
  fmt     cargo fmt --check
  build   the three feature builds: worker and wasm on $WASM, reqwest on the host
  clippy  the same three under clippy; warnings denied for worker and reqwest
  test    unit and doc tests on the host: reqwest, and worker (the default feature)
  doc     cargo doc --no-deps for the worker feature; a broken doc link fails it
  help    this list
nothing here needs credentials or the network beyond cargo's own downloads
EOF
}

# Each gate runs in its own subshell with errexit on; a gate may leave a short reason in $NOTE.

gate_fmt() {
  cargo fmt --check
}

gate_build() {
  local failed=""
  cargo build "${WORKER[@]}" || failed+=" worker"
  cargo build "${WEB[@]}" || failed+=" wasm"
  cargo build "${REQWEST[@]}" || failed+=" reqwest"
  if [ -n "$failed" ]; then
    echo "failed:$failed" >"$NOTE"
    return 1
  fi
}

# The wasm feature is not lint-clean, so it fails on errors only and its warnings stay visible.
gate_clippy() {
  local failed=""
  cargo clippy "${WORKER[@]}" -- -D warnings || failed+=" worker"
  cargo clippy "${WEB[@]}" || failed+=" wasm"
  cargo clippy "${REQWEST[@]}" -- -D warnings || failed+=" reqwest"
  if [ -n "$failed" ]; then
    echo "failed:$failed" >"$NOTE"
    return 1
  fi
}

# The worker feature runs on the host as --lib and --doc: Cargo.toml's cf-worker [[example]]
# names a directory, and a plain cargo test refuses that. The reqwest run compiles
# examples/native.rs but never runs it; its doc examples only define functions.
gate_test() {
  local failed=""
  cargo test "${REQWEST[@]}" || failed+=" reqwest"
  cargo test --target "$HOST" --lib || failed+=" worker-unit"
  cargo test --target "$HOST" --doc || failed+=" worker-doc"
  if [ -n "$failed" ]; then
    echo "failed:$failed" >"$NOTE"
    return 1
  fi
}

# rustdoc skips function bodies, so a broken link is the main thing only this gate sees. Links
# to private items are not denied: public docs still carry some.
gate_doc() {
  RUSTDOCFLAGS="-D rustdoc::broken_intra_doc_links" cargo doc --no-deps "${WORKER[@]}"
}

case "${1:-}" in
  help | -h | --help)
    usage
    exit 0
    ;;
esac

for g in "$@" $SKIP; do
  case " $GATES " in
    *" $g "*) ;;
    *)
      echo "unknown gate: $g" >&2
      usage >&2
      exit 2
      ;;
  esac
done

NOTE="$(mktemp)"
trap 'rm -f "$NOTE"' EXIT
trap 'exit 130' INT TERM

# shellcheck disable=SC2086 # GATES is a list of words
[ $# -gt 0 ] || set -- $GATES
summary=()
status=0
start=$SECONDS
for g in "$@"; do
  case " $SKIP " in
    *" $g "*)
      summary+=("SKIP $g (named in SKIP)")
      continue
      ;;
  esac
  echo "==> $g"
  : >"$NOTE"
  set +e
  (
    set -e
    "gate_$g"
  )
  rc=$?
  set -e
  [ "$rc" -eq 130 ] && exit 130
  note="$(cat "$NOTE")"
  if [ "$rc" -eq 0 ]; then
    summary+=("PASS $g")
  else
    summary+=("FAIL $g${note:+ ($note)}")
    status=1
  fi
done

echo
echo "==> summary ($((SECONDS - start))s)"
printf '%s\n' "${summary[@]}"
exit "$status"
