---
title: department_is
layout: default
parent: Element Operators
nav_order: 1
---

# Department Is (Element Operator)

{: .note }
> The `department_is` filter only filters elements during asset *load*

## Example Configuration

```yaml
assets:
  3d:
    character:
    - suzanneA: 
        departments:
          rigging:
            - !department_is(anim) rig
```

The `department_is` operator will only return it's element if the current department matches it's key, which in this case is `anim`.

You can also return multiple elements by configuring as a list:

```yaml
assets:
  3d:
    character:
    - suzanneA: 
        departments:
          rigging:
            - !department_is(anim) 
              - rig
              - rigB
              - rigC
```

