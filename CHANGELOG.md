# Changelog

All notable changes to this project will be documented in this file.

This project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html).


## 0.2.5 - 2024-06-02

- [change] Replace `lazy_static` dependency with `std::sync::OnceCell` (#36)
- [change] Update syn (#40)
- [change] Include license text in the packaged crates (#41)


## 0.2.4 - 2023-02-01

- [bug] `logs_assert` should not get logs from other tests (#19)
- [bug] Fully qualify usage of `Result` in `logs_assert` (#17)
- [docs] Add note that integration tests need no-env-filter (#20) 


## 0.2.3 - 2022-07-20

- [feature] Add no-env-filter feature to disable log filtering (#16)


## 0.2.2 - 2022-06-03

- [bug] Ensure correct `Result` type is used (#15) 


## 0.2.1 - 2021-11-23

- [bug] Fix wrong internal dependencies


## 0.2.0 - 2021-11-23

- [feature] Add new `logs_assert` function that allows for more flexible log
  assertions (#7)
- [change] Bump `tokio` dev-dependency to version 1 (#9)
- [change] Bump `tracing-subscriber` dependency to 0.3 (#11)


## 0.1.0 - 2020-11-19

Initial release to crates.io.
