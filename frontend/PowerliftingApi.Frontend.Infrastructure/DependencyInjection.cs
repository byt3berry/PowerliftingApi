using Microsoft.Extensions.Configuration;
using Microsoft.Extensions.DependencyInjection;
using Microsoft.Extensions.DependencyInjection.Extensions;
using PowerliftingApi.Frontend.Infrastructure.Interfaces;
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
        }
        );

        return services;
    }
}
