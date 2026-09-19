using PowerliftingApi.Frontend.Infrastructure.Interfaces;
using PowerliftingApi.Frontend.Infrastructure.Models.Enums;

namespace PowerliftingApi.Frontend.Infrastructure.Adapters;

internal class SexAdapter : IAdapter<string, SexUI>
{
    public SexUI Adapt(string input)
        => Enum.TryParse(input, true, out SexUI sex) ? sex : SexUI.Unknown;
}
