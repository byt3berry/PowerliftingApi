using PowerliftingApi.Frontend.Infrastructure.Interfaces;
using PowerliftingApi.Frontend.Infrastructure.Models.Enums;

namespace PowerliftingApi.Frontend.Infrastructure.Adapters;

internal class EquipmentAdapter : IAdapter<string, EquipmentUI>
{
    public EquipmentUI Adapt(string input)
        => Enum.TryParse(input, true, out EquipmentUI equipment) ? equipment : EquipmentUI.Unknown;
}
