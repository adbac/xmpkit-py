---
icon: lucide/package-open
---

# Installation

## Requirements

Before installing `xmpkit-py`, ensure you have **Python 3.10+** installed on your machine.

## Install

Install `xmpkit-py` through `pip` directly or using a package manager like `uv` or `poetry`.

=== "`pip`"

    ``` sh
    pip install xmpkit-py
    ```

=== "UV"

    ``` sh
    uv add xmpkit-py
    ```

=== "Poetry"

    ``` sh
    poetry add xmpkit-py
    ```

## Use

You're all set! You can now use the package by importing it in a Python script:

``` py
from xmpkit import XmpFile, XmpOptions, XmpMeta, XmpValue

file = XmpFile()
file.open_with("image.jpg", XmpOptions().for_update())

if (meta := file.get_xmp()) is not None:
    meta.set_property(
        "http://ns.adobe.com/xap/1.0/",
        "CreatorTool",
        XmpValue.String("MyApp"),
    )
    file.put_xmp(meta)

# Changes are written to disk when try_close() is called
file.try_close()
```

Read the [API Reference][] to get more examples and see the package structure.
