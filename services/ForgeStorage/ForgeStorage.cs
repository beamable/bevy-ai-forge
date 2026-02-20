using Beamable.Server;

namespace Beamable.Server
{
	/// <summary>
	/// This class represents the existence of the ForgeStorage database.
	/// Use it for type safe access to the database.
	/// <code>
	/// var db = await Storage.GetDatabase&lt;ForgeStorage&gt;();
	/// </code>
	/// </summary>
	[StorageObject("ForgeStorage")]
	public class ForgeStorage : MongoStorageObject
	{

	}
}
