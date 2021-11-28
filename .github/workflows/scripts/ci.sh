build_test_clippy(){
  while read path; do
    printf "Project: %s\n" "$path"
    cargo build +nightly --verbose --manifest-path "$path"
    cargo test +nightly --verbose --manifest-path "$path"
    # cargo clippy --verbose --manifest-path "$path"
  done
}
find . -name 'Cargo.toml' | sort -u | build_test_clippy