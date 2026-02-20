using System.Collections.Generic;
using System.Threading.Tasks;
using Beamable.Common;
using MongoDB.Driver;

namespace ForgeService.Storage
{
    internal static class AiInventoryItemCollection
    {
        private static IMongoCollection<AiInventoryItem> _collection;

        public static async ValueTask<IMongoCollection<AiInventoryItem>> Get(IMongoDatabase db)
        {
            if (_collection is null)
            {
                _collection = db.GetCollection<AiInventoryItem>("inventory");
                await _collection.Indexes.CreateOneAsync(
                    new CreateIndexModel<AiInventoryItem>(
                        Builders<AiInventoryItem>.IndexKeys
                            .Ascending(x => x.GamerTag)
                            .Ascending(x => x.ContentId)
                            .Ascending(x => x.ItemId),
                        new CreateIndexOptions { Unique = true }
                    )
                );
            }

            return _collection;
        }

        public static async Task<List<AiInventoryItem>> GetAll(IMongoDatabase db, string gamerTag)
        {
            var collection = await Get(db);
            var mints = await collection
                .Find(x => x.GamerTag == gamerTag)
                .ToListAsync();
            return mints;
        }

        public static async Task<List<AiInventoryItem>> GetItem(IMongoDatabase db, string itemId)
        {
            var collection = await Get(db);
            var mints = await collection
                .Find(x => x.ItemId == itemId)
                .ToListAsync();
            return mints;
        }

        public static async Task Save(IMongoDatabase db, AiInventoryItem item)
        {
            var collection = await Get(db);
            await collection.InsertOneAsync(item);
        }

        public static async Task Delete(IMongoDatabase db, AiInventoryItem item)
        {
            var collection = await Get(db);
            await collection.DeleteOneAsync(x => x.ItemId == item.ItemId);
        }
        public static async Task<bool> DeleteByKey(IMongoDatabase db, string itemKey)
        {
            var collection = await Get(db);
            var deleteResult = await collection.DeleteOneAsync(x => x.ItemId == itemKey);
            BeamableLogger.Log("Deletion result: {deletedAmount}", deleteResult.IsAcknowledged ? deleteResult.DeletedCount : -1);
            return deleteResult.IsAcknowledged;
        }
    }
}
