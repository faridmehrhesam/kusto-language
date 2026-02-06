pub struct TokenStream<'a> {
    pub tokens: Vec<TokenKind>,
    pub source: &'a str,
}

impl<'a> TokenStream<'a> {
    pub fn slice(&self, span: &Span) -> &'a str {
        &self.source[span.start..span.end]
    }
}

#[derive(Debug, PartialEq, Clone, Copy, Eq)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

impl Span {
    pub fn len(&self) -> usize {
        self.end - self.start
    }
}

#[derive(Debug, PartialEq, Clone, Eq)]
pub enum TokenKind {
    Keyword(KeywordKind),
    Punctuation(PunctuationKind),
    Literal(LiteralKind),
    Identifier(Span),
    Directive(Span),
    Bad(Span),
    EndOfFile,
}

#[derive(Debug, PartialEq, Clone, Copy, Eq)]
pub enum KeywordKind {
    __CrossCluster,
    __CrossDB,
    __Id,
    __IsFuzzy,
    __NoWithSource,
    __PackedColumn,
    __SourceColumnIndex,
    _3DChart,

    Access,
    Accumulate,
    Alias,
    And,
    AnomalyChart,
    AreaChart,
    As,
    Asc,
    AssertSchema,

    BagExpansion,
    BarChart,
    Between,
    Bin,
    BinLegacy,
    By,

    CachingPolicy,
    Callout,
    Cancel,
    Card,
    ColumnChart,
    CommandsAndQueries,
    Consume,
    Contains,
    ContainsCs,
    ContainsCs2,
    ContextualDataTable,
    Count,
    Cycles,

    Database,
    DataExport,
    DataScope,
    DataTable,
    Declare,
    DecodeBlocks,
    Default,
    Desc,
    Distinct,

    Earliest,
    EncodingPolicy,
    EndsWith,
    EndsWithCs,
    EntityGroup,
    Evaluate,
    ExecuteAndCache,
    ExpandOutput,
    Extend,
    ExtentTagsRetention,
    ExternalData,
    ExternalData2,

    DataFormat,
    InlineExternalTable,
    DateTimePattern,

    Facet,
    Filter,
    Find,
    First,
    Flags,
    Fork,
    From,

    GetSchema,
    GrannyAsc,
    GrannyDesc,
    GraphMatch,
    GraphShortestPaths,
    GraphToTable,
    GraphEdges,

    HardDelete,
    HardRetention,
    Has,
    HasAny,
    HasAll,
    HasCs,
    HasPrefix,
    HasPrefixCs,
    HasSuffix,
    HasSuffixCs,
    HintDotConcurrency,
    HintDotDistribution,
    HintDotMaterialized,
    HintDotNumPartitions,
    HintDotShuffleKey,
    HintDotSpread,
    HintDotRemote,
    HintDotStrategy,
    HintDotProgressiveTop,
    HotCache,

    Id,
    In,
    InCs,
    Invoke,
    IsFuzzy,
    BestEffort,
    ForceRemote,

    Join,
    Journal,

    Kind,

    LadderChart,
    Last,
    Latest,
    Let,
    Like,
    LikeCs,
    Limit,
    LineChart,
    Lookup,

    MacroExpand,
    MakeGraph,
    GraphMarkComponents,
    GraphWhereNodes,
    GraphWhereEdges,
    MakeSeries,
    MatchesRegex,
    Materialize,
    MaterializedViewCombine,
    MaterializedViews,
    Mdm,
    Missing,
    MvApply,
    MvDashApply,
    MvDashExpand,
    MvExpand,

    Nodes,
    NoOptimization,
    NotBetween,
    NotContains,
    NotContainsCs,
    NotBangContains,
    NotBangContainsCs,
    NotEndsWith,
    NotEndsWithCs,
    NotHas,
    NotHasCs,
    NotHasPrefix,
    NotHasPrefixCs,
    NotHasSuffix,
    NotHasSuffixCs,
    NotIn,
    NotInCs,
    NotLike,
    NotLikeCs,
    NotStartsWith,
    NotStartsWithCs,
    Null,
    Nulls,

    Of,
    On,
    Optional,
    Or,
    Order,
    Others,
    Output,

    Pack,
    Parse,
    ParseWhere,
    ParseKv,
    PartitionedBy,
    PartitionBy,
    Partition,
    PathFormat,
    Pattern,
    PieChart,
    PivotChart,
    Plotly,
    Graph,
    Print,
    Project,
    ProjectAway,
    ProjectByNames,
    _ProjectAway,
    ProjectKeep,
    ProjectRename,
    ProjectReorder,
    ProjectSmart,

    Queries,
    QueryParameters,
    QueryResults,

    Range,
    Reduce,
    Regex,
    Relaxed,
    Render,
    RestrictedViewAccess,
    Restrict,
    RowLevelSecurity,
    Rowstore,
    RowstoreReferences,
    RowstoreSealInfo,
    RowstorePolicy,
    Rowstores,

    Sample,
    SampleDistinct,
    Scan,
    ScatterChart,
    Seal,
    Seals,
    Search,
    Serialize,
    Set,
    SetOrAppend,
    SetOrReplace,
    Shards,
    Simple,
    SoftDelete,
    SoftRetention,
    Sort,
    Sql,
    StackedAreaChart,
    StartsWith,
    StartsWithCs,
    Statistics,
    Step,
    StoredQueryResultContainers,
    Summarize,

    TablePurge,
    Take,
    TimeChart,
    Timeline,
    TimePivot,
    Title,
    To,
    Top,
    TopHitters,
    TopNested,
    ToScalar,
    ToTable,
    TreeMap,
    TypeOf,

    Union,
    UnrestrictedViewers,
    Uuid,

    Verbose,
    Viewers,
    View,
    Views,

    Where,
    WithItemIndex,
    With,
    WithMatchId,
    WithSource2,
    WithStepName,
    WithSource,
    WithNodeId,
    WriteAheadLog,

    // scalar type keywords
    Bool,
    Boolean,
    Int8,
    Char,
    UInt8,
    Byte,
    Int16,
    UInt16,
    Int,
    Int32,
    UInt,
    UInt32,
    Long,
    Int64,
    ULong,
    UInt64,
    Float,
    Real,
    Decimal,
    Double,
    String,
    Time,
    Timespan,
    Date,
    DateTime,
    Guid,
    UniqueId,
    Dynamic,
}

#[derive(Debug, PartialEq, Clone, Copy, Eq)]
pub enum PunctuationKind {
    OpenParen,
    CloseParen,
    OpenBracket,
    CloseBracket,
    OpenBrace,
    CloseBrace,
    Bar,
    LessThanBar,
    Plus,
    Minus,
    Asterisk,
    Slash,
    Percent,
    Dot,
    DotDot,
    Bang,
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
    Equal,
    EqualEqual,
    BangEqual,
    LessThanGreaterThan,
    Colon,
    Semicolon,
    Comma,
    EqualTilde,
    BangTilde,
    At,
    Question,
    FatArrow,
}

#[derive(Debug, PartialEq, Clone, Eq)]
pub enum LiteralKind {
    Boolean(Span),
    DateTime(Span),
    Decimal(Span),
    Guid(Span),
    Int(Span),
    Long(Span),
    RawGuid(Span),
    Real(Span),
    Timespan(Span),
    String(Span),
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
