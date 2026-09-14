using PowerliftingApi.Frontend.Infrastructure.Models;

namespace PowerliftingApi.Frontend.Infrastructure.Interfaces;

public interface IBackendService
{
    public Task<ICollection<PowerlifterUI>> GetPowerlifters(PowerliftersQueryUI query, CancellationToken token);
}
