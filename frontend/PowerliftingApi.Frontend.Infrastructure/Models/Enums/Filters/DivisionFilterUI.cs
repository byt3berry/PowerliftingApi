using System.ComponentModel;

namespace PowerliftingApi.Frontend.Infrastructure.Models.Enums.Filters;

public enum DivisionFilterUI
{
    [Description("Tout")]
    Any,

    [Description("Open")]
    Open,

    [Description("G")]
    G,

    [Description("Cadet")]
    Cadet,

    [Description("Elite")]
    Elite,

    [Description("SubJuniors")]
    SubJuniors,

    [Description("Juniors")]
    Juniors,

    [Description("Seniors")]
    Seniors,

    [Description("Masters")]
    Masters,

    [Description("Masters 1")]
    Masters1,

    [Description("Masters 2")]
    Masters2,

    [Description("Masters 3")]
    Masters3,

    [Description("Masters 4")]
    Masters4,
}
