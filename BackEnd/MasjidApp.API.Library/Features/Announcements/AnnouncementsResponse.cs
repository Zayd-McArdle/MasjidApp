using MasjidApp.API.Library.Shared.Responses;

namespace MasjidApp.API.Library.Features.Announcements;

public sealed record AnnouncementsResponse : IResponse
{
    public static AnnouncementsResponse SuccessfullyPostedAnnoucement()
    {
        return new()
        {
            IsSuccessful = true
        };
    }

    public static AnnouncementsResponse FailedToPostAnnouncement(string errorReason)
    {
        return new()
        {
            ErrorReason = errorReason
        };
    }
    public bool IsSuccessful { get; init; }
    public string ErrorReason { get; init; }
}