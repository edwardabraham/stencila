# Generated file; do not edit. See the Rust `schema-gen` crate.

from .prelude import *

from .block import Block
Comment = ForwardRef("Comment")
from .creative_work import CreativeWork
from .creative_work_type import CreativeWorkType
from .creative_work_type_or_text import CreativeWorkTypeOrText
from .date import Date
from .grant_or_monetary_grant import GrantOrMonetaryGrant
ImageObject = ForwardRef("ImageObject")
from .inline import Inline
from .person import Person
from .person_or_organization import PersonOrOrganization
from .person_or_organization_or_software_application import PersonOrOrganizationOrSoftwareApplication
from .property_value_or_str import PropertyValueOrStr
SoftwareApplication = ForwardRef("SoftwareApplication")
from .software_source_code_or_software_application_or_str import SoftwareSourceCodeOrSoftwareApplicationOrStr
from .str_or_float import StrOrFloat
from .text import Text
from .thing_type import ThingType


@dataclass(init=False)
class SoftwareSourceCode(CreativeWork):
    """
    Computer programming source code. Example: Full (compile ready) solutions, code snippet samples, scripts, templates.
    """

    type: Literal["SoftwareSourceCode"] = field(default="SoftwareSourceCode", init=False)

    code_repository: Optional[str] = None
    """Link to the repository where the un-compiled, human readable code and related code is located."""

    code_sample_type: Optional[str] = None
    """What type of code sample: full (compile ready) solution, code snippet, inline code, scripts, template."""

    programming_language: Optional[str] = None
    """The computer programming language."""

    runtime_platform: Optional[List[str]] = None
    """Runtime platform or script interpreter dependencies (Example - Java v1, Python2.3, .Net Framework 3.0)."""

    software_requirements: Optional[List[SoftwareSourceCodeOrSoftwareApplicationOrStr]] = None
    """Dependency requirements for the software."""

    target_products: Optional[List[SoftwareApplication]] = None
    """Target operating system or product to which the code applies."""

    def __init__(self, id: Optional[str] = None, alternate_names: Optional[List[str]] = None, description: Optional[Text] = None, identifiers: Optional[List[PropertyValueOrStr]] = None, images: Optional[List[ImageObject]] = None, name: Optional[str] = None, url: Optional[str] = None, about: Optional[List[ThingType]] = None, abstract: Optional[List[Block]] = None, authors: Optional[List[PersonOrOrganization]] = None, contributors: Optional[List[PersonOrOrganizationOrSoftwareApplication]] = None, editors: Optional[List[Person]] = None, maintainers: Optional[List[PersonOrOrganization]] = None, comments: Optional[List[Comment]] = None, date_created: Optional[Date] = None, date_received: Optional[Date] = None, date_accepted: Optional[Date] = None, date_modified: Optional[Date] = None, date_published: Optional[Date] = None, funders: Optional[List[PersonOrOrganization]] = None, funded_by: Optional[List[GrantOrMonetaryGrant]] = None, genre: Optional[List[str]] = None, keywords: Optional[List[str]] = None, is_part_of: Optional[CreativeWorkType] = None, licenses: Optional[List[CreativeWorkTypeOrText]] = None, parts: Optional[List[CreativeWorkType]] = None, publisher: Optional[PersonOrOrganization] = None, references: Optional[List[CreativeWorkTypeOrText]] = None, text: Optional[Text] = None, title: Optional[List[Inline]] = None, version: Optional[StrOrFloat] = None, code_repository: Optional[str] = None, code_sample_type: Optional[str] = None, programming_language: Optional[str] = None, runtime_platform: Optional[List[str]] = None, software_requirements: Optional[List[SoftwareSourceCodeOrSoftwareApplicationOrStr]] = None, target_products: Optional[List[SoftwareApplication]] = None):
        super().__init__(id = id, alternate_names = alternate_names, description = description, identifiers = identifiers, images = images, name = name, url = url, about = about, abstract = abstract, authors = authors, contributors = contributors, editors = editors, maintainers = maintainers, comments = comments, date_created = date_created, date_received = date_received, date_accepted = date_accepted, date_modified = date_modified, date_published = date_published, funders = funders, funded_by = funded_by, genre = genre, keywords = keywords, is_part_of = is_part_of, licenses = licenses, parts = parts, publisher = publisher, references = references, text = text, title = title, version = version)
        self.code_repository = code_repository
        self.code_sample_type = code_sample_type
        self.programming_language = programming_language
        self.runtime_platform = runtime_platform
        self.software_requirements = software_requirements
        self.target_products = target_products
