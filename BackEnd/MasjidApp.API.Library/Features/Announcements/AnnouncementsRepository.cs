using System;
using System.Reflection;
using MasjidApp.API.Library.Shared.DataAccess;

namespace MasjidApp.API.Library.Features.Announcements;

public sealed class AnnouncementsRepository : IAnnouncementsRepository
{
    public async Task<IEnumerable<Announcement>> GetAnnouncements()
    {
        using IDataAccess dataAccess = DataAccessFactory.CreateDataAccess("");
        IEnumerable<Announcement> announcements = await dataAccess.ReadRecordsFromDatabaseAsync<Announcement>("");
        return announcements;
    }

    public async Task PostAnnouncement(Announcement announcement)
    {
        using IDataAccess dataAccess = DataAccessFactory.CreateDataAccess("");
        await dataAccess.WriteToDatabaseAsync<dynamic>("", new {Title = announcement.Title, Description = announcement.Description, Image = announcement.Image, DatePosted = announcement.DatePosted});
    }
}
