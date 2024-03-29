﻿using System.Collections.Generic;
using Beamable;
using Beamable.Api.Autogenerated.Models;
using Beamable.Common.Content;
using Beamable.Common.Inventory;

[System.Serializable]
public class AiItemContentRef : ItemRef<AiItemContent> { }

[ContentType("AiItemContent")]
[System.Serializable]
public class AiItemContent : ItemContent
{
    public string itemType;
    public string itemTheme;
    public List<DynamicProperty> dynamicProperties;
}

[System.Serializable]
public class DynamicProperty
{
    public string propertyName;
    public string propertyDescription;
}


