using FluentAssertions;
using PowerliftingApi.Frontend.Infrastructure.Adapters;
using PowerliftingApi.Frontend.Infrastructure.Extensions;
using PowerliftingApi.Frontend.Infrastructure.Models.Enums;

namespace PowerliftingApi.Frontend.Infrastructure.UnitTests.Adapters;

internal class DivisionAdapterTests
{
    private static IEnumerable<TestCaseData<string, DivisionUI>> Adapt_Valid_Data()
    {
        foreach (DivisionUI division in Enum.GetValues<DivisionUI>())
        {
            yield return new(division.ToDescription(), division);
            yield return new(division.ToDescription().ToLower(), division);
            yield return new(division.ToDescription().ToUpper(), division);
        }
    }

    private static IEnumerable<TestCaseData<string, DivisionUI>> Adapt_Invalid_Data()
    {
        yield return new(String.Empty, DivisionUI.Unknown);
        yield return new("invalid", DivisionUI.Unknown);
        yield return new(null!, DivisionUI.Unknown);
    }

    [TestCaseSource(nameof(Adapt_Valid_Data))]
    [TestCaseSource(nameof(Adapt_Invalid_Data))]
    public void Adapt_ShouldAdapt(string input, DivisionUI expected)
    {
        DivisionAdapter adapter = new();

        DivisionUI output = adapter.Adapt(input);

        output.Should().Be(expected);
    }
}
