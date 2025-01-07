namespace MasjidApp.API.Library.Features.PrayerTimes;

public enum PrayerTimesFileResponse
{
    Success,
    NotFoundInDatabase,
    FailedToUpdatePrayerTimesFile,
    InternalServerError,
}