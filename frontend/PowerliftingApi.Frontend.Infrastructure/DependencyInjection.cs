using Microsoft.Extensions.Configuration;
using Microsoft.Extensions.DependencyInjection;
using Microsoft.Extensions.DependencyInjection.Extensions;
using PowerliftingApi.Frontend.Infrastructure.Adapters;
using PowerliftingApi.Frontend.Infrastructure.ApiContracts;
using PowerliftingApi.Frontend.Infrastructure.Interfaces;
using PowerliftingApi.Frontend.Infrastructure.Services;
using PowerliftingApi.Frontend.Infrastructure.ViewModels;
using Refit;

namespace PowerliftingApi.Frontend.Infrastructure;

public static class DependencyInjection
{
    public static IServiceCollection AddInfrastructure(this IServiceCollection services)
    {
        services.TryAddSingleton(sp =>
        {
            IConfiguration config = sp.GetRequiredService<IConfiguration>();
            AppSettings configuration = new();
            config.GetSection(nameof(PowerliftingApi)).Bind(configuration);

            return configuration;
        });

        services.TryAddTransient(sp =>
        {
            AppSettings settings = sp.GetRequiredService<AppSettings>();
            return RestService.For<IBackend>(settings.BackendUrl);
        });

        services.TryAddTransient<HomeViewModel>();
        services.TryAddTransient<IBackendService, BackendService>();
        services.TryAddTransient<DivisionAdapter>();
        services.TryAddTransient<DivisionFilterAdapter>();
        services.TryAddTransient<EquipmentAdapter>();
        services.TryAddTransient<EquipmentFilterAdapter>();
        services.TryAddTransient<FederationAdapter>();
        services.TryAddTransient<FederationFilterAdapter>();
        services.TryAddTransient<PowerlifterAdapter>();
        services.TryAddTransient<PowerliftersQueryAdapter>();
        services.TryAddTransient<SexAdapter>();
        services.TryAddTransient<SexFilterAdapter>();

        return services;
    }
}
