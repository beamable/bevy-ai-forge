﻿using Beamable.Server;
using System;
using System.IO;
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
			// inject data from the CLI
			await MicroserviceBootstrapper.Prepare<ForgeService>();
			
			// load environment variables from local file
			LoadEnvironmentVariables();
			
			// run the Microservice code
			await MicroserviceBootstrapper.Start<ForgeService>();
		}
		
		static void LoadEnvironmentVariables(string filePath=".env")
		{
			if (!File.Exists(filePath))
				throw new Exception($"No environment file found at path=[{filePath}]");

			foreach (var line in File.ReadAllLines(filePath))
			{
				var parts = line.Split(
					'=',
					StringSplitOptions.RemoveEmptyEntries);

				if (parts.Length != 2)
					continue;

				Environment.SetEnvironmentVariable(parts[0], parts[1]);
			}
		}
	}
}
