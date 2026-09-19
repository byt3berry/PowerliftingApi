using FluentAssertions;
using PowerliftingApi.Frontend.Infrastructure.Adapters;
using PowerliftingApi.Frontend.Infrastructure.ApiContracts;
using PowerliftingApi.Frontend.Infrastructure.Models.Enums.Filters;

namespace PowerliftingApi.Frontend.Infrastructure.UnitTests.Adapters;

internal class SexFilterAdapterTests
{
    [TestCase(SexFilterUI.Any, SexFilter.Any)]
    [TestCase(SexFilterUI.F, SexFilter.F)]
    [TestCase(SexFilterUI.M, SexFilter.M)]
    public void Adapt_ShouldAdapt(SexFilterUI input, SexFilter expected)
    {
        SexFilterAdapter adapter = new();

        SexFilter output = adapter.Adapt(input);

        output.Should().Be(expected);
    }
}
