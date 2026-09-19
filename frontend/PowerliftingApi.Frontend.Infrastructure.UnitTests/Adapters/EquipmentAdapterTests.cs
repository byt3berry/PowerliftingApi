using FluentAssertions;
using PowerliftingApi.Frontend.Infrastructure.Adapters;
using PowerliftingApi.Frontend.Infrastructure.Extensions;
using PowerliftingApi.Frontend.Infrastructure.Models.Enums;

namespace PowerliftingApi.Frontend.Infrastructure.UnitTests.Adapters;

internal class EquipmentAdapterTests
{
    private static IEnumerable<TestCaseData<string, EquipmentUI>> Adapt_Valid_Data()
    {
        foreach (EquipmentUI equipment in Enum.GetValues<EquipmentUI>())
        {
            yield return new(equipment.ToDescription(), equipment);
            yield return new(equipment.ToDescription().ToLower(), equipment);
            yield return new(equipment.ToDescription().ToUpper(), equipment);
        }
    }

    private static IEnumerable<TestCaseData<string, EquipmentUI>> Adapt_Invalid_Data()
    {
        yield return new(String.Empty, EquipmentUI.Unknown);
        yield return new("invalid", EquipmentUI.Unknown);
        yield return new(null!, EquipmentUI.Unknown);
    }

    [TestCaseSource(nameof(Adapt_Valid_Data))]
    [TestCaseSource(nameof(Adapt_Invalid_Data))]
    public void Adapt_ShouldAdapt(string input, EquipmentUI expected)
    {
        EquipmentAdapter adapter = new();

        EquipmentUI output = adapter.Adapt(input);

        output.Should().Be(expected);
    }
}
