cargo build --profile production --features runtime-benchmarks
./target/production/jungochain-node benchmark pallet \
  --chain=local \
  --pallet=pallet_commitments \
  --extrinsic="*" \
  --output=pallets/commitments/src/weights.rs \
  --template=./.maintain/frame-weight-template.hbs
