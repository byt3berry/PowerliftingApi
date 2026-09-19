using Bogus;
using FluentAssertions;
using Moq;
using PowerliftingApi.Frontend.Infrastructure.Adapters;
using PowerliftingApi.Frontend.Infrastructure.ApiContracts;
using PowerliftingApi.Frontend.Infrastructure.Extensions;
using PowerliftingApi.Frontend.Infrastructure.Models;
using PowerliftingApi.Frontend.Infrastructure.Models.Enums;
using PowerliftingApi.Frontend.Infrastructure.Models.Enums.Filters;
using PowerliftingApi.Frontend.Infrastructure.Services;

namespace PowerliftingApi.Frontend.Infrastructure.UnitTests.Services;

internal class BackendServiceTests
{
    private readonly Faker _faker = new();
    private readonly DivisionFilterAdapter _divisionFilterAdapter = new();
    private readonly EquipmentFilterAdapter _equipmentFilterAdapter = new();
    private readonly FederationFilterAdapter _federationFilterAdapter = new();
    private readonly SexFilterAdapter _sexFilterAdapter = new();
    private readonly DivisionAdapter _divisionAdapter = new();
    private readonly EquipmentAdapter _equipmentAdapter = new();
    private readonly FederationAdapter _federationAdapter = new();
    private readonly SexAdapter _sexAdapter = new();

    private PowerliftersQueryAdapter _powerliftersQueryAdapter;
    private PowerlifterAdapter _powerlifterAdapter;

    private PowerliftersQueryUI GeneratePowerliftersQueryUI() => new()
    {
        Division = _faker.PickRandom<DivisionFilterUI>(),
        Equipment = _faker.PickRandom<EquipmentFilterUI>(),
        Federation = _faker.PickRandom<FederationFilterUI>(),
        Sex = _faker.PickRandom<SexFilterUI>(),
        Powerlifters = _faker.Name.FullName(),
    };

    private IEnumerable<Powerlifter> GeneratePowerlifters()
    {
        while (true)
        {
            yield return new()
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
        }
    }

    [SetUp]
    public void Setup()
    {
        _powerliftersQueryAdapter = new(_divisionFilterAdapter,
                                        _equipmentFilterAdapter,
                                        _federationFilterAdapter,
                                        _sexFilterAdapter);
        _powerlifterAdapter = new(_divisionAdapter,
                                  _equipmentAdapter,
                                  _federationAdapter,
                                  _sexAdapter);
    }

    [Test]
    public async Task GetPowerlifters_ShouldCallApi()
    {
        PowerliftersQueryUI queryUI = GeneratePowerliftersQueryUI();
        PowerliftersQuery query = _powerliftersQueryAdapter.Adapt(queryUI);
        ICollection<Powerlifter> expected = GeneratePowerlifters().Take(1).ToList();
        ICollection<PowerlifterUI> expectedUI = expected.Select(_powerlifterAdapter.Adapt).ToList();
        Mock<IBackend> backendMock = new(MockBehavior.Strict);
        backendMock.Setup(s => s.Powerlifters(It.Is<PowerliftersQuery>(x => x.DivisionChoice == query.DivisionChoice
                                                                            && x.EquipmentChoice == query.EquipmentChoice
                                                                            && x.FederationChoice == query.FederationChoice
                                                                            && x.SexChoice == query.SexChoice
                                                                            && x.Powerlifters == query.Powerlifters
                                                                      ),
                                              It.IsAny<CancellationToken>()))
                   .ReturnsAsync(expected)
                   .Verifiable(Times.Once());
        BackendService service = new(backendMock.Object, _powerliftersQueryAdapter, _powerlifterAdapter);

        ICollection<PowerlifterUI> output = await service.GetPowerlifters(queryUI, CancellationToken.None);

        output.Should().BeEquivalentTo(expectedUI);
    }
}
