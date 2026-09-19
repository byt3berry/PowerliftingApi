using Microsoft.AspNetCore.Components.Web;
using Microsoft.AspNetCore.Components.WebAssembly.Hosting;
using MudBlazor.Services;
using PowerliftingApi.Frontend;
using PowerliftingApi.Frontend.Infrastructure;

WebAssemblyHostBuilder builder = WebAssemblyHostBuilder.CreateDefault(args);
builder.RootComponents.Add<App>("#app");
builder.RootComponents.Add<HeadOutlet>("head::after");

builder.Services
       .AddMudServices()
       .AddInfrastructure()
       .AddScoped<HttpClient>(_ => new()
       {
           BaseAddress = new(builder.HostEnvironment.BaseAddress)
       });

await builder.Build().RunAsync();
