using System;
using Beamable.Common;
using Beamable.Server.Api.RealmConfig;
using OpenAI.Chat;

namespace Beamable.ForgeService;

public class ChatAiService
{
    private readonly IMicroserviceRealmConfigService _realmConfigService;
    private Promise<ChatClient> _initPromise;
    private ChatClient _chat = null;

    public ChatAiService(IMicroserviceRealmConfigService realmConfigService)
    {
        _realmConfigService = realmConfigService;
    }

    public void Init()
    {
        _ = GetChat();
    }

    public Promise<ChatClient> GetChat()
    {
        if (_initPromise != null && (!_initPromise.IsCompleted || !_initPromise.IsFailed))
        {
            return _initPromise;
        }
        _initPromise = new Promise<ChatClient>();
        _realmConfigService.GetRealmConfigSettings().Then(config =>
        {

            var apiKey = config.GetSetting("ForgeService", "openApiKey", string.Empty);
        
            var apiModel = config.GetSetting("ForgeService", "openApiModel", string.Empty);
            if (string.IsNullOrEmpty(apiKey) || string.IsNullOrEmpty(apiModel))
            {
                BeamableLogger.LogError("Failed to get open api key and openApiModel from config");
                _initPromise.CompleteError(new Exception("Failed to get open api key and openApiModel from config"));
            }
            _chat = new ChatClient(model: apiModel, apiKey: apiKey);
            _initPromise.CompleteSuccess(_chat);
            
        });
        
        return _initPromise;
    }
}