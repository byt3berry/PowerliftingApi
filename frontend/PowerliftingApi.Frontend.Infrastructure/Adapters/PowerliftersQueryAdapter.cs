using PowerliftingApi.Frontend.Infrastructure.ApiContracts;
using PowerliftingApi.Frontend.Infrastructure.Interfaces;
using PowerliftingApi.Frontend.Infrastructure.Models;

namespace PowerliftingApi.Frontend.Infrastructure.Adapters;

internal class PowerliftersQueryAdapter(DivisionFilterAdapter divisionFilterAdapter,
                                        EquipmentFilterAdapter equipmentFilterAdapter,
                                        FederationFilterAdapter federationFilterAdapter,
                                        SexFilterAdapter sexFilterAdapter)
    : IAdapter<PowerliftersQueryUI, PowerliftersQuery>
{
    public PowerliftersQuery Adapt(PowerliftersQueryUI input) => new()
    {
        DivisionChoice = divisionFilterAdapter.Adapt(input.Division),
        EquipmentChoice = equipmentFilterAdapter.Adapt(input.Equipment),
        FederationChoice = federationFilterAdapter.Adapt(input.Federation),
        SexChoice = sexFilterAdapter.Adapt(input.Sex),
        Powerlifters = input.Powerlifters ?? String.Empty,
    };
}
