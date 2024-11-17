---
title: Version Control
layout: default
nav_order: 5
---

# Version Control

Conduct provides a few different version control systems which can be used to version asset elements.

### [Direct](./direct)
Direct version control doesn't actually version assets, and just always overwrites the same file on each export. This is intended for use with external version control software such as git.

### [Versioned Directories](./versioned_directories)
Versioned Directories outputs each export to a new directory, bumping the version number each time. This results in a directory structure like so:

### [Symlink](./symlink)
Symlink version control makes use of two directories, a `pool` directory and a `link` directory. Each export bumps a versioned directory in the pool folder, and the updates a symlink in the link directory to point to the new pool.