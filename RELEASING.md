# Releasing

Set variables:

    $ export VERSION=X.Y.Z
    $ export GPG_KEY=EA456E8BAF0109429583EED83578F667F2F3A5FA

Update version numbers (Including the dependency of `tracing-test` on
`tracing-test-macro`!):

    $ vim -p tracing-test/Cargo.toml tracing-test-macro/Cargo.toml

Ensure that everything still works:

    $ cargo check

Update changelog:

    $ vim CHANGELOG.md

Commit & tag:

    $ git commit -S${GPG_KEY} -m "Release v${VERSION}"
    $ git tag -s -u ${GPG_KEY} v${VERSION} -m "Version ${VERSION}"

Publish:

    $ cd tracing-test-macro && cargo publish
    $ cd ../tracing-test && cargo publish
    $ git push && git push --tags
