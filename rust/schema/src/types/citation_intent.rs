// Generated file; do not edit. See `schema-gen` crate.

use crate::prelude::*;

/// The type or nature of a citation, both factually and rhetorically.
#[derive(Debug, Clone, PartialEq, Display, Serialize, Deserialize, Strip, Read, Write, HtmlCodec, TextCodec)]
#[serde(crate = "common::serde")]
pub enum CitationIntent {
    AgreesWith,
    CitesAsAuthority,
    CitesAsDataSource,
    CitesAsEvidence,
    CitesAsMetadataDocument,
    CitesAsPotentialSolution,
    CitesAsRecommendedReading,
    CitesAsRelated,
    CitesAsSourceDocument,
    CitesForInformation,
    Compiles,
    Confirms,
    ContainsAssertionFrom,
    Corrects,
    Credits,
    Critiques,
    Derides,
    Describes,
    DisagreesWith,
    Discusses,
    Disputes,
    Documents,
    Extends,
    GivesBackgroundTo,
    GivesSupportTo,
    HasReplyFrom,
    IncludesExcerptFrom,
    IncludesQuotationFrom,
    IsAgreedWithBy,
    IsCitedAsAuthorityBy,
    IsCitedAsDataSourceBy,
    IsCitedAsEvidenceBy,
    IsCitedAsMetadataDocumentBy,
    IsCitedAsPontentialSolutionBy,
    IsCitedAsRecommendedReadingBy,
    IsCitedAsRelatedBy,
    IsCitedAsSourceDocumentBy,
    IsCitedBy,
    IsCitedForInformationBy,
    IsCompiledBy,
    IsConfirmedBy,
    IsCorrectedBy,
    IsCreditedBy,
    IsCritiquedBy,
    IsDeridedBy,
    IsDescribedBy,
    IsDisagreedWithBy,
    IsDiscussedBy,
    IsDisputedBy,
    IsDocumentedBy,
    IsExtendedBy,
    IsLinkedToBy,
    IsParodiedBy,
    IsPlagiarizedBy,
    IsQualifiedBy,
    IsRefutedBy,
    IsRetractedBy,
    IsReviewedBy,
    IsRidiculedBy,
    IsSpeculatedOnBy,
    IsSupportedBy,
    IsUpdatedBy,
    Likes,
    LinksTo,
    ObtainsBackgroundFrom,
    ObtainsSupportFrom,
    Parodies,
    Plagiarizes,
    ProvidesAssertionFor,
    ProvidesConclusionsFor,
    ProvidesDataFor,
    ProvidesExcerptFor,
    ProvidesMethodFor,
    ProvidesQuotationFor,
    Qualifies,
    Refutes,
    RepliesTo,
    Retracts,
    Reviews,
    Ridicules,
    SharesAuthorInstitutionWith,
    SharesAuthorWith,
    SharesFundingAgencyWith,
    SharesJournalWith,
    SharesPublicationVenueWith,
    SpeculatesOn,
    Supports,
    Updates,
    UsesConclusionsFrom,
    UsesDataFrom,
    UsesMethodIn,
}
