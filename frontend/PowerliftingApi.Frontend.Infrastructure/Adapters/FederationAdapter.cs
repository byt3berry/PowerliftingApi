using PowerliftingApi.Frontend.Infrastructure.Interfaces;
using PowerliftingApi.Frontend.Infrastructure.Models.Enums;

namespace PowerliftingApi.Frontend.Infrastructure.Adapters;

internal class FederationAdapter : IAdapter<string, FederationUI>
{
    public FederationUI Adapt(string input)
        => Enum.TryParse(input, true, out FederationUI federation) ? federation : FederationUI.Unknown;
}
