---
title: department_is_not
layout: default
parent: Element Operators
nav_order: 2
---

# Department Is Not (Element Operator)

## Example Configuration

```yaml
assets:
  3d:
    character:
    - suzanneA: 
        departments:
          rigging:
            - !department_is_not anim: anim_cache
```

The `department_is_not` operator will only return it's element if the current department does not match it's key, which in this case is `anim`.


You can also return multiple elements by configuring as a list:

```yaml
assets:
  3d:
    character:
    - suzanneA: 
        departments:
          rigging:
            - !department_is_not anim: 
              - anim_cache
              - anim_cacheB
              - anim_cacheC
```