using PowerliftingApi.Frontend.Infrastructure.ApiContracts;
using PowerliftingApi.Frontend.Infrastructure.Interfaces;
using PowerliftingApi.Frontend.Infrastructure.Models.Enums.Filters;
using System.ComponentModel;

namespace PowerliftingApi.Frontend.Infrastructure.Adapters;

internal class FederationFilterAdapter : IAdapter<FederationFilterUI, FederationFilter>
{
    public FederationFilter Adapt(FederationFilterUI input) => input switch
    {
        FederationFilterUI.Any => FederationFilter.Any,
        FederationFilterUI.Ffforce => FederationFilter.Ffforce,
        FederationFilterUI.Epf => FederationFilter.Epf,
        FederationFilterUI.Ipf => FederationFilter.Ipf,
        FederationFilterUI.Ffhmfac => FederationFilter.Ffhmfac,
        _ => throw new InvalidEnumArgumentException(nameof(input), (int)input, typeof(FederationFilterUI))
    };
}
