# Generated file; do not edit. See the Rust `schema-gen` crate.

from .prelude import *

from .automatic_execution import AutomaticExecution
from .block import Block
from .call_argument import CallArgument
from .code_error import CodeError
from .duration import Duration
from .execution_dependant import ExecutionDependant
from .execution_dependency import ExecutionDependency
from .execution_digest import ExecutionDigest
from .execution_required import ExecutionRequired
from .execution_status import ExecutionStatus
from .execution_tag import ExecutionTag
from .include import Include
from .timestamp import Timestamp


@dataclass(init=False)
class Call(Include):
    """
    Call another document, optionally with arguments, and include its executed content.
    """

    type: Literal["Call"] = field(default="Call", init=False)

    arguments: List[CallArgument]
    """The value of the source document's parameters to call it with"""

    def __init__(self, source: str, arguments: List[CallArgument], id: Optional[str] = None, auto_exec: Optional[AutomaticExecution] = None, compilation_digest: Optional[ExecutionDigest] = None, execution_digest: Optional[ExecutionDigest] = None, execution_dependencies: Optional[List[ExecutionDependency]] = None, execution_dependants: Optional[List[ExecutionDependant]] = None, execution_tags: Optional[List[ExecutionTag]] = None, execution_count: Optional[int] = None, execution_required: Optional[ExecutionRequired] = None, execution_kernel: Optional[str] = None, execution_status: Optional[ExecutionStatus] = None, execution_ended: Optional[Timestamp] = None, execution_duration: Optional[Duration] = None, errors: Optional[List[CodeError]] = None, media_type: Optional[str] = None, select: Optional[str] = None, content: Optional[List[Block]] = None):
        super().__init__(id = id, auto_exec = auto_exec, compilation_digest = compilation_digest, execution_digest = execution_digest, execution_dependencies = execution_dependencies, execution_dependants = execution_dependants, execution_tags = execution_tags, execution_count = execution_count, execution_required = execution_required, execution_kernel = execution_kernel, execution_status = execution_status, execution_ended = execution_ended, execution_duration = execution_duration, errors = errors, source = source, media_type = media_type, select = select, content = content)
        self.arguments = arguments
