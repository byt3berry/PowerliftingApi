using FluentAssertions;
using PowerliftingApi.Frontend.Infrastructure.Adapters;
using PowerliftingApi.Frontend.Infrastructure.ApiContracts;
using PowerliftingApi.Frontend.Infrastructure.Models.Enums.Filters;

namespace PowerliftingApi.Frontend.Infrastructure.UnitTests.Adapters;

internal class EquipmentFilterAdapterTests
{
    [TestCase(EquipmentFilterUI.Bare, EquipmentFilter.Bare)]
    [TestCase(EquipmentFilterUI.Multi, EquipmentFilter.Multi)]
    [TestCase(EquipmentFilterUI.Raw, EquipmentFilter.Raw)]
    [TestCase(EquipmentFilterUI.Single, EquipmentFilter.Single)]
    [TestCase(EquipmentFilterUI.Sleeves, EquipmentFilter.Sleeves)]
    [TestCase(EquipmentFilterUI.Straps, EquipmentFilter.Straps)]
    [TestCase(EquipmentFilterUI.Unlimited, EquipmentFilter.Unlimited)]
    [TestCase(EquipmentFilterUI.Wraps, EquipmentFilter.Wraps)]
    public void Adapt_ShouldAdapt(EquipmentFilterUI input, EquipmentFilter expected)
    {
        EquipmentFilterAdapter adapter = new();

        EquipmentFilter output = adapter.Adapt(input);

        output.Should().Be(expected);
    }
}
