using PowerliftingApi.Frontend.Infrastructure.ApiContracts;
using PowerliftingApi.Frontend.Infrastructure.Interfaces;
using PowerliftingApi.Frontend.Infrastructure.Models;

namespace PowerliftingApi.Frontend.Infrastructure.Adapters;

internal class PowerlifterAdapter(DivisionAdapter divisionAdapter,
                                  EquipmentAdapter equipmentAdapter,
                                  FederationAdapter federationAdapter,
                                  SexAdapter sexAdapter) : IAdapter<Powerlifter, PowerlifterUI>
{
    public PowerlifterUI Adapt(Powerlifter input) => new()
    {
        Name = input.Name,
        Rank = input.Rank,
        Division = divisionAdapter.Adapt(input.Division),
        Equipment = equipmentAdapter.Adapt(input.Equipment),
        Federation = federationAdapter.Adapt(input.Federation),
        Sex = sexAdapter.Adapt(input.Sex),
        Bodyweight = input.Bodyweight,
        WeightClass = input.WeightClass,
        BestSquat = input.BestSquat,
        BestBench = input.BestBench,
        BestDeadlift = input.BestDeadlift,
        Total = input.Total,
    };
}
