using CommunityToolkit.Mvvm.ComponentModel;
using PowerliftingApi.Frontend.Infrastructure.Models.Enums;

namespace PowerliftingApi.Frontend.Infrastructure.Models;

public partial class PowerlifterUI : ObservableObject
{
    [ObservableProperty]
    public partial string Name { get; set; }

    [ObservableProperty]
    public partial long Rank { get; set; }

    [ObservableProperty]
    public partial DivisionUI Division { get; set; }

    [ObservableProperty]
    public partial EquipmentUI Equipment { get; set; }

    [ObservableProperty]
    public partial FederationUI Federation { get; set; }

    [ObservableProperty]
    public partial SexUI Sex { get; set; }

    [ObservableProperty]
    public partial double Bodyweight { get; set; }

    [ObservableProperty]
    public partial double? WeightClass { get; set; }

    [ObservableProperty]
    public partial double? BestSquat { get; set; }

    [ObservableProperty]
    public partial double? BestBench { get; set; }

    [ObservableProperty]
    public partial double? BestDeadlift { get; set; }

    [ObservableProperty]
    public partial double? Total { get; set; }
}
