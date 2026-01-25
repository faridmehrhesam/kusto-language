use std::ops::Range;

#[derive(Debug, PartialEq, Clone, Copy, Eq)]
pub enum TokenKind {
    // keywords
    __CrossClusterKeyword,
    __CrossDBKeyword,
    __IdKeyword,
    __IsFuzzyKeyword,
    __NoWithSourceKeyword,
    __PackedColumnKeyword,
    __SourceColumnIndexKeyword,
    _3DChartKeyword,

    AccessKeyword,
    AccumulateKeyword,
    AliasKeyword,
    AndKeyword,
    AnomalyChartKeyword,
    AreaChartKeyword,
    AsKeyword,
    AscKeyword,
    AssertSchemaKeyword,

    BagExpansionKeyword,
    BarChartKeyword,
    BetweenKeyword,
    BinKeyword,
    BinLegacy,
    ByKeyword,

    CachingPolicyKeyword,
    CalloutKeyword,
    CancelKeyword,
    CardKeyword,
    ColumnChartKeyword,
    CommandsAndQueriesKeyword,
    ConsumeKeyword,
    ContainsKeyword,
    ContainsCsKeyword,
    ContainsCsKeyword2,
    ContextualDataTableKeyword,
    CountKeyword,
    CyclesKeyword,

    DatabaseKeyword,
    DataExportKeyword,
    DataScopeKeyword,
    DataTableKeyword,
    DeclareKeyword,
    DecodeBlocksKeyword,
    DefaultKeyword,
    DescKeyword,
    DistinctKeyword,

    EarliestKeyword,
    EncodingPolicyKeyword,
    EndsWithKeyword,
    EndsWithCsKeyword,
    EntityGroupKeyword,
    EvaluateKeyword,
    ExecuteAndCacheKeyword,
    ExpandOutputKeyword,
    ExtendKeyword,
    ExtentTagsRetentionKeyword,
    ExternalDataKeyword,
    ExternalDataKeyword2,

    //Inline External Table Keywords
    DataFormatKeyword,
    InlineExternalTableKeyword,
    DateTimePatternKeyword,
    // End Inline External Table Keywords
    FacetKeyword,
    FilterKeyword,
    FindKeyword,
    FirstKeyword,
    FlagsKeyword,
    ForkKeyword,
    FromKeyword,

    GetSchemaKeyword,
    GrannyAscKeyword,
    GrannyDescKeyword,
    GraphMatchKeyword,
    GraphShortestPathsKeyword,
    GraphToTableKeyword,
    GraphEdgesKeyword,

    HardDeleteKeyword,
    HardRetentionKeyword,
    HasKeyword,
    HasAnyKeyword,
    HasAllKeyword,
    HasCsKeyword,
    HasPrefixKeyword,
    HasPrefixCsKeyword,
    HasSuffixKeyword,
    HasSuffixCsKeyword,
    HintDotConcurrencyKeyword,
    HintDotDistributionKeyword,
    HintDotMaterializedKeyword,
    HintDotNumPartitions,
    HintDotShuffleKeyKeyword,
    HintDotSpreadKeyword,
    HintDotRemoteKeyword,
    HintDotStrategyKeyword,
    HintDotProgressiveTopKeyword,
    HotCacheKeyword,

    IdKeyword,
    InKeyword,
    InCsKeyword,
    InvokeKeyword,
    IsFuzzyKeyword,
    BestEffortKeyword,
    ForceRemoteKeyword,

    JoinKeyword,
    JournalKeyword,

    KindKeyword,

    LadderChartKeyword,
    LastKeyword,
    LatestKeyword,
    LetKeyword,
    LikeKeyword,
    LikeCsKeyword,
    LimitKeyword,
    LineChartKeyword,
    LookupKeyword,

    MacroExpandKeyword,
    MakeGraphKeyword,
    GraphMarkComponentsKeyword,
    GraphWhereNodesKeyword,
    GraphWhereEdgesKeyword,
    MakeSeriesKeyword,
    MatchesRegexKeyword,
    MaterializeKeyword,
    MaterializedViewCombineKeyword,
    MaterializedViewsKeyword,
    MdmKeyword,
    MissingKeyword,
    MvApplyKeyword,
    MvDashApplyKeyword,
    MvDashExpandKeyword,
    MvExpandKeyword,

    NodesKeyword,
    NoOptimizationKeyword,
    NotBetweenKeyword,
    NotContainsKeyword,
    NotContainsCsKeyword,
    NotBangContainsKeyword,
    NotBangContainsCsKeyword,
    NotEndsWithKeyword,
    NotEndsWithCsKeyword,
    NotHasKeyword,
    NotHasCsKeyword,
    NotHasPrefixKeyword,
    NotHasPrefixCsKeyword,
    NotHasSuffixKeyword,
    NotHasSuffixCsKeyword,
    NotInKeyword,
    NotInCsKeyword,
    NotLikeKeyword,
    NotLikeCsKeyword,
    NotStartsWithKeyword,
    NotStartsWithCsKeyword,
    NullKeyword,
    NullsKeyword,

    OfKeyword,
    OnKeyword,
    OptionalKeyword,
    OrKeyword,
    OrderKeyword,
    OthersKeyword,
    OutputKeyword,

    PackKeyword,
    ParseKeyword,
    ParseWhereKeyword,
    ParseKvKeyword,
    PartitionedByKeyword,
    PartitionByKeyword,
    PartitionKeyword,
    PathFormatKeyword,
    PatternKeyword,
    PieChartKeyword,
    PivotChartKeyword,
    PlotlyKeyword,
    GraphKeyword,
    PrintKeyword,
    ProjectKeyword,
    ProjectAwayKeyword,
    ProjectByNamesKeyword,
    _ProjectAwayKeyword,
    ProjectKeepKeyword,
    ProjectRenameKeyword,
    ProjectReorderKeyword,
    ProjectSmartKeyword,

