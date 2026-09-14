using System.ComponentModel;

namespace PowerliftingApi.Frontend.Infrastructure.Models.Enums.Filters;

public enum EquipmentFilterUI
{
    [Description("Raw")]
    Raw,

    [Description("Wraps")]
    Wraps,

    [Description("Single")]
    Single,

    [Description("Multi")]
    Multi,

    [Description("Straps")]
    Straps,

    [Description("Sleeves")]
    Sleeves,

    [Description("Bare")]
    Bare,

    [Description("Unlimited")]
    Unlimited,
}
