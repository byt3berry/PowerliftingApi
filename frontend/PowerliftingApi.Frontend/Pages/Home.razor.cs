using Microsoft.AspNetCore.Components;
using PowerliftingApi.Frontend.Infrastructure.Interfaces;
using PowerliftingApi.Frontend.Infrastructure.Models;

namespace PowerliftingApi.Frontend.Pages;

public partial class Home
{
    [Inject]
    private IBackend _backend { get; set; } = default!;

    protected override async Task OnInitializedAsync()
    {
        try
        {

            ICollection<Powerlifter> x = await _backend.Powerlifters(new()
            {
                DivisionChoice = DivisionFilter.Any,
                EquipmentChoice = EquipmentFilter.Raw,
                SexChoice = SexFilter.Any,
                Powerlifters = "mathis cocagne",
            });
        }
        catch (Exception e)
        {
        }

        await base.OnInitializedAsync();
    }
}
