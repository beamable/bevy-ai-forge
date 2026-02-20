using System.Collections.Generic;
using Beamable.Common.Content;
using Beamable.Common.Inventory;
using Newtonsoft.Json;

[System.Serializable]
public class AiItemContentRef : ItemRef<AiItemContent> { }

[ContentType("AiItemContent")]
[System.Serializable]
public class AiItemContent : ItemContent
{
    public string itemType;
    public string itemTheme;
    public List<DynamicProperty> dynamicProperties;
    public int itemForgePrice = 0;

    public int GetForgePrice() => itemForgePrice > 0 ? itemForgePrice : 50;

    public string StatKey()
    {
        return $"forged-times-{itemType}";
    }

    public string Prompt(long forgedTimes)
    {

        var prompt = @$"I want to create 1 game item.
						    Item should be of type: '{itemType}'.
						    Game theme is: '{itemTheme}'.\n";
        if (forgedTimes < 5)
        {
            prompt += "Item is crafted by someone without any experience with poor value.\n";
        }
        if (forgedTimes > 30)
        {
            prompt += "Item is crafted by someone with vast experience.\n";
        }
        prompt += @$"Here is the list of item properties with descriptions, as JSON: {JsonConvert.SerializeObject(dynamicProperties)}
						    Format the output as JSON object containing only properties.";
        return prompt;
    }
}

[System.Serializable]
public class DynamicProperty
{
    public string propertyName;
    public string propertyDescription;
}
