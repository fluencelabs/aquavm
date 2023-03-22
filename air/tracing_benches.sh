#!/usr/bin/env bash
set -o pipefail -e
#
# This command executes AquaVM on the same data as benches, but outputs a trace.
# Its dependencies:
#   + jq utility
#   + compiled AquaVM WASM binary
#   + installed air

function trace_len {
    jq '.trace | length'
}

function jq_prepend_trace {
    jq -c ".trace |= [ $1 ] + ."
}

function insert_left {
    local input="$1"
    local len=$(trace_len < "${input}")

    jq_prepend_trace "{par: [$len, 0]}" < "${input}"
}

function insert_right {
    local input="$1"
    local len=$(trace_len < "${input}")

    jq_prepend_trace "{par: [0, $len]}" < "${input}"
}

function error {
    echo "$1" >&1
    exit "${2:-1}"
}

function main {
    if [ "$#" \< 2 ]; then
        if [ x"$1" == x"--help" ]; then
            error "usage: $0 (--all|big|long) (--all|merge|nomerge) [ air run args ... ]"
        else
            error "Provide at least two arguments: (--all|big|long) and (--all|merge|nomerge)"
        fi
    fi
    local tests
    local subtests
    case $1 in
        big|long)
            tests=( "$1" )
            ;;
        --all)
            tests=( big long )
            ;;
        *)
            error "Unknown test type: $1"
            ;;
    esac
    case $2 in
        merge|nomerge)
            subtests=( "$2" )
            ;;
        --all)
            subtests=( merge nomerge )
            ;;
        *)
            error "Unknown subtest type: $2"
            ;;
    esac
    shift 2

    : "${AIR_INTERPRETER_WASM_PATH:=../target/wasm32-wasi/release/air_interpreter_server.wasm}"
    if [ ! -e "${AIR_INTERPRETER_WASM_PATH}" ]; then
        error "No AIR interpreter at ${AIR_INTERPRETER_WASM_PATH}."
    fi

    : "${INPUT_DATA_DIR:=./benches/data}"
    if [ ! -d "${INPUT_DATA_DIR}" ]; then
        error "No input data at ${INPUT_DATA_DIR}.  Set INPUT_DATA_DIR to proper value."
    fi

    export AIR_INTERPRETER_WASM_PATH

    prev_data_path=$(mktemp -t "tracing_benches_prev_data_XXXXXXX")
    current_data_path=$(mktemp -t "tracing_benches_current_data_XXXXXXX")
    trap 'rm -rf -- "$prev_data_path" "$current_data_path"' EXIT

    for tst in "${tests[@]}"; do
        case $tst in
            big)
                data_path="${INPUT_DATA_DIR}/anomaly_big.json"
                script_cmd="cat '${INPUT_DATA_DIR}/big.air'"
                ;;
            long)
                data_path="${INPUT_DATA_DIR}/anomaly_long.json"
                script_cmd="echo '(par (null) (null))'"
                ;;
        esac

        insert_left "$data_path" > "$current_data_path"

        for subt in "${subtests[@]}"; do
            case $subt in
                merge)
                    prev_side="left"
                    ;;
                nomerge)
                    prev_side="right"
                    ;;
            esac

            insert_$prev_side "$data_path" > "$prev_data_path"

            echo >&1
            echo "*** Running test ${tst}-${subt}..." >&1
            eval "${script_cmd}" | \
                air run "$@" --repeat 1 --plain \
                    --prev-data "$prev_data_path" \
                    --current-data "$current_data_path"
        done
    done
}

main "$@"
