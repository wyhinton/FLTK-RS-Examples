build_test_clippy(){
  while read path; do
    printf "Project: %s\n" "$path"
    cargo build --verbose --manifest-path "$path"
    cargo test --verbose --manifest-path "$path"
    cargo clippy --verbose --manifest-path "$path"
  done
}
find . -name 'Cargo.toml' | sort -u | build_test_clippy