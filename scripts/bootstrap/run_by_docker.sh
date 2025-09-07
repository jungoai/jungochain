#!/bin/bash

# Initialize variable defaults
args=()
node_name="nodeX" # default node name
archive_opt=""
rpc_methods_opt=""
unsafe_rpc_external_opt=""
telemetry_url_opt=""

# Parse arguments
while [[ $# -gt 0 ]]; do
    case "$1" in
        --name)
            if [[ -n "$2" && $2 != "--" ]]; then
                node_name="$2"
                shift 2 # Remove both the option and its value
            else
                echo "Error: --option requires a value."
                exit 1
            fi
            ;;
        --archive)
            archive_opt="--state-pruning archive"
            shift # Remove the current argument
            ;;
        --rpc)
            rpc_methods_opt="--rpc-methods Safe"
            unsafe_rpc_external_opt="--unsafe-rpc-external"
            shift # Remove the current argument
            ;;
        --telemetry-url)
            if [[ -n "$2" && $2 != "--" ]]; then
                telemetry_url_opt="--telemetry-url $2"
                shift 2 # Remove both the option and its value
            else
                echo "Error: --option requires a value."
                exit 1
            fi
            ;;
        --)
            shift # Remove the '--' delimiter
            args+=("$@") # Collect the remaining arguments
            break
            ;;
        *)
            echo "Unknown argument: $1"
            exit 1
            ;;
    esac
done

# Variables
chain_image="ghcr.io/jungoai/jungochain:0.1.0-devnet"
chain_type="devnet"
base_path_="$HOME/.jungochain" #"/var/lib/jungochain"
address="0.0.0.0"
port=30333
rpc_port=9944
# TODO: add other boot nodes
boot_nodes="/ip4/52.14.41.79/tcp/30333/p2p/12D3KooWPVLR5YFa6nBcXU4wi7KJtwSx1VGuYQ4gzS3QTLkTFhSm"

base_path="$base_path_/$node_name"
chain_spec="$base_path_/$chain_type"_spec.json
chain_spec_raw="${chain_spec%.*}"_raw.json

log_max_size="10m"
log_max_file=10

mkdir -p "$base_path_"

export_spec() {
    docker run -v "$base_path_":"$base_path_" "$chain_image" \
        build-spec                                           \
        --disable-default-bootnode                           \
        --chain "$chain_type"                                \
        > "$chain_spec"
}

spec_to_raw() {
    docker run -v "$base_path_":"$base_path_" "$chain_image" \
        build-spec                                           \
        --chain "$chain_spec"                                \
        --raw                                                \
        --disable-default-bootnode                           \
        > "$chain_spec_raw"
}

export_raw_spec() {
    export_spec
    spec_to_raw
}

run_() {
    docker run                                      \
        --network       host                        \
        -v              "$base_path_:$base_path_"   \
        --log-driver    json-file                   \
        --log-opt       max-size="$log_max_size"    \
        --log-opt       max-file="$log_max_file"    \
        -d                                          \
        "$chain_image"                              \
        \
        --base-path             "$base_path"                    \
        --chain                 "$chain_spec_raw"               \
        --name                  "$node_name"                    \
        --public-addr           /ip4/"$address"/tcp/"$port"     \
        --port                  "$port"                         \
        --rpc-port              "$rpc_port"                     \
        --rpc-cors              all                             \
        --rpc-max-connections   5000                            \
        $rpc_methods_opt                                        \
        $unsafe_rpc_external_opt                                \
        $archive_opt                                            \
        $telemetry_url_opt                                      \
        --bootnodes             "$boot_nodes"                   \
        --unsafe-force-node-key-generation
}

run() {
    export_raw_spec
    run_
}

run
