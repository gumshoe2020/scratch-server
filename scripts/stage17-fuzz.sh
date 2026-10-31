#!/bin/sh
# Stage 17 M2: fuzz one target for SECONDS (default 1800) with WORKERS (default 2) forked
# libFuzzer workers. Keep WORKERS x running targets <= 8 (another run shares this Mac).
# Any crash, panic, >1 s input or >256 MB RSS stops the run and leaves the input under
# fuzz/artifacts/<target>/. Needs the nightly toolchain on PATH and STAGE17_TLS_DIR.
# Usage: scripts/stage17-fuzz.sh <target>
set -eu
cd "$(dirname "$0")/.."
target="$1"
seconds="${SECONDS_BUDGET:-1800}"
workers="${WORKERS:-2}"
case "$target" in
  server_after_handshake) max_len=20000 ;;
  *) max_len=4096 ;;
esac
: "${STAGE17_TLS_DIR:?set STAGE17_TLS_DIR to the throwaway key directory}"
mkdir -p logs "fuzz/corpus/$target"
log="logs/stage17-fuzz-$target.log"
start=$(date +%s)
# ASan's default 256 MB free-quarantine counts toward RSS and trips -rss_limit_mb=256 with
# no real growth (NOTES.md, stage 17); 16 MB keeps use-after-free detection.
export ASAN_OPTIONS="${ASAN_OPTIONS:+$ASAN_OPTIONS:}quarantine_size_mb=16"
set +e
cargo fuzz run -O "$target" "fuzz/corpus/$target" "fuzz/seeds/$target" -- \
  -fork="$workers" -max_total_time="$seconds" -timeout=1 -rss_limit_mb=256 \
  -ignore_crashes=0 -ignore_timeouts=0 -ignore_ooms=0 -max_len="$max_len" \
  -close_fd_mask=1 -print_final_stats=1 > "$log" 2>&1
status=$?
set -e
end=$(date +%s)
echo "stage17: target=$target status=$status wall_seconds=$((end - start)) corpus_files=$(ls "fuzz/corpus/$target" | wc -l | tr -d ' ')" >> "$log"
tail -n 3 "$log"
exit "$status"
