using Bogus;
using FluentAssertions;
using PowerliftingApi.Frontend.Infrastructure.Adapters;
using PowerliftingApi.Frontend.Infrastructure.ApiContracts;
using PowerliftingApi.Frontend.Infrastructure.Models;
using PowerliftingApi.Frontend.Infrastructure.Models.Enums.Filters;

namespace PowerliftingApi.Frontend.Infrastructure.UnitTests.Adapters;

internal class PowerlifterQueryAdapterTests
{
    private readonly Faker _faker = new();
    private readonly DivisionFilterAdapter _divisionFilterAdapter = new();
    private readonly EquipmentFilterAdapter _equipmentFilterAdapter = new();
    private readonly FederationFilterAdapter _federationFilterAdapter = new();
    private readonly SexFilterAdapter _sexFilterAdapter = new();

    [Test]
    public void Adapt_Valid_ShouldAdapt()
    {
        PowerliftersQueryAdapter adapter = new(_divisionFilterAdapter,
                                               _equipmentFilterAdapter,
                                               _federationFilterAdapter,
                                               _sexFilterAdapter);
        PowerliftersQueryUI input = new()
        {
            Division = _faker.PickRandom<DivisionFilterUI>(),
            Equipment = _faker.PickRandom<EquipmentFilterUI>(),
            Federation = _faker.PickRandom<FederationFilterUI>(),
            Sex = _faker.PickRandom<SexFilterUI>(),
            Powerlifters = String.Join(Environment.NewLine,
                                       _faker.Make(_faker.Random.Int(0, 10), _ => _faker.Name.FullName())),
        };
        PowerliftersQuery expected = new()
        {
            DivisionChoice = _divisionFilterAdapter.Adapt(input.Division),
            EquipmentChoice = _equipmentFilterAdapter.Adapt(input.Equipment),
            FederationChoice = _federationFilterAdapter.Adapt(input.Federation),
            SexChoice = _sexFilterAdapter.Adapt(input.Sex),
            Powerlifters = input.Powerlifters,
            AdditionalProperties = new Dictionary<string, object>(),
        };

        PowerliftersQuery output = adapter.Adapt(input);

        output.Should().BeEquivalentTo(expected);
    }

    [Test]
    public void Adapt_Invalid_ShouldAdapt()
    {
        PowerliftersQueryAdapter adapter = new(_divisionFilterAdapter,
                                               _equipmentFilterAdapter,
                                               _federationFilterAdapter,
                                               _sexFilterAdapter);
        PowerliftersQueryUI input = new()
        {
            Division = _faker.PickRandom<DivisionFilterUI>(),
            Equipment = _faker.PickRandom<EquipmentFilterUI>(),
            Federation = _faker.PickRandom<FederationFilterUI>(),
            Sex = _faker.PickRandom<SexFilterUI>(),
            Powerlifters = null!,
        };
        PowerliftersQuery expected = new()
        {
            DivisionChoice = _divisionFilterAdapter.Adapt(input.Division),
            EquipmentChoice = _equipmentFilterAdapter.Adapt(input.Equipment),
            FederationChoice = _federationFilterAdapter.Adapt(input.Federation),
            SexChoice = _sexFilterAdapter.Adapt(input.Sex),
            Powerlifters = String.Empty,
            AdditionalProperties = new Dictionary<string, object>(),
        };

        PowerliftersQuery output = adapter.Adapt(input);

        output.Should().BeEquivalentTo(expected);
    }
}
