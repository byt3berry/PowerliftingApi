using Bogus;
using FluentAssertions;
using Moq;
using PowerliftingApi.Frontend.Infrastructure.Interfaces;
using PowerliftingApi.Frontend.Infrastructure.Models;
using PowerliftingApi.Frontend.Infrastructure.Models.Enums;
using PowerliftingApi.Frontend.Infrastructure.ViewModels;

namespace PowerliftingApi.Frontend.Infrastructure.UnitTests.ViewModels;

internal class HomeViewModelTests
{
    private readonly Faker _faker = new();

    private IEnumerable<PowerlifterUI> GeneratePowerlifters()
    {
        while (true)
        {
            yield return new()
            {
                Name = _faker.Name.FullName(),
                Rank = _faker.Random.Long(),
                Division = _faker.PickRandom<DivisionUI>(),
                Equipment = _faker.PickRandom<EquipmentUI>(),
                Federation = _faker.PickRandom<FederationUI>(),
                Sex = _faker.PickRandom<SexUI>(),
                Bodyweight = _faker.Random.Double(-100, 100),
                WeightClass = _faker.Random.Double(-100, 100),
                BestSquat = _faker.Random.Double(Double.MinValue, Double.MaxValue),
                BestBench = _faker.Random.Double(Double.MinValue, Double.MaxValue),
                BestDeadlift = _faker.Random.Double(Double.MinValue, Double.MaxValue),
                Total = _faker.Random.Double(Double.MinValue, Double.MaxValue),
            };
        }
    }

    [Test]
    public async Task SearchCommand_ShouldStorePowerlifters()
    {
        ICollection<PowerlifterUI> powerlifters = GeneratePowerlifters().Take(1).ToList();
        Mock<IBackendService> backendServiceMock = new(MockBehavior.Strict);
        backendServiceMock.Setup(s => s.GetPowerlifters(It.IsAny<PowerliftersQueryUI>(), It.IsAny<CancellationToken>()))
                          .ReturnsAsync(powerlifters)
                          .Verifiable(Times.Once());
        HomeViewModel viewModel = new(backendServiceMock.Object);

        Func<Task> action = () => viewModel.SearchCommand.ExecuteAsync(null);

        await action.Should().NotThrowAsync();
    }
}
