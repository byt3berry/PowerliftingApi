using FluentAssertions;
using PowerliftingApi.Frontend.Infrastructure.Adapters;
using PowerliftingApi.Frontend.Infrastructure.ApiContracts;
using PowerliftingApi.Frontend.Infrastructure.Models.Enums.Filters;

namespace PowerliftingApi.Frontend.Infrastructure.UnitTests.Adapters;

internal class DivisionFilterAdapterTests
{
    [TestCase(DivisionFilterUI.Any, DivisionFilter.Any)]
    [TestCase(DivisionFilterUI.Cadet, DivisionFilter.Cadet)]
    [TestCase(DivisionFilterUI.Elite, DivisionFilter.Elite)]
    [TestCase(DivisionFilterUI.G, DivisionFilter.G)]
    [TestCase(DivisionFilterUI.Juniors, DivisionFilter.Juniors)]
    [TestCase(DivisionFilterUI.Masters, DivisionFilter.Masters)]
    [TestCase(DivisionFilterUI.Masters1, DivisionFilter.Masters1)]
    [TestCase(DivisionFilterUI.Masters2, DivisionFilter.Masters2)]
    [TestCase(DivisionFilterUI.Masters3, DivisionFilter.Masters3)]
    [TestCase(DivisionFilterUI.Masters4, DivisionFilter.Masters4)]
    [TestCase(DivisionFilterUI.Open, DivisionFilter.Open)]
    [TestCase(DivisionFilterUI.Seniors, DivisionFilter.Seniors)]
    public void Adapt_ShouldAdapt(DivisionFilterUI input, DivisionFilter expected)
    {
        DivisionFilterAdapter adapter = new();

        DivisionFilter output = adapter.Adapt(input);

        output.Should().Be(expected);
    }
}
