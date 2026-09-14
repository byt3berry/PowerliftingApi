using PowerliftingApi.Frontend.Infrastructure.ApiContracts;
using PowerliftingApi.Frontend.Infrastructure.Interfaces;
using PowerliftingApi.Frontend.Infrastructure.Models.Enums.Filters;
using System.ComponentModel;

namespace PowerliftingApi.Frontend.Infrastructure.Adapters;

internal class DivisionFilterAdapter : IAdapter<DivisionFilterUI, DivisionFilter>
{
    public DivisionFilter Adapt(DivisionFilterUI input) => input switch
    {
        DivisionFilterUI.Any => DivisionFilter.Any,
        DivisionFilterUI.Open => DivisionFilter.Open,
        DivisionFilterUI.G => DivisionFilter.G,
        DivisionFilterUI.Cadet => DivisionFilter.Cadet,
        DivisionFilterUI.Elite => DivisionFilter.Elite,
        DivisionFilterUI.SubJuniors => DivisionFilter.SubJuniors,
        DivisionFilterUI.Juniors => DivisionFilter.Juniors,
        DivisionFilterUI.Masters => DivisionFilter.Masters,
        DivisionFilterUI.Seniors => DivisionFilter.Seniors,
        DivisionFilterUI.Masters1 => DivisionFilter.Masters1,
        DivisionFilterUI.Masters2 => DivisionFilter.Masters2,
        DivisionFilterUI.Masters3 => DivisionFilter.Masters3,
        DivisionFilterUI.Masters4 => DivisionFilter.Masters3,
        _ => throw new InvalidEnumArgumentException(nameof(input), (int)input, typeof(DivisionFilterUI))
    };
}
