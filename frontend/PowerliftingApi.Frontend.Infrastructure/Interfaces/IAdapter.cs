namespace PowerliftingApi.Frontend.Infrastructure.Interfaces;

internal interface IAdapter<TInput, TOutput>
{
    public TOutput Adapt(TInput input);
}
