using MasjidApp.API.Library.Shared.DataAccess;
using MySql.Data.MySqlClient;

namespace MasjidApp.API.Library.Features.PrayerTimes;

public class PrayerTimesRepository : IPrayerTimesRepository
{

    public async Task<byte[]> GetPrayerTimes()
    {
        using IDataAccess dataAccess = DataAccessFactory.CreateDataAccess("");
        byte[] prayerTimesFile = await dataAccess.ReadFirstRecordFromDatabaseAsync<byte[]>("");
        return prayerTimesFile;
    }

    public async Task UpdatePrayerTimes(byte[] updatedPrayerTimesBytes)
    {
        using IDataAccess dataAccess = DataAccessFactory.CreateDataAccess("");
        await dataAccess.WriteToDatabaseAsync<dynamic>("", new { PrayerTimesFile = updatedPrayerTimesBytes });
    }
}