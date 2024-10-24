using System;
using System.Reflection;
using MasjidApp.API.Library.Shared.DataAccess;

namespace MasjidApp.API.Library.Features.Announcements;

public sealed class AnnouncementsRepository(string connectionString) : IAnnouncementsRepository
{
    private static async Task<AnnouncementsResponse> VerifyAnnouncementPosted(IDataAccess dataAccess, Announcement announcement)
    {
        int announcementCount = await dataAccess.ReadRecordCountFromDatabaseAsync<dynamic>("get_announcement", new { announcement.Title, announcement.Description, announcement.Image, announcement.DatePosted });
        if (announcementCount == 0)
        {
            return AnnouncementsResponse.FailedToPostAnnouncement("There was an error posting an announcement.");
        }
        return AnnouncementsResponse.SuccessfullyPostedAnnoucement();
    }
    public async Task<IEnumerable<Announcement>> GetAnnouncements()
    {
        using IDataAccess dataAccess = DataAccessFactory.EstablishDbConnnection(connectionString);
        IEnumerable<Announcement> announcements = await dataAccess.ReadRecordsFromDatabaseAsync<Announcement>("get_announcements");
        return announcements;
    }

    public async Task<AnnouncementsResponse> PostAnnouncement(Announcement announcement)
    {
        using IDataAccess dataAccess = DataAccessFactory.EstablishDbConnnection(connectionString);
        await dataAccess.WriteToDatabaseAsync<dynamic>("post_announcement", new { announcement.Title, announcement.Description, announcement.Image, announcement.DatePosted});
        return await VerifyAnnouncementPosted(dataAccess, announcement);
    }
}
