# Publishing a new version

When looking to publish a new version, we can bump all the individual versions of the crates with

```
cargo workspaces version patch --all --no-git-push --no-git-tag --allow-branch "*"
```

This ensures that the workspaces have the right versioning
