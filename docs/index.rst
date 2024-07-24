pyjiff
======

Python bindings to the Jiff_ datetime
library.

.. note::

    This project is in the early stages of development and is not yet
    feature complete.

Installation
------------

You can install the package from PyPI:

.. code-block:: bash

    pip install jiff

Or you can install the package from source if you have
`Rust <https://www.rust-lang.org/tools/install>`_ installed:

.. code-block:: bash

    git clone https://github.com/TkTech/pyjiff.git
    cd pyjiff
    pip install .

Usage
-----

For the most part, the API mimics the classes and methods provided by the
Jiff_ library that it wraps. Here's a simple example, which is functionally
identical to the example provided in the Jiff README:


.. code-block:: python

    from jiff import Zoned, Timestamp, Span

    ts = Timestamp.from_string("2024-07-11T01:14:00Z")
    span = Span()
    span.months = 1
    span.hours = 2
    zoned = ts.intz("America/New_York") + span
    assert str(zoned) == "2024-08-10T23:14:00-04:00[America/New_York]"


.. toctree::
   :maxdepth: 2
   :caption: Contents:

   jiff


.. _Jiff: https://github.com/BurntSushi/jiff