# Generated file; do not edit. See the Rust `schema-gen` crate.

from .prelude import *

from .automatic_execution import AutomaticExecution
from .code_error import CodeError
from .duration import Duration
from .execution_dependant import ExecutionDependant
from .execution_dependency import ExecutionDependency
from .execution_digest import ExecutionDigest
from .execution_required import ExecutionRequired
from .execution_status import ExecutionStatus
from .execution_tag import ExecutionTag
from .node import Node
from .parameter import Parameter
from .timestamp import Timestamp
Validator = ForwardRef("Validator")


@dataclass(init=False)
class CallArgument(Parameter):
    """
    The value of a `Parameter` to call a document with.
    """

    type: Literal["CallArgument"] = field(default="CallArgument", init=False)

    code: str
    """The code to be evaluated for the parameter."""

    programming_language: Optional[str] = None
    """The programming language of the code."""

    def __init__(self, name: str, code: str, id: Optional[str] = None, auto_exec: Optional[AutomaticExecution] = None, compilation_digest: Optional[ExecutionDigest] = None, execution_digest: Optional[ExecutionDigest] = None, execution_dependencies: Optional[List[ExecutionDependency]] = None, execution_dependants: Optional[List[ExecutionDependant]] = None, execution_tags: Optional[List[ExecutionTag]] = None, execution_count: Optional[int] = None, execution_required: Optional[ExecutionRequired] = None, execution_kernel: Optional[str] = None, execution_status: Optional[ExecutionStatus] = None, execution_ended: Optional[Timestamp] = None, execution_duration: Optional[Duration] = None, errors: Optional[List[CodeError]] = None, label: Optional[str] = None, value: Optional[Node] = None, default: Optional[Node] = None, validator: Optional[Validator] = None, hidden: Optional[bool] = None, derived_from: Optional[str] = None, programming_language: Optional[str] = None):
        super().__init__(id = id, auto_exec = auto_exec, compilation_digest = compilation_digest, execution_digest = execution_digest, execution_dependencies = execution_dependencies, execution_dependants = execution_dependants, execution_tags = execution_tags, execution_count = execution_count, execution_required = execution_required, execution_kernel = execution_kernel, execution_status = execution_status, execution_ended = execution_ended, execution_duration = execution_duration, errors = errors, name = name, label = label, value = value, default = default, validator = validator, hidden = hidden, derived_from = derived_from)
        self.code = code
        self.programming_language = programming_language
