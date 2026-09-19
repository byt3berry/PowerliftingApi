using CommunityToolkit.Mvvm.ComponentModel;
using PowerliftingApi.Frontend.Infrastructure.Models.Enums.Filters;

namespace PowerliftingApi.Frontend.Infrastructure.Models;

public partial class PowerliftersQueryUI : ObservableObject
{
    [ObservableProperty]
    public partial DivisionFilterUI Division { get; set; } = DivisionFilterUI.Any;

    [ObservableProperty]
    public partial EquipmentFilterUI Equipment { get; set; } = EquipmentFilterUI.Raw;

    [ObservableProperty]
    public partial FederationFilterUI Federation { get; set; } = FederationFilterUI.Any;

    [ObservableProperty]
    public partial SexFilterUI Sex { get; set; } = SexFilterUI.Any;

    [ObservableProperty]
    public partial string Powerlifters { get; set; } = String.Empty;
}
