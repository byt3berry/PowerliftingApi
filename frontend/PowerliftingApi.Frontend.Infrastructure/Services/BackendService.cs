using PowerliftingApi.Frontend.Infrastructure.Adapters;
using PowerliftingApi.Frontend.Infrastructure.ApiContracts;
using PowerliftingApi.Frontend.Infrastructure.Interfaces;
using PowerliftingApi.Frontend.Infrastructure.Models;

namespace PowerliftingApi.Frontend.Infrastructure.Services;

internal class BackendService(IBackend backend,
                              PowerliftersQueryAdapter queryAdapter,
                              PowerlifterAdapter powerlifterAdapter) : IBackendService
{
    public async Task<ICollection<PowerlifterUI>> GetPowerlifters(PowerliftersQueryUI query, CancellationToken token)
    {
        ICollection<Powerlifter> powerlifters = await backend.Powerlifters(queryAdapter.Adapt(query), token);

        return powerlifters.Select(powerlifterAdapter.Adapt).ToList();
    }
}
