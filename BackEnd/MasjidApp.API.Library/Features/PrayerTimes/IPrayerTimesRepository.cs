namespace MasjidApp.API.Library.Features.PrayerTimes;

public interface IPrayerTimesRepository
{
    Task<byte[]> GetPrayerTimes();
}