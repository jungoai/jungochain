cargo build --release --features runtime-benchmarks
./target/production/jungochain-node benchmark pallet \
  --chain=local \
  --pallet=pallet_registry \
  --extrinsic="*" \
  --output=pallets/registry/src/weights.rs \
  --template=./.maintain/frame-weight-template.hbs
