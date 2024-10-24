using MasjidApp.API.Library.Shared.Responses;

namespace MasjidApp.API.Library.Features.Announcements;

public enum AnnouncementsResponse
{
    FailedToPostAnnouncement,
    FailedToEditAnnouncement,
    SuccessfullyPostedAnnouncement,
    SuccessfullyEditedAnnouncement,
    AnnouncementNotFound,
    NewAnnouncementMatchesOldAnnouncement,
}