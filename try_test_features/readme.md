# Testing

### Test with Feature AAA
```
cargo test  --features "AAA"
```
```
running 2 tests
test tests::test_feature_AAA ... ok
test tests::test_no_feature ... ok
```

### Test with Feature BBB
```
cargo test  --features "BBB"
```
```
running 2 tests
test tests::test_no_feature ... ok
test tests::test_feature_BBB ... ok
```

### Test with no feature
```
cargo test
```
```
running 1 test
test tests::test_no_feature ... ok
```

# Conclusion
- `cargo test` runs everything what matches feature or has no feature specified!
- you can constrain test imports by feature
- if a test is using feature conditioned by feature import, it won't compile