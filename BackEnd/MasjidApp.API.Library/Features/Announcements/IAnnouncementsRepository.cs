using System;

namespace MasjidApp.API.Library.Features.Announcements;

public interface IAnnouncementsRepository
{
    Task<IEnumerable<Announcement>> GetAnnouncements();
    Task<AnnouncementsResponse> PostAnnouncement(Announcement announcement);
}
