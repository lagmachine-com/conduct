---
title: Versioned Directories
layout: default
parent: Version Control
nav_order: 2
---

# Versioned Directories (Version Control)
Versioned Directories outputs each export to a new directory, bumping the version number each time. This results in a directory structure like so:

```
export/
├─ asset/
│  ├─ suzanneA/
│  │  ├─ model/
│  │  │  ├─ mesh/
│  │  │  │  ├─ v001/
│  │  │  │  ├─ v002/
│  │  │  │  ├─ v003/
```

## Example Config
```yaml
version_control:
  type: versioned_directories
```