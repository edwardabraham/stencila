# Generated file; do not edit. See the Rust `schema-gen` crate.

from .prelude import *

from .inline import Inline
from .suggestion import Suggestion


@dataclass(init=False)
class Insert(Suggestion):
    """
    A suggestion to insert some inline content.
    """

    type: Literal["Insert"] = field(default="Insert", init=False)

    def __init__(self, content: List[Inline], id: Optional[str] = None):
        super().__init__(id = id, content = content)
        
