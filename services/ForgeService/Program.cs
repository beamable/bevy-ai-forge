using Beamable.Server;
using System.Threading.Tasks;

namespace Beamable.ForgeService
{
	public class Program
	{
		/// <summary>
		/// The entry point for the <see cref="ForgeService"/> service.
		/// </summary>
		public static async Task Main()
		{
			// inject data from the CLI.
			await MicroserviceBootstrapper.Prepare<ForgeService>();
			
			// run the Microservice code
			await MicroserviceBootstrapper.Start<ForgeService>();
		}
	}
}
