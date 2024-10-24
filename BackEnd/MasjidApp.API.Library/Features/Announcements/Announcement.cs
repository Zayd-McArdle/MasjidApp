using System;

namespace MasjidApp.API.Library.Features.Announcements;

public sealed class Announcement
{
    public required string Title { get; init; }
    public required string Description { get; init; }
    public byte[] Image { get; init; }
    public DateOnly DatePosted { get; init; }
}
