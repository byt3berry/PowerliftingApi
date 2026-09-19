using System.ComponentModel;

namespace PowerliftingApi.Frontend.Infrastructure.Models.Enums.Filters;

public enum FederationFilterUI
{

    [Description("Tout")]
    Any,

    [Description("FFForce")]
    Ffforce,

    [Description("EPF")]
    Epf,

    [Description("IPF")]
    Ipf,

    [Description("FFHMFAC")]
    Ffhmfac,
}
