using System;
using System.Collections.Generic;
using System.Linq;
using Beamable.Common;
using Beamable.Common.Content;
using Beamable.Server.Api.Content;

namespace Beamable.ForgeService;

public class AiContentService
{
    private IMicroserviceContentApi _contentApi;
    private Dictionary<string, AiItemContent> _content;

    public bool TryGetContent(string contentId, out AiItemContent content)
    {
        return _content.TryGetValue(contentId, out content);
    }
    public AiContentService(IMicroserviceContentApi contentApi)
    {
        _contentApi = contentApi;
    }

    public async Promise Init()
    {
        BeamableLogger.Log("Initializing content");
        var manifest = await _contentApi.GetManifest();
        var content = await manifest.ResolveAll();
        _content = content.Select(c =>
        {
            if (c is AiItemContent ccc)
            {
                return ccc;
            }

            return null;
        }).Where(c => c != null).ToDictionary(c => c.Id);
        BeamableLogger.Log("Initializing content completed with {itemsCount} items", _content.Count);
    }
}