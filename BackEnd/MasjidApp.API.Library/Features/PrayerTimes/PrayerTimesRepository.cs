using MasjidApp.API.Library.Shared.DataAccess;
using MySql.Data.MySqlClient;

namespace MasjidApp.API.Library.Features.PrayerTimes;

public class PrayerTimesRepository : IPrayerTimesRepository
{

    public async Task<byte[]> GetPrayerTimes()
    {
        using IDataAccess dataAccess = DataAccessFactory.EstablishDbConnnection("");
        byte[] prayerTimesFile = await dataAccess.ReadFirstRecordFromDatabaseAsync<byte[]>("");
        return prayerTimesFile;
    }

    public async Task UpdatePrayerTimes(byte[] updatedPrayerTimesBytes)
    {
        using IDataAccess dataAccess = DataAccessFactory.EstablishDbConnnection("");
        await dataAccess.WriteToDatabaseAsync<dynamic>("", new { PrayerTimesFile = updatedPrayerTimesBytes });
    }
}