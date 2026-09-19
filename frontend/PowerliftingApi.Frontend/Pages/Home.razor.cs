using Microsoft.AspNetCore.Components;
using PowerliftingApi.Frontend.Infrastructure.ViewModels;

namespace PowerliftingApi.Frontend.Pages;

public partial class Home
{
    [Inject]
    private HomeViewModel _viewModel { get; set; } = default!;
}
