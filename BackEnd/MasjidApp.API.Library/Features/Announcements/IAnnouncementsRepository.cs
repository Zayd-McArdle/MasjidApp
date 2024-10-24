using System;

namespace MasjidApp.API.Library.Features.Announcements;

public interface IAnnouncementsRepository
{
    Task<IEnumerable<AnnouncementDto>> GetAnnouncements();
    Task<AnnouncementsResponse> PostAnnouncement(AnnouncementDto announcement);
    Task<AnnouncementsResponse> EditAnnouncement(AnnouncementDto editedAnnouncementDto);
}
