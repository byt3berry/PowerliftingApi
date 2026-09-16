using FluentAssertions;
using PowerliftingApi.Frontend.Infrastructure.Adapters;
using PowerliftingApi.Frontend.Infrastructure.Extensions;
using PowerliftingApi.Frontend.Infrastructure.Models.Enums;

namespace PowerliftingApi.Frontend.Infrastructure.UnitTests.Adapters;

internal class SexAdapterTests
{
    private static IEnumerable<TestCaseData<string, SexUI>> Adapt_Valid_Data()
    {
        foreach (SexUI sex in Enum.GetValues<SexUI>())
        {
            yield return new(sex.ToDescription(), sex);
            yield return new(sex.ToDescription().ToLower(), sex);
            yield return new(sex.ToDescription().ToUpper(), sex);
        }
    }

    private static IEnumerable<TestCaseData<string, SexUI>> Adapt_Invalid_Data()
    {
        yield return new(String.Empty, SexUI.Unknown);
        yield return new("invalid", SexUI.Unknown);
        yield return new(null!, SexUI.Unknown);
    }

    [TestCaseSource(nameof(Adapt_Valid_Data))]
    [TestCaseSource(nameof(Adapt_Invalid_Data))]
    public void Adapt_ShouldAdapt(string input, SexUI expected)
    {
        SexAdapter adapter = new();

        SexUI output = adapter.Adapt(input);

        output.Should().Be(expected);
    }
}
