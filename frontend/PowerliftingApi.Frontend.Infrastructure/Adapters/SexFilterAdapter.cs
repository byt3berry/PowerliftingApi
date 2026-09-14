using PowerliftingApi.Frontend.Infrastructure.ApiContracts;
using PowerliftingApi.Frontend.Infrastructure.Interfaces;
using PowerliftingApi.Frontend.Infrastructure.Models.Enums.Filters;
using System.ComponentModel;

namespace PowerliftingApi.Frontend.Infrastructure.Adapters;

internal class SexFilterAdapter : IAdapter<SexFilterUI, SexFilter>
{
    public SexFilter Adapt(SexFilterUI input) => input switch
    {
        SexFilterUI.Any => SexFilter.Any,
        SexFilterUI.M => SexFilter.M,
        SexFilterUI.F => SexFilter.F,
        _ => throw new InvalidEnumArgumentException(nameof(input), (int)input, typeof(SexFilterUI))
    };
}
