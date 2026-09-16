using FluentAssertions;
using PowerliftingApi.Frontend.Infrastructure.Adapters;
using PowerliftingApi.Frontend.Infrastructure.Extensions;
using PowerliftingApi.Frontend.Infrastructure.Models.Enums;

namespace PowerliftingApi.Frontend.Infrastructure.UnitTests.Adapters;

internal class FederationAdapterTests
{
    private static IEnumerable<TestCaseData<string, FederationUI>> Adapt_Valid_Data()
    {
        foreach (FederationUI federation in Enum.GetValues<FederationUI>())
        {
            yield return new(federation.ToDescription(), federation);
            yield return new(federation.ToDescription().ToLower(), federation);
            yield return new(federation.ToDescription().ToUpper(), federation);
        }
    }

    private static IEnumerable<TestCaseData<string, FederationUI>> Adapt_Invalid_Data()
    {
        yield return new(String.Empty, FederationUI.Unknown);
        yield return new("invalid", FederationUI.Unknown);
        yield return new(null!, FederationUI.Unknown);
    }

    [TestCaseSource(nameof(Adapt_Valid_Data))]
    [TestCaseSource(nameof(Adapt_Invalid_Data))]
    public void Adapt_ShouldAdapt(string input, FederationUI expected)
    {
        FederationAdapter adapter = new();

        FederationUI output = adapter.Adapt(input);

        output.Should().Be(expected);
    }
}
