using PowerliftingApi.Frontend.Infrastructure.ApiContracts;
using PowerliftingApi.Frontend.Infrastructure.Interfaces;
using PowerliftingApi.Frontend.Infrastructure.Models.Enums.Filters;
using System.ComponentModel;

namespace PowerliftingApi.Frontend.Infrastructure.Adapters;

internal class EquipmentFilterAdapter : IAdapter<EquipmentFilterUI, EquipmentFilter>
{
    public EquipmentFilter Adapt(EquipmentFilterUI input) => input switch
    {
        EquipmentFilterUI.Raw => EquipmentFilter.Raw,
        EquipmentFilterUI.Wraps => EquipmentFilter.Wraps,
        EquipmentFilterUI.Single => EquipmentFilter.Single,
        EquipmentFilterUI.Multi => EquipmentFilter.Multi,
        EquipmentFilterUI.Straps => EquipmentFilter.Straps,
        EquipmentFilterUI.Sleeves => EquipmentFilter.Sleeves,
        EquipmentFilterUI.Bare => EquipmentFilter.Bare,
        EquipmentFilterUI.Unlimited => EquipmentFilter.Unlimited,
        _ => throw new InvalidEnumArgumentException(nameof(input), (int)input, typeof(EquipmentFilterUI))
    };
}
