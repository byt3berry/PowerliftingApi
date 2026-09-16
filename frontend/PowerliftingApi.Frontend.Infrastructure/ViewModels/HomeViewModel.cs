using CommunityToolkit.Mvvm.ComponentModel;
using CommunityToolkit.Mvvm.Input;
using PowerliftingApi.Frontend.Infrastructure.Interfaces;
using PowerliftingApi.Frontend.Infrastructure.Models;
using System.Collections.ObjectModel;

namespace PowerliftingApi.Frontend.Infrastructure.ViewModels;

public partial class HomeViewModel(IBackendService backendService) : ObservableObject
{
    private readonly IBackendService _backendService = backendService;

    [ObservableProperty]
    public partial PowerliftersQueryUI Query { get; set; } = new();

    [ObservableProperty]
    public partial ObservableCollection<PowerlifterUI> Powerlifters { get; set; } = [];

    [RelayCommand]
    private async Task Search(CancellationToken token)
    {
        Powerlifters = [.. await _backendService.GetPowerlifters(Query, token)];
    }
}
