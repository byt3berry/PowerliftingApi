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

    [Description("Masters")]
    Masters,

    [Description("Seniors")]
    Seniors,

    [Description("Masters1")]
    Masters1,

    [Description("Masters2")]
    Masters2,

    [Description("Masters3")]
    Masters3,

    [Description("Masters4")]
    Masters4,
}
