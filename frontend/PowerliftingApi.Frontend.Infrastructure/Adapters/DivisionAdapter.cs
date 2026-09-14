using PowerliftingApi.Frontend.Infrastructure.Interfaces;
using PowerliftingApi.Frontend.Infrastructure.Models.Enums;

namespace PowerliftingApi.Frontend.Infrastructure.Adapters;

internal class DivisionAdapter : IAdapter<string, DivisionUI>
{
    public DivisionUI Adapt(string input)
        => Enum.TryParse(input, true, out DivisionUI division) ? division : DivisionUI.Unknown;
}
