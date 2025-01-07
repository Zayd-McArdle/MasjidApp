using MasjidApp.API.Library.Shared.DataAccess;
using MySql.Data.MySqlClient;

namespace MasjidApp.API.Library.Features.PrayerTimes;

public class PrayerTimesRepository(IDataAccessFactory dataAccessFactory) : IPrayerTimesRepository
{
    public async Task<byte[]> GetPrayerTimes()
    {
        using IDataAccess dataAccess = dataAccessFactory.EstablishDbConnection();
        byte[] prayerTimesFile = await dataAccess.ReadFirstRecordFromDatabaseAsync<byte[]>("get_prayer_times_file");
        return prayerTimesFile;
    }

    public async Task<PrayerTimesFileResponse> UpdatePrayerTimes(byte[] updatedPrayerTimesBytes)
    {
        using IDataAccess dataAccess = dataAccessFactory.EstablishDbConnection();
        await dataAccess.WriteToDatabaseAsync<dynamic>("update_prayer_times_file", new { PrayerTimesFile = updatedPrayerTimesBytes });
        byte[] prayerTimesFile = await dataAccess.ReadFirstRecordFromDatabaseAsync<byte[]>("get_prayer_times_file");
        if (prayerTimesFile == null)
        {
            return PrayerTimesFileResponse.FailedToUpdatePrayerTimesFile;
        }
        return PrayerTimesFileResponse.Success;
    }
}