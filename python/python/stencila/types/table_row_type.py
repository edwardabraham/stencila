# Generated file; do not edit. See the Rust `schema-gen` crate.

from .prelude import *


class TableRowType(StrEnum):
    """
    Indicates whether the row is in the header, body or footer of the table.
    """

    Header = "Header"
    Body = "Body"
    Footer = "Footer"
