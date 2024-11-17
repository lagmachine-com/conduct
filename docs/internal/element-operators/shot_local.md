---
title: shot_local
layout: default
parent: Element Operators
nav_order: 4
---

# Shot Local (Element Operator)

## Example Configuration

```yaml
assets:
  3d:
    character:
    - suzanneA: 
        departments:
          model: 
            - mesh
          anim:
            - !shot_local anim_cache
```

The `scene_local` operator sets a flag internally which causes this element to be written to and read from a unique file on a per-shot basis. In this example configuration, it's being used to output a unique `anim_cache` per shot.