using Bogus;
using FluentAssertions;
using PowerliftingApi.Frontend.Infrastructure.Adapters;
using PowerliftingApi.Frontend.Infrastructure.ApiContracts;
using PowerliftingApi.Frontend.Infrastructure.Extensions;
using PowerliftingApi.Frontend.Infrastructure.Models;
using PowerliftingApi.Frontend.Infrastructure.Models.Enums;

namespace PowerliftingApi.Frontend.Infrastructure.UnitTests.Adapters;

internal class PowerlifterAdapterTests
{
    private readonly Faker _faker = new();
    private readonly DivisionAdapter _divisionAdapter = new();
    private readonly EquipmentAdapter _equipmentAdapter = new();
    private readonly FederationAdapter _federationAdapter = new();
    private readonly SexAdapter _sexAdapter = new();

    [Test]
    public void Adapt_Valid_ShouldAdapt()
    {
        PowerlifterAdapter adapter = new(_divisionAdapter, _equipmentAdapter, _federationAdapter, _sexAdapter);
        Powerlifter input = new()
        {
            Name = _faker.Name.FullName(),
            Rank = _faker.Random.Long(),
            Division = _faker.PickRandom<DivisionUI>().ToDescription(),
            Equipment = _faker.PickRandom<EquipmentUI>().ToDescription(),
            Federation = _faker.PickRandom<FederationUI>().ToDescription(),
            Sex = _faker.PickRandom<SexUI>().ToDescription(),
            Bodyweight = _faker.Random.Double(-100, 100),
            WeightClass = _faker.Random.Double(-100, 100),
            BestSquat = _faker.Random.Double(Double.MinValue, Double.MaxValue),
            BestBench = _faker.Random.Double(Double.MinValue, Double.MaxValue),
            BestDeadlift = _faker.Random.Double(Double.MinValue, Double.MaxValue),
            Total = _faker.Random.Double(Double.MinValue, Double.MaxValue),
        };
        PowerlifterUI expected = new()
        {
            Name = input.Name,
            Rank = input.Rank,
            Division = _divisionAdapter.Adapt(input.Division),
            Equipment = _equipmentAdapter.Adapt(input.Equipment),
            Federation = _federationAdapter.Adapt(input.Federation),
            Sex = _sexAdapter.Adapt(input.Sex),
            Bodyweight = input.Bodyweight,
            WeightClass = input.WeightClass,
            BestSquat = input.BestSquat,
            BestBench = input.BestBench,
            BestDeadlift = input.BestDeadlift,
            Total = input.Total,
        };

        PowerlifterUI output = adapter.Adapt(input);

        output.Should().BeEquivalentTo(expected);
    }

    [Test]
    public void Adapt_Invalid_ShouldAdapt()
    {
        PowerlifterAdapter adapter = new(new(), new(), new(), new());
        Powerlifter input = new()
        {
            Name = null,
            Rank = _faker.Random.Long(),
            Division = null,
            Equipment = null,
            Federation = null,
            Sex = null,
            Bodyweight = _faker.Random.Double(-100, 100),
            WeightClass = null,
            BestSquat = null,
            BestBench = null,
            BestDeadlift = null,
            Total = null,
        };
        PowerlifterUI expected = new()
        {
            Name = String.Empty,
            Rank = input.Rank,
            Division = DivisionUI.Unknown,
            Equipment = EquipmentUI.Unknown,
            Federation = FederationUI.Unknown,
            Sex = SexUI.Unknown,
            Bodyweight = input.Bodyweight,
            WeightClass = input.WeightClass,
            BestSquat = input.BestSquat,
            BestBench = input.BestBench,
            BestDeadlift = input.BestDeadlift,
            Total = input.Total,
        };

        PowerlifterUI output = adapter.Adapt(input);

        output.Should().BeEquivalentTo(expected);
    }
}
