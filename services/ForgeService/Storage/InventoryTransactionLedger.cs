using System;
using System.Collections.Generic;
using System.Linq;
using System.Threading.Tasks;
using MongoDB.Bson;
using MongoDB.Bson.Serialization.Attributes;
using MongoDB.Driver;

namespace ForgeService.Storage
{
    public sealed class InventoryTransactionLedger
    {
        [BsonElement("_id")] public ObjectId Id { get; set; } = ObjectId.GenerateNewId();
        public string PlayerId { get; set; }
        public string TransactionId { get; set; }
        public DateTime CreatedAtUtc { get; set; } = DateTime.UtcNow;
        public List<InventoryTransactionItem> Items { get; set; } = new();
    }

    public sealed class InventoryTransactionItem
    {
        public string ItemId { get; set; }
        public string ContentId { get; set; }
        public long ForgedTimes { get; set; }
    }

    internal static class InventoryTransactionLedgerCollection
    {
        private static IMongoCollection<InventoryTransactionLedger> _collection;

        private static async ValueTask<IMongoCollection<InventoryTransactionLedger>> Get(IMongoDatabase db)
        {
            if (_collection != null)
            {
                return _collection;
            }

            _collection = db.GetCollection<InventoryTransactionLedger>("inventory_transactions");
            await _collection.Indexes.CreateOneAsync(
                new CreateIndexModel<InventoryTransactionLedger>(
                    Builders<InventoryTransactionLedger>.IndexKeys
                        .Ascending(x => x.PlayerId)
                        .Ascending(x => x.TransactionId),
                    new CreateIndexOptions { Unique = true }));
            return _collection;
        }

        public static async Task<(InventoryTransactionLedger Ledger, bool Claimed)> ClaimAndApply(
            IMongoDatabase db,
            InventoryTransactionLedger proposed,
            IReadOnlyCollection<AiInventoryItem> creates,
            IReadOnlyCollection<string> deletes)
        {
            var ledgers = await Get(db);
            var inventory = await AiInventoryItemCollection.Get(db);
            using var session = await db.Client.StartSessionAsync();
            return await session.WithTransactionAsync(async (transactionSession, _) =>
            {
                var existing = await ledgers
                    .Find(transactionSession, x =>
                        x.PlayerId == proposed.PlayerId && x.TransactionId == proposed.TransactionId)
                    .FirstOrDefaultAsync();
                if (existing != null)
                {
                    return (existing, false);
                }

                await ledgers.InsertOneAsync(transactionSession, proposed);
                if (creates.Count > 0)
                {
                    var writes = creates.Select(item =>
                        (WriteModel<AiInventoryItem>)new ReplaceOneModel<AiInventoryItem>(
                            Builders<AiInventoryItem>.Filter.And(
                                Builders<AiInventoryItem>.Filter.Eq(x => x.GamerTag, item.GamerTag),
                                Builders<AiInventoryItem>.Filter.Eq(x => x.ItemId, item.ItemId)),
                            item)
                        {
                            IsUpsert = true
                        });
                    await inventory.BulkWriteAsync(transactionSession, writes);
                }

                if (deletes.Count > 0)
                {
                    await inventory.DeleteManyAsync(
                        transactionSession,
                        Builders<AiInventoryItem>.Filter.And(
                            Builders<AiInventoryItem>.Filter.Eq(x => x.GamerTag, proposed.PlayerId),
                            Builders<AiInventoryItem>.Filter.In(x => x.ItemId, deletes)));
                }
                return (proposed, true);
            });
        }
    }
}
