---
title: Symlink
layout: default
parent: Version Control
nav_order: 3
---

# Symlink (Version Control)
Symlink version control makes use of two directories, a `pool` directory and a `link` directory. Each export bumps a versioned directory in the pool folder, and the updates a symlink in the link directory to point to the new pool.

### Pool Directory
```
pool/
├─ asset/
│  ├─ suzanneA/
│  │  ├─ model/
│  │  │  ├─ mesh/
│  │  │  │  ├─ v001/
│  │  │  │  ├─ v002/
```

### Links

```
links/
├─ asset/
│  ├─ suzanneA/
│  │  ├─ model/
│  │  │  ├─ mesh/ 🔗 -> pool/asset/suzanneA/model/mesh/v002/
```

This has two main benefits:

### Automatic Element Updates
Since programs will be reading from the `link` directory, when the link is updated to point to a new element version any other file which reads from that link will automatically bring in the new file 

### Store Elements Outside Project
Since the pool directory can be configured to point elsewhere, the actual element export files can be stored outside of the project directory while still being versioned by the links.

This is particularly useful when coupled with git, as you wont need to check in large binary files, but the versions being used will still be versioned by git

## Example Configuration
```yaml
version_control:
  type: symlink
  relative: false
  pool: /project/example/pool
```

```yaml
version_control:
  type: symlink
  relative: false
  pool: ../pool
```