    QueriesKeyword,
    QueryParametersKeyword,
    QueryResultsKeyword,

    RangeKeyword,
    ReduceKeyword,
    RegexKeyword,
    RelaxedKeyword,
    RenderKeyword,
    RestrictedViewAccessKeyword,
    RestrictKeyword,
    RowLevelSecurityKeyword,
    RowstoreKeyword,
    RowstoreReferencesKeyword,
    RowstoreSealInfoKeyword,
    RowstorePolicyKeyword,
    RowstoresKeyword,

    SampleKeyword,
    SampleDistinctKeyword,
    ScanKeyword,
    ScatterChartKeyword,
    SealKeyword,
    SealsKeyword,
    SearchKeyword,
    SerializeKeyword,
    SetKeyword,
    SetOrAppendKeyword,
    SetOrReplaceKeyword,
    ShardsKeyword,
    SimpleKeyword,
    SoftDeleteKeyword,
    SoftRetentionKeyword,
    SortKeyword,
    SqlKeyword,
    StackedAreaChartKeyword,
    StartsWithKeyword,
    StartsWithCsKeyword,
    StatisticsKeyword,
    StepKeyword,
    StoredQueryResultContainersKeyword,
    SummarizeKeyword,

    TablePurgeKeyword,
    TakeKeyword,
    TimeChartKeyword,
    TimelineKeyword,
    TimePivotKeyword,
    TitleKeyword,
    ToKeyword,
    TopKeyword,
    TopHittersKeyword,
    TopNestedKeyword,
    ToScalarKeyword,
    ToTableKeyword,
    TreeMapKeyword,
    TypeOfKeyword,

    UnionKeyword,
    UnrestrictedViewersKeyword,
    UuidKeyword,

    VerboseKeyword,
    ViewersKeyword,
    ViewKeyword,
    ViewsKeyword,

    WhereKeyword,
    WithItemIndexKeyword,
    WithKeyword,
    WithMatchIdKeyword,
    WithSourceKeyword2,
    WithStepNameKeyword,
    WithSourceKeyword,
    WithNodeIdKeyword,
    WriteAheadLogKeyword,

    // scalar type keyword tokens
    BoolKeyword,
    BooleanKeyword,
    Int8Keyword,
    CharKeyword,
    UInt8Keyword,
    ByteKeyword,
    Int16Keyword,
    UInt16Keyword,
    IntKeyword,
    Int32Keyword,
    UIntKeyword,
    UInt32Keyword,
    LongKeyword,
    Int64Keyword,
    ULongKeyword,
    UInt64Keyword,
    FloatKeyword,
    RealKeyword,
    DecimalKeyword,
    DoubleKeyword,
    StringKeyword,
    TimeKeyword,
    TimespanKeyword,
    DateKeyword,
    DateTimeKeyword,
    GuidKeyword,
    UniqueIdKeyword,
    DynamicKeyword,

    // punctuation tokens
    OpenParenToken,
    CloseParenToken,
    OpenBracketToken,
    CloseBracketToken,
    OpenBraceToken,
    CloseBraceToken,
    BarToken,
    LessThanBarToken,
    PlusToken,
    MinusToken,
    AsteriskToken,
    SlashToken,
    PercentToken,
    DotToken,
    DotDotToken,
    BangToken,
    LessThanToken,
    LessThanOrEqualToken,
    GreaterThanToken,
    GreaterThanOrEqualToken,
    EqualToken,
    EqualEqualToken,
    BangEqualToken,
    LessThanGreaterThanToken,
    ColonToken,
    SemicolonToken,
    CommaToken,
    EqualTildeToken,
    BangTildeToken,
    AtToken,
    QuestionToken,
    FatArrowToken,

    // literal tokens
    BooleanLiteralToken,
    IntLiteralToken,
    LongLiteralToken,
    RealLiteralToken,
    DecimalLiteralToken,
    DateTimeLiteralToken,
    TimespanLiteralToken,
    GuidLiteralToken,
    RawGuidLiteralToken, // this is a raw guid only legal in specific syntax
    StringLiteralToken,

    // identifier
    IdentifierToken,

    // other tokens
    DirectiveToken,
    EndOfTextToken,
    BadToken,
}

#[derive(Debug, Clone, Copy)]
pub struct ParseOptions {
    pub always_produce_end_tokens: bool,
    pub allow_literals_with_line_breaks: bool,
}

impl Default for ParseOptions {
    fn default() -> Self {
        Self {
            always_produce_end_tokens: true,
            allow_literals_with_line_breaks: false,
        }
    }
}

impl ParseOptions {
    pub fn with_always_produce_end_tokens(mut self, value: bool) -> Self {
        self.always_produce_end_tokens = value;
        self
    }

    pub fn with_allow_literals_with_line_breaks(mut self, value: bool) -> Self {
        self.allow_literals_with_line_breaks = value;
        self
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub trivia_span: Range<usize>,
    pub text_span: Range<usize>,
    pub text_content: String,
}

impl Token {
    pub fn new(
        kind: TokenKind,
        trivia_span: Range<usize>,
        text_span: Range<usize>,
        text_content: String,
    ) -> Self {
        Self {
            kind,
            trivia_span,
            text_span,
            text_content,
        }
    }

    /// Returns total length (trivia + text)
    pub fn len(&self) -> usize {
        (self.trivia_span.end - self.trivia_span.start)
            + (self.text_span.end - self.text_span.start)
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
