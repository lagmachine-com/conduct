---
title: Direct
layout: default
parent: Version Control
nav_order: 1
---

# Direct (Version Control)
Direct version control doesn't actually version assets, and just always overwrites the same file on each export. This is intended for use with external version control software such as git and may be preferable in a game development context, where assets generally need to always be in the same location.


## Example Config
```yaml
version_control:
  type: direct
```