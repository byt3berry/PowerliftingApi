using System.ComponentModel;
using System.Reflection;

namespace PowerliftingApi.Frontend.Infrastructure.Extensions;

public static class EnumExtension
{
    public static string ToDescription<TEnum>(this TEnum value) where TEnum : struct, Enum
    {
        FieldInfo? info = value.GetType().GetField(value.ToString());

        return info?.GetCustomAttributes(typeof(DescriptionAttribute), false) is DescriptionAttribute[] attributes && attributes.Length != 0
            ? attributes.First().Description
            : value.ToString();
    }
}
