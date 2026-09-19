using FluentAssertions;
using PowerliftingApi.Frontend.Infrastructure.Adapters;
using PowerliftingApi.Frontend.Infrastructure.ApiContracts;
using PowerliftingApi.Frontend.Infrastructure.Models.Enums.Filters;

namespace PowerliftingApi.Frontend.Infrastructure.UnitTests.Adapters;

internal class FederationFilterAdapterTests
{
    [TestCase(FederationFilterUI.Any, FederationFilter.Any)]
    [TestCase(FederationFilterUI.Epf, FederationFilter.Epf)]
    [TestCase(FederationFilterUI.Ffforce, FederationFilter.Ffforce)]
    [TestCase(FederationFilterUI.Ffhmfac, FederationFilter.Ffhmfac)]
    [TestCase(FederationFilterUI.Ipf, FederationFilter.Ipf)]
    public void Adapt_ShouldAdapt(FederationFilterUI input, FederationFilter expected)
    {
        FederationFilterAdapter adapter = new();

        FederationFilter output = adapter.Adapt(input);

        output.Should().Be(expected);
    }
}
